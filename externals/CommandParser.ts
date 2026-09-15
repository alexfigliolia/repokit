import { parseArgs } from "node:util";
import { join } from "node:path";
import { availableParallelism } from "node:os";
import { stat } from "node:fs/promises";
import { existsSync } from "node:fs";

import { ThreadPool } from "@figliolia/thread-pool/node";

import type { ILocatedCommand } from "./types";
import { TSCompiler } from "./TSCompiler";
import { RepoKitTemplate } from "./RepoKitTemplate";
import { RepoKitCommand } from "./RepoKitCommand";
import { FileParseResult } from "./FileParseResult";
import { FileParseError } from "./FileParseError";

export class CommandParser extends TSCompiler {
  public static readonly parse = this.wrapParsingOperation(async () => {
    const { paths, root } = this.parsePaths();
    if (!root || !existsSync(root) || !(await stat(root)).isDirectory()) {
      return FileParseResult.error(
        new FileParseError(root, "The root path is not a directory"),
      );
    }
    const pathList = paths.split(",").filter(Boolean);
    const maxConcurrency = Math.floor((availableParallelism() / 2) * 10);
    if (pathList.length >= maxConcurrency) {
      return this.multiThread(root, pathList, maxConcurrency);
    }
    return FileParseResult.from<ILocatedCommand[], FileParseError>(
      pathList.flatMap(path => this.parseCommand(root, path)),
    );
  });

  public static parseCommand(root: string, path: string) {
    const commands: ILocatedCommand[] = [];
    const commandFileExports = super.compile(join(root, path));
    for (const key in commandFileExports) {
      if (
        !(commandFileExports[key] instanceof RepoKitTemplate) &&
        commandFileExports[key] instanceof RepoKitCommand
      ) {
        // oxlint-disable-next-line typescript-eslint/no-misused-spread
        commands.push({ ...commandFileExports[key], location: path });
      }
    }
    return commands;
  }

  private static async multiThread(
    root: string,
    pathList: string[],
    maxConcurrency: number,
  ) {
    const pool = new ThreadPool<ThreadArgs, ILocatedCommand[]>({
      maxConcurrency,
      workerScript: new URL(
        "./workers/ParseCommandsWorker.mjs",
        // @ts-expect-error "node assumes a common.js target when type: "module" is not specified in package.json"
        import.meta.url,
      ),
    });
    const threadResults = await Promise.all(
      pathList.map(path =>
        pool.enqueueTask({ root, path }).catch(error => {
          void pool.shutDown();
          throw error;
        }),
      ),
    );
    void pool.shutDown();
    return FileParseResult.from<ILocatedCommand[], FileParseError>(
      threadResults.flatMap(commands => commands),
    );
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
