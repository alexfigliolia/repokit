import { RepoKitCommand } from "@repokit/core";

export const Commands = new RepoKitCommand({
  name: "release",
  owner: "Alex Figliolia",
  description: "Release workflows for repokit",
  commands: {
    patch: {
      command: "pnpm tsx $(pwd)/run.ts -t patch",
      description: `Bumps the patch version and lints the code base`,
    },
    minor: {
      command: "pnpm tsx $(pwd)/run.ts -t minor",
      description: `Bumps the minor version and lints the code base`,
    },
    major: {
      command: "pnpm tsx $(pwd)/run.ts -t major",
      description: `Bumps the major version and lints the code base`,
    },
  },
});
