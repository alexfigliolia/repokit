import { parseArgs } from "node:util";
import { join } from "node:path";
import { stat } from "node:fs/promises";
import { existsSync } from "node:fs";

import type { ILocatedCommand } from "./types";
import { TSCompiler } from "./TSCompiler";
import { RepoKitCommand } from "./RepoKitCommand";
/* oxlint-disable typescript-eslint(no-misused-spread) */

export class CommandParser extends TSCompiler {
  public static async parse() {
    const { paths, root } = this.parsePaths();
    if (!root || !existsSync(root) || !(await stat(root)).isDirectory()) {
      return console.log(JSON.stringify([]));
    }
    const pathList = paths.split(",").filter(Boolean);
    const results = pathList.map(path => this.parseCommand(join(root, path)));
    console.log(JSON.stringify(results.flat()));
  }

  private static parseCommand(path: string) {
    const commands: ILocatedCommand[] = [];
    const declaredExports = require(path);
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
