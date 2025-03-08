use crate::object::{GitObject, ObjectInner, ObjectType};
use crate::repository::Repository;
use crate::signature::{Signature, SignaturePayload};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
/// Determine whether a tag name is valid, meaning that (when prefixed with refs/tags/) that
/// it is a valid reference name, and that any additional tag name restrictions are imposed
/// (eg, it cannot start with a -).
pub fn is_valid_tag_name(tag_name: String) -> bool {
  git2::Tag::is_valid_name(&tag_name)
}

#[napi]
/// A structure to represent a git [tag][1]
/// @hideconstructor
///
/// [1]: http://git-scm.com/book/en/Git-Basics-Tagging
pub struct Tag {
  pub(crate) inner: SharedReference<Repository, git2::Tag<'static>>,
}

#[napi]
impl Tag {
  #[napi]
  /// Get the id (SHA1) of a repository tag
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Get the message of a tag
  ///
  /// Returns `null` if there is no message or if it is not valid utf8
  pub fn message(&self) -> Option<String> {
    self
      .inner
      .message_bytes()
      .and_then(|x| std::str::from_utf8(x).ok())
      .map(|x| x.to_string())
  }

  #[napi]
  /// Get the name of a tag
  ///
  /// Throws error if it is not valid utf8
  pub fn name(&self) -> crate::Result<String> {
    let name = std::str::from_utf8(self.inner.name_bytes())?.to_string();
    Ok(name)
  }

  #[napi]
  /// Recursively peel a tag until a non tag `GitObject` is found
  pub fn peel(&self) -> crate::Result<GitObject> {
    let object = self.inner.peel()?;
    Ok(GitObject {
      inner: ObjectInner::Owned(object),
    })
  }

  #[napi]
  /// Get the tagger (author) of a tag
  ///
  /// If the author is unspecified, then `null` is returned.
  pub fn tagger(&self) -> Option<Signature> {
    self.inner.tagger().and_then(|x| Signature::try_from(x).ok())
  }

  #[napi]
  /// Get the tagged object of a tag
  ///
  /// This method performs a repository lookup for the given object and
  /// returns it
  pub fn target(&self) -> crate::Result<GitObject> {
    let object = self.inner.target()?;
    Ok(GitObject {
      inner: ObjectInner::Owned(object),
    })
  }

  #[napi]
  /// Get the OID of the tagged object of a tag
  pub fn target_id(&self) -> String {
    self.inner.target_id().to_string()
  }

  #[napi]
  /// Get the ObjectType of the tagged object of a tag
  pub fn target_type(&self) -> Option<ObjectType> {
    self.inner.target_type().map(|x| x.into())
  }
}

#[napi(object)]
pub struct CreateTagOptions {
  /// Signature for tagger.
  ///
  /// If not provided, default signature of repository will be used.
  /// If there is no default signature set for the repository, an error will occur.
  pub tagger: Option<SignaturePayload>,
  pub force: Option<bool>,
}

#[napi(object)]
pub struct CreateAnnotationTagOptions {
  /// Signature for tagger.
  ///
  /// If not provided, default signature of repository will be used.
  /// If there is no default signature set for the repository, an error will occur.
  pub tagger: Option<SignaturePayload>,
}

#[napi(object)]
pub struct CreateLightweightTagOptions {
  pub force: Option<bool>,
}

#[napi]
impl Repository {
  #[napi]
  /// Lookup a tag object by prefix hash from the repository.
  ///
  /// Returns `null` if it does not exist
  pub fn find_tag(&self, this: Reference<Repository>, env: Env, oid: String) -> Option<Tag> {
    self.get_tag(this, env, oid).ok()
  }

