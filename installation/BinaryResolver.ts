import { machine, platform } from "node:os";

import { familySync, GLIBC, MUSL } from "detect-libc";

export class BinaryResolver {
  public static readonly BINARY_TARGETS = [
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "i686-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "aarch64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "armv7-unknown-linux-gnueabihf",
    "aarch64-unknown-linux-musl",
    "x86_64-unknown-freebsd",
    "aarch64-pc-windows-msvc",
  ];
  public static glibc = familySync();
  public static readonly arch = machine();
  public static readonly platform = platform();
  public static readonly CPU_MAP = {
    x86_64: "x86_64",
    arm: "armv7",
    arm64: "aarch64",
    aarch64: "aarch64",
    i686: "i686",
  };
  public static readonly PLATFORM_MAP = {
    darwin: "darwin",
    freebsd: "freebsd",
    linux: "linux",
    win32: "windows",
  };
  public static readonly GLIBC_MAP = {
    [GLIBC]: "gnu",
    [MUSL]: "musl",
  };

  public static matchBinary() {
    try {
      const OS = this.matchOrThrow(
        BinaryResolver.PLATFORM_MAP,
        BinaryResolver.platform,
      );
      let candidates = this.filterTargets(OS);
      const CPU = this.matchOrThrow(
        BinaryResolver.CPU_MAP,
        BinaryResolver.arch,
      );
      candidates = this.filterTargets(CPU, candidates);
      const GLIBC = this.match(BinaryResolver.GLIBC_MAP, BinaryResolver.glibc);
      if (GLIBC !== undefined) {
        const glibcMatches = candidates.filter(t =>
          t.includes(GLIBC as string),
        );
        if (glibcMatches.length) {
          return glibcMatches[0];
        }
      }
      return candidates[0];
    } catch {
      return undefined;
    }
  }

  public static matchOrThrow<
    T extends Record<string, string>,
    K extends string,
  >(map: T, value: K) {
    if (value in map) {
      return map[value] as string;
    }
    throw new Error(`${value} is not supported`);
  }

  public static match<T extends Record<string, string>, K extends string>(
    map: T,
    value?: K | null,
  ) {
    if (typeof value !== "string") {
      return undefined;
    }
    return map[value];
  }

  public static filterTargets(
    filter: string,
    targets = BinaryResolver.BINARY_TARGETS,
  ) {
    const candidates = targets.filter(t => t.includes(filter));
    if (!candidates.length) {
      throw new Error("This operating system is not supported");
    }
    return candidates;
  }
}
