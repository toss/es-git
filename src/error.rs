#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Git2(#[from] git2::Error),
}

impl From<Error> for napi::Error {
  fn from(value: Error) -> Self {
    napi::Error::new(napi::Status::GenericFailure, format!("libgit2 error: {value}"))
  }
}

impl From<Error> for napi::JsError {
  fn from(value: Error) -> Self {
    napi::JsError::from(napi::Error::from(value))
  }
}
