/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'esgit.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./esgit.android-arm64.node')
          } else {
            nativeBinding = require('esgit-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'esgit.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./esgit.android-arm-eabi.node')
          } else {
            nativeBinding = require('esgit-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'esgit.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./esgit.win32-x64-msvc.node')
          } else {
            nativeBinding = require('esgit-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'esgit.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./esgit.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('esgit-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'esgit.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./esgit.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('esgit-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'esgit.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./esgit.darwin-universal.node')
      } else {
        nativeBinding = require('esgit-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'esgit.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./esgit.darwin-x64.node')
          } else {
            nativeBinding = require('esgit-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'esgit.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./esgit.darwin-arm64.node')
          } else {
            nativeBinding = require('esgit-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'esgit.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./esgit.freebsd-x64.node')
      } else {
        nativeBinding = require('esgit-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'esgit.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./esgit.linux-x64-musl.node')
            } else {
              nativeBinding = require('esgit-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'esgit.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./esgit.linux-x64-gnu.node')
            } else {
              nativeBinding = require('esgit-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'esgit.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./esgit.linux-arm64-musl.node')
            } else {
              nativeBinding = require('esgit-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'esgit.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./esgit.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('esgit-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'esgit.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./esgit.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('esgit-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'esgit.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./esgit.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('esgit-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'esgit.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./esgit.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('esgit-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'esgit.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./esgit.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('esgit-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'esgit.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./esgit.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('esgit-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { Commit, IndexStage, Index, IndexEntries, ObjectType, GitObject, ReferenceType, Reference, isReferenceNameValid, ReferenceFormat, normalizeReferenceName, Direction, CredentialType, FetchPrune, AutotagOption, RemoteRedirect, Remote, RepositoryState, RepositoryOpenFlags, Repository, initRepository, openRepository, discoverRepository, cloneRepository, RevparseMode, revparseModeContains, RevwalkSort, Revwalk, createSignature } = nativeBinding

module.exports.Commit = Commit
module.exports.IndexStage = IndexStage
module.exports.Index = Index
module.exports.IndexEntries = IndexEntries
module.exports.ObjectType = ObjectType
module.exports.GitObject = GitObject
module.exports.ReferenceType = ReferenceType
module.exports.Reference = Reference
module.exports.isReferenceNameValid = isReferenceNameValid
module.exports.ReferenceFormat = ReferenceFormat
module.exports.normalizeReferenceName = normalizeReferenceName
module.exports.Direction = Direction
module.exports.CredentialType = CredentialType
module.exports.FetchPrune = FetchPrune
module.exports.AutotagOption = AutotagOption
module.exports.RemoteRedirect = RemoteRedirect
module.exports.Remote = Remote
module.exports.RepositoryState = RepositoryState
module.exports.RepositoryOpenFlags = RepositoryOpenFlags
module.exports.Repository = Repository
module.exports.initRepository = initRepository
module.exports.openRepository = openRepository
module.exports.discoverRepository = discoverRepository
module.exports.cloneRepository = cloneRepository
module.exports.RevparseMode = RevparseMode
module.exports.revparseModeContains = revparseModeContains
module.exports.RevwalkSort = RevwalkSort
module.exports.Revwalk = Revwalk
module.exports.createSignature = createSignature
