import { createReadStream } from "node:fs";
import { writeFile } from "node:fs/promises";
import { join } from "node:path";
import { createInterface } from "node:readline/promises";
import { ChildProcess } from "@figliolia/child-process";
import { Logger, SemverRelease } from "@figliolia/semver";

export class Release extends SemverRelease {
  private static readonly INSTALL_SCRIPT = join(
    this.ROOT,
    "installation",
    "install.sh",
  );
  private static readonly CARGO_FILE_PATH = join(this.ROOT, "Cargo.toml");
  static {
    console.log(this.ROOT);
  }
  constructor() {
    super({
      onComplete: async version => {
        await Release.writeVersion(version);
        Logger.info("Linting Everything...");
        await new ChildProcess("yarn lint:ts").handler;
        await new ChildProcess("yarn lint:rust").handler;
        Logger.info("Compiling for production...");
        await new ChildProcess("yarn build:ts").handler;
      },
    });
  }

  private static writeVersion(version: string) {
    return Promise.all([
      this.writeCargoVersion(version),
      this.updateInstallScript(version),
    ]);
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
}
