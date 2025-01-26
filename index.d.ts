/* tslint:disable */
/* eslint-disable */

/**
 * TODO:
 * napi does not support union types when converting rust enum types to TypeScript.
 * This feature will be provided starting from v3, so create a custom TypeScript until the v3 stable releases.
 */
export const enum DiffFlags {
  /** File(s) treated as binary data. (1 << 0) */
  Binary = 1,
  /** File(s) treated as text data. (1 << 1) */
  NotBinary = 2,
  /** `id` value is known correct. (1 << 2) */
  ValidId = 4,
  /** File exists at this side of the delta. (1 << 3) */
  Exists = 8
}
/** Check diff flags contains given flags. */
export declare function diffFlagsContains(source: number, target: number): boolean
/** What type of change is described by a `DiffDelta`? */
export const enum DeltaType {
  /** No changes */
  Unmodified = 'Unmodified',
  /** Entry does not exist in old version */
  Added = 'Added',
  /** Entry does not exist in new version */
  Deleted = 'Deleted',
  /** Entry content changed between old and new */
  Modified = 'Modified',
  /** Entry was renamed between old and new */
  Renamed = 'Renamed',
  /** Entry was copied from another old entry */
  Copied = 'Copied',
  /** Entry is ignored item in workdir */
  Ignored = 'Ignored',
  /** Entry is untracked item in workdir */
  Untracked = 'Untracked',
  /** Type of entry changed between old and new */
  Typechange = 'Typechange',
  /** Entry is unreadable */
  Unreadable = 'Unreadable',
  /** Entry in the index is conflicted */
  Conflicted = 'Conflicted'
}
/** Formatting options for diff stats */
export const enum DiffStatsFormat {
  /** Don't generate any stats */
  None = 0,
  /** Equivalent of `--stat` in git (1 << 0) */
  Full = 1,
  /** Equivalent of `--shortstat` in git (1 << 1) */
  Short = 2,
  /** Equivalent of `--numstat` in git (1 << 2) */
  Number = 4,
  /**
   * Extended header information such as creations, renames and mode
   * changes, equivalent of `--summary` in git
   * (1 << 3)
   */
  IncludeSummary = 8
}
export interface PrintDiffStatsOptions {
  format?: number;
  width?: number;
}
/** Valid modes for index and tree entries. */
export const enum FileMode {
  Unreadable = 'Unreadable',
  Tree = 'Tree',
  Blob = 'Blob',
  /** Group writable blob. Obsolete mode kept for compatibility reasons */
  BlobGroupWritable = 'BlobGroupWritable',
  BlobExecutable = 'BlobExecutable',
  Link = 'Link',
  Commit = 'Commit'
}
/** Structure describing options about how the diff should be executed. */
export interface DiffOptions {
  /** Flag indicating whether the sides of the diff will be reversed. */
  reverse?: boolean;
  /** Flag indicating whether ignored files are included. */
  includeIgnored?: boolean;
  /** Flag indicating whether ignored directories are traversed deeply or not. */
  recurseIgnoredDirs?: boolean;
  /** Flag indicating whether untracked files are in the diff */
  includeUntracked?: boolean;
  /**
   * Flag indicating whether untracked directories are traversed deeply or
   * not.
   */
  recurseUntrackedDirs?: boolean;
  /** Flag indicating whether unmodified files are in the diff. */
  includeUnmodified?: boolean;
  /** If enabled, then Typechange delta records are generated. */
  includeTypechange?: boolean;
  /**
   * Event with `includeTypechange`, the tree returned generally shows a
   * deleted blob. This flag correctly labels the tree transitions as a
   * typechange record with the `new_file`'s mode set to tree.
   *
   * Note that the tree SHA will not be available.
   */
  includeTypechangeTrees?: boolean;
  /** Flag indicating whether file mode changes are ignored. */
  ignoreFilemode?: boolean;
  /** Flag indicating whether all submodules should be treated as unmodified. */
  ignoreSubmodules?: boolean;
  /** Flag indicating whether case insensitive filenames should be used. */
  ignoreCase?: boolean;
  /**
   * If pathspecs are specified, this flag means that they should be applied
   * as an exact match instead of a fnmatch pattern.
   */
  disablePathspecMatch?: boolean;
  /**
   * Disable updating the `binary` flag in delta records. This is useful when
   * iterating over a diff if you don't need hunk and data callbacks and want
   * to avoid having to load a file completely.
   */
  skipBinaryCheck?: boolean;
  /**
   * When diff finds an untracked directory, to match the behavior of core
   * Git, it scans the contents for ignored and untracked files. If all
   * contents are ignored, then the directory is ignored; if any contents are
   * not ignored, then the directory is untracked. This is extra work that
   * may not matter in many cases.
   *
   * This flag turns off that scan and immediately labels an untracked
   * directory as untracked (changing the behavior to not match core git).
   */
  enableFastUntrackedDirs?: boolean;
  /**
   * When diff finds a file in the working directory with stat information
   * different from the index, but the OID ends up being the same, write the
   * correct stat information into the index. Note: without this flag, diff
   * will always leave the index untouched.
   */
  updateIndex?: boolean;
  /** Include unreadable files in the diff */
  includeUnreadable?: boolean;
  /** Include unreadable files in the diff as untracked files */
  includeUnreadableAsUntracked?: boolean;
  /** Treat all files as text, disabling binary attributes and detection. */
  forceText?: boolean;
  /** Treat all files as binary, disabling text diffs */
  forceBinary?: boolean;
  /** Ignore all whitespace */
  ignoreWhitespace?: boolean;
  /** Ignore changes in the amount of whitespace */
  ignoreWhitespaceChange?: boolean;
  /** Ignore whitespace at the end of line */
  ignoreWhitespaceEol?: boolean;
  /** Ignore blank lines */
  ignoreBlankLines?: boolean;
  /**
   * When generating patch text, include the content of untracked files.
   *
   * This automatically turns on `includeUntracked` but it does not turn on
   * `recurseUntrackedDirs`. Add that flag if you want the content of every
   * single untracked file.
   */
  showUntrackedContent?: boolean;
  /**
   * When generating output, include the names of unmodified files if they
   * are included in the `Diff`. Normally these are skipped in the formats
   * that list files (e.g. name-only, name-status, raw). Even with this these
   * will not be included in the patch format.
   */
  showUnmodified?: boolean;
  /** Use the "patience diff" algorithm */
  patience?: boolean;
  /** Take extra time to find the minimal diff */
  minimal?: boolean;
  /**
   * Include the necessary deflate/delta information so that `git-apply` can
   * apply given diff information to binary files.
   */
  showBinary?: boolean;
  /**
   * Use a heuristic that takes indentation and whitespace into account
   * which generally can produce better diffs when dealing with ambiguous
   * diff hunks.
   */
  indentHeuristic?: boolean;
  /**
   * Set the number of unchanged lines that define the boundary of a hunk
   * (and to display before and after).
   *
   * The default value for this is 3.
   */
  contextLines?: number;
  /**
   * Set the maximum number of unchanged lines between hunk boundaries before
   * the hunks will be merged into one.
   *
   * The default value for this is 0.
   */
  interhunkLines?: number;
  /** The default value for this is `core.abbrev` or 7 if unset. */
  idAbbrev?: number;
  /**
   * Maximum size (in bytes) above which a blob will be marked as binary
   * automatically.
   *
   * A negative value will disable this entirely.
   *
   * The default value for this is 512MB.
   */
  maxSize?: number;
  /**
   * The virtual "directory" to prefix old file names with in hunk headers.
   *
   * The default value for this is "a".
   */
  oldPrefix?: string;
  /**
   * The virtual "directory" to prefix new file names with in hunk headers.
   *
   * The default value for this is "b".
   */
  newPrefix?: string;
  /** Add to the array of paths/fnmatch patterns to constrain the diff. */
  pathspecs?: Array<string>;
}
export interface IndexEntry {
  ctime: Date;
  mtime: Date;
  dev: number;
  ino: number;
  mode: number;
  uid: number;
  gid: number;
  fileSize: number;
  id: string;
  flags: number;
  flagsExtended: number;
  /**
   * The path of this index entry as a byte vector. Regardless of the
   * current platform, the directory separator is an ASCII forward slash
   * (`0x2F`). There are no terminating or internal NUL characters, and no
   * trailing slashes. Most of the time, paths will be valid utf-8 — but
   * not always. For more information on the path storage format, see
   * [these git docs][git-index-docs]. Note that libgit2 will take care of
   * handling the prefix compression mentioned there.
   *
   * [git-index-docs]: https://github.com/git/git/blob/a08a83db2bf27f015bec9a435f6d73e223c21c5e/Documentation/technical/index-format.txt#L107-L124
   */
  path: Buffer;
}
export interface IndexOnMatchCallbackArgs {
  /** The path of entry. */
  path: string;
  /** The patchspec that matched it. */
  pathspec: string;
}
export interface IndexAddAllOptions {
  /**
   * Files that are ignored will be skipped (unlike `addPath`). If a file is
   * already tracked in the index, then it will be updated even if it is
   * ignored. Pass the `force` flag to skip the checking of ignore rules.
   */
  force?: boolean;
  /**
   * The `pathspecs` are a list of file names or shell glob patterns that
   * will matched against files in the repository's working directory. Each
   * file that matches will be added to the index (either updating an
   * existing entry or adding a new entry). You can disable glob expansion
   * and force exact matching with the `disablePathspecMatch` flag.
   */
  disablePathspecMatch?: boolean;
  /**
   * To emulate `git add -A` and generate an error if the pathspec contains
   * the exact path of an ignored file (when not using `force`), add the
   * `checkPathspec` flag. This checks that each entry in `pathspecs`
   * that is an exact match to a filename on disk is either not ignored or
   * already in the index. If this check fails, the function will return
   * an error.
   */
  checkPathspec?: boolean;
  /**
   * If you provide a callback function, it will be invoked on each matching
   * item in the working directory immediately before it is added to /
   * updated in the index. Returning zero will add the item to the index,
   * greater than zero will skip the item, and less than zero will abort the
   * scan an return an error to the caller.
   */
  onMatch?: (args: IndexOnMatchCallbackArgs) => number;
}
export type IndexStage =
/** Match any index stage. */
  | 'Any'
  /** A normal staged file in the index. (default) */
  | 'Normal'
  /** The ancestor side of a conflict. */
  | 'Ancestor'
  /** The "ours" side of a conflict. */
  | 'Ours'
  /** The "theirs" side of a conflict. */
  | 'Theirs';
