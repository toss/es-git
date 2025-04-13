use crate::repository::Repository;
use crate::signature::Signature;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::Deref;
use std::path::Path;

const MAX_SCAN_LINES: u32 = 10000;

#[derive(Clone)]
/// How blame ranges should be interpreted when blaming a file
pub enum BlameRangeMode {
  /// No line range specified, blame the entire file
  None,
  /// Blame only the single specified line
  SingleLine,
  /// Blame a range of lines from min to max inclusive
  Range,
}

impl Default for BlameRangeMode {
  fn default() -> Self {
    Self::None
  }
}

/// Represents a range of lines for applying blame
#[derive(Default, Clone)]
pub struct BlameLineRange {
  /// The type of line range
  pub range_type: BlameRangeMode,
  /// Start line of the range (1-based index)
  pub start: u32,
  /// End line of the range (1-based index), only used when range_type is Range
  pub end: Option<u32>,
}

impl BlameLineRange {
  /// Create a new line range for a single line
  pub fn for_line(line: u32) -> Self {
    Self {
      range_type: BlameRangeMode::SingleLine,
      start: line,
      end: None,
    }
  }

  /// Create a new line range from start to end (inclusive)
  pub fn for_range(start: u32, end: u32) -> Self {
    Self {
      range_type: BlameRangeMode::Range,
      start,
      end: Some(end),
    }
  }
}

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
}

impl BlameOptions {
  /// Create new options for a single line
  pub fn for_line(line: u32) -> Self {
    let mut options = Self::default();
    options.line = Some(line);
    options
  }

  /// Create new options for a range of lines
  pub fn for_range(start: u32, end: u32) -> Self {
    let mut options = Self::default();
    options.range = Some(vec![start, end]);
    options
  }

  fn get_effective_line_range(&self) -> Option<BlameLineRange> {
    if let Some(range) = &self.range {
      if range.len() >= 2 {
        return Some(BlameLineRange::for_range(range[0], range[1]));
      }
    }
    
    if let Some(line) = self.line {
      return Some(BlameLineRange::for_line(line));
    }
    
    None
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
}

impl From<&BlameOptions> for git2::BlameOptions {
  fn from(options: &BlameOptions) -> Self {
    let mut git_opts = git2::BlameOptions::new();

    if let Some(line_range) = options.get_effective_line_range() {
      match line_range.range_type {
        BlameRangeMode::SingleLine => {
          git_opts.min_line(line_range.start as usize);
          git_opts.max_line(line_range.start as usize);
        }
        BlameRangeMode::Range => {
          git_opts.min_line(line_range.start as usize);
          if let Some(end) = line_range.end {
            git_opts.max_line(end as usize);
          }
        }
        BlameRangeMode::None => {}
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

    git_opts
  }
}

#[napi]
/// A class representing a git blame analysis result
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

    let signature = Signature::try_from(hunk.final_signature()).ok();
    let path = hunk.path().map(|p| p.to_string_lossy().to_string());

    Ok(BlameHunk {
      commit_id: hunk.final_commit_id().to_string(),
      final_start_line_number: hunk.final_start_line() as u32,
      lines_in_hunk: hunk.lines_in_hunk() as u32,
      signature,
      path,
      orig_start_line_number: hunk.orig_start_line() as u32,
      is_boundary: hunk.is_boundary(),
    })
  }

  #[napi]
  /// Gets an array of blame hunks for all lines
  ///
  /// @category Blame/Methods
  /// @signature
  /// ```ts
  /// class Blame {
  ///   getHunks(): BlameHunk[];
  /// }
  /// ```
  ///
  /// @returns Array of blame hunks
  pub fn get_hunks(&self) -> Result<Vec<BlameHunk>> {
    let hunk_count = self.get_hunk_count() as usize;
    
    if hunk_count == 0 {
      return Ok(Vec::new());
    }
    
    let mut hunks = Vec::with_capacity(hunk_count);
    let mut seen_hunks = HashSet::new();
    let mut line = 1;
    
    while hunks.len() < hunk_count && line < MAX_SCAN_LINES {
      if let Ok(hunk) = self.get_hunk_by_line(line) {
        let hunk_key = (hunk.final_start_line_number, hunk.lines_in_hunk);
        
        if seen_hunks.insert(hunk_key) {
          line += hunk.lines_in_hunk;
          hunks.push(hunk);
          continue;
        }
      }
      
      line += 1;
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
