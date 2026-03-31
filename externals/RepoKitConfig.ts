import { RepoKitCommand } from "./RepoKitCommand";
import type { RepoKitTheme } from "./RepoKitTheme";
import type { ICommand, IRepoKitConfig } from "./types";
/* eslint-disable typescript-eslint(no-misused-spread */

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
