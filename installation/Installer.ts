import { cwd } from "node:process";
import { join } from "node:path";
import { readdir } from "node:fs/promises";
import { existsSync, statSync } from "node:fs";

import { ChildProcess } from "@figliolia/child-process";

import { BinaryPackage } from "../distribution/BinaryPackage";

export class Installer {
  public static INSTALL_DIRECTORY = cwd();
  public static readonly ROOT = this.findRoot();
  public static readonly INSTALL_TARGET = join(
    this.ROOT,
    "node_modules",
    ".bin",
    "repokit",
  );

  public static INSTALL_OPTIONS = [
    () => this.linkInstalledBinary(),
    () => this.compileSource(),
  ];

  public static async run() {
    for (const installOption of this.INSTALL_OPTIONS) {
      if ((await installOption().catch(() => {})) === true) {
        return;
      }
    }
    throw new Error("Platform not supported");
  }

  private static async compileSource() {
    await new ChildProcess("cargo build --release").handler;
    return this.moveAndMakeExecutable(
      join(this.INSTALL_DIRECTORY, "target", "release"),
    );
  }

  private static async linkInstalledBinary() {
    const orgDirectory = join(this.INSTALL_DIRECTORY, "..");
    const packages = await readdir(orgDirectory);
    for (const installedPackage of packages) {
      if (installedPackage.startsWith(BinaryPackage.NPM_PREFIX)) {
        const path = join(orgDirectory, installedPackage);
        return this.moveAndMakeExecutable(path);
      }
    }
    return false;
  }

  private static async moveAndMakeExecutable(path: string) {
    const binaries = ["repokit", "repokit.exe"];
    for (const binary of binaries) {
      const binaryPath = join(path, binary);
      if (existsSync(binaryPath) && statSync(binaryPath).isFile()) {
        await new ChildProcess(`cp ${binaryPath} ${this.INSTALL_TARGET}`)
          .handler;
        await new ChildProcess(`chmod +x ${this.INSTALL_TARGET}`).handler;
        return true;
      }
    }
    return false;
  }

  private static findRoot() {
    const startingDirectory = join(cwd(), "..");
    const tokens = startingDirectory.split("/");
    let attempts = 10;
    while (
      --attempts > 0 &&
      !existsSync(join(tokens.join("/"), "package.json"))
    ) {
      tokens.push("..");
    }
    const joined = tokens.join("/");
    if (!existsSync(join(joined, "package.json"))) {
      throw new Error("Failed to find installation root");
    }
    return joined;
  }
}
