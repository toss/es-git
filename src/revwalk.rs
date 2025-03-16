use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[repr(u32)]
pub enum RevwalkSort {
  None = 0,
  Topological = 1,
  Time = 2,
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
  ///
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   reset(): this;
  /// }
  /// ```
  pub fn reset(&mut self) -> Result<&Self> {
    self.inner.reset().map_err(crate::Error::from).map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Set the order in which commits are visited.
  ///
  /// @category Revwalk/Methods
  ///
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   setSorting(sort: number): this;
  /// }
  /// ```
  ///
  /// @param {number} sort - Orderings that may be specified for Revwalk iteration.
  /// - `RevwalkSort.None` : Sort the repository contents in no particular ordering.
  /// This sorting is arbitrary, implementation-specific, and subject to
  /// change at any time. This is the default sorting for new walkers.
  /// - `RevwalkSort.Topological` : Sort the repository contents in topological order
  /// (children before parents).
  /// This sorting mode can be combined with time sorting.
  /// - `RevwalkSort.Time` : Sort the repository contents by commit time.
  /// This sorting mode can be combined with topological sorting.
  /// - `RevwalkSort.Reverse` : Iterate through the repository contents in reverse order.
  /// This sorting mode can be combined with any others.
  ///
  /// @example
  /// ```ts
  /// import { openRepository, RevwalkSort } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const revwalk = repo.revwalk();
  /// revwalk.setSorting(RevwalkSort.Time | RevwalkSort.Reverse);
  /// ```
  pub fn set_sorting(&mut self, sort: u32) -> Result<&Self> {
    self
      .inner
      .set_sorting(git2::Sort::from_bits_truncate(sort))
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Simplify the history by first-parent.
  ///
  /// No parents other than the first for each commit will be enqueued.
  ///
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   simplifyFirstParent(): this;
  /// }
  /// ```
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
  ///
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   push(oid: string): this;
  /// }
  /// ```
  ///
  /// @param {string} oid - OID which belong to a commitish on the walked repository.
  pub fn push(&mut self, oid: String) -> Result<&Self> {
    let oid = git2::Oid::from_str(&oid)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    self.inner.push(oid).map_err(crate::Error::from).map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Push the repository's HEAD.
  ///
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   pushHead(): this;
  /// }
  /// ```
  pub fn push_head(&mut self) -> Result<&Self> {
    self
      .inner
      .push_head()
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Push matching references.
  ///
  /// The OIDs pointed to by the references that match the given glob pattern
  /// will be pushed to the revision walker.
  ///
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   pushGlob(glob: string): this;
  /// }
  /// ```
  ///
  /// @param {string} glob - A leading 'refs/' is implied if not present as well as a trailing `/ \
  /// *` if the glob lacks '?', ' \ *' or '['.
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
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   pushRange(range: string): this;
  /// }
  /// ```
  ///
  /// @param {string} range - The range should be of the form `<commit>..<commit>` where each
  /// `<commit>` is in the form accepted by `revparseSingle`. The left-hand
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
  /// Push the OID pointed to by a reference.
  ///
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   pushRef(reference: string): this;
  /// }
  /// ```
  ///
  /// @param {string} reference - The reference must point to a commitish.
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
  ///
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   hide(oid: string): this;
  /// }
  /// ```
  ///
  /// @param {string} oid - Marked commit OID as not of interest of this revwalk.
  pub fn hide(&mut self, oid: String) -> Result<&Self> {
    let oid = git2::Oid::from_str(&oid)
      .map_err(crate::Error::from)
      .map_err(Error::from)?;
    self.inner.hide(oid).map_err(crate::Error::from).map_err(Error::from)?;
    Ok(self)
  }

  #[napi]
  /// Hide the repository's HEAD.
  ///
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   hideHead(): this;
  /// }
  /// ```
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
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   hideGlob(glob: string): this;
  /// }
  /// ```
  ///
  /// @param {string} glob - A leading 'refs/' is implied if not present as well as a trailing `/ \
  /// *` if the glob lacks '?', ' \ *' or '['.
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
  /// @category Revwalk/Methods
  /// @signature
  /// ```ts
  /// class Revwalk {
  ///   hideRef(reference: string): this;
  /// }
  /// ```
  ///
  /// @param {string} reference - The reference must point to a commitish.
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
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   revwalk(): Revwalk;
  /// }
  /// ```
  ///
  /// @returns Revwalk to traverse the commit graph in this repository.
  pub fn revwalk(&self, this: Reference<Repository>, env: Env) -> crate::Result<Revwalk> {
    let inner = this.share_with(env, |repo| {
      repo.inner.revwalk().map_err(crate::Error::from).map_err(|e| e.into())
    })?;
    Ok(Revwalk { inner })
  }
}