export interface IndexRemoveOptions {
  stage?: IndexStage;
}
export interface IndexRemoveAllOptions {
  /**
   * If you provide a callback function, it will be invoked on each matching
   * item in the index immediately before it is removed. Return 0 to remove
   * the item, > 0 to skip the item, and < 0 to abort the scan.
   */
  onMatch?: (args: IndexOnMatchCallbackArgs) => number;
}
export interface IndexUpdateAllOptions {
  /**
   * If you provide a callback function, it will be invoked on each matching
   * item in the index immediately before it is updated (either refreshed or
   * removed depending on working directory state). Return 0 to proceed with
   * updating the item, > 0 to skip the item, and < 0 to abort the scan.
   */
  onMatch?: (args: IndexOnMatchCallbackArgs) => number;
}
/**
 * Ensure the reference name is well-formed.
 *
 * Validation is performed as if [`ReferenceFormat::ALLOW_ONELEVEL`]
 * was given to [`Reference::normalize_name`]. No normalization is
 * performed, however.
 *
 * ```ts
 * import { isReferenceNameValid } from 'es-git';
 *
 * console.assert(isReferenceNameValid("HEAD"));
 * console.assert(isReferenceNameValid("refs/heads/main"));
 *
 * // But:
 * console.assert(!isReferenceNameValid("main"));
 * console.assert(!isReferenceNameValid("refs/heads/*"));
 * console.assert(!isReferenceNameValid("foo//bar"));
 * ```
 */
