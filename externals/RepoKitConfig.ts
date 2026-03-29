import { RepoKitCommand } from "./RepoKitCommand";
import type { RepoKitTheme } from "./RepoKitTheme";
import type { ICommand, IRepoKitConfig } from "./types";
/* eslint-disable typescript-eslint(no-misused-spread */

export class RepoKitConfig {
  project: string;
  theme?: RepoKitTheme;
  thirdParty: RepoKitCommand[];
  commands: Record<string, ICommand>;
  constructor({
    theme,
    project,
    commands = {},
    thirdParty = [],
  }: IRepoKitConfig) {
    this.theme = theme;
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
