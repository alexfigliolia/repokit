import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["externals/**/*"],
  dts: true,
  shims: true,
  clean: true,
  unbundle: true,
  skipNodeModulesBundle: true,
  tsconfig: "./tsconfig.build.json",
});
