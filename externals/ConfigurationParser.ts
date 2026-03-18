import { existsSync } from "node:fs";
import { join } from "node:path";
import { parseArgs } from "node:util";
import { RepoKitConfig } from "./RepoKitConfig";
import { TSCompiler } from "./TSCompiler";

export class ConfigurationParser extends TSCompiler {
  public static parse() {
    const root = this.parseRoot();
    const path = join(root, "repokit.ts");
    if (!existsSync(path)) {
      return;
    }
    const config = super.compile(path);
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
