use crate::repository::Repository;
use crate::signature::Signature;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::Deref;
use std::path::Path;

/// Represents a hunk of a blame operation, which is a range of lines
/// and information about who last modified them.
#[napi(object)]
pub struct BlameHunk {
  /// The oid of the commit where this line was last changed.
  pub final_commit_id: String,
  /// The 1-based line number in the final file where this hunk starts.
  pub final_start_line_number: u32,
  /// The number of lines in this hunk.
  pub lines_in_hunk: u32,
  /// The signature of the commit where this line was last changed.
  pub final_signature: Option<Signature>,
  /// The path to the file where this line was originally written.
  pub path: Option<String>,
  /// The 1-based line number in the original file where this hunk starts.
  pub orig_start_line_number: u32,
  /// The oid of the commit where this line was originally written.
  pub orig_commit_id: String,
  /// The signature of the commit where this line was originally written.
  pub orig_signature: Option<Signature>,
  /// True if the hunk has been determined to be a boundary commit (the commit
  /// when the file was first introduced to the repository).
  pub is_boundary: bool,
}

/// Options for controlling blame behavior
#[napi(object)]
#[derive(Default)]
pub struct BlameOptions {
  /// The minimum line number to blame (1-based index)
  pub min_line: Option<u32>,
  /// The maximum line number to blame (1-based index)
  pub max_line: Option<u32>,
  /// The oid of the newest commit to consider. The blame algorithm will stop
  /// when this commit is reached.
  pub newest_commit: Option<String>,
  /// The oid of the oldest commit to consider. The blame algorithm will
  /// stop when this commit is reached.
  pub oldest_commit: Option<String>,
  /// The path to the file being worked on. Path has to be relative to the
  /// repo root.
  pub path: Option<String>,
  /// Track lines that have moved within a file. This is the git-blame -M
  /// option.
  pub track_lines_movement: Option<bool>,
  /// Restrict search to commits reachable following only first parents.
  pub first_parent: Option<bool>,
  /// Ignore whitespace differences.
  pub ignore_whitespace: Option<bool>,
  /// Track lines that have been copied from another file that exists in any commit.
  pub track_copies_any_commit_copies: Option<bool>,
  /// Track lines that have been copied from another file that exists in the same commit.
  pub track_copies_same_commit_copies: Option<bool>,
  /// Track lines that have moved across files in the same commit.
  pub track_copies_same_commit_moves: Option<bool>,
  /// Use mailmap file to map author and committer names and email addresses to canonical real names and email addresses.
  pub use_mailmap: Option<bool>,
}

/// A wrapper around git2::Blame providing Node.js bindings
#[napi]
pub struct Blame {
  pub(crate) inner: BlameInner,
}

/// Inner implementation of Blame that handles shared references
pub(crate) enum BlameInner {
  Repo(SharedReference<Repository, git2::Blame<'static>>),
}

/// An iterator over blame hunks.
#[napi(iterator)]
pub struct BlameHunks {
  pub(crate) inner: Blame,
  pub(crate) current_index: u32,
  pub(crate) total_count: u32,
}

/// Iterator over blame hunks collected line by line.
#[napi(iterator)]
pub struct BlameHunksByLine {
  pub(crate) inner: Blame,
  pub(crate) current_line: u32,
  pub(crate) processed_hunks: HashSet<(u32, u32)>,
  pub(crate) total_hunk_count: usize,
}

impl From<&BlameOptions> for git2::BlameOptions {
  fn from(options: &BlameOptions) -> Self {
    let mut git_opts = git2::BlameOptions::new();

    if let Some(min_line) = options.min_line {
      git_opts.min_line(min_line as usize);
    }

    if let Some(max_line) = options.max_line {
      git_opts.max_line(max_line as usize);
    }

    if let Some(ref newest_commit) = options.newest_commit {
      if let Ok(oid) = git2::Oid::from_str(newest_commit) {
        git_opts.newest_commit(oid);
      }
    }

    if let Some(ref oldest_commit) = options.oldest_commit {
      if let Ok(oid) = git2::Oid::from_str(oldest_commit) {
        git_opts.oldest_commit(oid);
      }
    }

    if let Some(track_lines_movement) = options.track_lines_movement {
      git_opts.track_copies_same_file(track_lines_movement);
    }

    if let Some(first_parent) = options.first_parent {
      git_opts.first_parent(first_parent);
    }

    if let Some(ignore_whitespace) = options.ignore_whitespace {
      git_opts.ignore_whitespace(ignore_whitespace);
    }

    if let Some(track_copies_any_commit_copies) = options.track_copies_any_commit_copies {
      git_opts.track_copies_any_commit_copies(track_copies_any_commit_copies);
    }

    if let Some(track_copies_same_commit_copies) = options.track_copies_same_commit_copies {
      git_opts.track_copies_same_commit_copies(track_copies_same_commit_copies);
    }

    if let Some(track_copies_same_commit_moves) = options.track_copies_same_commit_moves {
      git_opts.track_copies_same_commit_moves(track_copies_same_commit_moves);
    }

    if let Some(use_mailmap) = options.use_mailmap {
      git_opts.use_mailmap(use_mailmap);
    }

    git_opts
  }
}

impl Deref for BlameInner {
  type Target = git2::Blame<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(repo) => repo.deref(),
    }
  }
}

