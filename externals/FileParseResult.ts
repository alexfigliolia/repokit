import type { ParseResult } from "./types";

export class FileParseResult<T, E = unknown> {
  public readonly error?: E;
  public readonly result?: T;
  constructor({ result, error }: ParseResult<T, E>) {
    this.result = result;
    this.error = error;
  }

  public static from<T, E = unknown>(result: T) {
    return new FileParseResult<T, E>({ result });
  }

  public static error<T, E = unknown>(error: E) {
    return new FileParseResult<T, E>({ error });
  }
}
