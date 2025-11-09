use crate::object::{GitObject, ObjectInner, ObjectType};
use crate::repository::Repository;
use crate::signature::{Signature, SignaturePayload};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
/// Determine whether a tag name is valid, meaning that (when prefixed with refs/tags/) that
/// it is a valid reference name, and that any additional tag name restrictions are imposed
/// (eg, it cannot start with a -).
///
/// @category Tag
/// @signature
/// ```ts
/// function isValidTagName(tagName: string): boolean;
/// ```
///
/// @param {string} tagName - Tag name to check if it is valid.
/// @returns Returns `true` if tag name is valid.
pub fn is_valid_tag_name(tag_name: String) -> bool {
  git2::Tag::is_valid_name(&tag_name)
}

#[napi]
/// A class to represent a git [tag][1].
///
/// [1]: https://git-scm.com/book/en/Git-Basics-Tagging
pub struct Tag {
  pub(crate) inner: SharedReference<Repository, git2::Tag<'static>>,
}

#[napi]
impl Tag {
  #[napi]
  /// Get the id (SHA1) of a repository tag.
  ///
  /// @category Tag/Methods
  /// @signature
  /// ```ts
  /// class Tag {
  ///   id(): string;
  /// }
  /// ```
  ///
  /// @returns ID(SHA1) of this tag.
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Get the message of a tag.
  ///
  /// @category Tag/Methods
  /// @signature
  /// ```ts
  /// class Tag {
  ///   message(): string | null;
  /// }
  /// ```
  ///
  /// @returns Returns `null` if there is no message or if it is not valid utf8.
  pub fn message(&self) -> Option<String> {
    self
      .inner
      .message_bytes()
      .and_then(|x| std::str::from_utf8(x).ok())
      .map(|x| x.to_string())
  }

  #[napi]
  /// Get the name of a tag.
  ///
  /// @category Tag/Methods
  /// @signature
  /// ```ts
  /// class Tag {
  ///   name(): string;
  /// }
  /// ```
  ///
  /// @returns Name of tag.
  /// @throws Throws error if it is not valid utf8.
  pub fn name(&self) -> crate::Result<String> {
    let name = std::str::from_utf8(self.inner.name_bytes())?.to_string();
    Ok(name)
  }

  #[napi]
  /// Recursively peel a tag until a non tag Git object is found.
  ///
  /// @category Tag/Methods
  /// @signature
  /// ```ts
  /// class Tag {
  ///   peel(): GitObject;
  /// }
  /// ```
  ///
  /// @returns Git object for this tag.
  pub fn peel(&self) -> crate::Result<GitObject> {
    let object = self.inner.peel()?;
    Ok(GitObject {
      inner: ObjectInner::Owned(object),
    })
  }

  #[napi]
  /// Get the tagger (author) of a tag.
  ///
  /// @category Tag/Methods
  /// @signature
  /// ```ts
  /// class Tag {
  ///   tagger(): Signature | null;
  /// }
  /// ```
  ///
  /// @returns If the author is unspecified, then `null` is returned.
  pub fn tagger(&self) -> Option<Signature> {
    self.inner.tagger().and_then(|x| Signature::try_from(x).ok())
  }

  #[napi]
  /// Get the tagged object of a tag.
  ///
  /// This method performs a repository lookup for the given object and
  /// returns it.
  ///
  /// @category Tag/Methods
  /// @signature
  /// ```ts
  /// class Tag {
  ///   target(): GitObject;
  /// }
  /// ```
  ///
  /// @returns Tagged git object of a tag.
  pub fn target(&self) -> crate::Result<GitObject> {
    let object = self.inner.target()?;
    Ok(GitObject {
      inner: ObjectInner::Owned(object),
    })
  }

  #[napi]
  /// Get the OID of the tagged object of a tag.
  ///
  /// @category Tag/Methods
  /// @signature
  /// ```ts
  /// class Tag {
  ///   targetId(): string;
  /// }
  /// ```
  ///
  /// @returns OID of the tagged object of a tag.
  pub fn target_id(&self) -> String {
    self.inner.target_id().to_string()
  }

  #[napi]
  /// Get the ObjectType of the tagged object of a tag.
  ///
  /// @category Tag/Methods
  /// @signature
  /// ```ts
  /// class Tag {
  ///   targetType(): ObjectType | null;
  /// }
  /// ```
  ///
  /// @returns ObjectType of the tagged object of a tag.
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
  /// If `force` is true and a reference already exists with the given name, it'll be replaced.
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
  /// If `force` is true and a reference already exists with the given name, it'll be replaced.
  pub force: Option<bool>,
}

