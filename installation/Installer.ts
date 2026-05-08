import { cwd } from "node:process";
import { join } from "node:path";

import { ChildProcess } from "@figliolia/child-process";

import { Linker } from "./Linker";
import { Downloader } from "./Downloader";
import { BinaryResolver } from "./BinaryResolver";

export class Installer extends Linker {
  public static async run() {
    const binary = BinaryResolver.matchBinary();
    console.log("using", binary);
    if (
      binary !== undefined &&
      BinaryResolver.BINARY_TARGETS.includes(binary)
    ) {
      if (
        (await Downloader.installBinary(`repokit-${binary}`).catch(
          () => {},
        )) === true
      ) {
        return;
      }
    }
    if (!(await this.compileSource())) {
      throw new Error("Unsupported platform");
    }
  }

  private static async compileSource() {
    try {
      await new ChildProcess("cargo build --release").handler;
      const binaryPath = join(cwd(), "target", "release");
      return await this.moveAndMakeCallable(binaryPath);
    } catch {
      return false;
    }
  }
}
