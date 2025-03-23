use crate::repository::Repository;
use crate::util::path_to_js_string;
use napi::bindgen_prelude::*;
use napi::JsString;
use napi_derive::napi;
use std::path::Path;

#[napi(string_enum)]
/// - `ProgramData` : System-wide on Windows, for compatibility with portable git.
/// - `System` : System-wide configuration file. (e.g. `/etc/gitconfig`)
/// - `XDG` : XDG-compatible configuration file. (e.g. `~/.config/git/config`)
/// - `Global` : User-specific configuration. (e.g. `~/.gitconfig`)
/// - `Local` : Repository specific config. (e.g. `$PWD/.git/config`)
/// - `Worktree` : Worktree specific configuration file. (e.g. `$GIT_DIR/config.worktree`)
/// - `App` : Application specific configuration file.
/// - `Highest` : Highest level available.
pub enum ConfigLevel {
  ProgramData,
  System,
  XDG,
  Global,
  Local,
  Worktree,
  App,
  Highest,
}

impl From<git2::ConfigLevel> for ConfigLevel {
  fn from(value: git2::ConfigLevel) -> Self {
    match value {
      git2::ConfigLevel::ProgramData => Self::ProgramData,
      git2::ConfigLevel::System => Self::System,
      git2::ConfigLevel::XDG => Self::XDG,
      git2::ConfigLevel::Global => Self::Global,
      git2::ConfigLevel::Local => Self::Local,
      git2::ConfigLevel::Worktree => Self::Worktree,
      git2::ConfigLevel::App => Self::App,
      git2::ConfigLevel::Highest => Self::Highest,
    }
  }
}

#[napi(object)]
pub struct ConfigEntry {
  /// The name of this entry.
  pub name: String,
  /// The value of this entry. If no value is defined, the value will be `null`.
  pub value: Option<String>,
  /// The configuration level of this entry.
  pub level: ConfigLevel,
  /// Depth of includes where this variable was found
  pub include_depth: u32,
}

impl TryFrom<&git2::ConfigEntry<'_>> for ConfigEntry {
  type Error = crate::Error;
  fn try_from(entry: &git2::ConfigEntry) -> std::result::Result<Self, Self::Error> {
    let name = std::str::from_utf8(entry.name_bytes())
      .map_err(crate::Error::from)?
      .to_string();
    let value = match entry.has_value() {
      true => {
        let value = std::str::from_utf8(entry.value_bytes())
          .map_err(crate::Error::from)?
          .to_string();
        Some(value)
      }
      false => None,
    };
    let level = ConfigLevel::from(entry.level());
    let include_depth = entry.include_depth();
    Ok(ConfigEntry {
      name,
      value,
      level,
      include_depth,
    })
  }
}

#[napi(iterator)]
/// An iterator over the `ConfigEntry` values of a config.
pub struct ConfigEntries {
  pub(crate) inner: SharedReference<Config, git2::ConfigEntries<'static>>,
}

#[napi]
impl Generator for ConfigEntries {
  type Yield = ConfigEntry;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    self
      .inner
      .next()
      .and_then(|x| x.ok())
      .map(ConfigEntry::try_from)
      .and_then(|x| x.ok())
  }
}

#[napi]
pub struct Config {
  pub(crate) inner: git2::Config,
}

#[napi]
impl Config {
  #[napi]
  /// Delete a config variable from the config file with the highest level
  /// (usually the local one).
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   remove(name: string): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  pub fn remove(&mut self, name: String) -> crate::Result<()> {
    self.inner.remove(&name)?;
    Ok(())
  }

  #[napi]
  /// Remove multivar config variables in the config file with the highest level (usually the
  /// local one).
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   removeMultivar(name: string, regexp: string): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @param {string} regexp - The regular expression is applied case-sensitively on the value.
  pub fn remove_multivar(&mut self, name: String, regexp: String) -> crate::Result<()> {
    self.inner.remove_multivar(&name, &regexp)?;
    Ok(())
  }

  #[napi]
  /// Get the value of a boolean config variable.
  ///
  /// All config files will be looked into, in the order of their defined
  /// level. A higher level means a higher priority. The first occurrence of
  /// the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   getBool(name: string): boolean;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of a boolean config variable.
  pub fn get_bool(&self, name: String) -> crate::Result<bool> {
    let value = self.inner.get_bool(&name)?;
    Ok(value)
  }

  #[napi]
  /// Find the value of a boolean config variable.
  ///
  /// All config files will be looked into, in the order of their defined
  /// level. A higher level means a higher priority. The first occurrence of
  /// the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   findBool(name: string): boolean | null;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of a boolean config variable.
  pub fn find_bool(&self, name: String) -> Option<bool> {
    self.get_bool(name).ok()
  }

