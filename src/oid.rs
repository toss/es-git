use crate::object::ObjectType;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
/// Check if given string is valid Oid.
///
/// @category Oid
/// @signature
/// ```ts
/// function isValidOid(value: string): boolean;
/// ```
///
/// @param {string} value - String to check if is valid Oid.
/// @returns Returns `false` if the string is empty, is longer than 40 hex
/// characters, or contains any non-hex characters.
pub fn is_valid_oid(value: String) -> bool {
  git2::Oid::from_str(&value).is_ok()
}

#[napi]
/// Test if this Oid is all zeros.
///
/// @category Oid
/// @signature
/// ```ts
/// function isZeroOid(value: string): boolean;
/// ```
///
/// @param {string} value - String to check if is zero Oid.
/// @returns Returns `true` if the string is zero Oid.
/// @example
/// ```ts
/// import { zeroOid, isZeroOid } from 'es-git';
///
/// console.assert(isZeroOid(zeroOid());
/// ```
pub fn is_zero_oid(value: String) -> bool {
  if let Ok(oid) = git2::Oid::from_str(&value) {
    return oid.is_zero();
  }
  false
}

#[napi]
/// Creates an all zero Oid structure.
///
/// @category Oid
/// @signature
/// ```ts
/// function zeroOid(): string;
/// ```
///
/// @returns Zero Oid string.
pub fn zero_oid() -> String {
  git2::Oid::zero().to_string()
}

#[napi]
/// Hashes the provided data as an object of the provided type, and returns
/// an Oid corresponding to the result. This does not store the object
/// inside any object database or repository.
///
/// @category Oid
/// @signature
/// ```ts
/// function hashObjectOid(objType: ObjectType, bytes: Buffer): string;
/// ```
///
/// @param {ObjectType} objType - Git object type.
/// @param {Buffer} bytes - Data to hashed.
/// @returns Hashed string.
pub fn hash_object_oid(obj_type: ObjectType, bytes: Buffer) -> crate::Result<String> {
  let oid = git2::Oid::hash_object(obj_type.into(), bytes.as_ref())?;
  Ok(oid.to_string())
}

#[napi]
/// Hashes the content of the provided file as an object of the provided type,
/// and returns an Oid corresponding to the result. This does not store the object
/// inside any object database or repository.
///
/// @category Oid
/// @signature
/// ```ts
/// function hashFileOid(objType: ObjectType, path: string): string;
/// ```
///
/// @param {ObjectType} objType - Git object type.
/// @param {string} path - File path to make hash.
/// @returns Hashed string.
pub fn hash_file_oid(obj_type: ObjectType, path: String) -> crate::Result<String> {
  let oid = git2::Oid::hash_file(obj_type.into(), path)?;
  Ok(oid.to_string())
}
