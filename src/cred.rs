use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;

#[napi]
#[repr(u8)]
#[derive(Copy, Clone)]
/// - `UserpassPlaintext` : Username and password in plain text.
/// - `SshKey` : SSH key (from file).
/// - `SshCustom` : SSH key with custom signature.
/// - `Default` : Default (e.g. NTLM, Kerberos).
/// - `SshInteractive` : SSH interactive.
/// - `Username` : Username only.
/// - `SshMemory` : SSH key from memory.
pub enum CredType {
  UserpassPlaintext,
  SshKey,
  SshCustom,
  Default,
  SshInteractive,
  Username,
  SshMemory,
}

impl From<u32> for CredType {
  fn from(bits: u32) -> Self {
    let cred_type = git2::CredentialType::from_bits_truncate(bits);
    if cred_type.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
      return Self::UserpassPlaintext;
    }
    if cred_type.contains(git2::CredentialType::SSH_KEY) {
      return Self::SshKey;
    }
    if cred_type.contains(git2::CredentialType::SSH_CUSTOM) {
      return Self::SshCustom;
    }
    if cred_type.contains(git2::CredentialType::DEFAULT) {
      return Self::Default;
    }
    if cred_type.contains(git2::CredentialType::SSH_INTERACTIVE) {
      return Self::SshInteractive;
    }
    if cred_type.contains(git2::CredentialType::USERNAME) {
      return Self::Username;
    }
    if cred_type.contains(git2::CredentialType::SSH_MEMORY) {
      return Self::SshMemory;
    }
    Self::Default
  }
}

/// Stores the parameters needed to construct a `git2::Cred` on demand.
/// Call `to_git2_cred()` to materialise the credential at the desired point in time.
#[derive(Clone)]
pub(crate) enum CredRecipe {
  Default,
  SshKeyFromAgent {
    username: String,
  },
  SshKey {
    username: String,
    public_key_path: Option<String>,
    private_key_path: String,
    passphrase: Option<String>,
  },
  SshKeyFromMemory {
    username: String,
    public_key: Option<String>,
    private_key: String,
    passphrase: Option<String>,
  },
  UserpassPlaintext {
    username: String,
    password: String,
  },
  Username {
    username: String,
  },
  CredentialHelper {
    url: String,
    username: Option<String>,
  },
}

#[napi]
#[derive(Clone)]
/// A structure to represent git credentials in libgit2.
///
/// Create instances via the static factory methods: `default()`, `sshKeyFromAgent()`,
/// `sshKey()`, `sshKeyFromMemory()`, `userpassPlaintext()`, `username()`, `credentialHelper()`.
pub struct Cred {
  pub(crate) recipe: CredRecipe,
}

// Required so that `Cred` can be used as a field inside `#[napi(object)]` structs (e.g. `FetchOptions`).
// napi-rs does not auto-generate `FromNapiValue` for `#[napi]` classes, so we implement it manually.
// Because `Cred: Clone`, we extract the raw pointer via `napi_unwrap` and clone the value.
impl FromNapiValue for Cred {
  unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
    let mut ptr = std::ptr::null_mut::<std::ffi::c_void>();
    check_status!(
      napi::sys::napi_unwrap(env, napi_val, &mut ptr),
      "Failed to unwrap `Cred` from JS value"
    )?;
    Ok((*(ptr as *const Cred)).clone())
  }
}

// Crate-internal methods: not exposed to JS. Converts the stored recipe into a live `git2::Cred`.
impl Cred {
  pub(crate) fn to_git2_cred(&self) -> std::result::Result<git2::Cred, git2::Error> {
    match &self.recipe {
      CredRecipe::Default => git2::Cred::default(),
      CredRecipe::SshKeyFromAgent { username } => git2::Cred::ssh_key_from_agent(username),
      CredRecipe::SshKey {
        username,
        public_key_path,
        private_key_path,
        passphrase,
      } => git2::Cred::ssh_key(
        username,
        public_key_path.as_deref().map(Path::new),
        Path::new(private_key_path),
        passphrase.as_deref(),
      ),
      CredRecipe::SshKeyFromMemory {
        username,
        public_key,
        private_key,
        passphrase,
      } => git2::Cred::ssh_key_from_memory(username, public_key.as_deref(), private_key, passphrase.as_deref()),
      CredRecipe::UserpassPlaintext { username, password } => git2::Cred::userpass_plaintext(username, password),
      CredRecipe::Username { username } => git2::Cred::username(username),
      CredRecipe::CredentialHelper { url, username } => {
        let config = git2::Config::open_default()?;
        git2::Cred::credential_helper(&config, url, username.as_deref())
      }
    }
  }
}

#[napi]
impl Cred {
  #[napi(js_name = "default")]
  /// Create a "default" credential usable for Negotiate mechanisms like NTLM or Kerberos authentication.
  ///
  /// @category Cred
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   static default(): Cred;
  /// }
  /// ```
  ///
  /// @returns {Cred} A new Cred instance.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.default();
  /// ```
  pub fn create_default() -> Cred {
    Cred {
      recipe: CredRecipe::Default,
    }
  }

  #[napi]
  /// Create a new SSH key credential object used for querying an ssh-agent.
  ///
  /// The username specified is the username to authenticate.
  ///
  /// @category Cred
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   static sshKeyFromAgent(username: string): Cred;
  /// }
  /// ```
  ///
  /// @param {string} username - The username to authenticate.
  /// @returns {Cred} A new Cred instance.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.sshKeyFromAgent('git');
  /// ```
  pub fn ssh_key_from_agent(username: String) -> Cred {
    Cred {
      recipe: CredRecipe::SshKeyFromAgent { username },
    }
  }