  #[napi]
  /// Get the value of an integer config variable.
  ///
  /// All config files will be looked into, in the order of their defined
  /// level. A higher level means a higher priority. The first occurrence of
  /// the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   getI32(name: string): number;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of an integer config variable.
  pub fn get_i32(&self, name: String) -> crate::Result<i32> {
    let value = self.inner.get_i32(&name)?;
    Ok(value)
  }

  #[napi]
  /// Find the value of an integer config variable.
  ///
  /// All config files will be looked into, in the order of their defined
  /// level. A higher level means a higher priority. The first occurrence of
  /// the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   findI32(name: string): number | null;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of an integer config variable.
  pub fn find_i32(&self, name: String) -> Option<i32> {
    self.get_i32(name).ok()
  }

  #[napi]
  /// Get the value of an integer config variable.
  ///
  /// All config files will be looked into, in the order of their defined
  /// level. A higher level means a higher priority. The first occurrence of
  /// the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   getI64(name: string): number;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of an integer config variable.
  pub fn get_i64(&self, name: String) -> crate::Result<i64> {
    let value = self.inner.get_i64(&name)?;
    Ok(value)
  }

  #[napi]
  /// Find the value of an integer config variable.
  ///
  /// All config files will be looked into, in the order of their defined
  /// level. A higher level means a higher priority. The first occurrence of
  /// the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   findI64(name: string): number | null;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of an integer config variable.
  pub fn find_i64(&self, name: String) -> Option<i64> {
    self.get_i64(name).ok()
  }

  #[napi]
  /// Get the value of a string config variable as a byte slice.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   getBytes(name: string): Uint8Array;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of a string config variable as a byte slice.
  pub fn get_bytes(&mut self, name: String) -> crate::Result<Uint8Array> {
    // `get_bytes` can only be called on a snapshot.
    let snapshot = self.inner.snapshot()?;
    let value = snapshot.get_bytes(&name)?;
    Ok(value.to_vec().into())
  }

  #[napi]
  /// Find the value of a string config variable as a byte slice.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   findBytes(name: string): Uint8Array | null;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of a string config variable as a byte slice.
  pub fn find_bytes(&mut self, name: String) -> Option<Uint8Array> {
    self.get_bytes(name).ok()
  }

  #[napi]
  /// Get the value of a string config variable as an owned string.
  ///
  /// All config files will be looked into, in the order of their
  /// defined level. A higher level means a higher priority. The
  /// first occurrence of the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   getString(name: string): string;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of a string config variable.
  /// @throws An error will be returned if the config value is not valid utf-8.
  pub fn get_string(&self, name: String) -> crate::Result<String> {
    let value = self.inner.get_string(&name)?;
    Ok(value)
  }

  #[napi]
  /// Find the value of a string config variable as an owned string.
  ///
  /// All config files will be looked into, in the order of their
  /// defined level. A higher level means a higher priority. The
  /// first occurrence of the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   findString(name: string): string | null;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of a string config variable.
  pub fn find_string(&self, name: String) -> Option<String> {
    self.get_string(name).ok()
  }

  #[napi]
  /// Get the value of a path config variable.
  ///
  /// A leading '~' will be expanded to the global search path (which
  /// defaults to the user's home directory but can be overridden via
  /// [`git_libgit2_opts`][1].
  ///
  /// [1]: https://libgit2.org/docs/reference/v1.9.0/common/git_libgit2_opts.html
  ///
  /// All config files will be looked into, in the order of their
  /// defined level. A higher level means a higher priority. The
  /// first occurrence of the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   getPath(name: string): string;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of a path config variable.
  pub fn get_path(&self, env: Env, name: String) -> crate::Result<JsString> {
    let value = self.inner.get_path(&name)?;
    let path = path_to_js_string(&env, &value)?;
    Ok(path)
  }

  #[napi]
  /// Find the value of a path config variable.
  ///
  /// A leading '~' will be expanded to the global search path (which
  /// defaults to the user's home directory but can be overridden via
  /// [`git_libgit2_opts`][1].
  ///
  /// [1]: https://libgit2.org/docs/reference/v1.9.0/common/git_libgit2_opts.html
  ///
  /// All config files will be looked into, in the order of their
  /// defined level. A higher level means a higher priority. The
  /// first occurrence of the variable will be returned here.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   findPath(name: string): string | null;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns The value of a path config variable.
  pub fn find_path(&self, env: Env, name: String) -> Option<JsString> {
    self.get_path(env, name).ok()
  }

