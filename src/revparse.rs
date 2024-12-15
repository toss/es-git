use crate::repository::Repository;
use crate::util::bitflags_contain;
use napi_derive::napi;

#[napi]
pub struct RevparseMode;

#[napi]
impl RevparseMode {
  #[napi(js_name = "Single")]
  pub fn single() -> u32 {
    git2::RevparseMode::SINGLE.bits()
  }

  #[napi(js_name = "Range")]
  pub fn range() -> u32 {
    git2::RevparseMode::RANGE.bits()
  }

  #[napi(js_name = "MergeBase")]
  pub fn merge_base() -> u32 {
    git2::RevparseMode::MERGE_BASE.bits()
  }
}

#[napi]
/// Check revparse mode contains specific flags.
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
  pub fn revparse(&self, spec: String) -> crate::Result<Revspec> {
    let revspec = self.inner.revparse(&spec)?;
    Ok(revspec.into())
  }

  #[napi]
  /// Find a single object, as specified by a revision string.
  pub fn revparse_single(&self, spec: String) -> crate::Result<String> {
    let object = self.inner.revparse_single(&spec)?;
    Ok(object.id().to_string())
  }
}
