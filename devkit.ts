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
    "lint:ts": {
      command:
        "yarn oxlint --type-aware --type-check --report-unused-disable-directives --fix && yarn oxfmt",
      description: "Lints typescript files",
    },
  },
});