  #[napi]
  /// Get the entry for a config variable.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   getEntry(name: string): ConfigEntry;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns `ConfigEntry` object representing a certain entry owned by a `Config` instance.
  pub fn get_entry(&self, name: String) -> crate::Result<ConfigEntry> {
    let value = self.inner.get_entry(&name)?;
    let entry = ConfigEntry::try_from(&value)?;
    Ok(entry)
  }

  #[napi]
  /// Find the entry for a config variable.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   findEntry(name: string): ConfigEntry | null;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @returns `ConfigEntry` object representing a certain entry owned by a `Config` instance.
  pub fn find_entry(&self, name: String) -> Option<ConfigEntry> {
    self.get_entry(name).ok()
  }

  #[napi]
  /// Iterate over all the config variables.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   entries(glob?: string): ConfigEntries;
  /// }
  /// ```
  ///
  /// @param {string} [glob] - If `glob` is provided, then the iterator will only iterate over all
  /// variables whose name matches the pattern.
  /// The regular expression is applied case-sensitively on the normalized form of
  /// the variable name: the section and variable parts are lower-cased. The
  /// subsection is left unchanged.
  ///
  /// @returns An iterator over the `ConfigEntry` values of a config.
  /// @example
  ///
  /// ```ts
  /// import { openDefaultConfig } from 'es-git';
  ///
  /// const config = openDefaultConfig();
  /// for (const entry of config.entries()) {
  ///   console.log(entry.name, entry.value);
  /// }
  /// ```
  pub fn entries(&self, env: Env, this: Reference<Config>, glob: Option<String>) -> crate::Result<ConfigEntries> {
    let inner = this.share_with(env, |config| {
      config
        .inner
        .entries(glob.as_deref())
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(ConfigEntries { inner })
  }

  #[napi]
  /// Iterate over the values of a multivar.
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   multivar(name: string, regexp?: string): ConfigEntries;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @param {string} [regexp] - If `regexp` is provided, then the iterator will only iterate over all
  /// values which match the pattern.
  /// The regular expression is applied case-sensitively on the normalized form of
  /// the variable name: the section and variable parts are lower-cased. The
  /// subsection is left unchanged.
  ///
  /// @returns An iterator over the `ConfigEntry` values of a config.
  pub fn multivar(
    &self,
    env: Env,
    this: Reference<Config>,
    name: String,
    regexp: Option<String>,
  ) -> crate::Result<ConfigEntries> {
    let inner = this.share_with(env, |config| {
      config
        .inner
        .multivar(&name, regexp.as_deref())
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(ConfigEntries { inner })
  }

  #[napi]
  /// Set the value of a boolean config variable in the config file with the
  /// highest level (usually the local one).
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   setBool(name: string, value: boolean): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @param {boolean} value - The value of config entry.
  pub fn set_bool(&mut self, name: String, value: bool) -> crate::Result<()> {
    self.inner.set_bool(&name, value)?;
    Ok(())
  }

  #[napi]
  /// Set the value of an integer config variable in the config file with the
  /// highest level (usually the local one).
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   setI32(name: string, value: number): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @param {number} value - The value of config entry.
  pub fn set_i32(&mut self, name: String, value: i32) -> crate::Result<()> {
    self.inner.set_i32(&name, value)?;
    Ok(())
  }

  #[napi]
  /// Set the value of an integer config variable in the config file with the
  /// highest level (usually the local one).
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   setI64(name: string, value: number): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @param {number} value - The value of config entry.
  pub fn set_i64(&mut self, name: String, value: i64) -> crate::Result<()> {
    self.inner.set_i64(&name, value)?;
    Ok(())
  }

  #[napi]
  /// Set the value of an multivar config variable in the config file with the
  /// highest level (usually the local one).
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   setMultivar(name: string, regexp: string, value: string): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @param {string} regexp - The regular expression is applied case-sensitively on the value.
  /// @param {string} value - The value of config entry.
  pub fn set_multivar(&mut self, name: String, regexp: String, value: String) -> crate::Result<()> {
    self.inner.set_multivar(&name, &regexp, &value)?;
    Ok(())
  }

  #[napi]
  /// Set the value of a string config variable in the config file with the
  /// highest level (usually the local one).
  ///
  /// @category Config/Methods
  /// @signature
  /// ```ts
  /// class Config {
  ///   setString(name: string, value: string): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of config entry.
  /// @param {string} value - The value of config entry.
  pub fn set_string(&mut self, name: String, value: String) -> crate::Result<()> {
    self.inner.set_str(&name, &value)?;
    Ok(())
  }
}

#[napi]
/// Create a new config instance containing a single on-disk file
///
/// @category Config
/// @signature
/// ```ts
/// function openConfig(path: string): Config;
/// ```
///
/// @param {string} path - Path to config file.
/// @returns Config instance representing a git configuration key/value store.
pub fn open_config(path: String) -> crate::Result<Config> {
  let inner = git2::Config::open(Path::new(&path))?;
  Ok(Config { inner })
}

#[napi]
/// Open the global, XDG and system configuration files
///
/// Utility wrapper that finds the global, XDG and system configuration
/// files and opens them into a single prioritized config object that can
/// be used when accessing default config data outside a repository.
///
/// @category Config
/// @signature
/// ```ts
/// function openDefaultConfig(): Config;
/// ```
///
/// @returns Config instance representing a git configuration key/value store.
pub fn open_default_config() -> crate::Result<Config> {
  let inner = git2::Config::open_default()?;
  Ok(Config { inner })
}

#[napi]
/// Locate the path to the global configuration file.
///
/// The user or global configuration file is usually located in
/// `$HOME/.gitconfig`.
///
/// This method will try to guess the full path to that file, if the file
/// exists. The returned path may be used on any method call to load
/// the global configuration file.
///
/// This method will not guess the path to the XDG compatible config file
/// (`.config/git/config`).
///
/// @category Config
/// @signature
/// ```ts
/// function findGlobalConfigPath(): string | null;
/// ```
///
/// @returns The path to the global configuration file.
pub fn find_global_config_path(env: Env) -> Option<JsString> {
  git2::Config::find_global()
    .ok()
    .map(|x| path_to_js_string(&env, &x))
    .and_then(|x| x.ok())
}

#[napi]
/// Locate the path to the system configuration file.
///
/// If `/etc/gitconfig` doesn't exist, it will look for `%PROGRAMFILES%`.
///
/// @category Config
/// @signature
/// ```ts
/// function findSystemConfigPath(): string | null;
/// ```
///
/// @returns The path to the system configuration file.
pub fn find_system_config_path(env: Env) -> Option<JsString> {
  git2::Config::find_system()
    .ok()
    .map(|x| path_to_js_string(&env, &x))
    .and_then(|x| x.ok())
}

#[napi]
/// Locate the path to the global XDG compatible configuration file.
///
/// The XDG compatible configuration file is usually located in
/// `$HOME/.config/git/config`.
///
/// @category Config
/// @signature
/// ```ts
/// function findXdgConfigPath(): string | null;
/// ```
///
/// @returns The path to the XDG compatible configuration file.
pub fn find_xdg_config_path(env: Env) -> Option<JsString> {
  git2::Config::find_xdg()
    .ok()
    .map(|x| path_to_js_string(&env, &x))
    .and_then(|x| x.ok())
}

#[napi]
/// Parse a string as a bool.
///
/// @category Config
/// @signature
/// ```ts
/// function parseConfigBool(value: string): boolean;
/// ```
///
/// @param {string} value - Input value.
/// @returns Interprets "true", "yes", "on", 1, or any non-zero number as `true`.
/// Interprets "false", "no", "off", 0, or an empty string as `false`.
pub fn parse_config_bool(value: String) -> crate::Result<bool> {
  let parsed = git2::Config::parse_bool(value)?;
  Ok(parsed)
}

#[napi]
/// Parse a string as an i32; handles suffixes like k, M, or G, and
/// multiplies by the appropriate power of 1024.
///
/// @category Config
/// @signature
/// ```ts
/// function parseConfigI32(value: string): number;
/// ```
///
/// @param {string} value - Input value.
/// @returns Parsed i32 value.
pub fn parse_config_i32(value: String) -> crate::Result<i32> {
  let parsed = git2::Config::parse_i32(value)?;
  Ok(parsed)
}

#[napi]
/// Parse a string as an i64; handles suffixes like k, M, or G, and
/// multiplies by the appropriate power of 1024.
///
/// @category Config
/// @signature
/// ```ts
/// function parseConfigI64(value: string): number;
/// ```
///
/// @param {string} value - Input value.
/// @returns Parsed i64 value.
pub fn parse_config_i64(value: String) -> crate::Result<i64> {
  let parsed = git2::Config::parse_i64(value)?;
  Ok(parsed)
}

#[napi]
impl Repository {
  #[napi]
  /// Get the configuration file for this repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   config(): Config;
  /// }
  /// ```
  ///
  /// @returns If a configuration file has not been set, the default config set for the
  /// repository will be returned, including global and system configurations
  /// (if they are available).
  pub fn config(&self) -> crate::Result<Config> {
    let inner = self.inner.config()?;
    Ok(Config { inner })
  }
}
