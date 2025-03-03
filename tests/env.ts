export const CI = !!process.env.CI;

function getTarget() {
  switch (process.platform) {
    case 'win32': {
      switch (process.arch) {
        case 'x64':
          return ['win32', 'x64'] as const;
        case 'ia32':
          return ['win32', 'ia32'] as const;
        case 'arm64':
          return ['win32', 'arm64'] as const;
      }
      break;
    }
    case 'darwin': {
      switch (process.arch) {
        case 'x64':
          return ['darwin', 'x64'] as const;
        case 'arm64':
          return ['darwin', 'arm64'] as const;
      }
      break;
    }
    case 'linux': {
      const isMusl = !(process.report.getReport() as any).header.glibcVersionRuntime;
      const type = isMusl ? 'musl' : 'gnu';
      switch (process.arch) {
        case 'x64':
          return ['linux', 'x64', type] as const;
        case 'arm64':
          return ['linux', 'arm64', type] as const;
      }
    }
  }
  throw new Error('Unsupported target');
}

export const TARGET = getTarget();
