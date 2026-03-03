import { parseArgs } from "node:util";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { mkdir, rm, symlink } from "node:fs/promises";
import { existsSync } from "node:fs";
import type { ChildProcess as Child_Process } from "node:child_process";

import { ChildProcess } from "@figliolia/child-process";

export class Compiler {
  private static TS_PROCESS?: Child_Process;
  private static readonly TSC_COMMAND = "yarn tsdown";
  // @ts-ignore
  private static readonly FILE = fileURLToPath(import.meta.url);
  private static readonly SCRIPT_ORIGIN = dirname(this.FILE);
  private static readonly DIST_DIRECTORY = join(this.SCRIPT_ORIGIN, "../dist");
  private static readonly SOURCE_DIRECTORY = join(
    this.SCRIPT_ORIGIN,
    "../externals",
  );
  private static readonly SYMLINK_ROOT = join(
    this.SCRIPT_ORIGIN,
    "../node_modules/@repokit/core",
  );
  private static readonly PACKAGE_FILE = join(
    this.SCRIPT_ORIGIN,
    "../package.json",
  );
  private static readonly SYMLINK_BUILD_TARGET = join(
    this.SYMLINK_ROOT,
    "dist",
  );
  private static readonly SYMLINK_SOURCE_TARGET = join(
    this.SYMLINK_ROOT,
    "externals",
  );
  private static readonly SYMLINK_PACKAGE_FILE_TARGET = join(
    this.SYMLINK_ROOT,
    "package.json",
  );

  public static run() {
    const { watch, clean } = this.parseArgs();
    if (watch) {
      return this.watch();
    } else if (clean) {
      return this.clean();
    }
    return this.build();
  }

  public static async watch() {
    await this.createSymlinks();
    this.bindToExit();
    const { handler, process } = new ChildProcess(
      `${this.TSC_COMMAND} --watch`,
    );
    this.TS_PROCESS = process;
    return handler;
  }

  public static async build() {
    await this.clean();
    await new ChildProcess(this.TSC_COMMAND).handler;
    await this.createSymlinks();
  }

  private static bindToExit() {
    process.on("exit", () => this.close());
    process.on("SIGINT", () => this.close());
  }

  private static close() {
    this.TS_PROCESS?.kill?.();
    void this.clean();
  }

  private static clean() {
    return Promise.all([
      rm(this.DIST_DIRECTORY, { recursive: true, force: true }),
      rm(join(this.SYMLINK_ROOT, "../"), { recursive: true, force: true }),
    ]);
  }

  private static async createSymlinks() {
    if (!existsSync(this.SYMLINK_ROOT)) {
      await mkdir(this.SYMLINK_ROOT, { recursive: true });
    }
    if (!existsSync(this.SYMLINK_BUILD_TARGET)) {
      await symlink(this.DIST_DIRECTORY, this.SYMLINK_BUILD_TARGET);
    }
    if (!existsSync(this.SYMLINK_SOURCE_TARGET)) {
      await symlink(this.SOURCE_DIRECTORY, this.SYMLINK_SOURCE_TARGET);
    }
    if (!existsSync(this.SYMLINK_PACKAGE_FILE_TARGET)) {
      await symlink(this.PACKAGE_FILE, this.SYMLINK_PACKAGE_FILE_TARGET);
    }
  }

  private static parseArgs() {
    return parseArgs({
      options: {
        watch: {
          default: false,
          short: "w",
          type: "boolean",
          multiple: false,
        },
        clean: {
          default: false,
          short: "c",
          type: "boolean",
          multiple: false,
        },
      },
    }).values;
  }
}
