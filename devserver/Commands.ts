import { RepoKitCommand } from "@repokit/core";

export const Commands = new RepoKitCommand({
  name: "devserver",
  owner: "Alex Figliolia",
  description: "Development workflows for repokit",
  commands: {
    build: {
      command: "yarn tsx $(pwd)/run.ts",
      description: "Setup symlinks and build typescript files for production",
    },
    watch: {
      command: "yarn tsx $(pwd)/run.ts -w",
      description: "Setup symlinks and watch typescript files for development",
    },
    clean: {
      command: "yarn tsx $(pwd)/run.ts -c",
      description: "Clean build folder",
    },
  },
});
