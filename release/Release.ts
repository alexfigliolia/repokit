import { createInterface } from "node:readline/promises";
import { join } from "node:path";
import { writeFile } from "node:fs/promises";
import { createReadStream } from "node:fs";

import { Logger, SemverRelease } from "@figliolia/semver";
import { ChildProcess } from "@figliolia/child-process";

export class Release extends SemverRelease {
  private static readonly CARGO_FILE_PATH = join(this.ROOT, "Cargo.toml");
  constructor() {
    super({
      onComplete: async version => {
        await Release.writeCargoVersion(version);
        Logger.info("Linting Everything...");
        await new ChildProcess("pnpm lint:all").handler;
        Logger.info("Compiling for production...");
        await new ChildProcess("pnpm build").handler;
      },
    });
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
    reader.close();
    return lines.join("\n");
  }
}
