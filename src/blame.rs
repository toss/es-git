use crate::repository::Repository;
use crate::signature::Signature;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::Deref;
use std::path::Path;

const MAX_SCAN_LINES: u32 = 10000;

#[napi(object)]
/// Represents a hunk of a blame operation, which is a range of lines
/// and information about who last modified them.
pub struct BlameHunk {
  /// The oid of the commit where this line was last changed.
  pub commit_id: String,
  /// The 1-based line number in the final file where this hunk starts.
  pub final_start_line_number: u32,
  /// The number of lines in this hunk.
  pub lines_in_hunk: u32,
  /// The signature of the commit where this line was last changed.
  pub signature: Option<Signature>,
  /// The path to the file where this line was originally written.
  pub path: Option<String>,
  /// The 1-based line number in the original file where this hunk starts.
  pub orig_start_line_number: u32,
  /// True if the hunk has been determined to be a boundary commit (the commit
  /// when the file was first introduced to the repository).
  pub is_boundary: bool,
}

#[napi(object)]
#[derive(Default)]
/// Options for controlling blame behavior
pub struct BlameOptions {
  /// A single line to blame (1-based index)
  pub line: Option<u32>,
  /// An array of two numbers [start, end] to blame a range of lines
  pub range: Option<Vec<u32>>,
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

impl BlameOptions {
  /// Create new options for a single line
  pub fn for_line(line: u32) -> Self {
    Self {
      line: Some(line),
      range: None,
      newest_commit: None,
      oldest_commit: None,
      path: None,
      track_lines_movement: None,
      first_parent: None,
      ignore_whitespace: None,
      track_copies_any_commit_copies: None,
      track_copies_same_commit_copies: None,
      track_copies_same_commit_moves: None,
      use_mailmap: None,
    }
  }

  /// Create new options for a range of lines
  pub fn for_range(start: u32, end: u32) -> Self {
    Self {
      line: None,
      range: Some(vec![start, end]),
      newest_commit: None,
      oldest_commit: None,
      path: None,
      track_lines_movement: None,
      first_parent: None,
      ignore_whitespace: None,
      track_copies_any_commit_copies: None,
      track_copies_same_commit_copies: None,
      track_copies_same_commit_moves: None,
      use_mailmap: None,
    }
  }

  /// Set the path to the file
  pub fn with_path(mut self, path: &str) -> Self {
    self.path = Some(path.to_string());
    self
  }

  /// Set the newest commit to consider
  pub fn with_newest_commit(mut self, commit: &str) -> Self {
    self.newest_commit = Some(commit.to_string());
    self
  }

  /// Set the oldest commit to consider
  pub fn with_oldest_commit(mut self, commit: &str) -> Self {
    self.oldest_commit = Some(commit.to_string());
    self
  }

  /// Set whether to track line movements
  pub fn with_track_lines_movement(mut self, track: bool) -> Self {
    self.track_lines_movement = Some(track);
    self
  }

  /// Set whether to restrict search to commits reachable following only first parents
  pub fn with_first_parent(mut self, first_parent: bool) -> Self {
    self.first_parent = Some(first_parent);
    self
  }

  /// Set whether to ignore whitespace differences
  pub fn with_ignore_whitespace(mut self, ignore_whitespace: bool) -> Self {
    self.ignore_whitespace = Some(ignore_whitespace);
    self
  }

  /// Set whether to track lines that have been copied from another file that exists in any commit
  pub fn with_track_copies_any_commit_copies(mut self, track_copies_any_commit_copies: bool) -> Self {
    self.track_copies_any_commit_copies = Some(track_copies_any_commit_copies);
    self
  }

  /// Set whether to track lines that have been copied from another file that exists in the same commit
  pub fn with_track_copies_same_commit_copies(mut self, track_copies_same_commit_copies: bool) -> Self {
    self.track_copies_same_commit_copies = Some(track_copies_same_commit_copies);
    self
  }

  /// Set whether to track lines that have moved across files in the same commit
  pub fn with_track_copies_same_commit_moves(mut self, track_copies_same_commit_moves: bool) -> Self {
    self.track_copies_same_commit_moves = Some(track_copies_same_commit_moves);
    self
  }

