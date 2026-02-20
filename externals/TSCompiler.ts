// @ts-ignore
import { register as pathRegister } from "tsconfig-paths";
import { register } from "ts-node";

export class TSCompiler {
  static {
    pathRegister();
    const service = register();
    service.enabled(true);
  }
}
