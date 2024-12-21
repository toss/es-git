/* tslint:disable */
/* eslint-disable */

/**
 * TODO:
 * napi does not support union types when converting rust enum types to TypeScript.
 * This feature will be provided starting from v3, so create a custom TypeScript until the v3 stable releases.
 */

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
  recursive?: boolean
  fetch?: FetchOptions
}
export declare function initRepository(path: string, options?: RepositoryInitOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<Repository>
export declare function openRepository(path: string, options?: RepositoryOpenOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<Repository>
export declare function discoverRepository(path: string, signal?: AbortSignal | undefined | null): Promise<Repository>
export declare function cloneRepository(url: string, path: string, options?: RepositoryCloneOptions | undefined | null, signal?: AbortSignal | undefined | null): Promise<Repository>
/** An enumeration of all possible kinds of references. */
export type ReferenceType =
  /** A reference which points at an object id. */
  | 'Direct'
  /** A reference which points at another reference. */
  | 'Symbolic';
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
export declare class Repository {
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
  isBare(): boolean
  isShallow(): boolean
  isWorktree(): boolean
  isEmpty(): boolean
  path(): string
  state(): RepositoryState
  workdir(): string | null
  /** Lookup a reference to one of the objects in a repository. */
  findReference(name: string): Reference | null
  /** Lookup a reference to one of the objects in a repository. */
  getReference(name: string): Reference
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
