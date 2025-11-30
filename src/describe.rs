use crate::object::GitObject;
use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;

pub(crate) enum DescribeInner {
  Repo(SharedReference<Repository, git2::Describe<'static>>),
  Object(SharedReference<GitObject, git2::Describe<'static>>),
}

impl Deref for DescribeInner {
  type Target = git2::Describe<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(repo) => repo.deref(),
      Self::Object(obj) => obj.deref(),
    }
  }
}

#[napi]
/// The result of a `describe` operation on either an `Describe` or a
/// `Repository`.
pub struct Describe {
  pub(crate) inner: DescribeInner,
}

#[napi(object)]
pub struct DescribeFormatOptions {
  /// Sets the size of the abbreviated commit id to use.
  ///
  /// The value is the lower bound for the length of the abbreviated string,
  /// and the default is 7.
  pub abbreviated_size: Option<u32>,
  /// Sets whether or not the long format is used even when a shorter name
  /// could be used.
  pub always_use_long_format: Option<bool>,
  /// If the workdir is dirty and this is set, this string will be appended to
  /// the description string.
  pub dirty_suffix: Option<String>,
}

impl From<DescribeFormatOptions> for git2::DescribeFormatOptions {
  fn from(value: DescribeFormatOptions) -> Self {
    let mut opts = git2::DescribeFormatOptions::new();
    if let Some(size) = value.abbreviated_size {
      opts.abbreviated_size(size);
    }
    if let Some(long) = value.always_use_long_format {
      opts.always_use_long_format(long);
    }
    if let Some(ref suffix) = value.dirty_suffix {
      opts.dirty_suffix(suffix);
    }
    opts
  }
}

#[napi]
impl Describe {
  #[napi]
  /// Prints this describe result, returning the result as a string.
  ///
  /// @category Describe/Methods
  /// @signature
  /// ```ts
  /// class Describe {
  ///   format(options?: DescribeFormatOptions | null | undefined): string;
  /// }
  /// ```
  ///
  /// @param {DescribeFormatOptions} [options] - Options for formatting describe.
  /// @returns Formatted string for this describe.
  pub fn format(&self, options: Option<DescribeFormatOptions>) -> crate::Result<String> {
    let opts = options.map(git2::DescribeFormatOptions::from);
    let str = self.inner.format(opts.as_ref())?;
    Ok(str)
  }
}

#[napi(object)]
pub struct DescribeOptions {
  pub max_candidates_tags: Option<u32>,
  /// Sets the reference lookup strategy
  ///
  /// This behaves like the `--tags` option to git-describe.
  pub describe_tags: Option<bool>,
  /// Sets the reference lookup strategy
  ///
  /// This behaves like the `--all` option to git-describe.
  pub describe_all: Option<bool>,
  /// Indicates when calculating the distance from the matching tag or
  /// reference whether to only walk down the first-parent ancestry.
  pub only_follow_first_parent: Option<bool>,
  /// If no matching tag or reference is found whether a describe option would
  /// normally fail. This option indicates, however, that it will instead fall
  /// back to showing the full id of the commit.
  pub show_commit_oid_as_fallback: Option<bool>,
  pub pattern: Option<String>,
}

impl From<DescribeOptions> for git2::DescribeOptions {
  fn from(value: DescribeOptions) -> Self {
    let mut opts = git2::DescribeOptions::new();
    if let Some(max) = value.max_candidates_tags {
      opts.max_candidates_tags(max);
    }
    if let Some(true) = value.describe_tags {
      opts.describe_tags();
    }
    if let Some(true) = value.describe_all {
      opts.describe_all();
    }
    if let Some(follow) = value.only_follow_first_parent {
      opts.only_follow_first_parent(follow);
    }
    if let Some(show) = value.show_commit_oid_as_fallback {
      opts.show_commit_oid_as_fallback(show);
    }
    if let Some(ref pattern) = value.pattern {
      opts.pattern(pattern);
    }
    opts
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Describes a commit
  ///
  /// Performs a describe operation on the current commit and the worktree.
  /// After performing a describe on `HEAD`, a status is run and description is
  /// considered to be dirty if there are.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   describe(options?: DescribeOptions | null | undefined): Describe;
  /// }
  /// ```
  ///
  /// @param {DescribeOptions} [options] - Options for describe operation.
  /// @returns Instance of describe.
  pub fn describe(
    &self,
    this: Reference<Repository>,
    env: Env,
    options: Option<DescribeOptions>,
  ) -> crate::Result<Describe> {
    let opts = options.map_or_else(git2::DescribeOptions::new, git2::DescribeOptions::from);
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .describe(&opts)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Describe {
      inner: DescribeInner::Repo(inner),
    })
  }
}

#[napi]
impl GitObject {
  #[napi]
  /// Describes a commit
  ///
  /// Performs a describe operation on this commitish object.
  ///
  /// @category GitObject/Methods
  /// @signature
  /// ```ts
  /// class Object {
  ///   describe(options?: DescribeOptions | null | undefined): Describe;
  /// }
  /// ```
  ///
  /// @param {DescribeOptions} [options] - Options for describe operation.
  /// @returns Instance of describe.
  pub fn describe(
    &self,
    this: Reference<GitObject>,
    env: Env,
    options: Option<DescribeOptions>,
  ) -> crate::Result<Describe> {
    let opts = options.map_or_else(git2::DescribeOptions::new, git2::DescribeOptions::from);
    let inner = this.share_with(env, |obj| {
      obj
        .inner
        .describe(&opts)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Describe {
      inner: DescribeInner::Object(inner),
    })
  }
}
