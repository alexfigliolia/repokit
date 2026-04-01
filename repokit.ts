import { RepoKitCommand, RepoKitConfig } from "@repokit/core";

export const RepoKit = new RepoKitConfig({
  project: "Repokit",
  thirdParty: [
    new RepoKitCommand({
      name: "third-party-command",
      description: "Test description",
      commands: {
        test: {
          command: ["echo HELLO", "exit 1"].join("\n"),
          description: "run some tests",
          args: {
            "--coverage | -c": "Whether to report coverage",
          },
        },
      },
    }),
  ],
});
