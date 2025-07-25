use crate::repository::Repository;
use crate::tree::{Tree, TreeInner};
use napi::Env;
use napi_derive::napi;

#[napi(string_enum)]
/// - `Direct` : A reference which points at an object id.
/// - `Symbolic` : A reference which points at another reference.
pub enum ReferenceType {
  Direct,
  Symbolic,
}

impl From<ReferenceType> for git2::ReferenceType {
  fn from(value: ReferenceType) -> Self {
    match value {
      ReferenceType::Direct => git2::ReferenceType::Direct,
      ReferenceType::Symbolic => git2::ReferenceType::Symbolic,
    }
  }
}

impl From<git2::ReferenceType> for ReferenceType {
  fn from(value: git2::ReferenceType) -> Self {
    match value {
      git2::ReferenceType::Direct => ReferenceType::Direct,
      git2::ReferenceType::Symbolic => ReferenceType::Symbolic,
    }
  }
}

#[napi]
/// A class to represent a git [reference][1].
///
/// [1]: https://git-scm.com/book/en/Git-Internals-Git-References
pub struct Reference {
  pub(crate) inner: napi::bindgen_prelude::SharedReference<Repository, git2::Reference<'static>>,
}

#[napi]
/// Ensure the reference name is well-formed.
///
/// Validation is performed as if `ReferenceFormat.AllowOnelevel`
/// was given to `normalizeReferenceName`  No normalization is performed, however.
///
/// @category Reference
/// @signature
/// ```ts
/// function isValidReferenceName(refname: string): boolean;
/// ```
///
/// @param {string} refname - Reference name to check if it is valid.
/// @returns Returns `true` if reference name is valid.
///
/// @example
/// ```ts
/// import { isValidReferenceName } from 'es-git';
///
/// console.assert(isValidReferenceName("HEAD"));
/// console.assert(isValidReferenceName("refs/heads/main"));
///
/// // But:
/// console.assert(!isValidReferenceName("main"));
/// console.assert(!isValidReferenceName("refs/heads/*"));
/// console.assert(!isValidReferenceName("foo//bar"));
/// ```
pub fn is_valid_reference_name(refname: String) -> bool {
  git2::Reference::is_valid_name(&refname)
}

#[napi]
#[repr(u32)]
/// - `ReferenceFormat.Normal` : No particular normalization.
/// - `ReferenceFormat.AllowOnelevel` : Control whether one-level refname are accepted
/// (i.e., refnames that do not contain multiple `/`-separated components). Those are
/// expected to be written only using uppercase letters and underscore
/// (e.g. `HEAD`, `FETCH_HEAD`).
/// - `ReferenceFormat.RefspecPattern` : Interpret the provided name as a reference pattern
/// for a refspec (as used with remote repositories). If this option is enabled, the name
/// is allowed to contain a single `*` in place of a full pathname
/// components (e.g., `foo/*\/bar` but not `foo/bar*`).
/// - `ReferenceFormat.RefspecShorthand` : Interpret the name as part of a refspec in shorthand
/// form so the `AllowOnelevel` naming rules aren't enforced and `main` becomes a valid name.
pub enum ReferenceFormat {
  Normal = 0,
  AllowOnelevel = 1,
  RefspecPattern = 2,
  RefspecShorthand = 4,
}

impl Default for ReferenceFormat {
  fn default() -> Self {
    Self::Normal
  }
}

