use napi_derive::napi;

#[napi(object)]
/// A Signature is used to indicate authorship of various actions throughout the
/// library.
///
/// Signatures contain a name, email, and timestamp.
pub struct Signature {
  /// Name on the signature.
  pub name: String,
  /// Email on the signature.
  pub email: String,
  /// Time in seconds, from epoch
  pub timestamp: i64,
}

impl<'a> TryFrom<git2::Signature<'a>> for Signature {
  type Error = crate::Error;

  fn try_from(value: git2::Signature<'a>) -> Result<Self, Self::Error> {
    let name = std::str::from_utf8(value.name_bytes())?.to_string();
    let email = std::str::from_utf8(value.email_bytes())?.to_string();
    let timestamp = value.when().seconds();
    Ok(Self { name, email, timestamp })
  }
}

#[napi(object)]
pub struct CreateSignatureOptions {
  /// Time in seconds, from epoch
  pub timestamp: i64,
  /// Timezone offset, in minutes
  pub offset: Option<i32>,
}

#[napi]
/// Create a new action signature.
pub fn create_signature(
  name: String,
  email: String,
  options: Option<CreateSignatureOptions>,
) -> crate::Result<Signature> {
  let git_signature = match options {
    Some(opts) => git2::Signature::new(
      &name,
      &email,
      &git2::Time::new(opts.timestamp, opts.offset.unwrap_or(0)),
    ),
    None => git2::Signature::now(&name, &email),
  }?;
  let signature = Signature::try_from(git_signature)?;
  Ok(signature)
}
