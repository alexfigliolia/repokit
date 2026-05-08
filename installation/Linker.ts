import { cwd } from "node:process";
import { join } from "node:path";
import { existsSync } from "node:fs";

import { ChildProcess } from "@figliolia/child-process";

export class Linker {
  public static readonly TARGET_BINARY_PATH = join(
    cwd(),
    "node_modules",
    ".bin",
    "repokit",
  );

  public static async moveAndMakeCallable(binaryPath: string) {
    const binaryNames = ["repokit", "repokit.exe"];
    for (const name of binaryNames) {
      const path = join(binaryPath, name);
      if (existsSync(path)) {
        await new ChildProcess(`cp ${path} ${Linker.TARGET_BINARY_PATH}`)
          .handler;
        await new ChildProcess(`chmod +x ${Linker.TARGET_BINARY_PATH}`).handler;
        return true;
      }
    }
    return false;
  }
}
