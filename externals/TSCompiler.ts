import { register } from "ts-node";
import type { RegisterOptions } from "ts-node";

import type { UnwrappedBrigdeOperation } from "./types";

export class TSCompiler {
  private static PARSE_INDICATOR =
    "=============== REPOKIT PARSE FLAG ===============";
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
    try {
      const compiler = register(this.compilerOptions);
      compiler.enabled(true);
      const result = require(path) as T;
      compiler.enabled(false);
      return result;
    } catch (error) {
      console.error(`Failure to parse module at path ${path}`, error);
      return {} as T;
    }
  }

  public static wrapParsingOperation<F extends (...args: unknown[]) => unknown>(
    operation: F,
  ) {
    return (...params: Parameters<F>) => {
      const restore = this.plugExits();
      const result = operation(...params);
      if (result instanceof Promise) {
        return result.then(v =>
          this.toStdout(v, restore),
        ) as UnwrappedBrigdeOperation<F>;
      }
      return this.toStdout(result, restore) as UnwrappedBrigdeOperation<F>;
    };
  }

  private static toStdout<T>(result: T, ...callbacks: (() => void)[]) {
    if (typeof result !== "undefined") {
      console.log(
        `${this.PARSE_INDICATOR}${JSON.stringify(result)}${this.PARSE_INDICATOR}`,
      );
    }
    callbacks.forEach(c => c());
  }

  private static plugExits() {
    // oxlint-disable-next-line typescript-eslint(unbound-method)
    const { exit, abort } = process;
    process.abort = () => undefined as never;
    process.exit = (_code?: string | number | null) => undefined as never;
    return () => {
      process.exit = exit;
      process.abort = abort;
      process.exitCode = 0;
    };
  }
}
