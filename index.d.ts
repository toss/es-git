/* tslint:disable */
/* eslint-disable */

import { RepositoryState } from './es-git';

/**
 * TODO:
 * napi does not support union types when converting rust enum types to TypeScript.
 * This feature will be provided starting from v3, so create a custom TypeScript until the v3 stable releases.
 */

export type Credential =
  | { type: 'Default' }
  | { type: 'SSHKeyFromAgent'; username?: string }
  | { type: 'SSHKeyFromPath'; username?: string; publicKeyPath?: string; privateKeyPath: string; passphrase?: string }
  | { type: 'SSHKey'; username?: string; publicKey?: string; privateKey: string; passphrase?: string }
  | { type: 'Plain'; username?: string; password: string };
export interface ProxyOptions {
  auto?: boolean
  url?: string
}
export type FetchPrune =
  /** Use the setting from the configuration */
  | 'Unspecified'
  /** Force pruning on */
  | 'On'
  /** Force pruning off */
  | 'Off';
export type AutotagOption =
  /** Use the setting from the remote's configuration */
  | 'Unspecified'
  /** Ask the server for tags pointing to objects we're already downloading */
  | 'Auto'
  /** Don't ask for any tags beyond the refspecs */
  | 'None'
  /** Ask for all the tags */
  | 'All';
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
export interface Progress {
  totalObjects: number
  indexedObjects: number
  receivedObjects: number
  localObjects: number
  totalDeltas: number
  indexedDeltas: number
  receivedBytes: number
}
export interface PushProgress {
  current: number
  total: number
  bytes: number
}
export interface FetchOptions {
  credential?: Credential
  proxy?: ProxyOptions
  prune?: FetchPrune
  depth?: number
  downloadTags?: AutotagOption
  followRedirects?: RemoteRedirect
  customHeaders?: Array<string>
  onTransferProgress?: (progress: Progress) => void
  onPushTransferProgress?: (progress: PushProgress) => void
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
  bare?: boolean
  initialHead?: string
  originUrl?: string
}
export interface RepositoryOpenOptions {
  flags: RepositoryOpenFlags
  ceilingDirs?: Array<string>
}
export type RepositoryOpenFlags =
  /** Only open the specified path; don't walk upward searching. */
  | 'NoSearch'
  /** Search across filesystem boundaries. */
  | 'CrossFS'
  /** Force opening as bare repository, and defer loading its config. */
  | 'Bare'
  /** Don't try appending `/.git` to the specified repository path. */
  | 'NoDotGit'
  /** Respect environment variables like `$GIT_DIR`. */
  | 'FromEnv';
export interface RepositoryCloneOptions {
  recursive?: boolean
  fetch?: FetchOptions
}
export declare class Repository {
  static init(path: string, options?: RepositoryInitOptions | undefined | null): Repository
  static open(path: string, options?: RepositoryOpenOptions | undefined | null): Repository
  static discover(path: string): Repository
  static clone(url: string, path: string, options?: RepositoryCloneOptions | undefined | null): Repository
  isBare(): boolean
  isShallow(): boolean
  isWorktree(): boolean
  isEmpty(): boolean
  path(): string
  state(): RepositoryState
  workdir(): string | null
  remoteNames(): Array<string>
}
