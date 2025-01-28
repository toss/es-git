#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Git2(#[from] git2::Error),
  #[error(transparent)]
  Napi(#[from] napi::Error),
  #[error(transparent)]
  Utf8Error(#[from] std::str::Utf8Error),
  #[error("invalid time")]
  InvalidTime,
  #[error("signature not found")]
  SignatureNotFound,
}

impl From<Error> for napi::Error {
  fn from(value: Error) -> Self {
    match value {
      Error::Git2(e) => napi::Error::new(napi::Status::GenericFailure, format!("libgit2 error: {e}")),
      Error::Napi(e) => e,
      Error::Utf8Error(e) => napi::Error::new(napi::Status::GenericFailure, format!("utf8 error: {e}")),
      Error::InvalidTime => napi::Error::new(napi::Status::GenericFailure, format!("{value}")),
      Error::SignatureNotFound => napi::Error::new(napi::Status::GenericFailure, format!("{value}")),
    }
  }
}

impl From<Error> for napi::JsError {
  fn from(value: Error) -> Self {
    napi::JsError::from(napi::Error::from(value))
  }
}
