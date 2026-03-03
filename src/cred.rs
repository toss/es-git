use crate::config::Config;
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

#[napi]
/// A structure to represent git credentials in libgit2.
///
/// Create instances via the static factory methods: `default()`, `sshKeyFromAgent()`,
/// `sshKey()`, `sshKeyFromMemory()`, `userpassPlaintext()`, `username()`, `credentialHelper()`.
pub struct Cred {
  pub(crate) inner: git2::Cred,
}

impl From<git2::Cred> for Cred {
  fn from(inner: git2::Cred) -> Self {
    Cred { inner }
  }
}

impl From<Cred> for git2::Cred {
  fn from(cred: Cred) -> Self {
    cred.inner
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
  /// @throws Throws error if the credential cannot be created.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.default();
  /// ```
  pub fn create_default() -> crate::Result<Cred> {
    let inner = git2::Cred::default().map_err(crate::Error::from)?;
    Ok(Cred { inner })
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
  /// @throws Throws error if the ssh-agent is not available or the credential cannot be created.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.sshKeyFromAgent('git');
  /// ```
  pub fn ssh_key_from_agent(username: String) -> crate::Result<Cred> {
    let inner = git2::Cred::ssh_key_from_agent(&username)?;
    Ok(Cred { inner })
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
  /// @throws Throws error if the key files cannot be read or the credential cannot be created.
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
  ) -> crate::Result<Cred> {
    let inner = git2::Cred::ssh_key(
      &username,
      public_key_path.as_deref().map(Path::new),
      Path::new(&private_key_path),
      passphrase.as_deref(),
    )?;
    Ok(Cred { inner })
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
  /// @throws Throws error if the key content is invalid or the credential cannot be created.
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
  ) -> crate::Result<Cred> {
    let inner = git2::Cred::ssh_key_from_memory(&username, public_key.as_deref(), &private_key, passphrase.as_deref())?;
    Ok(Cred { inner })
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
  /// @throws Throws error if the credential cannot be created.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.userpassPlaintext('user', 'password');
  /// ```
  pub fn userpass_plaintext(username: String, password: String) -> crate::Result<Cred> {
    let inner = git2::Cred::userpass_plaintext(&username, &password)?;
    Ok(Cred { inner })
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
  ///     config: Config,
  ///     url: string,
  ///     username?: string | null | undefined,
  ///   ): Cred;
  /// }
  /// ```
  ///
  /// @param {Config} config - Git configuration to read `credential.helper` from.
  /// @param {string} url - The URL to get credentials for.
  /// @param {string | null | undefined} [username] - Optional username hint.
  /// @returns {Cred} A new Cred instance containing the username/password from the helper.
  /// @throws Throws error if the helper fails or no credential is found.
  ///
  /// @example
  ///
  /// ```ts
  /// import { openRepository, Cred } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const config = repo.config();
  /// const cred = Cred.credentialHelper(config, 'https://github.com/user/repo');
  /// ```
  pub fn credential_helper(config: &Config, url: String, username: Option<String>) -> crate::Result<Cred> {
    let inner = git2::Cred::credential_helper(&config.inner, &url, username.as_deref())?;
    Ok(Cred { inner })
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
  /// @throws Throws error if the credential cannot be created.
  ///
  /// @example
  ///
  /// ```ts
  /// import { Cred } from 'es-git';
  ///
  /// const cred = Cred.username('git');
  /// ```
  pub fn username(username: String) -> crate::Result<Cred> {
    let inner = git2::Cred::username(&username)?;
    Ok(Cred { inner })
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
  pub fn has_username(&self) -> bool {
    self.inner.has_username()
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
    self.inner.credtype().into()
  }
}
