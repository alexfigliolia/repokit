import { RepoKitCommand } from "./RepoKitCommand";

/**
 * ## RepoKitTemplate
 *
 * Reuseable templates for scaffolding repokit commands around common toolchains
 *
 * ```typescript
 * import { RepoKitTemplate, RepoKitConfig } from "@repokit/core";
 *
 * export const BazelTemplate = new RepoKitTemplate({
 *   name: "bazel-template",
 *   description: "A set of tools for working with bazel packages",
 *   commands: {
 *     build: {
 *       command: "bazel build //src/main:app",
 *       description: "Builds the current package for production",
 *       args: {
 *         "(--features=<feature>)": "Turns specific build features on or off",
 *         "(--jobs=<N>)": "Limits the number of CPU cores Bazel can use at the same time",
 *         "(--sandbox_debug)": "Helps you debug build failures by showing exactly what happened inside the isolated build sandbox"
 *       }
 *     },
 *     run: {
 *       command: "bazel run //src/main:app",
 *       description: "Builds and runs the current package"
 *     },
 *     run: {
 *       command: "ibazel build //src/main:app",
 *       description: "Runs the current package in development mode"
 *     }
 *   }
 * });
 *
 * export const Repokit = new RepokitConfig({
 *   templates: [BazelTemplate]
 * });
 * ```
 *
 * To use your template when registering commands:
 *
 * ```bash
 * repokit register ./path/to/packages --template bazel-template
 * ```
 */
export class RepoKitTemplate extends RepoKitCommand {}
