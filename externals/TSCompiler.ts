import type { RegisterOptions } from "ts-node";
import { register } from "ts-node";

export class TSCompiler {
  private static readonly compilerOptions: RegisterOptions = {
    swc: true,
    typeCheck: false,
    transpileOnly: true,
    compilerOptions: {
      noEmit: true,
      module: "commonjs",
      isolatedModules: false,
    },
    moduleTypes: {
      "**": "cjs",
    },
  };

  public static compile<T extends Record<string, unknown>>(path: string) {
    const compiler = register(this.compilerOptions);
    compiler.enabled(true);
    const result = require(path) as T;
    compiler.enabled(false);
    return result;
  }
}
