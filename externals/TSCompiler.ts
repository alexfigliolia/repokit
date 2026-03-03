import { basename, resolve } from "node:path";
import { readFile, rm, writeFile } from "node:fs/promises";

import { transform } from "@swc/core";

export class TSCompiler {
  private static readonly TMP_FILE_NAME = ".repokit_tmp.js";
  constructor(
    public readonly root: string,
    public readonly command: string,
  ) {}

  public async compile(path: string) {
    const source = (await readFile(path)).toString();
    const result = await transform(source, {
      module: {
        type: "es6",
      },
    });
    const target = `${resolve(path, "../")}/${basename(path)}${TSCompiler.TMP_FILE_NAME}`;
    await writeFile(target, result.code);
    const config = await import(target);
    await rm(target, { force: true });
    return config;
  }
}
