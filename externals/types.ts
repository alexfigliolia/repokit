import type { RepoKitConfig } from "./RepoKitConfig";

export interface IRepoKitConfig extends Omit<
  Partial<RepoKitConfig>,
  "project"
> {
  project: string;
}

export interface IRepoKitCommand {
  name: string;
  owner?: string;
  description: string;
  commands: Record<string, ICommand>;
}

export interface ICommand {
  command: string;
  description: string;
  args?: Record<string, string>;
}

export interface ILocatedCommand extends IRepoKitCommand {
  location: string;
}

export type AsyncTask<T> = () => Promise<T>;

type OptionalSpace = " " | "";

export type RGBString =
  `rgb(${number},${OptionalSpace}${number},${OptionalSpace}${number})`;

export interface RepoKitThemeColors {
  prefixColor?: RGBString;
  commandColor?: RGBString;
  subcommandColor?: RGBString;
  argColor?: RGBString;
  descriptionColor?: RGBString;
  errorPrefixColor?: RGBString;
  highlightColor?: RGBString;
}

export interface IRepoKitTheme {
  name: string;
  colors: RepoKitThemeColors;
}

export type UnwrappedBrigdeOperation<
  F extends (...args: unknown[]) => unknown,
  T = void,
> = ReturnType<F> extends Promise<unknown> ? Promise<T> : T;