  /// Set whether to use mailmap file to map author and committer names and email addresses
  pub fn with_use_mailmap(mut self, use_mailmap: bool) -> Self {
    self.use_mailmap = Some(use_mailmap);
    self
  }
}

impl From<&BlameOptions> for git2::BlameOptions {
  fn from(options: &BlameOptions) -> Self {
    let mut git_opts = git2::BlameOptions::new();

    if let Some(line) = options.line {
      git_opts.min_line(line as usize);
      git_opts.max_line(line as usize);
    } else if let Some(range) = &options.range {
      if range.len() >= 2 {
        git_opts.min_line(range[0] as usize);
        git_opts.max_line(range[1] as usize);
      }
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

#[napi]
pub struct Blame {
  pub(crate) inner: BlameInner,
}

pub(crate) enum BlameInner {
  Repo(SharedReference<Repository, git2::Blame<'static>>),
}

impl Deref for BlameInner {
  type Target = git2::Blame<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(repo) => repo.deref(),
    }
  }
}

/// Iterator over blame hunks.
pub struct BlameIter<'a> {
  blame: &'a Blame,
  idx: usize,
  len: usize,
}

impl Iterator for BlameIter<'_> {
  type Item = Result<BlameHunk>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.idx >= self.len {
      return None;
    }

    let result = self.blame.get_hunk_by_index(self.idx as u32);
    self.idx += 1;

    Some(result)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.len - self.idx;
    (remaining, Some(remaining))
  }
}

