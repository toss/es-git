/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Branch {
  name: string
  oid: string
}
export interface CreateBranchOptions {
  branchName: string
  targetSha: string
  force?: boolean
}
export interface GetBranchOptions {
  branchName: string
}
export interface FindBranchOptions {
  branchName: string
}
export interface DeleteBranchOptions {
  branchName: string
}
export function createBranch(options: CreateBranchOptions, context: GitContext): Branch
export function getBranch(options: GetBranchOptions, context: GitContext): Branch
export function deleteBranch(options: DeleteBranchOptions, context: GitContext): void
export interface GitContext {
  dir: string
}
export function removeRef(gitRef: string, context: GitContext): void
export function getRemoteUrl(name: string, context: GitContext): string | null
export function getSha(gitRef: string, context: GitContext): string
export function getHeadSha(context: GitContext): string
export function getGitRootPath(context: GitContext): string
export function hasMergeConflicts(ref1: string, ref2: string, context: GitContext): boolean
export interface Conflict {
  ancestor?: string
  our?: string
  their?: string
}
export function getConflictingFiles(ref1: string, ref2: string, context: GitContext): Array<Conflict>
export interface CreateTagOptions {
  name: string
  message: string
  sha: string
}
export interface CreateTagResult {
  oid: string
}
export function createTag(options: CreateTagOptions, context: GitContext): CreateTagResult
export function deleteTag(name: string, context: GitContext): void
