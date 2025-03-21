export const isCI = !!process.env.CI;

type Target =
  | { platform: 'win32'; arch: 'x64' | 'ia32' | 'arm64' }
  | { platform: 'darwin'; arch: 'x64' | 'arm64' }
  | { platform: 'linux'; arch: 'x64' | 'arm64'; type: 'musl' | 'gnu' };

function getTarget(): Target {
  switch (process.platform) {
    case 'win32': {
      switch (process.arch) {
        case 'x64':
          return { platform: 'win32', arch: 'x64' };
        case 'ia32':
          return { platform: 'win32', arch: 'ia32' };
        case 'arm64':
          return { platform: 'win32', arch: 'arm64' };
      }
      break;
    }
    case 'darwin': {
      switch (process.arch) {
        case 'x64':
          return { platform: 'darwin', arch: 'x64' };
        case 'arm64':
          return { platform: 'darwin', arch: 'arm64' };
      }
      break;
    }
    case 'linux': {
      const isMusl = !(process.report.getReport() as any).header.glibcVersionRuntime;
      const type = isMusl ? 'musl' : 'gnu';
      switch (process.arch) {
        case 'x64':
          return { platform: 'linux', arch: 'x64', type };
        case 'arm64':
          return { platform: 'linux', arch: 'arm64', type };
      }
    }
  }
  throw new Error('Unsupported target');
}

type Platform = Target['platform'];
type Arch<T extends Platform> = Extract<Target, { platform: T }>['arch'];
type Type = Extract<Target, { platform: 'linux' }>['type'];

export function isTarget(platform: 'win32', arch?: Arch<'win32'>): boolean;
export function isTarget(platform: 'darwin', arch?: Arch<'darwin'>): boolean;
export function isTarget(platform: 'linux', arch?: Arch<'linux'>, type?: Type): boolean;
export function isTarget(platform: Platform, arch?: Arch<Platform>, type?: Type): boolean {
  const target = getTarget();
  switch (platform) {
    case 'win32':
    case 'darwin':
      return target.platform === platform && (arch != null ? target.arch === arch : true);
    case 'linux':
      return (
        target.platform === platform &&
        (arch != null ? target.arch === arch : true) &&
        (type != null ? target.type === type : true)
      );
  }
}
