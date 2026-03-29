import { RGBString } from "./types";

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
 *   theme: new RepoKitTheme({
 *     prefixColor: "rgb(220, 36, 91)",
 *     commandColor: "rgb(220, 36, 36)",
 *     subcommandColor: "rgb(220, 131, 36)",
 *     argColor: "rgb(220, 205, 36)",
 *     descriptionColor: "rgb(95, 28, 71)",
 *     errorPrefixColor: "rgb(220, 36, 39)",
 *     hightlightColor: "rgb(8, 98, 255)",
 *   }),
 * });
 * ```
 */
export class RepoKitTheme {
  prefixColor?: RGBString;
  commandColor?: RGBString;
  subcommandColor?: RGBString;
  argColor?: RGBString;
  descriptionColor?: RGBString;
  errorPrefixColor?: RGBString;
  highlightColor?: RGBString;
  constructor({
    prefixColor,
    commandColor,
    subcommandColor,
    argColor,
    descriptionColor,
    errorPrefixColor,
    highlightColor,
  }: RepoKitTheme) {
    this.prefixColor = prefixColor;
    this.commandColor = commandColor;
    this.subcommandColor = subcommandColor;
    this.argColor = argColor;
    this.descriptionColor = descriptionColor;
    this.errorPrefixColor = errorPrefixColor;
    this.highlightColor = highlightColor;
  }
}
