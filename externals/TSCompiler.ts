import { register, type RegisterOptions } from "ts-node";

import { FileParseResult } from "./FileParseResult";
import { FileParseError } from "./FileParseError";

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
    } catch (error: unknown) {
      throw FileParseResult.error<T, FileParseError>(
        // @ts-expect-error "error unknown"
        new FileParseError(path, error?.message),
      );
    }
  }

  public static wrapParsingOperation<
    F extends (
      ...args: unknown[]
    ) =>
      | FileParseResult<any, FileParseError>
      | Promise<FileParseResult<any, FileParseError>>,
  >(operation: F) {
    return (...params: Parameters<F>) => {
      const restore = this.plugExits();
      try {
        const result = operation(...params);
        if (result instanceof Promise) {
          void result
            .then(v => this.toStdout(v, restore))
            .catch(e => this.toStdout(e, restore));
        } else {
          this.toStdout(result, restore);
        }
      } catch (error) {
        this.toStdout(
          error instanceof FileParseResult
            ? error
            : FileParseResult.error(error),
          restore,
        );
      }
    };
  }

  private static toStdout<T>(
    result: FileParseResult<T>,
    ...callbacks: (() => void)[]
  ) {
    if (typeof result !== "undefined") {
      console.log(
        `${this.PARSE_INDICATOR}${JSON.stringify(result)}${this.PARSE_INDICATOR}`,
      );
    }
    callbacks.forEach(c => c());
  }

  private static plugExits() {
    // oxlint-disable-next-line typescript-eslint/unbound-method
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
