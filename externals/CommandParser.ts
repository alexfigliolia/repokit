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
    const commands = pathList.map(path => this.parseCommand(root, path));
    console.log(JSON.stringify(commands.flat()));
  }

  private static parseCommand(root: string, path: string) {
    const commands: ILocatedCommand[] = [];
    const declaredExports = super.compile(join(root, path));
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
