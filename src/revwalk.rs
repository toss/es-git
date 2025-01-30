use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[repr(u32)]
/// Orderings that may be specified for Revwalk iteration.
pub enum RevwalkSort {
  /// Sort the repository contents in no particular ordering.
  ///
  /// This sorting is arbitrary, implementation-specific, and subject to
  /// change at any time. This is the default sorting for new walkers.
  None = 0,
  /// Sort the repository contents in topological order (children before
  /// parents).
  ///
  /// This sorting mode can be combined with time sorting.
  /// (1 << 0)
  Topological = 1,
  /// Sort the repository contents by commit time.
  ///
  /// This sorting mode can be combined with topological sorting.
  /// (1 << 1)
  Time = 2,
  /// Iterate through the repository contents in reverse order.
  ///
  /// This sorting mode can be combined with any others.
  /// (1 << 2)
  Reverse = 4,
}

#[napi(iterator)]
/// A revwalk allows traversal of the commit graph defined by including one or
/// more leaves and excluding one or more roots.
pub struct Revwalk {
  pub(crate) inner: SharedReference<Repository, git2::Revwalk<'static>>,
}

#[napi]
impl Generator for Revwalk {
  type Yield = String;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    self.inner.next().and_then(|x| x.ok().map(|x| x.to_string()))
  }
}

#[napi]
impl Revwalk {
  #[napi]
  /// Reset a revwalk to allow re-configuring it.
  ///
  /// The revwalk is automatically reset when iteration of its commits
  /// completes.
  pub fn reset(&mut self) -> Result<&Self> {
    self.inner.reset().map_err(crate::Error::from).map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Set the order in which commits are visited.
  pub fn set_sorting(&mut self, sort: u32) -> Result<&Self> {
    self
      .inner
      .set_sorting(git2::Sort::from_bits_truncate(sort))
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Simplify the history by first-parent
  ///
  /// No parents other than the first for each commit will be enqueued.
  pub fn simplify_first_parent(&mut self) -> Result<&Self> {
    self
      .inner
      .simplify_first_parent()
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Mark a commit to start traversal from.
  ///
  /// The given OID must belong to a commitish on the walked repository.
  ///
  /// The given commit will be used as one of the roots when starting the
  /// revision walk. At least one commit must be pushed onto the walker before
  /// a walk can be started.
  pub fn push(&mut self, oid: String) -> Result<&Self> {
    let oid = git2::Oid::from_str(&oid)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    self.inner.push(oid).map_err(crate::Error::from).map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Push the repository's HEAD
  ///
  /// For more information, see `push`.
  pub fn push_head(&mut self) -> Result<&Self> {
    self
      .inner
      .push_head()
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Push matching references
  ///
  /// The OIDs pointed to by the references that match the given glob pattern
  /// will be pushed to the revision walker.
  ///
  /// A leading 'refs/' is implied if not present as well as a trailing `/ \
  /// *` if the glob lacks '?', ' \ *' or '['.
  ///
  /// Any references matching this glob which do not point to a commitish
  /// will be ignored.
  pub fn push_glob(&mut self, glob: String) -> Result<&Self> {
    self
      .inner
      .push_glob(&glob)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Push and hide the respective endpoints of the given range.
  ///
  /// The range should be of the form `<commit>..<commit>` where each
  /// `<commit>` is in the form accepted by `revparse_single`. The left-hand
  /// commit will be hidden and the right-hand commit pushed.
  pub fn push_range(&mut self, range: String) -> Result<&Self> {
    self
      .inner
      .push_range(&range)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Push the OID pointed to by a reference
  ///
  /// The reference must point to a commitish.
  pub fn push_ref(&mut self, reference: String) -> Result<&Self> {
    self
      .inner
      .push_ref(&reference)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Mark a commit as not of interest to this revwalk.
  pub fn hide(&mut self, oid: String) -> Result<&Self> {
    let oid = git2::Oid::from_str(&oid)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    self.inner.hide(oid).map_err(crate::Error::from).map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Hide the repository's HEAD
  ///
  /// For more information, see `hide`.
  pub fn hide_head(&mut self) -> Result<&Self> {
    self
      .inner
      .hide_head()
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Hide matching references.
  ///
  /// The OIDs pointed to by the references that match the given glob pattern
  /// and their ancestors will be hidden from the output on the revision walk.
  ///
  /// A leading 'refs/' is implied if not present as well as a trailing `/ \
  /// *` if the glob lacks '?', ' \ *' or '['.
  ///
  /// Any references matching this glob which do not point to a commitish
  /// will be ignored.
  pub fn hide_glob(&mut self, glob: String) -> Result<&Self> {
    self
      .inner
      .hide_glob(&glob)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Hide the OID pointed to by a reference.
  ///
  /// The reference must point to a commitish.
  pub fn hide_ref(&mut self, reference: String) -> Result<&Self> {
    self
      .inner
      .hide_ref(&reference)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Create a revwalk that can be used to traverse the commit graph.
  pub fn revwalk(&self, this: Reference<Repository>, env: Env) -> crate::Result<Revwalk> {
    let inner = this.share_with(env, |repo| {
      repo.inner.revwalk().map_err(crate::Error::from).map_err(|e| e.into())
    })?;
    Ok(Revwalk { inner })
  }
}
