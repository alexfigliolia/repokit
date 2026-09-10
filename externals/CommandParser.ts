import { parseArgs } from "node:util";
import { fileURLToPath } from "node:url";
import { join } from "node:path";
import { stat } from "node:fs/promises";
import { existsSync } from "node:fs";

import type { ILocatedCommand } from "./types";
import { TSCompiler } from "./TSCompiler";
import { RepoKitTemplate } from "./RepoKitTemplate";
import { RepoKitCommand } from "./RepoKitCommand";
import { WorkerPool } from "./concurrency";

export class CommandParser extends TSCompiler {
  public static readonly parse = this.wrapParsingOperation(async () => {
    const { paths, root } = this.parsePaths();
    if (!root || !existsSync(root) || !(await stat(root)).isDirectory()) {
      return [];
    }
    const pathList = paths.split(",").filter(Boolean);
    const maxConcurrency = Math.floor((WorkerPool.CORES / 2) * 10);
    if (pathList.length >= maxConcurrency) {
      const pool = new WorkerPool<ThreadArgs, ThreadResult>({
        maxConcurrency,
        maxUtilization: 0.5,
        workerScript: new URL(
          "./workers/ParseCommandsWorker.mjs",
          fileURLToPath(__dirname),
        ),
      });
      const threadResults = await Promise.all(
        pathList.map(path => pool.enqueue({ root, path }).then(v => v.result)),
      );
      void pool.shutDownBackground();
      return threadResults.flatMap(commands => commands);
    }
    return pathList.map(path => this.parseCommand(root, path)).flat();
  });

  public static parseCommand(root: string, path: string) {
    const commands: ILocatedCommand[] = [];
    const declaredExports = super.compile(join(root, path));
    for (const key in declaredExports) {
      if (
        !(declaredExports[key] instanceof RepoKitTemplate) &&
        declaredExports[key] instanceof RepoKitCommand
      ) {
        // oxlint-disable-next-line typescript-eslint/no-misused-spread
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

interface ThreadArgs {
  root: string;
  path: string;
}

export interface ThreadResult {
  result: ILocatedCommand[];
}
