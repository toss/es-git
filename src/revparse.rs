use crate::repository::Repository;
use crate::util::bitflags_contain;
use napi_derive::napi;

#[napi]
#[repr(u32)]
/// Flags for the revparse.
/// - `Single` : The spec targeted a single object.
/// - `Range` : The spec targeted a range of commits.
/// - `MergeBase` : The spec used the `...` operator, which invokes special semantics.
pub enum RevparseMode {
  Single = 1,
  Range = 2,
  MergeBase = 4,
}

#[napi]
/// Check revparse mode contains specific flags.
///
/// @category Revparse
/// @signature
/// ```ts
/// function revparseModeContains(source: number, target: number): boolean;
/// ```
///
/// @param {number} source - Source flags.
/// @param {number} target - Target flags.
/// @returns Returns `true` is source flags contains target flags.
///
/// @example
/// ```ts
/// import { openRepository, revparseModeContains, RevparseMode } from 'es-git';
///
/// const repo = await openRepository('.');
/// const spec = repo.revparse('main..other');
///
/// console.assert(revparseModeContains(spec.mode, RevparseMode.Range));
/// ```
pub fn revparse_mode_contains(source: u32, target: u32) -> bool {
  bitflags_contain(
    git2::RevparseMode::from_bits_retain(source),
    git2::RevparseMode::from_bits_retain(target),
  )
}

#[napi(object)]
/// A revspec represents a range of revisions within a repository.
pub struct Revspec {
  /// Access the `from` range of this revspec.
  pub from: Option<String>,
  /// Access the `to` range of this revspec.
  pub to: Option<String>,
  /// Returns the intent of the revspec.
  pub mode: u32,
}

impl<'a> From<git2::Revspec<'a>> for Revspec {
  fn from(value: git2::Revspec<'a>) -> Self {
    Self {
      from: value.from().map(|x| x.id().to_string()),
      to: value.to().map(|x| x.id().to_string()),
      mode: value.mode().bits(),
    }
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Execute a rev-parse operation against the `spec` listed.
  ///
  /// The resulting revision specification is returned, or an error is
  /// returned if one occurs.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   revparse(spec: string): Revspec;
  /// }
  /// ```
  ///
  /// @param {string} spec - Revision string.
  /// @returns
  pub fn revparse(&self, spec: String) -> crate::Result<Revspec> {
    let revspec = self.inner.revparse(&spec)?;
    Ok(revspec.into())
  }

  #[napi]
  /// Find a single object, as specified by a revision string.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   revparseSingle(spec: string): string;
  /// }
  /// ```
  ///
  /// @param {string} spec - Revision string.
  /// @returns OID of single object.
  /// @throws Throws error if the object does not exist.
  pub fn revparse_single(&self, spec: String) -> crate::Result<String> {
    let object = self.inner.revparse_single(&spec)?;
    Ok(object.id().to_string())
  }
}
