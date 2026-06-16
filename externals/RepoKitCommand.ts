import type { ICommand, IRepoKitCommand } from "./types";

/**
 * ## RepoKitCommand
 *
 * The command definition interface for your repokit toolchain.
 * Collocated with your feature code, you can export a `RepoKitCommand`
 * instance to produce a published toolchain for your feature - available
 * in the repokit cli
 *
 * ```typescript
 * import { RepoKitCommand } from "@repokit/core";
 *
 * export const Commands = new RepoKitCommand({
 *   name: "my-feature",
 *   description: "A set of tools for working with 'my-feature'",
 *   commands: {
 *     build: {
 *       command: "node ./build.js",
 *       description: "builds 'my-feature' for production",
 *       args: {
 *         (--compress | -c): "When specified compresses static files",
 *         (--out-dir | -o): "An absolute path to your desired output directory"
 *       }
 *     }
 *   }
 * });
 * ```
 */
export class RepoKitCommand {
  name: string;
  owner: string;
  description: string;
  commands: Record<string, ICommand>;
  constructor({ name, description, owner = "", commands }: IRepoKitCommand) {
    this.name = name;
    this.owner = owner;
    this.commands = commands;
    this.description = description;
  }
}