impl BlameInner {
  fn clone_with_env(&self, env: Env) -> crate::Result<Self> {
    match self {
      Self::Repo(repo) => Ok(Self::Repo(repo.clone(env)?)),
    }
  }
}

impl<'a> From<&git2::BlameHunk<'a>> for BlameHunk {
  fn from(hunk: &git2::BlameHunk<'a>) -> Self {
    let final_oid = hunk.final_commit_id();
    let final_commit_id = final_oid.to_string();
    let is_final_zero_commit = final_oid.is_zero();

    let final_signature = if is_final_zero_commit {
      None
    } else {
      Signature::try_from(hunk.final_signature()).ok()
    };

    let orig_oid = hunk.orig_commit_id();
    let orig_commit_id = orig_oid.to_string();
    let is_orig_zero_commit = orig_oid.is_zero();

    let orig_signature = if is_orig_zero_commit {
      None
    } else {
      Signature::try_from(hunk.orig_signature()).ok()
    };

    let path = hunk.path().map(|p| p.to_string_lossy().to_string());

    BlameHunk {
      final_commit_id,
      final_start_line_number: hunk.final_start_line() as u32,
      lines_in_hunk: hunk.lines_in_hunk() as u32,
      final_signature,
      path,
      orig_start_line_number: hunk.orig_start_line() as u32,
      orig_commit_id,
      orig_signature,
      is_boundary: hunk.is_boundary(),
    }
  }
}

#[napi]
impl Generator for BlameHunks {
  type Yield = BlameHunk;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    if self.current_index >= self.total_count {
      return None;
    }

    let result = self.inner.get_hunk_by_index(self.current_index).ok();
    self.current_index += 1;

    result
  }
}

#[napi]
impl Generator for BlameHunksByLine {
  type Yield = BlameHunk;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    if self.processed_hunks.len() >= self.total_hunk_count {
      return None;
    }

    while self.processed_hunks.len() < self.total_hunk_count {
      if let Ok(hunk) = self.inner.get_hunk_by_line(self.current_line) {
        let hunk_key = (hunk.final_start_line_number, hunk.lines_in_hunk);

        if self.processed_hunks.insert(hunk_key) {
          self.current_line = hunk.final_start_line_number + hunk.lines_in_hunk;
          return Some(hunk);
        }
      }

      self.current_line += 1;

      if self.processed_hunks.len() >= self.total_hunk_count {
        break;
      }
    }

    None
  }
}

