import { cwd } from "node:process";
import { join } from "node:path";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { existsSync } from "node:fs";

import { ChildProcess } from "@figliolia/child-process";

import { BinaryPackage } from "./BinaryPackage";

export class PackageBinaries {
  public static readonly SUPPORTED_PLATFORMS = [
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-gnu",
    "i686-pc-windows-msvc",
    "armv7-unknown-linux-gnueabihf",
    "aarch64-linux-android",
    "x86_64-unknown-freebsd",
    "aarch64-unknown-linux-musl",
    "aarch64-pc-windows-msvc",
    "armv7-linux-androideabi",
  ];
  public static readonly OS_MAP = {
    windows: "win32",
    androideabi: "android",
  };
  public static readonly CPU_ARCH_MAP = {
    x86_64: "x64",
    aarch64: "arm64",
    armv7: "arm",
    i686: "ia32",
  };
  public static readonly LIB_C_MAP = {
    gnu: "glibc",
    musl: "musl",
  };
  public static readonly FLAVOR_MAP = {
    androideabi: "eabi",
  };

  public static async run() {
    await this.buildPackages();
    await ChildProcess.execute("pnpm lint:ts");
  }

  private static async buildPackages() {
    const root = await this.findRoot();
    const packageJSON = await this.getPackageJSON(root);
    const packagePath = await this.clean(root);
    const packages = this.buildTasks(packageJSON["version"]);
    await Promise.all(packages.map(t => t.build(packagePath)));
    const artifacts = join(root, "artifacts");
    await Promise.all(packages.map(t => t.shipBinary(artifacts)));
    packageJSON["optionalDependencies"] = {};
    for (const binaryPackage of packages) {
      packageJSON["optionalDependencies"][
        `@repokit/${binaryPackage.packageName}`
      ] = packageJSON["version"];
    }
    await writeFile(
      join(root, "package.json"),
      JSON.stringify(packageJSON, null, 2),
    );
  }

  private static buildTasks(version: string) {
    const tasks: BinaryPackage[] = [];
    for (const platform of this.SUPPORTED_PLATFORMS) {
      const [cpu, vendor, os, libcOrFlavor] = platform.split("-");
      const CPU = this.mapTo(this.CPU_ARCH_MAP, cpu ?? "");
      const OS = this.mapTo(this.OS_MAP, os ?? "");
      const LIBC = this.tryMap(this.LIB_C_MAP, libcOrFlavor);
      const FLAVOR = libcOrFlavor ?? vendor;
      if (!!CPU && !!OS && typeof FLAVOR === "string") {
        tasks.push(
          new BinaryPackage({
            OS,
            CPU,
            LIBC,
            FLAVOR,
            version,
            platform,
            binaryName: OS === "win32" ? "repokit.exe" : "repokit",
          }),
        );
      } else {
        throw new Error(`Failed to construct Binary Package for: ${platform}`, {
          cause: {
            OS,
            CPU,
            LIBC,
            FLAVOR,
            version,
            platform,
            binaryName: OS === "win32" ? "repokit.exe" : "repokit",
          },
        });
      }
    }
    return tasks;
  }

  private static mapTo<T extends Record<string, string>>(
    map: T,
    value: string,
  ) {
    return this.tryMap(map, value) ?? value;
  }

  private static tryMap<T extends Record<string, string>>(
    map: T,
    value?: string,
  ) {
    return typeof value === "string" && value in map ? map[value] : undefined;
  }

  private static async getPackageJSON(root: string) {
    const packageFile = (await readFile(join(root, "package.json"))).toString();
    // oxlint-disable-next-line typescript-eslint(no-explicit-any)
    return JSON.parse(packageFile) as Record<string, any>;
  }

  private static async findRoot() {
    const { stdout } = await ChildProcess.execute(
      "git rev-parse --show-toplevel",
    );
    const root = stdout.trim();
    if (existsSync(root)) {
      return root;
    }
    const currentDir = cwd();
    const tokens = currentDir.split("/");
    let tries = 10;
    while (--tries > 0 && !existsSync(join(tokens.join("/"), "package.json"))) {
      tokens.push("..");
      if (!tries) {
        throw new Error("Failed to locate root");
      }
    }
    return tokens.join("/");
  }

  private static async clean(root: string) {
    const packagePath = join(root, "npm");
    await rm(packagePath, { force: true, recursive: true });
    await mkdir(packagePath, { recursive: true });
    return packagePath;
  }
}
