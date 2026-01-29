import { DevKitConfig } from "@devkit/core";

export const DevKit = new DevKitConfig({
  project: "Devkit",
  workspaces: ["./workspace-1/*", "./workspace-2/*"],
  commands: {
    "lint:rust": {
      command: "cargo clippy",
      description: "Lints rust files",
    },
    "format:rust": {
      command: "cargo clippy --fix",
      description: "Formats rust files",
    },
    "build:rust": {
      command: "cargo build --release",
      description: "Build CLI in production mode",
    },
    "run:rust": {
      command: "cargo run",
      description: "Run CLI in production mode",
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
  },
});