#[napi]
impl Blame {
  #[napi]
  /// Gets the number of hunks in the blame result
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   getHunkCount(): number;
  /// }
  /// ```
  ///
  /// @returns The number of hunks in the blame result
  pub fn get_hunk_count(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  /// Checks if the blame result is empty
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   isEmpty(): boolean;
  /// }
  /// ```
  ///
  /// @returns True if the blame result contains no hunks
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  /// Gets blame information for the specified index
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   getHunkByIndex(index: number): BlameHunk;
  /// }
  /// ```
  ///
  /// @param {number} index - Index of the hunk to get (0-based)
  /// @returns Blame information for the specified index
  /// @throws If no hunk is found at the index
  pub fn get_hunk_by_index(&self, index: u32) -> crate::Result<BlameHunk> {
    let hunk = self
      .inner
      .get_index(index as usize)
      .ok_or_else(|| Error::new(Status::InvalidArg, format!("No blame hunk found at index {index}")))?;

    Ok(BlameHunk::from(&hunk))
  }

  #[napi]
  /// Gets blame information for the specified line
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   getHunkByLine(line: number): BlameHunk;
  /// }
  /// ```
  ///
  /// @param {number} line - Line number to get blame information for (1-based)
  /// @returns Blame information for the specified line
  /// @throws If no hunk is found for the line
  pub fn get_hunk_by_line(&self, line: u32) -> crate::Result<BlameHunk> {
    let hunk = self
      .inner
      .get_line(line as usize)
      .ok_or_else(|| Error::new(Status::InvalidArg, format!("No blame hunk found for line {line}")))?;

    Ok(BlameHunk::from(&hunk))
  }

  #[napi(iterator)]
  /// Gets all blame hunks as an iterator
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   iter(): Generator<BlameHunk>;
  /// }
  /// ```
  ///
  /// @returns Iterator of all blame hunks
  /// @example
  /// ```ts
  /// // Using for...of loop
  /// for (const hunk of blame.iter()) {
  ///   console.log(hunk.finalCommitId);
  /// }
  ///
  /// // Using spread operator to collect all hunks
  /// const hunks = [...blame.iter()];
  /// ```
  pub fn iter(&self, env: Env) -> crate::Result<BlameHunks> {
    let inner = Blame {
      inner: self.inner.clone_with_env(env)?,
    };

    Ok(BlameHunks {
      inner,
      current_index: 0,
      total_count: self.get_hunk_count(),
    })
  }

  #[napi(iterator)]
  /// Collects blame hunks by scanning file lines as an iterator
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   iterByLine(): Generator<BlameHunk>;
  /// }
  /// ```
  ///
  /// @returns Iterator of blame hunks collected by line scanning
  /// @example
  /// ```ts
  /// // Using for...of loop
  /// for (const hunk of blame.iterByLine()) {
  ///   console.log(hunk.finalCommitId);
  /// }
  ///
  /// // Using spread operator to collect all hunks
  /// const hunks = [...blame.iterByLine()];
  /// ```
  pub fn iter_by_line(&self, env: Env) -> crate::Result<BlameHunksByLine> {
    let inner = Blame {
      inner: self.inner.clone_with_env(env)?,
    };

    Ok(BlameHunksByLine {
      inner,
      current_line: 1,
      processed_hunks: HashSet::new(),
      total_hunk_count: self.get_hunk_count() as usize,
    })
  }

  #[napi]
  /// Generates blame information from an in-memory buffer
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   buffer(buffer: Buffer): Blame;
  /// }
  /// ```
  ///
  /// @example
  /// ```ts
  /// const buffer = Buffer.from('modified content');
  /// const bufferBlame = blame.buffer(buffer);
  /// ```
  ///
  /// @param {Buffer} buffer - Buffer containing file content to blame
  /// @returns A new Blame object for the buffer content
  pub fn buffer(&self, buffer: Buffer, env: Env) -> crate::Result<Blame> {
    let blame = match &self.inner {
      BlameInner::Repo(shared_ref) => {
        let cloned = shared_ref.clone(env)?;

        cloned.share_with(env, |git_blame| {
          git_blame
            .blame_buffer(buffer.as_ref())
            .map_err(|e| crate::Error::from(e).into())
        })?
      }
    };

    Ok(Blame {
      inner: BlameInner::Repo(blame),
    })
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Creates a blame object for the file at the given path
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   blameFile(path: string, options?: BlameOptions): Blame;
  /// }
  /// ```
  ///
  /// @example
  /// ```ts
  /// // Blame the entire file
  /// const blame = repo.blameFile('path/to/file.js');
  ///
  /// // Blame a single line
  /// const lineBlame = repo.blameFile('path/to/file.js', { minLine: 10, maxLine: 10 });
  ///
  /// // Blame a range of lines
  /// const rangeBlame = repo.blameFile('path/to/file.js', { minLine: 5, maxLine: 15 });
  /// ```
  ///
  /// @param {string} path - Path to the file to blame
  /// @param {BlameOptions} [options] - Options to control blame behavior
  /// @returns Blame object for the specified file
  /// @throws If the file doesn't exist or can't be opened
  pub fn blame_file(
    &self,
    path: String,
    options: Option<BlameOptions>,
    this: Reference<Repository>,
    env: Env,
  ) -> crate::Result<Blame> {
    let file_path = Path::new(&path);

    let blame = this.share_with(env, |repo| {
      let result = match &options {
        Some(options) => {
          let mut git_options = git2::BlameOptions::from(options);
          repo.inner.blame_file(file_path, Some(&mut git_options))
        }
        None => repo.inner.blame_file(file_path, None),
      };

      result.map_err(|e| crate::Error::from(e).into())
    })?;

    Ok(Blame {
      inner: BlameInner::Repo(blame),
    })
  }
}
