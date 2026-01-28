import { parseArgs } from "node:util"
import { DevKitCommand } from "./DevKitCommand";
import { ILocatedCommand } from "./types";

export class CommandParser {
  public static async parse() {
    const paths = this.parsePaths().split(",").filter(Boolean);
    const commands: ILocatedCommand[] = [];
    for(const path of paths) {
      const declaredExports = await import(path);
      for(const key in declaredExports) {
        if(declaredExports[key] instanceof DevKitCommand) {
          commands.push({...declaredExports[key], location: path})
        }
      }
    }
    console.log(JSON.stringify(commands));
  }

  private static parsePaths() {
    try {
      return parseArgs({
        options: {
          paths: {
            default: "",
            multiple: false,
            short: "p",
            type: "string"
          }
        }
      }).values.paths;
    } catch {
      return ""
    }
  }
}