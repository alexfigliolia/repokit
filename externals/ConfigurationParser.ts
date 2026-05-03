import { parseArgs } from "node:util";
import { join } from "node:path";
import { existsSync } from "node:fs";

import { TSCompiler } from "./TSCompiler";
import { RepoKitConfig } from "./RepoKitConfig";

export class ConfigurationParser extends TSCompiler {
  public static readonly parse = this.wrapParsingOperation(() => {
    const root = this.parseRoot();
    const path = join(root, "repokit.ts");
    if (!existsSync(path)) {
      return undefined;
    }
    const config = super.compile(path);
    for (const key in config) {
      if (config[key] instanceof RepoKitConfig) {
        return config[key].toScoped(path);
      }
    }
    return undefined;
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
