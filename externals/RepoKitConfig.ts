import type { ICommand, IRepoKitConfig } from "./types";
import type { RepoKitTheme } from "./RepoKitTheme";
import { RepoKitCommand } from "./RepoKitCommand";
/* oxlint-disable typescript-eslint/no-misused-spread */

/**
 * ## RepoKitConfig
 *
 * Your interface for configuring RepoKit for your repository
 *
 * ```typescript
 * import { RepoKitConfig, RepoKitTheme } from "@repokit/core";
 * import { Compression, CriticalPath } from "@repokit/ui-performance";
 *
 * export const Kit = new RepoKitConfig({
 *   project: "My Project Name",
 *   thirdParty: [Compression, CriticalPath],
 *   themes: [
 *     new RepoKitTheme({
 *       name: "my-awesome-theme",
 *       colors: {
 *         prefixColor: "rgb(220, 36, 91)",
 *         commandColor: "rgb(220, 36, 36)",
 *         subcommandColor: "rgb(220, 131, 36)",
 *         argColor: "rgb(220, 205, 36)",
 *         descriptionColor: "rgb(179, 100, 151)",
 *         errorPrefixColor: "rgb(220, 36, 39)",
 *         highlightColor: "rgb(237, 175, 41)",
 *       },
 *     }),
 *   ]
 * });
 * ```
 */
export class RepoKitConfig {
  project: string;
  themes: RepoKitTheme[];
  thirdParty: RepoKitCommand[];
  commands: Record<string, ICommand>;
  constructor({
    project,
    themes = [],
    commands = {},
    thirdParty = [],
  }: IRepoKitConfig) {
    this.themes = themes;
    this.project = project;
    this.commands = commands;
    this.thirdParty = thirdParty.map(command => new RepoKitCommand(command));
  }

  public toScoped(location: string) {
    return {
      ...this,
      thirdParty: this.thirdParty.map(command => ({ ...command, location })),
    };
  }
}
