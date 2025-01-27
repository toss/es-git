use crate::object::ObjectType;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
/// Check if given string is valid Oid.
///
/// Returns `false` if the string is empty, is longer than 40 hex
/// characters, or contains any non-hex characters.
pub fn is_valid_oid(value: String) -> bool {
  git2::Oid::from_str(&value).is_ok()
}

#[napi]
/// Test if this Oid is all zeros.
pub fn is_zero_oid(value: String) -> bool {
  if let Ok(oid) = git2::Oid::from_str(&value) {
    return oid.is_zero();
  }
  false
}

#[napi]
/// Creates an all zero Oid structure.
pub fn zero_oid() -> String {
  git2::Oid::zero().to_string()
}

#[napi]
/// Hashes the provided data as an object of the provided type, and returns
/// an Oid corresponding to the result. This does not store the object
/// inside any object database or repository.
pub fn hash_object_oid(obj_type: ObjectType, bytes: Buffer) -> crate::Result<String> {
  let oid = git2::Oid::hash_object(obj_type.into(), bytes.as_ref())?;
  Ok(oid.to_string())
}

#[napi]
/// Hashes the content of the provided file as an object of the provided type,
/// and returns an Oid corresponding to the result. This does not store the object
/// inside any object database or repository.
pub fn hash_file_oid(obj_type: ObjectType, path: String) -> crate::Result<String> {
  let oid = git2::Oid::hash_file(obj_type.into(), path)?;
  Ok(oid.to_string())
}
