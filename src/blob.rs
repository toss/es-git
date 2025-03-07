use crate::object::GitObject;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;

pub(crate) enum BlobInner {
  GitObject(SharedReference<GitObject, git2::Blob<'static>>),
}

impl Deref for BlobInner {
  type Target = git2::Blob<'static>;

  fn deref(&self) -> &git2::Blob<'static> {
    match self {
      BlobInner::GitObject(inner) => inner.deref(),
    }
  }
}

/// A structure to represent a git [blob][1]
/// @hideconstructor
///
/// [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects
#[napi]
pub struct Blob {
  pub(crate) inner: BlobInner,
}

#[napi]
impl Blob {
  #[napi]
  /// Get the id (SHA1) of a repository blob
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Determine if the blob content is most certainly binary or not.
  pub fn is_binary(&self) -> bool {
    self.inner.is_binary()
  }

  #[napi]
  /// Get the content of this blob.
  pub fn content(&self) -> Uint8Array {
    self.inner.content().to_vec().into()
  }

  #[napi]
  /// Get the size in bytes of the contents of this blob.
  pub fn size(&self) -> u64 {
    self.inner.size() as u64
  }
}