export declare function isReferenceNameValid(refname: string): boolean
/** An enumeration of all possible kinds of references. */
export type ReferenceType =
/** A reference which points at an object id. */
  | 'Direct'
  /** A reference which points at another reference. */
  | 'Symbolic';
/** Options for normalize reference name. */
export const enum ReferenceFormat {
  /** No particular normalization. */
  Normal = 0,
  /**
   * Control whether one-level refname are accepted (i.e., refnames that
   * do not contain multiple `/`-separated components). Those are
   * expected to be written only using uppercase letters and underscore
   * (e.g. `HEAD`, `FETCH_HEAD`).
   * (1 << 0)
   */
  AllowOnelevel = 1,
  /**
   * Interpret the provided name as a reference pattern for a refspec (as
   * used with remote repositories). If this option is enabled, the name
   * is allowed to contain a single `*` in place of a full pathname
   * components (e.g., `foo/*\/bar` but not `foo/bar*`).
   * (1 << 1)
   */
  RefspecPattern = 2,
  /**
   * Interpret the name as part of a refspec in shorthand form so the
   * `AllowOnelevel` naming rules aren't enforced and `main` becomes a
   * valid name.
   * (1 << 2)
   */
  RefspecShorthand = 4
}
/**
 * Normalize reference name and check validity.
 *
 * This will normalize the reference name by collapsing runs of adjacent
 * slashes between name components into a single slash. It also validates
 * the name according to the following rules:
 *
 * 1. If `ReferenceFormat.AllowOnelevel` is given, the name may
 *    contain only capital letters and underscores, and must begin and end
 *    with a letter. (e.g. "HEAD", "ORIG_HEAD").
 * 2. The flag `ReferenceFormat.RefspecShorthand` has an effect
 *    only when combined with `ReferenceFormat.AllowOnelevel`. If
 *    it is given, "shorthand" branch names (i.e. those not prefixed by
 *    `refs/`, but consisting of a single word without `/` separators)
 *    become valid. For example, "main" would be accepted.
 * 3. If `ReferenceFormat.RefspecPattern` is given, the name may
 *    contain a single `*` in place of a full pathname component (e.g.
 *    `foo/*\/bar`, `foo/bar*`).
 * 4. Names prefixed with "refs/" can be almost anything. You must avoid
 *    the characters '~', '^', ':', '\\', '?', '[', and '*', and the
 *    sequences ".." and "@{" which have special meaning to revparse.
 *
 * If the reference passes validation, it is returned in normalized form,
 * otherwise an `null` is returned.
 *
 * @example
 * ```ts
 * import { normalizeReferenceName, ReferenceFormat } from 'es-git';
 *
 * console.assert(
 *   normalizeReferenceName('foo//bar"),
 *   'foo/bar'
 * );
 * console.assert(
 *   normalizeReferenceName(
 *     'HEAD',
 *     ReferenceFormat.AllowOnelevel
 *   ),
 *   'HEAD'
 * );
 * console.assert(
 *   normalizeReferenceName(
 *     'refs/heads/*',
 *     ReferenceFormat.RefspecPattern
 *   ),
 *   'refs/heads/*'
 * );
 * console.assert(
 *   normalizeReferenceName(
 *     'main',
 *     ReferenceFormat.AllowOnelevel | ReferenceFormat.RefspecShorthand
 *   ),
 *   'main'
 * );
 * ```
 */
