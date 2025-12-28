use crate::repository::Repository;
use napi::bindgen_prelude::{Buffer, Null};
use napi::Either;
use napi_derive::napi;
use std::path::Path;

pub type AttrValue = Either<bool, Either<String, Either<Buffer, Null>>>;

fn into_attr_value(bytes: Option<&[u8]>) -> AttrValue {
  let val = git2::AttrValue::from_bytes(bytes);
  match val {
    git2::AttrValue::True => Either::A(true),
    git2::AttrValue::False => Either::A(false),
    git2::AttrValue::String(str) => Either::B(Either::A(str.to_string())),
    git2::AttrValue::Bytes(bytes) => Either::B(Either::B(Either::A(Buffer::from(bytes)))),
    git2::AttrValue::Unspecified => Either::B(Either::B(Either::B(Null))),
  }
}

#[napi(object)]
pub struct AttrOptions {
  /// Check the working directory, then the index.
  pub check_file_then_index: Option<bool>,
  /// Check the index, then the working directory.
  pub check_index_then_file: Option<bool>,
  /// Check the index only.
  pub check_index_only: Option<bool>,
  /// Do not use the system gitattributes file.
  pub check_no_system: Option<bool>,
}

fn flag(flag: &mut git2::AttrCheckFlags, opt: git2::AttrCheckFlags, val: bool) {
  if val {
    *flag |= opt;
  } else {
    *flag &= !opt;
  }
}

impl From<AttrOptions> for git2::AttrCheckFlags {
  fn from(value: AttrOptions) -> Self {
    let mut flags = git2::AttrCheckFlags::default();
    if let Some(val) = value.check_file_then_index {
      flag(&mut flags, git2::AttrCheckFlags::FILE_THEN_INDEX, val);
    }
    if let Some(val) = value.check_index_then_file {
      flag(&mut flags, git2::AttrCheckFlags::INDEX_THEN_FILE, val);
    }
    if let Some(val) = value.check_index_only {
      flag(&mut flags, git2::AttrCheckFlags::INDEX_ONLY, val);
    }
    if let Some(val) = value.check_no_system {
      flag(&mut flags, git2::AttrCheckFlags::NO_SYSTEM, val);
    }
    flags
  }
}

#[napi]
impl Repository {
  #[napi(ts_return_type = "boolean | string | Buffer | null")]
  /// Get the value of a git attribute for a path.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getAttr(
  ///     path: string,
  ///     name: string,
  ///     options?: AttrOptions | null | undefined
  ///   ): boolean | string | Buffer | null;
  /// ```
  ///
  /// @param {string} path - The path to check for attributes. Relative paths are interpreted relative to the repo root.
  /// @param {string} name - The name of the attribute to look up.
  /// @param {AttrOptions} [options] - Options for attribute lookup.
  ///
  /// @returns Output of the value of the attribute.
  pub fn get_attr(&self, path: String, name: String, options: Option<AttrOptions>) -> crate::Result<AttrValue> {
    let flags = options.map(git2::AttrCheckFlags::from).unwrap_or_default();
    let raw = self.inner.get_attr_bytes(Path::new(&path), &name, flags)?;
    let value = into_attr_value(raw);
    Ok(value)
  }
}
