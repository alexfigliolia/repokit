import { RepoKitCommand, RepoKitConfig, RepoKitTemplate } from "@repokit/core";

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
  templates: [
    new RepoKitTemplate({
      name: "bazel-template",
      description: "A set of tools for working with bazel packages",
      commands: {
        build: {
          command: "bazel build //src/main:app",
          description: "Builds the current package for production",
          args: {
            "(--features=<feature>)": "Turns specific build features on or off",
            "(--jobs=<N>)":
              "Limits the number of CPU cores Bazel can use at the same time",
            "(--sandbox_debug)":
              "Helps you debug build failures by showing exactly what happened inside the isolated build sandbox",
          },
        },
        run: {
          command: "bazel run //src/main:app",
          description: "Builds and runs the current package",
        },
        watch: {
          command: "ibazel build //src/main:app",
          description: "Runs the current package in development mode",
        },
      },
    }),
  ],
});