export declare function normalizeReferenceName(refname: string, format?: number | undefined | null): string | null
export interface RenameReferenceOptions {
  /**
   * If the force flag is not enabled, and there's already a reference with
   * the given name, the renaming will fail.
   */
  force?: boolean;
  logMessage?: string;
}
export type Direction =
  | 'Fetch'
  | 'Push';
export interface RefspecObject {
  direction: Direction;
  src: string;
  dst: string;
  force: boolean;
}
export type Credential =
/** Create a "default" credential usable for Negotiate mechanisms like NTLM or Kerberos authentication. */
  | { type: 'Default' }
  /**
   * Create a new ssh key credential object used for querying an ssh-agent.
   * The username specified is the username to authenticate.
   */
  | { type: 'SSHKeyFromAgent'; username?: string }
  /** Create a new passphrase-protected ssh key credential object. */
  | { type: 'SSHKeyFromPath'; username?: string; publicKeyPath?: string; privateKeyPath: string; passphrase?: string }
  /** Create a new ssh key credential object reading the keys from memory. */
  | { type: 'SSHKey'; username?: string; publicKey?: string; privateKey: string; passphrase?: string }
  /** Create a new plain-text username and password credential object. */
  | { type: 'Plain'; username?: string; password: string };
/** Options which can be specified to various fetch operations. */
export interface ProxyOptions {
  /**
   * Try to auto-detect the proxy from the git configuration.
   *
   * Note that this will override `url` specified before.
   */
  auto?: boolean;
  /**
   * Specify the exact URL of the proxy to use.
   *
   * Note that this will override `auto` specified before.
   */
  url?: string;
}
export type FetchPrune =
/** Use the setting from the configuration */
  | 'Unspecified'
  /** Force pruning on */
  | 'On'
  /** Force pruning off */
  | 'Off';
/** Automatic tag following options. */
export type AutotagOption =
/** Use the setting from the remote's configuration */
  | 'Unspecified'
  /** Ask the server for tags pointing to objects we're already downloading */
  | 'Auto'
  /** Don't ask for any tags beyond the refspecs */
  | 'None'
  /** Ask for all the tags */
  | 'All';
/**
 * Remote redirection settings; whether redirects to another host are
 * permitted.
 *
 * By default, git will follow a redirect on the initial request
 * (`/info/refs`), but not subsequent requests.
 */
export type RemoteRedirect =
/** Do not follow any off-site redirects at any stage of the fetch or push. */
  | 'None'
  /**
   * Allow off-site redirects only upon the initial request. This is the
   * default.
   */
  | 'Initial'
  /** Allow redirects at any stage in the fetch or push. */
  | 'All';
/** Options which can be specified to various fetch operations. */
export interface FetchOptions {
  credential?: Credential;
  /** Set the proxy options to use for the fetch operation. */
  proxy?: ProxyOptions;
  /** Set whether to perform a prune after the fetch. */
  prune?: FetchPrune;
  /**
   * Set fetch depth, a value less or equal to 0 is interpreted as pull
   * everything (effectively the same as not declaring a limit depth).
   */
  depth?: number;
  /**
   * Set how to behave regarding tags on the remote, such as auto-downloading
   * tags for objects we're downloading or downloading all of them.
   *
   * The default is to auto-follow tags.
   */
  downloadTags?: AutotagOption;
  /**
   * Set remote redirection settings; whether redirects to another host are
   * permitted.
   *
   * By default, git will follow a redirect on the initial request
   * (`/info/refs`), but not subsequent requests.
   */
  followRedirects?: RemoteRedirect;
  /** Set extra headers for this fetch operation. */
  customHeaders?: Array<string>;
}
/** Options to control the behavior of a git push. */
export interface PushOptions {
  credential?: Credential;
  /** Set the proxy options to use for the push operation. */
  proxy?: ProxyOptions;
  /**
   * If the transport being used to push to the remote requires the creation
   * of a pack file, this controls the number of worker threads used by the
   * packbuilder when creating that pack file to be sent to the remote.
   *
   * if set to 0 the packbuilder will auto-detect the number of threads to
   * create, and the default value is 1.
   */
  pbParallelism?: number;
  /**
   * Set remote redirection settings; whether redirects to another host are
   * permitted.
   *
   * By default, git will follow a redirect on the initial request
   * (`/info/refs`), but not subsequent requests.
   */
  followRedirects?: RemoteRedirect;
  /** Set extra headers for this push operation. */
  customHeaders?: Array<string>;
  /** Set "push options" to deliver to the remote. */
  remoteOptions?: Array<string>;
}
export interface CreateRemoteOptions {
  fetchRefspec?: string;
}
export interface FetchRemoteOptions {
  fetch?: FetchOptions;
  reflogMsg?: string;
}
export interface PruneOptions {
  credential?: Credential;
}
export type RepositoryState =
  | 'Clean'
  | 'Merge'
  | 'Revert'
  | 'RevertSequence'
  | 'CherryPick'
  | 'CherryPickSequence'
  | 'Bisect'
  | 'Rebase'
  | 'RebaseInteractive'
  | 'RebaseMerge'
  | 'ApplyMailbox'
  | 'ApplyMailboxOrRebase';