impl ExactSizeIterator for BlameIter<'_> {
  fn len(&self) -> usize {
    self.len - self.idx
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
  /// @returns true if the blame result contains no hunks, false otherwise
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  /// Returns an iterator over the hunks in this blame.
  ///
  /// This internal method is used to implement getHunks() and is not
  /// directly exposed to JavaScript.
  pub fn iter(&self) -> BlameIter {
    BlameIter {
      blame: self,
      idx: 0,
      len: self.get_hunk_count() as usize,
    }
  }

  #[napi]
  /// Generates blame information from an in-memory buffer
  ///
  /// This method allows generating blame information for content that exists in memory
  /// rather than in a file on disk.
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   buffer(buffer: Buffer, buffer_len: number): Blame;
  /// }
  /// ```
  ///
  /// @example
  /// ```ts
  /// // Get blame for a file
  /// const blame = repo.blameFile('path/to/file.js');
  ///
  /// // Then create a modified buffer with some changes
  /// const buffer = Buffer.from('modified content');
  ///
  /// // Get blame for the modified content
  /// const bufferBlame = blame.buffer(buffer, buffer.length);
  /// ```
  ///
  /// @param {Buffer} buffer - The buffer containing file content to blame
  /// @param {number} buffer_len - The length of the buffer in bytes
  /// @returns A new Blame object for the buffer content
  pub fn buffer(&self, buffer: Buffer, buffer_len: u32, env: Env) -> Result<Blame> {
    let content = std::str::from_utf8(&buffer[..buffer_len as usize])
      .map_err(|e| Error::from_reason(format!("Invalid UTF-8 in buffer: {}", e)))?;

    let blame = match &self.inner {
      BlameInner::Repo(shared_ref) => {
        let cloned = shared_ref.clone(env)?;

        cloned.share_with(env, |git_blame| {
          git_blame
            .blame_buffer(content.as_bytes())
            .map_err(|e| Error::from(crate::Error::from(e)))
        })?
      }
    };

    Ok(Blame {
      inner: BlameInner::Repo(blame),
    })
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
  /// @param {number} index - The index of the hunk to get (0-based)
  /// @returns Blame information for the specified index
  /// @throws If no hunk is found for the specified index
  pub fn get_hunk_by_index(&self, index: u32) -> Result<BlameHunk> {
    let hunk = self
      .inner
      .get_index(index as usize)
      .ok_or_else(|| Error::from_reason(format!("No blame hunk found at index {}", index)))?;

    let commit_id = hunk.final_commit_id().to_string();
    let is_zero_commit = commit_id == "0000000000000000000000000000000000000000";

    let signature = if is_zero_commit {
      None
    } else {
      Signature::try_from(hunk.final_signature()).ok()
    };

    let path = hunk.path().map(|p| p.to_string_lossy().to_string());

    Ok(BlameHunk {
      commit_id,
      final_start_line_number: hunk.final_start_line() as u32,
      lines_in_hunk: hunk.lines_in_hunk() as u32,
      signature,
      path,
      orig_start_line_number: hunk.orig_start_line() as u32,
      is_boundary: hunk.is_boundary(),
    })
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
  /// @param {number} line - The line number to get blame information for (1-based)
  /// @returns Blame information for the specified line
  /// @throws If no hunk is found for the specified line
  pub fn get_hunk_by_line(&self, line: u32) -> Result<BlameHunk> {
    let hunk = self
      .inner
      .get_line(line as usize)
      .ok_or_else(|| Error::from_reason(format!("No blame hunk found for line {}", line)))?;

    let commit_id = hunk.final_commit_id().to_string();
    let is_zero_commit = commit_id == "0000000000000000000000000000000000000000";

    let signature = if is_zero_commit {
      None
    } else {
      Signature::try_from(hunk.final_signature()).ok()
    };

    let path = hunk.path().map(|p| p.to_string_lossy().to_string());

    Ok(BlameHunk {
      commit_id,
      final_start_line_number: hunk.final_start_line() as u32,
      lines_in_hunk: hunk.lines_in_hunk() as u32,
      signature,
      path,
      orig_start_line_number: hunk.orig_start_line() as u32,
      is_boundary: hunk.is_boundary(),
    })
  }

  #[napi]
  /// Gets all blame hunks by index
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   getHunks(): BlameHunk[];
  /// }
  /// ```
  ///
  /// @returns An array of all blame hunks
  pub fn get_hunks(&self) -> Result<Vec<BlameHunk>> {
    let hunk_count = self.get_hunk_count() as usize;

    if hunk_count == 0 {
      return Ok(Vec::new());
    }

    // Use the iterator to collect hunks
    self.iter().collect()
  }

  #[napi]
  /// Iterates through each hunk in the blame result and calls the callback function for each one.
  /// Returns true to continue iteration, false to stop.
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   forEachHunk(callback: (hunk: BlameHunk, index: number) => boolean): void;
  /// }
  /// ```
  ///
  /// @example
  /// ```ts
  /// // Process each hunk individually
  /// blame.forEachHunk((hunk, index) => {
  ///   console.log(`Hunk ${index}: ${hunk.commitId}`);
  ///   // Return true to continue, false to stop iteration
  ///   return true;
  /// });
  /// ```
  ///
  /// @param {Function} callback - A function to be called for each hunk. 
  ///        Return true to continue iteration, false to stop.
  pub fn for_each_hunk(&self, callback: Function<(BlameHunk, u32), bool>) -> crate::Result<()> {
    let hunk_count = self.get_hunk_count();
    
    for idx in 0..hunk_count {
      if let Ok(hunk) = self.get_hunk_by_index(idx) {
        if !callback.call((hunk, idx)).unwrap_or(false) {
          break;
        }
      }
    }
    
    Ok(())
  }

  #[napi]
  /// Scans through file lines to collect blame hunks
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   getHunksByLine(): BlameHunk[];
  /// }
  /// ```
  ///
  /// @returns An array of blame hunks collected by scanning file lines
  pub fn get_hunks_by_line(&self) -> Result<Vec<BlameHunk>> {
    let hunk_count = self.get_hunk_count() as usize;

    if hunk_count == 0 {
      return Ok(Vec::new());
    }

    let mut hunks = Vec::new();
    let mut seen_hunks = HashSet::new();
    let mut line = 1;

    while line < MAX_SCAN_LINES {
      if let Ok(hunk) = self.get_hunk_by_line(line) {
        let hunk_key = (hunk.final_start_line_number, hunk.lines_in_hunk);

        if seen_hunks.insert(hunk_key) {
          hunks.push(BlameHunk {
            commit_id: hunk.commit_id,
            final_start_line_number: hunk.final_start_line_number,
            lines_in_hunk: hunk.lines_in_hunk,
            signature: hunk.signature,
            path: hunk.path,
            orig_start_line_number: hunk.orig_start_line_number,
            is_boundary: hunk.is_boundary,
          });
          line = hunk.final_start_line_number + hunk.lines_in_hunk;
          continue;
        }
      }

      line += 1;

      if line >= MAX_SCAN_LINES || hunks.len() >= hunk_count {
        break;
      }
    }

    Ok(hunks)
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Get a blame object for the file at the given path with all configurable options
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   blameFile(path: string, options?: BlameOptions | null | undefined): Blame;
  /// }
  /// ```
  ///
  /// @example
  /// ```ts
  /// // Blame the entire file
  /// const blame = repo.blameFile('path/to/file.js');
  ///
  /// // Blame a single line (line 10)
  /// const lineBlame = repo.blameFile('path/to/file.js', { line: 10 });
  ///
  /// // Blame a range of lines (lines 5-15)
  /// const rangeBlame = repo.blameFile('path/to/file.js', { range: [5, 15] });
  /// ```
  ///
  /// @param {string} path - Path to the file to blame. This path takes precedence over any path specified in options.
  /// @param {BlameOptions} [options] - Options to control blame behavior.
  ///        You can specify line ranges in two ways:
  ///        1. `options.line`: A single line number to blame
  ///        2. `options.range`: An array of two numbers [start, end] to blame a range
  /// @returns Blame object for the specified file
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

      result.map_err(|e| Error::from(crate::Error::from(e)))
    })?;

    Ok(Blame {
      inner: BlameInner::Repo(blame),
    })
  }
}
