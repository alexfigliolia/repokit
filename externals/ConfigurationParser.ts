import { parseArgs } from "node:util";
import { join } from "node:path";
import { existsSync } from "node:fs";

import { TSCompiler } from "./TSCompiler";
import { RepoKitConfig } from "./RepoKitConfig";

export class ConfigurationParser extends TSCompiler {
  public static async parse() {
    const root = this.parseRoot();
    const path = join(root, "repokit.ts");
    if (!existsSync(path)) {
      return;
    }
    const config = require(path);
    for (const key in config) {
      if (config[key] instanceof RepoKitConfig) {
        return console.log(JSON.stringify(config[key].toScoped(path)));
      }
    }
  }

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
