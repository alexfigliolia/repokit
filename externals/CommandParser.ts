import { existsSync } from "node:fs";
import { stat } from "node:fs/promises";
import { join } from "node:path";
import { parseArgs } from "node:util";
import { RepoKitCommand } from "./RepoKitCommand";
import { TSCompiler } from "./TSCompiler";
import type { ILocatedCommand } from "./types";
/* oxlint-disable typescript-eslint(no-misused-spread) */

export class CommandParser extends TSCompiler {
  public static async parse() {
    const { paths, root } = this.parsePaths();
    if (!root || !existsSync(root) || !(await stat(root)).isDirectory()) {
      return console.log(JSON.stringify([]));
    }
    const pathList = paths.split(",").filter(Boolean);
    const commands = pathList.map(path => this.parseCommand(join(root, path)));
    console.log(JSON.stringify(commands.flat()));
  }

  private static parseCommand(path: string) {
    const commands: ILocatedCommand[] = [];
    const declaredExports = super.compile(path);
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