#[napi]
/// Normalize reference name and check validity.
///
/// This will normalize the reference name by collapsing runs of adjacent
/// slashes between name components into a single slash. It also validates
/// the name according to the following rules:
///
/// 1. If `ReferenceFormat.AllowOnelevel` is given, the name may
///    contain only capital letters and underscores, and must begin and end
///    with a letter. (e.g. "HEAD", "ORIG_HEAD").
/// 2. The flag `ReferenceFormat.RefspecShorthand` has an effect
///    only when combined with `ReferenceFormat.AllowOnelevel`. If
///    it is given, "shorthand" branch names (i.e. those not prefixed by
///    `refs/`, but consisting of a single word without `/` separators)
///    become valid. For example, "main" would be accepted.
/// 3. If `ReferenceFormat.RefspecPattern` is given, the name may
///    contain a single `*` in place of a full pathname component (e.g.
///    `foo/*\/bar`, `foo/bar*`).
/// 4. Names prefixed with "refs/" can be almost anything. You must avoid
///    the characters '~', '^', ':', '\\', '?', '[', and '*', and the
///    sequences ".." and "@{" which have special meaning to revparse.
///
/// @category Reference
/// @signature
/// ```ts
/// function normalizeReferenceName(refname: string, format?: number | null | undefined): string | null;
/// ```
///
/// @param {string} refname - Reference name to normalize.
/// @param {number} [format] - Reference format flags which used for normalize.
///
/// @returns If the reference passes validation, it is returned in normalized form,
/// otherwise an `null` is returned.
///
/// @example
/// ```ts
/// import { normalizeReferenceName, ReferenceFormat } from 'es-git';
///
/// console.assert(
///   normalizeReferenceName('foo//bar'),
///   'foo/bar'
/// );
/// console.assert(
///   normalizeReferenceName(
///     'HEAD',
///     ReferenceFormat.AllowOnelevel
///   ),
///   'HEAD'
/// );
/// console.assert(
///   normalizeReferenceName(
///     'refs/heads/*',
///     ReferenceFormat.RefspecPattern
///   ),
///   'refs/heads/*'
/// );
/// console.assert(
///   normalizeReferenceName(
///     'main',
///     ReferenceFormat.AllowOnelevel | ReferenceFormat.RefspecShorthand
///   ),
///   'main'
/// );
/// ```
pub fn normalize_reference_name(refname: String, format: Option<u32>) -> Option<String> {
  let ref_format = git2::ReferenceFormat::from_bits_truncate(format.unwrap_or(ReferenceFormat::default() as u32));
  git2::Reference::normalize_name(&refname, ref_format).ok()
}

#[napi(object)]
pub struct RenameReferenceOptions {
  /// If the force flag is not enabled, and there's already a reference with
  /// the given name, the renaming will fail.
  pub force: Option<bool>,
  pub log_message: Option<String>,
}

#[napi]
impl Reference {
  #[napi]
  /// Delete an existing reference.
  ///
  /// This method works for both direct and symbolic references. The reference
  /// will be immediately removed on disk.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   delete(): void;
  /// }
  /// ```
  ///
  /// @throws This method will throws an error if the reference has changed from the
  /// time it was looked up.
  pub fn delete(&mut self) -> crate::Result<()> {
    self.inner.delete()?;
    Ok(())
  }

  #[napi]
  /// Check if a reference is a local branch.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   isBranch(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if a reference is a local branch.
  pub fn is_branch(&self) -> bool {
    self.inner.is_branch()
  }

  #[napi]
  /// Check if a reference is a note.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   isNote(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if a reference is a note.
  pub fn is_note(&self) -> bool {
    self.inner.is_note()
  }

  #[napi]
  /// Check if a reference is a remote tracking branch.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   isRemote(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if a reference is a remote tracking branch.
  pub fn is_remote(&self) -> bool {
    self.inner.is_remote()
  }

  #[napi]
  /// Check if a reference is a tag.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   isTag(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if a reference is a tag.
  pub fn is_tag(&self) -> bool {
    self.inner.is_tag()
  }

  #[napi(js_name = "type")]
  /// Get the reference type of a reference.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   type(): ReferenceType | null;
  /// }
  /// ```
  ///
  /// @returns Returns `null` if the type is unknown.
  pub fn kind(&self) -> Option<ReferenceType> {
    self.inner.kind().map(|x| x.into())
  }

  #[napi]
  /// Get the full name of a reference.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   name(): string;
  /// }
  /// ```
  ///
  /// @returns Full name of a reference.
  /// @throws Throws error if the name is not valid utf-8.
  pub fn name(&self) -> crate::Result<String> {
    let name = std::str::from_utf8(self.inner.name_bytes())?.to_string();
    Ok(name)
  }

  #[napi]
  /// Get the full shorthand of a reference.
  ///
  /// This will transform the reference name into a name "human-readable"
  /// version. If no shortname is appropriate, it will return the full name.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   shorthand(): string;
  /// }
  /// ```
  ///
  /// @returns Full shorthand of a reference.
  /// @throws Throws error if the shorthand is not valid utf-8.
  pub fn shorthand(&self) -> crate::Result<String> {
    let shorthand = std::str::from_utf8(self.inner.shorthand_bytes())?.to_string();
    Ok(shorthand)
  }

  #[napi]
  /// Get the OID pointed to by a direct reference.
  ///
  /// Only available if the reference is direct (i.e. an object id reference,
  /// not a symbolic one).
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   target(): string | null;
  /// }
  /// ```
  ///
  /// @returns OID pointed to by a direct reference.
  pub fn target(&self) -> Option<String> {
    self.inner.target().map(|x| x.to_string())
  }

