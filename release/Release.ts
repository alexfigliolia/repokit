import { execSync } from "node:child_process";
import { createReadStream } from "node:fs";
import { writeFile } from "node:fs/promises";
import { join } from "node:path";
import { exit } from "node:process";
import { createInterface } from "node:readline/promises";
import { parseArgs } from "node:util";
import chalk from "chalk";
import { ChildProcess } from "@figliolia/child-process";
import packageFile from "../package.json";

export class Release {
  private static readonly ROOT = execSync("git rev-parse --show-toplevel")
    .toString()
    .trim();
  private static readonly RELEASE_TYPES = ["patch", "minor", "major"] as const;
  private static readonly CARGO_FILE_PATH = join(this.ROOT, "Cargo.toml");
  private static readonly PACKAGE_FILE_PATH = join(this.ROOT, "package.json");
  private static readonly INSTALL_SCRIPT = join(
    this.ROOT,
    "installation",
    "install.sh",
  );

  public static async run() {
    const releaseType = this.getReleaseType();
    const nextVersion = this.getNextVersion(releaseType);
    if (!nextVersion || nextVersion === packageFile.version) {
      return this.logAndExit(
        `Bumping ${chalk.red.bold(packageFile.version)} by a ${chalk.green.bold(releaseType)} failed. Please inspect the version and try again`,
      );
    }
    await this.writeVersion(nextVersion);
    console.log("Linting Everything...");
    await new ChildProcess("yarn lint:ts").handler;
    await new ChildProcess("yarn lint:rust").handler;
    console.log("Compiling for production...");
    await new ChildProcess("yarn build:ts").handler;
    console.log("Fin! 🚀");
  }

  private static getNextVersion(
    releaseType: ReturnType<typeof Release.getReleaseType>,
  ) {
    const { version } = packageFile;
    const [major, minor, patch] = version.split(".");
    if (!major || !minor || !patch) {
      return this.logAndExit(
        `The existing package version ${chalk.red.bold(version)} is not following semver. Fix this`,
      );
    }
    let nextVersion: string = version;
    switch (releaseType) {
      case "major":
        nextVersion = `${parseInt(major) + 1}.0.0`;
        break;
      case "minor":
        nextVersion = `${major}.${parseInt(minor) + 1}.0`;
        break;
      case "patch":
        nextVersion = `${major}.${minor}.${parseInt(patch) + 1}`;
        break;
    }
    if (nextVersion === packageFile.version) {
      return this.logAndExit(
        `Bumping ${chalk.red.bold(packageFile.version)} by a ${chalk.green.bold(releaseType)} failed. Please inspect the version and try again`,
      );
    }
    return nextVersion;
  }

  private static writeVersion(version: string) {
    return Promise.all([
      this.writeCargoVersion(version),
      this.writePackageVersion(version),
      this.updateInstallScript(version),
    ]);
  }

  private static async writePackageVersion(version: string) {
    packageFile.version = version;
    await writeFile(
      this.PACKAGE_FILE_PATH,
      JSON.stringify(packageFile, null, 2),
    );
  }

  private static async updateInstallScript(version: string) {
    let write = true;
    const content = await this.streamFileContent(this.INSTALL_SCRIPT, line => {
      if (write && line.startsWith('CURRENT_VERSION="')) {
        write = false;
        return `CURRENT_VERSION="${version}"`;
      }
      return line;
    });
    await writeFile(this.INSTALL_SCRIPT, content);
  }

  private static async writeCargoVersion(version: string) {
    let write = true;
    const content = await this.streamFileContent(this.CARGO_FILE_PATH, line => {
      if (write && line.startsWith('version = "')) {
        write = false;
        return `version = "${version}"`;
      }
      return line;
    });
    await writeFile(this.CARGO_FILE_PATH, content);
  }

  private static async streamFileContent(
    path: string,
    onLine: (line: string) => string,
  ) {
    const reader = createInterface({
      input: createReadStream(path),
      crlfDelay: Infinity,
    });
    const lines: string[] = [];
    for await (const line of reader) {
      lines.push(onLine(line));
    }
    return lines.join("\n");
  }

  private static getReleaseType() {
    const {
      values: { type },
    } = parseArgs({
      options: {
        type: {
          default: "patch",
          multiple: false,
          type: "string",
          short: "t",
        },
      },
    });
    const release = type as (typeof Release.RELEASE_TYPES)[number];
    if (!this.RELEASE_TYPES.includes(release)) {
      this.logAndExit(
        `The release type ${chalk.red.bold(type)} is invalid. Please specify one of ${chalk.green.bold(Array.from(this.RELEASE_TYPES).join(" | "))}`,
      );
      exit(0);
    }
    return release;
  }

  private static logAndExit(msg: string) {
    console.log(msg);
    exit(0);
  }
}
