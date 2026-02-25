import { register } from "ts-node";

export class TSCompiler {
  static {
    const service = register();
    service.enabled(true);
  }
}