  #[napi]
  /// Return the peeled OID target of this reference.
  ///
  /// This peeled OID only applies to direct references that point to a hard.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   targetPeel(): string | null;
  /// }
  /// ```
  ///
  /// @returns Peeled OID of this reference.
  pub fn target_peel(&self) -> Option<String> {
    self.inner.target_peel().map(|x| x.to_string())
  }

  #[napi]
  /// Peel a reference to a tree.
  ///
  /// This method recursively peels the reference until it reaches
  /// a tree.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   peelToTree(): Tree;
  /// }
  /// ```
  ///
  /// @returns Peeled `Tree` of this reference.
  pub fn peel_to_tree(&self, this: napi::bindgen_prelude::Reference<Reference>, env: Env) -> crate::Result<Tree> {
    Ok(Tree {
      inner: TreeInner::Reference(this.share_with(env, |reference| {
        reference
          .inner
          .peel_to_tree()
          .map_err(crate::Error::from)
          .map_err(|e| e.into())
      })?),
    })
  }

  #[napi]
  /// Get full name to the reference pointed to by a symbolic reference.
  ///
  /// Only available if the reference is symbolic.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   symbolicTarget(): string | null;
  /// }
  /// ```
  ///
  /// @returns Full name of the reference pointed to by a symbolic reference.
  pub fn symbolic_target(&self) -> crate::Result<Option<String>> {
    match self.inner.symbolic_target_bytes() {
      Some(bytes) => Ok(Some(std::str::from_utf8(bytes)?.to_string())),
      None => Ok(None),
    }
  }

  #[napi]
  /// Resolve a symbolic reference to a direct reference.
  ///
  /// This method iteratively peels a symbolic reference until it resolves to
  /// a direct reference to an OID.
  ///
  /// If a direct reference is passed as an argument, a copy of that
  /// reference is returned.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   resolve(): Reference;
  /// }
  /// ```
  ///
  /// @returns Resolved reference.
  pub fn resolve(&self, env: Env) -> crate::Result<Reference> {
    let inner = self.inner.clone(env)?.share_with(env, |reference| {
      reference.resolve().map_err(crate::Error::from).map_err(|e| e.into())
    })?;
    Ok(Self { inner })
  }

  #[napi]
  /// Rename an existing reference.
  ///
  /// This method works for both direct and symbolic references.
  ///
  /// @category Reference/Methods
  /// @signature
  /// ```ts
  /// class Reference {
  ///   rename(newName: string, options?: RenameReferenceOptions | null | undefined): Reference;
  /// }
  /// ```
  ///
  /// @param {string} newName - Name to rename an existing reference.
  /// @param {RenameReferenceOptions} [options] - Options to rename an existing reference.
  /// @returns Renamed reference.
  pub fn rename(
    &mut self,
    env: Env,
    new_name: String,
    options: Option<RenameReferenceOptions>,
  ) -> crate::Result<Reference> {
    let inner = self.inner.clone(env)?.share_with(env, |reference| {
      let (force, msg) = options.map(|x| (x.force, x.log_message)).unwrap_or((None, None));
      reference
        .rename(
          &new_name,
          force.unwrap_or_default(),
          &msg.unwrap_or(format!("Renaming reference into {new_name}")),
        )
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Self { inner })
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Lookup a reference to one of the objects in a repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findReference(name: string): Reference | null;
  /// }
  /// ```
  ///
  /// @param {string} name - Reference name to lookup.
  /// @returns Returns `null` if the reference does not exist.
  ///
  /// @example
  ///
  /// Get `HEAD` reference from the repository.
  ///
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const reference = repo.findReference('HEAD');
  /// ```
  pub fn find_reference(
    &self,
    this: napi::bindgen_prelude::Reference<Repository>,
    env: Env,
    name: String,
  ) -> Option<Reference> {
    self.get_reference(this, env, name).ok()
  }

  #[napi]
  /// Lookup a reference to one of the objects in a repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getReference(name: string): Reference;
  /// }
  /// ```
  ///
  /// @param {string} name - Reference name to lookup.
  /// @returns Git reference.
  /// @throws Throws error if the reference does not exist.
  ///
  /// @example
  ///
  /// Get `HEAD` reference from the repository.
  ///
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const reference = repo.getReference('HEAD');
  /// ```
  pub fn get_reference(
    &self,
    this: napi::bindgen_prelude::Reference<Repository>,
    env: Env,
    name: String,
  ) -> crate::Result<Reference> {
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .find_reference(&name)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Reference { inner })
  }
}
