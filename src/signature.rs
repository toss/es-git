use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
/// A Signature is used to indicate authorship of various actions throughout the
/// library.
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

impl TryFrom<Signature> for git2::Signature<'static> {
  type Error = crate::Error;

  fn try_from(value: Signature) -> Result<Self, Self::Error> {
    let signature = git2::Signature::new(&value.name, &value.email, &git2::Time::new(value.timestamp, 0))?;
    Ok(signature)
  }
}

#[napi(object)]
pub struct SignatureTimeOptions {
  /// Time in seconds, from epoch
  pub timestamp: i64,
  /// Timezone offset, in minutes
  pub offset: Option<i32>,
}

#[napi]
/// Create a new action signature.
///
/// @category Signature
/// @signature
/// ```ts
/// function createSignature(
///   name: string,
///   email: string,
///   timeOptions?: SignatureTimeOptions,
/// ): Signature;
/// ```
///
/// @param {string} name - Name on the signature.
/// @param {string} email - Email on the signature.
/// @param {SignatureTimeOptions} [timeOptions] - Time options for signature.
///
/// @returns
///
/// @example
/// ```ts
/// import { createSignature } from 'es-git';
///
/// const author = createSignature(
///   'Seokju Na',
///   'seokju.me@toss.im',
/// );
/// ```
pub fn create_signature(
  name: String,
  email: String,
  time_options: Option<SignatureTimeOptions>,
) -> crate::Result<Signature> {
  let git_signature = match time_options {
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

#[napi(object)]
pub struct SignaturePayload {
  /// Name on the signature.
  pub name: String,
  /// Email on the signature.
  pub email: String,
  pub time_options: Option<SignatureTimeOptions>,
}

impl TryFrom<SignaturePayload> for Signature {
  type Error = crate::Error;

  fn try_from(value: SignaturePayload) -> Result<Self, Self::Error> {
    let signature = create_signature(value.name, value.email, value.time_options)?;
    Ok(signature)
  }
}