#[napi]
impl Repository {
  #[napi]
  /// Lookup a tag object by prefix hash from the repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findTag(oid: string): Tag | null;
  /// }
  /// ```
  ///
  /// @param {string} oid - Prefix hash.
  /// @returns Returns `null` if tag does not exist.
  pub fn find_tag(&self, this: Reference<Repository>, env: Env, oid: String) -> Option<Tag> {
    self.get_tag(this, env, oid).ok()
  }

  #[napi]
  /// Lookup a tag object by prefix hash from the repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getTag(oid: string): Tag;
  /// }
  /// ```
  ///
  /// @param {string} oid - Prefix hash.
  /// @throws Throws error if tag does not exist.
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
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   tagNames(pattern?: string | null | undefined): string[];
  /// }
  /// ```
  ///
  /// @param {string} [pattern] - An optional fnmatch pattern can also be specified.
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
  /// Iterate over all tags calling `callback` on each.
  /// The callback is provided the tag id and name.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   tagForeach(callback: (oid: string, name: string) => boolean): void;
  /// }
  /// ```
  ///
  /// @param {(oid: string, name: string) => boolean} callback - If you wish to stop iteration,
  /// return `false` in the callback.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const tags = [];
  /// repo.tagForeach((sha, name) => {
  ///   tags.push([name, sha]);
  ///   return true;
  /// });
  ///
  /// console.log(tags);
  /// // [['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
  /// //  ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1'],
  /// //  ['567aa5c6b219312dc7758ab88ebb7a1e5d36d26b', 'refs/tags/v2']]
  /// ```
  pub fn tag_foreach(&self, callback: Function<FnArgs<(String, String)>, bool>) -> crate::Result<()> {
    self.inner.tag_foreach(|oid, name| {
      let oid = oid.to_string();
      let name = std::str::from_utf8(name).ok().map(|x| x.to_string());
      if let Some(name) = name {
        callback.call((oid, name).into()).unwrap_or(false)
      } else {
        false
      }
    })?;
    Ok(())
  }

  #[napi]
  /// Delete an existing tag reference.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   deleteTag(name: string): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The tag name will be checked for validity, see `isValidTagName`
  /// for some rules about valid names.
  pub fn delete_tag(&self, name: String) -> crate::Result<()> {
    self.inner.tag_delete(&name)?;
    Ok(())
  }

  #[napi]
  /// Create a new tag in the repository from an object.
  ///
  /// A new reference will also be created pointing to this tag object.
  ///
  /// The message will not be cleaned up.
  ///
  /// The tag name will be checked for validity. You must avoid the characters
  /// '~', '^', ':', ' \ ', '?', '[', and '*', and the sequences ".." and " @
  /// {" which have special meaning to revparse.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   createTag(
  ///     name: string,
  ///     target: GitObject,
  ///     message: string,
  ///     options?: CreateTagOptions | null | undefined,
  ///   ): string;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of tag.
  /// @param {GitObject} target - Git object to pointed by this tag.
  /// @param {string} message - The message of tag.
  /// @param {CreateTagOptions} [options] - Options for creating the tag.
  ///
  /// @returns Tag OID(SHA1) which created.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const commit = repo.getCommit('828954df9f08dc8e172447cdacf0ddea1adf9e63');
  ///
  /// const sha = repo.createTag(
  ///   'mytag',
  ///   commit.asObject(),
  ///   'this is my tag message',
  ///   {
  ///     tagger: {
  ///       name: 'Seokju Na',
  ///       email: 'seokju.me@toss.im',
  ///     },
  ///   },
  /// );
  /// const tag = repo.getTag(sha);
  /// console.log(tag.name()); // "mytag"
  /// console.log(tag.target().id()); // "828954df9f08dc8e172447cdacf0ddea1adf9e63"
  /// ```
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
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   createAnnotationTag(
  ///     name: string,
  ///     target: GitObject,
  ///     message: string,
  ///     options?: CreateAnnotationTagOptions | null | undefined,
  ///   ): string;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of tag.
  /// @param {GitObject} target - Git object to pointed by this tag.
  /// @param {string} message - The message of tag.
  /// @param {CreateAnnotationTagOptions} [options] - Options for creating the tag.
  ///
  /// @returns Tag OID(SHA1) which created.
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
  /// Create a new lightweight tag pointing at a target object.
  ///
  /// A new direct reference will be created pointing to this target object.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   createLightweightTag(
  ///     name: string,
  ///     target: GitObject,
  ///     options?: CreateLightweightTagOptions | null | undefined,
  ///   ): string;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of tag.
  /// @param {GitObject} target - Git object to pointed by this tag.
  /// @param {CreateLightweightTagOptions} [options] - Options for creating the tag.
  ///
  /// @returns Tag OID(SHA1) which created.
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
