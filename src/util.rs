use bitflags::Flags;
use std::path::Path;

pub(crate) fn path_to_string(p: &Path) -> String {
  #[cfg(unix)]
  {
    p.to_string_lossy().to_string()
  }
  #[cfg(windows)]
  {
    use std::os::windows::ffi::OsStrExt;
    let path_buf = p.as_os_str().encode_wide().collect::<Vec<u16>>();
    let str = String::from_utf16_lossy(path_buf.as_slice()).to_string();
    str
  }
}

pub(crate) fn bitflags_contain<T: Flags>(source: T, target: T) -> bool {
  source.contains(target)
}