  #[napi]
  /// Lookup a tag object by prefix hash from the repository.
  ///
  /// Throws error if it does not exist
  pub fn get_tag(&self, this: Reference<Repository>, env: Env, oid: String) -> crate::Result<Tag> {
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .find_tag_by_prefix(&oid)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Tag { inner })
  }

  #[napi]
  /// Get a list with all the tags in the repository.
  ///
  /// An optional fnmatch pattern can also be specified.
  pub fn tag_names(&self, pattern: Option<String>) -> crate::Result<Vec<String>> {
    let names = self
      .inner
      .tag_names(pattern.as_deref())?
      .iter()
      .filter_map(|x| x.map(|x| x.to_string()))
      .collect::<Vec<_>>();
    Ok(names)
  }

  #[napi(ts_args_type = "callback: (oid: string, name: string) => boolean")]
  /// iterate over all tags calling `callback` on each.
  /// the callback is provided the tag id and name
  pub fn tag_foreach(&self, callback: Function<(String, String), bool>) -> crate::Result<()> {
    self.inner.tag_foreach(|oid, name| {
      let oid = oid.to_string();
      let name = std::str::from_utf8(name).ok().map(|x| x.to_string());
      if let Some(name) = name {
        callback.call((oid, name)).unwrap_or(false)
      } else {
        false
      }
    })?;
    Ok(())
  }

  #[napi]
  /// Delete an existing tag reference.
  ///
  /// The tag name will be checked for validity, see `isValidTagName` for some rules
  /// about valid names.
  pub fn delete_tag(&self, name: String) -> crate::Result<()> {
    self.inner.tag_delete(&name)?;
    Ok(())
  }

  #[napi]
  /// Create a new tag in the repository from an object
  ///
  /// A new reference will also be created pointing to this tag object. If
  /// `force` is true and a reference already exists with the given name,
  /// it'll be replaced.
  ///
  /// The message will not be cleaned up.
  ///
  /// The tag name will be checked for validity. You must avoid the characters
  /// '~', '^', ':', ' \ ', '?', '[', and '*', and the sequences ".." and " @
  /// {" which have special meaning to revparse.
  pub fn create_tag(
    &self,
    name: String,
    target: &GitObject,
    message: String,
    options: Option<CreateTagOptions>,
  ) -> crate::Result<String> {
    let (tagger, force) = match options {
      Some(opts) => {
        let tagger = opts.tagger.and_then(|x| Signature::try_from(x).ok());
        let force = opts.force.unwrap_or_default();
        (tagger, force)
      }
      None => (None, false),
    };
    let tagger = tagger
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;
    let oid = self
      .inner
      .tag(&name, &target.inner, &tagger, &message, force)?
      .to_string();
    Ok(oid)
  }

  #[napi]
  /// Create a new tag in the repository from an object without creating a reference.
  ///
  /// The message will not be cleaned up.
  ///
  /// The tag name will be checked for validity. You must avoid the characters
  /// '~', '^', ':', ' \ ', '?', '[', and '*', and the sequences ".." and " @
  /// {" which have special meaning to revparse.
  pub fn create_annotation_tag(
    &self,
    name: String,
    target: &GitObject,
    message: String,
    options: Option<CreateAnnotationTagOptions>,
  ) -> crate::Result<String> {
    let tagger = options
      .and_then(|x| x.tagger)
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;
    let oid = self
      .inner
      .tag_annotation_create(&name, &target.inner, &tagger, &message)?
      .to_string();
    Ok(oid)
  }

  #[napi]
  /// Create a new lightweight tag pointing at a target object
  ///
  /// A new direct reference will be created pointing to this target object.
  /// If force is true and a reference already exists with the given name,
  /// it'll be replaced.
  pub fn create_lightweight_tag(
    &self,
    name: String,
    target: &GitObject,
    options: Option<CreateLightweightTagOptions>,
  ) -> crate::Result<String> {
    let oid = self
      .inner
      .tag_lightweight(&name, &target.inner, options.and_then(|x| x.force).unwrap_or_default())?
      .to_string();
    Ok(oid)
  }
}
