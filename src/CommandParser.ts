import { parseArgs } from "node:util";
import { join } from "node:path";
import { stat } from "node:fs/promises";
import { existsSync } from "node:fs";

import type { ILocatedCommand } from "./types";
import { TaskPooler } from "./TaskPooler";
import { RepoKitCommand } from "./RepoKitCommand";
/* oxlint-disable typescript-eslint(no-misused-spread) */

export class CommandParser {
  public static async parse() {
    const { paths, root } = this.parsePaths();
    if (!root || !existsSync(root) || !(await stat(root)).isDirectory()) {
      return console.log(JSON.stringify([]));
    }
    const pathList = paths.split(",").filter(Boolean);
    const pool = new TaskPooler<ILocatedCommand[]>();
    const results = await Promise.all(
      pathList.map(path =>
        pool.enqueue(() => this.parseCommand(join(root, path))),
      ),
    );
    console.log(JSON.stringify(results.flat()));
  }

  private static async parseCommand(path: string) {
    const commands: ILocatedCommand[] = [];
    const declaredExports = await import(path);
    for (const key in declaredExports) {
      if (declaredExports[key] instanceof RepoKitCommand) {
        commands.push({ ...declaredExports[key], location: path });
      }
    }
    return commands;
  }

  private static parsePaths() {
    try {
      return parseArgs({
        options: {
          paths: {
            default: "",
            multiple: false,
            short: "p",
            type: "string",
          },
          root: {
            default: "",
            multiple: false,
            short: "r",
            type: "string",
          },
        },
      }).values;
    } catch {
      return { paths: "", root: "" };
    }
  }
}