export interface RepositoryInitOptions {
  bare?: boolean;
  initialHead?: string;
  originUrl?: string;
}
export interface RepositoryOpenOptions {
  flags: RepositoryOpenFlags;
  ceilingDirs?: Array<string>;
}
export const enum RepositoryOpenFlags {
  /** Only open the specified path; don't walk upward searching. */
  NoSearch = 0,
  /** Search across filesystem boundaries. */
  CrossFS = 1,
  /** Force opening as a bare repository, and defer loading its config. */
  Bare = 2,
  /** Don't try appending `/.git` to the specified repository path. */
  NoDotGit = 3,
  /** Respect environment variables like `$GIT_DIR`. */
  FromEnv = 4
}
export interface RepositoryCloneOptions {
  recursive?: boolean;
  fetch?: FetchOptions;
}
/** Creates a new repository in the specified folder. */
export declare function initRepository(path: string, options?: RepositoryInitOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<Repository>
/** Attempt to open an already-existing repository at `path`. */
export declare function openRepository(path: string, options?: RepositoryOpenOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<Repository>
/**
 * Attempt to open an already-existing repository at or above `path`
 *
 * This starts at `path` and looks up the filesystem hierarchy
 * until it finds a repository.
 */
export declare function discoverRepository(path: string, signal?: AbortSignal | undefined | null): Promise<Repository>
/**
 * Clone a remote repository.
 *
 * This will use the options configured so far to clone the specified URL
 * into the specified local path.
 */
export declare function cloneRepository(url: string, path: string, options?: RepositoryCloneOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<Repository>
/** Flags for the Revspec. */
export const enum RevparseMode {
  /** The spec targeted a single object (1 << 0) */
  Single = 1,
  /** The spec targeted a range of commits (1 << 1) */
  Range = 2,
  /** The spec used the `...` operator, which invokes special semantics. (1 << 2) */
  MergeBase = 4
}
/** Check revparse mode contains specific flags. */
export declare function revparseModeContains(source: number, target: number): boolean
/** A revspec represents a range of revisions within a repository. */
export interface Revspec {
  /** Access the `from` range of this revspec. */
  from?: string;
  /** Access the `to` range of this revspec. */
  to?: string;
  /** Returns the intent of the revspec. */
  mode: number;
}
/**
 * A Signature is used to indicate authorship of various actions throughout the
 * library.
 *
 * Signatures contain a name, email, and timestamp.
 */
export interface Signature {
  /** Name on the signature. */
  name: string;
  /** Email on the signature. */
  email: string;
  /** Time in seconds, from epoch */
  timestamp: number;
}
export interface CreateSignatureOptions {
  /** Time in seconds, from epoch */
  timestamp: number;
  /** Timezone offset, in minutes */
  offset?: number;
}
/** Create a new action signature. */
export declare function createSignature(name: string, email: string, options?: CreateSignatureOptions | undefined | null): Signature
/** An enumeration all possible kinds objects may have. */
export const enum ObjectType {
  /** Any kind of git object */
  Any = 0,
  /** An object which corresponds to a git commit */
  Commit = 1,
  /** An object which corresponds to a git tree */
  Tree = 2,
  /** An object which corresponds to a git blob */
  Blob = 3,
  /** An object which corresponds to a git tag */
  Tag = 4
}
/** A structure to represent a git commit */
export declare class Commit {
  /** Get the id (SHA1) of a repository commit */
  id(): string
  /** Get the author of this commit. */
  author(): Signature
  /** Get the committer of this commit. */
  committer(): Signature
  /**
   * Get the full message of a commit.
   *
   * The returned message will be slightly prettified by removing any
   * potential leading newlines.
   *
   * Throws error if the message is not valid utf-8
   */
  message(): string
  /**
   * Get the short "summary" of the git commit message.
   *
   * The returned message is the summary of the commit, comprising the first
   * paragraph of the message with whitespace trimmed and squashed.
   *
   * Throws error if the summary is not valid utf-8.
   */
  summary(): string | null
  /**
   * Get the long "body" of the git commit message.
   *
   * The returned message is the body of the commit, comprising everything
   * but the first paragraph of the message. Leading and trailing whitespaces
   * are trimmed.
   *
   * Throws error if the summary is not valid utf-8.
   */
  body(): string | null
  /**
   * Get the commit time (i.e. committer time) of a commit.
   *
   * The first element of the tuple is the time, in seconds, since the epoch.
   * The second element is the offset, in minutes, of the time zone of the
   * committer's preferred time zone.
   */
  time(): Date
}
/**
 * A structure to represent a git [index][1]
 *
 * [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects
 */
export declare class Index {
  /**
   * Get index on-disk version.
   *
   * Valid return values are 2, 3, or 4.  If 3 is returned, an index
   * with version 2 may be written instead, if the extension data in
   * version 3 is not necessary.
   */
  version(): number
  /**
   * Set index on-disk version.
   *
   * Valid values are 2, 3, or 4.  If 2 is given, git_index_write may
   * write an index with version 3 instead, if necessary to accurately
   * represent the index.
   */
  setVersion(version: number): void
  /** Get one of the entries in the index by its path. */
  getByPath(path: string, stage?: IndexStage | undefined | null): IndexEntry | null
  /**
   * Add or update an index entry from a file on disk
   *
   * The file path must be relative to the repository's working folder and
   * must be readable.
   *
   * This method will fail in bare index instances.
   *
   * This forces the file to be added to the index, not looking at gitignore
   * rules.
   *
   * If this file currently is the result of a merge conflict, this file will
   * no longer be marked as conflicting. The data about the conflict will be
   * moved to the "resolve undo" (REUC) section.
   */
  addPath(path: string): void
  /**
   * Add or update index entries matching files in the working directory.
   *
   * This method will fail in bare index instances.
   *
   * The `pathspecs` are a list of file names or shell glob patterns that
   * will matched against files in the repository's working directory. Each
   * file that matches will be added to the index (either updating an
   * existing entry or adding a new entry).
   *
   * @example
   *
   * Emulate `git add *`:
   *
   * ```ts
   * import { openRepository } from 'es-git';
   *
   * const repo = await openRepository('.');
   * const index = repo.index();
   * index.addAll(['*']);
   * index.write();
   * ```
   */
  addAll(pathspecs: Array<string>, options?: IndexAddAllOptions | undefined | null): void
  /**
   * Update the contents of an existing index object in memory by reading
   * from the hard disk.
   *
   * If force is true, this performs a "hard" read that discards in-memory
   * changes and always reloads the on-disk index data. If there is no
   * on-disk version, the index will be cleared.
   *
   * If force is false, this does a "soft" read that reloads the index data
   * from disk only if it has changed since the last time it was loaded.
   * Purely in-memory index data will be untouched. Be aware: if there are
   * changes on disk, unwritten in-memory changes are discarded.
   */
  read(force?: boolean | undefined | null): void
  /**
   * Write the index as a tree.
   *
   * This method will scan the index and write a representation of its
   * current state back to disk; it recursively creates tree objects for each
   * of the subtrees stored in the index, but only returns the OID of the
   * root tree. This is the OID that can be used e.g. to create a commit.
   *
   * The index instance cannot be bare, and needs to be associated to an
   * existing repository.
   *
   * The index must not contain any file in conflict.
   */
  writeTree(): string
  /**
   * Remove an index entry corresponding to a file on disk.
   *
   * The file path must be relative to the repository's working folder. It
   * may exist.
   *
   * If this file currently is the result of a merge conflict, this file will
   * no longer be marked as conflicting. The data about the conflict will be
   * moved to the "resolve undo" (REUC) section.
   */
  removePath(path: string, options?: IndexRemoveOptions | undefined | null): void
  /** Remove all matching index entries. */
  removeAll(pathspecs: Array<string>, options?: IndexRemoveAllOptions | undefined | null): void
  /**
   * Update all index entries to match the working directory
   *
   * This method will fail in bare index instances.
   *
   * This scans the existing index entries and synchronizes them with the
   * working directory, deleting them if the corresponding working directory
   * file no longer exists otherwise updating the information (including
   * adding the latest version of file to the ODB if needed).
   */
  updateAll(pathspecs: Array<string>, options?: IndexUpdateAllOptions | undefined | null): void
  /** Get the count of entries currently in the index */
  count(): number
  /** Return `true` is there is no entry in the index */
  isEmpty(): boolean
  /**
   * Get the full path to the index file on disk.
   *
   * Returns `None` if this is an in-memory index.
   */
  path(): string | null
  /**
   * Does this index have conflicts?
   *
   * Returns `true` if the index contains conflicts, `false` if it does not.
   */
  hasConflicts(): boolean
  /** Get an iterator over the entries in this index. */
  entries(): IndexEntries
}
/** An iterator over the entries in an index */
export declare class IndexEntries {
  [Symbol.iterator](): Iterator<IndexEntry, void, void>
}
/**
 * A structure representing a [remote][1] of a git repository.
 *
 * [1]: http://git-scm.com/book/en/Git-Basics-Working-with-Remotes
 */
export declare class Remote {
  /**
   * Get the remote's name.
   *
   * Returns `null` if this remote has not yet been named, and
   * Throws error if the URL is not valid utf-8
   */
  name(): string | null
  /**
   * Get the remote's URL.
   *
   * Throws error if the URL is not valid utf-8
   */
  url(): string
  /**
   * Get the remote's URL.
   *
   * Returns `null` if push url not exists, and
   * Throws error if the URL is not valid utf-8
   */
  pushurl(): string | null
  /**
   * List all refspecs.
   *
   * Filter refspec if has not valid src or dst with utf-8
   */
  refspecs(): Array<RefspecObject>
  /**
   * Download new data and update tips
   *
   * Convenience function to connect to a remote, download the data, disconnect and update the remote-tracking branches.
   */
  fetch(refspecs: Array<string>, options?: FetchRemoteOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<void>
  /**
   * Perform a push
   *
   * Perform all the steps for a push.
   * If no refspecs are passed, then the configured refspecs will be used.
   */
  push(refspecs: Array<string>, options?: PushOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<void>
  /** Prune tracking refs that are no longer present on remote */
  prune(options?: PruneOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<void>
  /** Get the remote’s default branch. */
  defaultBranch(signal?: AbortSignal | undefined | null): Promise<string>
}
/**
 * An owned git repository, representing all state associated with the
 * underlying filesystem.
 *
 * This structure corresponds to a `git_repository` in libgit2.
 *
 * When a repository goes out of scope, it is freed in memory but not deleted
 * from the filesystem.
 */
export declare class Repository {
  /**
   * Lookup a reference to one of the commits in a repository.
   *
   * Returns `null` if the commit does not exist.
   */
  findCommit(oid: string): Commit | null
  /** Lookup a reference to one of the commits in a repository. */
  getCommit(oid: string): Commit
  /**
   * Create a diff between two index objects.
   *
   * The first index will be used for the "old_file" side of the delta, and
   * the second index will be used for the "new_file" side of the delta.
   */
  diffIndexToIndex(oldIndex: Index, newIndex: Index, options?: DiffOptions | undefined | null): Diff
  /**
   * Get the Index file for this repository.
   *
   * If a custom index has not been set, the default index for the repository
   * will be returned (the one located in .git/index).
   *
   * **Caution**: If the [`git2::Repository`] of this index is dropped, then this
   * [`git2::Index`] will become detached, and most methods on it will fail. See
   * [`git2::Index::open`]. Be sure the repository has a binding such as a local
   * variable to keep it alive at least as long as the index.
   */
  index(): Index
  /**
   * Lookup a reference to one of the objects in a repository.
   *
   * Returns `null` if the object does not exist.
   */
  findObject(oid: string): GitObject | null
  /** Lookup a reference to one of the objects in a repository. */
  getObject(oid: string): GitObject
  /** Lookup a reference to one of the objects in a repository. */
  findReference(name: string): Reference | null
  /** Lookup a reference to one of the objects in a repository. */
  getReference(name: string): Reference
  /** List all remotes for a given repository */
  remoteNames(): Array<string>
  /**
   * Get remote from repository
   *
   * Throws error if not exists
   */
  getRemote(name: string): Remote
  /** Find remote from repository */
  findRemote(name: string): Remote | null
  /** Add a remote with the default fetch refspec to the repository’s configuration. */
  createRemote(name: string, url: string, options?: CreateRemoteOptions | undefined | null): Remote
  /** Tests whether this repository is a bare repository or not. */
  isBare(): boolean
  /** Tests whether this repository is a shallow clone. */
  isShallow(): boolean
  /** Tests whether this repository is a worktree. */
  isWorktree(): boolean
  /** Tests whether this repository is empty. */
  isEmpty(): boolean
  /**
   * Returns the path to the `.git` folder for normal repositories or the
   * repository itself for bare repositories.
   */
  path(): string
  /** Returns the current state of this repository */
  state(): RepositoryState
  /**
   * Get the path of the working directory for this repository.
   *
   * If this repository is bare, then `null` is returned.
   */
  workdir(): string | null
  /**
   * Execute a rev-parse operation against the `spec` listed.
   *
   * The resulting revision specification is returned, or an error is
   * returned if one occurs.
   */
  revparse(spec: string): Revspec
  /** Find a single object, as specified by a revision string. */
  revparseSingle(spec: string): string
  /** Create a revwalk that can be used to traverse the commit graph. */
  revwalk(): Revwalk
}
/**
 * A structure to represent a git [object][1]
 *
 * [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects
 */
export declare class GitObject {
  /** Get the id (SHA1) of a repository object */
  id(): string
  /**
   * Get the object type of object.
   *
   * If the type is unknown, then `null` is returned.
   */
  type(): ObjectType | null
  /**
   * Recursively peel an object until an object of the specified type is met.
   *
   * If you pass `Any` as the target type, then the object will be
   * peeled until the type changes (e.g. a tag will be chased until the
   * referenced object is no longer a tag).
   */
  peel(objType: ObjectType): GitObject
  /** Recursively peel an object until a commit is found */
  peelToCommit(): Commit
  /**
   * Attempt to view this object as a commit.
   *
   * Returns `null` if the object is not actually a commit.
   */
  asCommit(): Commit | null
}
/**
 * A structure to represent a git [reference][1].
 *
 * [1]: http://git-scm.com/book/en/Git-Internals-Git-References
 */
export declare class Reference {
  /**
   * Delete an existing reference.
   *
   * This method works for both direct and symbolic references. The reference
   * will be immediately removed on disk.
   *
   * This function will return an error if the reference has changed from the
   * time it was looked up.
   */
  delete(): void
  /** Check if a reference is a local branch. */
  isBranch(): boolean
  /** Check if a reference is a note. */
  isNote(): boolean
  /** Check if a reference is a remote tracking branch */
  isRemote(): boolean
  /** Check if a reference is a tag */
  isTag(): boolean
  /**
   * Get the reference type of a reference.
   *
   * If the type is unknown, then `null` is returned.
   */
  type(): ReferenceType | null
  /**
   * Get the full name of a reference.
   *
   * Throws error if the name is not valid utf-8.
   */
  name(): string
  /**
   * Get the full shorthand of a reference.
   *
   * This will transform the reference name into a name "human-readable"
   * version. If no shortname is appropriate, it will return the full name.
   *
   * Throws error if the shorthand is not valid utf-8.
   */
  shorthand(): string
  /**
   * Get the OID pointed to by a direct reference.
   *
   * Only available if the reference is direct (i.e. an object id reference,
   * not a symbolic one).
   */
  target(): string | null
  /**
   * Return the peeled OID target of this reference.
   *
   * This peeled OID only applies to direct references that point to a hard
   * Tag object: it is the result of peeling such Tag.
   */
  targetPeel(): string | null
  /**
   * Get full name to the reference pointed to by a symbolic reference.
   *
   * Only available if the reference is symbolic.
   */
  symbolicTarget(): string | null
  /**
   * Resolve a symbolic reference to a direct reference.
   *
   * This method iteratively peels a symbolic reference until it resolves to
   * a direct reference to an OID.
   *
   * If a direct reference is passed as an argument, a copy of that
   * reference is returned.
   */
  resolve(): Reference
  /**
   * Rename an existing reference.
   *
   * This method works for both direct and symbolic references.
   *
   * If the force flag is not enabled, and there's already a reference with
   * the given name, the renaming will fail.
   */
  rename(newName: string, options?: RenameReferenceOptions | undefined | null): Reference
}
/** Orderings that may be specified for Revwalk iteration. */
export const enum RevwalkSort {
  /**
   * Sort the repository contents in no particular ordering.
   *
   * This sorting is arbitrary, implementation-specific, and subject to
   * change at any time. This is the default sorting for new walkers.
   */
  None = 0,
  /**
   * Sort the repository contents in topological order (children before
   * parents).
   *
   * This sorting mode can be combined with time sorting.
   * (1 << 0)
   */
  Topological = 1,
  /**
   * Sort the repository contents by commit time.
   *
   * This sorting mode can be combined with topological sorting.
   * (1 << 1)
   */
  Time = 2,
  /**
   * Iterate through the repository contents in reverse order.
   *
   * This sorting mode can be combined with any others.
   * (1 << 2)
   */
  Reverse = 4
}
/**
 * A revwalk allows traversal of the commit graph defined by including one or
 * more leaves and excluding one or more roots.
 */
export declare class Revwalk {
  [Symbol.iterator](): Iterator<string, void, void>
  /**
   * Reset a revwalk to allow re-configuring it.
   *
   * The revwalk is automatically reset when iteration of its commits
   * completes.
   */
  reset(): this
  /** Set the order in which commits are visited. */
  setSorting(sort: number): this
  /**
   * Simplify the history by first-parent
   *
   * No parents other than the first for each commit will be enqueued.
   */
  simplifyFirstParent(): this
  /**
   * Mark a commit to start traversal from.
   *
   * The given OID must belong to a commitish on the walked repository.
   *
   * The given commit will be used as one of the roots when starting the
   * revision walk. At least one commit must be pushed onto the walker before
   * a walk can be started.
   */
  push(oid: string): this
  /**
   * Push the repository's HEAD
   *
   * For more information, see `push`.
   */
  pushHead(): this
  /**
   * Push matching references
   *
   * The OIDs pointed to by the references that match the given glob pattern
   * will be pushed to the revision walker.
   *
   * A leading 'refs/' is implied if not present as well as a trailing `/ \
   * *` if the glob lacks '?', ' \ *' or '['.
   *
   * Any references matching this glob which do not point to a commitish
   * will be ignored.
   */
  pushGlob(glob: string): this
  /**
   * Push and hide the respective endpoints of the given range.
   *
   * The range should be of the form `<commit>..<commit>` where each
   * `<commit>` is in the form accepted by `revparse_single`. The left-hand
   * commit will be hidden and the right-hand commit pushed.
   */
  pushRange(range: string): this
  /**
   * Push the OID pointed to by a reference
   *
   * The reference must point to a commitish.
   */
  pushRef(reference: string): this
  /** Mark a commit as not of interest to this revwalk. */
  hide(oid: string): this
  /**
   * Hide the repository's HEAD
   *
   * For more information, see `hide`.
   */
  hideHead(): this
  /**
   * Hide matching references.
   *
   * The OIDs pointed to by the references that match the given glob pattern
   * and their ancestors will be hidden from the output on the revision walk.
   *
   * A leading 'refs/' is implied if not present as well as a trailing `/ \
   * *` if the glob lacks '?', ' \ *' or '['.
   *
   * Any references matching this glob which do not point to a commitish
   * will be ignored.
   */
  hideGlob(glob: string): this
  /**
   * Hide the OID pointed to by a reference.
   *
   * The reference must point to a commitish.
   */
  hideRef(reference: string): this
}