  #[napi]
  /// Create a new passphrase-protected SSH key credential object from file paths.
  ///
  /// @category Cred
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   static sshKey(
  ///     username: string,
  ///     publicKeyPath: string | null | undefined,
  ///     privateKeyPath: string,
  ///     passphrase?: string | null | undefined,
  ///   ): Cred;
  /// }
  /// ```
  ///
  /// @param {string} username - The username to authenticate.
  /// @param {string | null | undefined} publicKeyPath - Path to the public key file. If `null` or `undefined`, the public key is derived from the private key.
  /// @param {string} privateKeyPath - Path to the private key file.
  /// @param {string | null | undefined} [passphrase] - Passphrase for the private key, if any.
  /// @returns {Cred} A new Cred instance.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.sshKey('git', null, '/home/user/.ssh/id_ed25519', null);
  /// ```
  pub fn ssh_key(
    username: String,
    public_key_path: Option<String>,
    private_key_path: String,
    passphrase: Option<String>,
  ) -> Cred {
    Cred {
      recipe: CredRecipe::SshKey {
        username,
        public_key_path,
        private_key_path,
        passphrase,
      },
    }
  }

  #[napi]
  /// Create a new SSH key credential object reading the keys from memory.
  ///
  /// @category Cred
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   static sshKeyFromMemory(
  ///     username: string,
  ///     publicKey: string | null | undefined,
  ///     privateKey: string,
  ///     passphrase?: string | null | undefined,
  ///   ): Cred;
  /// }
  /// ```
  ///
  /// @param {string} username - The username to authenticate.
  /// @param {string | null | undefined} publicKey - The public key content. If `null` or `undefined`, the public key is derived from the private key.
  /// @param {string} privateKey - The private key content.
  /// @param {string | null | undefined} [passphrase] - Passphrase for the private key, if any.
  /// @returns {Cred} A new Cred instance.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const privateKey = await fs.readFile('/home/user/.ssh/id_ed25519', 'utf-8');
  /// const cred = Cred.sshKeyFromMemory('git', null, privateKey, null);
  /// ```
  pub fn ssh_key_from_memory(
    username: String,
    public_key: Option<String>,
    private_key: String,
    passphrase: Option<String>,
  ) -> Cred {
    Cred {
      recipe: CredRecipe::SshKeyFromMemory {
        username,
        public_key,
        private_key,
        passphrase,
      },
    }
  }

  #[napi]
  /// Create a new plain-text username and password credential object.
  ///
  /// @category Cred
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   static userpassPlaintext(username: string, password: string): Cred;
  /// }
  /// ```
  ///
  /// @param {string} username - The username to authenticate.
  /// @param {string} password - The password to authenticate.
  /// @returns {Cred} A new Cred instance.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.userpassPlaintext('user', 'password');
  /// ```
  pub fn userpass_plaintext(username: String, password: String) -> Cred {
    Cred {
      recipe: CredRecipe::UserpassPlaintext { username, password },
    }
  }

  #[napi]
  /// Attempt to read `credential.helper` according to gitcredentials(7).
  ///
  /// This function will attempt to parse the user's `credential.helper` configuration,
  /// invoke the necessary processes, and read off what the username/password should be
  /// for a particular URL. The returned credential will be a username/password credential
  /// if successful.
  ///
  /// @category Cred
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   static credentialHelper(
  ///     url: string,
  ///     username?: string | null | undefined,
  ///   ): Cred;
  /// }
  /// ```
  ///
  /// @param {string} url - The URL to get credentials for.
  /// @param {string | null | undefined} [username] - Optional username hint.
  /// @returns {Cred} A new Cred instance containing the username/password from the helper.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.credentialHelper('https://github.com/user/repo');
  /// ```
  pub fn credential_helper(url: String, username: Option<String>) -> Cred {
    Cred {
      recipe: CredRecipe::CredentialHelper { url, username },
    }
  }

  #[napi]
  /// Create a credential to specify a username.
  ///
  /// This is used with SSH authentication to query for the username if none is specified in the URL.
  ///
  /// @category Cred
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   static username(username: string): Cred;
  /// }
  /// ```
  ///
  /// @param {string} username - The username to authenticate.
  /// @returns {Cred} A new Cred instance.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.username('git');
  /// ```
  pub fn username(username: String) -> Cred {
    Cred {
      recipe: CredRecipe::Username { username },
    }
  }

  #[napi]
  /// Check whether a credential object contains username information.
  ///
  /// @category Cred/Methods
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   hasUsername(): boolean;
  /// }
  /// ```
  ///
  /// @returns {boolean} Returns `true` if this credential contains username information.
  ///
  /// @throws Throws if the underlying resource is no longer available (e.g. key file deleted, credential helper config changed) since the credential was created.
  pub fn has_username(&self) -> crate::Result<bool> {
    Ok(self.to_git2_cred()?.has_username())
  }

  #[napi]
  /// Return the type of credentials that this object represents.
  ///
  /// @category Cred/Methods
  ///
  /// @signature
  /// ```ts
  /// class Cred {
  ///   credtype(): CredType;
  /// }
  /// ```
  ///
  /// @returns {CredType} The type of this credential.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.userpassPlaintext('user', 'password');
  /// console.log(cred.credtype()); // 'UserpassPlaintext'
  /// ```
  pub fn credtype(&self) -> CredType {
    match &self.recipe {
      CredRecipe::Default => CredType::Default,
      CredRecipe::SshKeyFromAgent { .. } | CredRecipe::SshKey { .. } => CredType::SshKey,
      CredRecipe::SshKeyFromMemory { .. } => CredType::SshMemory,
      CredRecipe::UserpassPlaintext { .. } | CredRecipe::CredentialHelper { .. } => CredType::UserpassPlaintext,
      CredRecipe::Username { .. } => CredType::Username,
    }
  }
}
