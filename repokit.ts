import { RepoKitConfig } from "@repokit/core";

export const RepoKit = new RepoKitConfig({
  project: "Repokit",
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
      command: "cargo run --package repokit --bin repokit",
      description: "Run CLI in development mode",
    },
    "install:rust": {
      command: "repokit build:rust && cargo install --path .",
      description: "Installs the production CLI and adds it to your path",
    },
    "lint:ts": {
      command:
        "yarn oxlint --type-aware --type-check --report-unused-disable-directives --fix && yarn oxfmt",
      description: "Lints typescript files",
    },
  },
});
