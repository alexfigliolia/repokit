import { join } from "node:path"
import { parseArgs } from "node:util"
import { DevKitConfig } from "./DevKitConfig";
import { existsSync } from "node:fs";

export class ConfigurationParser {
  public static async parse() {
    const root = this.parseRoot()
    const path = join(root, 'devkit.ts');
    if(!existsSync(path)) {
      return;
    }
    const config = await import(path);
    for(const key in config) {
      if(config[key] instanceof DevKitConfig) {
        console.log(JSON.stringify(config[key]));
        return;
      }
    }
  }

  private static parseRoot() {
    return parseArgs({
      options: {
        root: {
          default: '',
          multiple: false,
          short: 'r',
          type: 'string'
        }
      }
    }).values.root
  }
}