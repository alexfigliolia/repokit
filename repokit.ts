import { RepoKitCommand, RepoKitConfig } from "@repokit/core";

export const RepoKit = new RepoKitConfig({
  project: "Repokit",
  thirdParty: [
    new RepoKitCommand({
      name: "test-external-package",
      owner: "Test Owner",
      description: "Test description",
      commands: {
        TEST: {
          command: "cargo test",
          description: "run some tests",
          args: {
            "--coverage | -c": "Whether to report coverage",
          },
        },
      },
    }),
  ],
});
