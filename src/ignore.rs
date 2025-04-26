use crate::repository::Repository;
use napi_derive::napi;
use std::path::Path;

#[napi]
impl Repository {
  #[napi]
  /// Add ignore rules for a repository.
  ///
  /// This adds ignore rules to the repository. The rules will be used
  /// in addition to any existing ignore rules (such as .gitignore files).
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   addIgnoreRule(rules: string): void;
  /// }
  /// ```
  ///
  /// @param {string} rules - Rules to add, separated by newlines.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// repo.addIgnoreRule("node_modules/");
  /// ```
  pub fn add_ignore_rule(&self, rules: String) -> crate::Result<()> {
    self.inner.add_ignore_rule(&rules)?;
    Ok(())
  }

  #[napi]
  /// Clear ignore rules that were explicitly added.
  ///
  /// Resets to the default internal ignore rules.
  /// This will not turn off rules in .gitignore files that actually exist in the filesystem.
  /// The default internal ignores ignore ".", ".." and ".git" entries.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   clearIgnoreRules(): void;
  /// }
  /// ```
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// repo.addIgnoreRule("*.log");
  /// // Later, clear all added rules
  /// repo.clearIgnoreRules();
  /// ```
  pub fn clear_ignore_rules(&self) -> crate::Result<()> {
    self.inner.clear_ignore_rules()?;
    Ok(())
  }

  #[napi]
  /// Test if the ignore rules apply to a given path.
  ///
  /// This function checks the ignore rules to see if they would apply to the given file.
  /// This indicates if the file would be ignored regardless of whether the file is already in the index or committed to the repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   isPathIgnored(path: string): boolean;
  /// }
  /// ```
  ///
  /// @param {string} path - The path to check.
  /// @returns {boolean} - True if the path is ignored, false otherwise.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const isIgnored = repo.isPathIgnored("node_modules/some-package");
  /// console.log(`Path is ${isIgnored ? "ignored" : "not ignored"}`);
  /// ```
  pub fn is_path_ignored(&self, path: String) -> crate::Result<bool> {
    Ok(self.inner.is_path_ignored(Path::new(&path))?)
  }
}
