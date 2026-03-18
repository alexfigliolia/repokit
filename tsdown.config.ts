import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["externals/index.ts", "externals/commands/*.ts"],
  dts: true,
  shims: true,
  clean: true,
  unbundle: true,
  tsconfig: "./tsconfig.build.json",
});
