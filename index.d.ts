/* tslint:disable */
/* eslint-disable */

/**
 * TODO:
 * napi does not support union types when converting rust enum types to TypeScript.
 * This feature will be provided starting from v3, so create a custom TypeScript until the v3 stable releases.
 */

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
  force?: boolean
  logMessage?: string
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
  auto?: boolean
  /**
   * Specify the exact URL of the proxy to use.
   *
   * Note that this will override `auto` specified before.
   */
  url?: string
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
  from?: string
  /** Access the `to` range of this revspec. */
  to?: string
  /** Returns the intent of the revspec. */
  mode: number
}
/**
 * A Signature is used to indicate authorship of various actions throughout the
 * library.
 *
 * Signatures contain a name, email, and timestamp.
 */
export interface Signature {
  /** Name on the signature. */
  name: string
  /** Email on the signature. */
  email: string
  /** Time in seconds, from epoch */
  timestamp: number
}
export interface CreateSignatureOptions {
  /** Time in seconds, from epoch */
  timestamp: number
  /** Timezone offset, in minutes */
  offset?: number
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
