import { parseArgs } from "node:util";
import { join } from "node:path";
import { existsSync } from "node:fs";

import { TSCompiler } from "./TSCompiler";
import { RepoKitConfig } from "./RepoKitConfig";
import { FileParseResult } from "./FileParseResult";
import { FileParseError } from "./FileParseError";

export class ConfigurationParser extends TSCompiler {
  public static readonly parse = this.wrapParsingOperation(() => {
    const root = this.parseRoot();
    const path = join(root, "repokit.ts");
    if (!existsSync(path)) {
      return FileParseResult.error(
        new FileParseError(path, "Repokit configuration does not exist"),
      );
    }
    const configurationFileExports = super.compile(path);
    for (const key in configurationFileExports) {
      if (configurationFileExports[key] instanceof RepoKitConfig) {
        return FileParseResult.from(
          configurationFileExports[key].toScoped(path),
        );
      }
    }
    return FileParseResult.error(new FileParseError(path));
  });

  private static parseRoot() {
    return parseArgs({
      options: {
        root: {
          default: "",
          multiple: false,
          short: "r",
          type: "string",
        },
      },
    }).values.root;
  }
}
