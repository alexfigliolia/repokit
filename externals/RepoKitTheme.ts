import type { IRepoKitTheme, RepoKitThemeColors } from "./types";

/**
 * Repokit Theme
 *
 * A repokit theme allows you to customize the color
 * usage of the CLI by adding a `RepoKitTheme` instance
 * to your `RepoKitConfig`
 *
 * ```typescript
 * import { RepoKitConfig, RepoKitTheme } from "@repokit/core";
 *
 * export const RepoKit = new RepoKitConfig({
 *   project: "Repokit",
 *   themes: [
 *     new RepoKitTheme({
 *       name: "my-theme",
 *       colors: {
 *         prefixColor: "rgb(220, 36, 91)",
 *         commandColor: "rgb(220, 36, 36)",
 *         subcommandColor: "rgb(220, 131, 36)",
 *         argColor: "rgb(220, 205, 36)",
 *         descriptionColor: "rgb(95, 28, 71)",
 *         errorPrefixColor: "rgb(220, 36, 39)",
 *         hightlightColor: "rgb(8, 98, 255)",
 *       }
 *     }),
 *   ]
 * });
 * ```
 *
 * To enable your theme you can run
 * ```bash
 * repokit themes --set my-theme
 * ```
 */
export class RepoKitTheme implements IRepoKitTheme {
  public readonly name: string;
  public readonly colors: RepoKitThemeColors;
  constructor({ name, colors }: IRepoKitTheme) {
    this.name = name;
    this.colors = colors;
  }
}
