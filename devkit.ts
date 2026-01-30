import { DevKitConfig } from "@devkit/core";

export const DevKit = new DevKitConfig({
  project: "Devkit",
  commands: {
    "lint:rust": {
      command: "cargo clippy",
      description: "Lints rust files",
    },
    "format:rust": {
      command: "cargo clippy --fix --allow-dirty",
      description: "Formats rust files",
    },
    "build:rust": {
      command: "cargo build --release",
      description: "Build CLI in production mode",
    },
    "run:rust": {
      command: "cargo run --package devkit --bin devkit",
      description: "Run CLI in development mode",
    },
    "install:rust": {
      command: "cargo install --path .",
      description: "Installs the production CLI and adds it to your path",
    },
    "lint:ts": {
      command:
        "yarn oxlint --type-aware --type-check --report-unused-disable-directives --fix && yarn oxfmt",
      description: "Lints typescript files",
    },
    "build:ts": {
      command: "yarn ts-packager -e src",
      description: "Builds the typescript package",
    },
  },
});
