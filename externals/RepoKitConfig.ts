import { RepoKitCommand } from "./RepoKitCommand";
import type { ICommand, IRepoKitConfig } from "./types";
/* eslint-disable typescript-eslint(no-misused-spread */

export class RepoKitConfig implements Required<IRepoKitConfig> {
  project: string;
  thirdParty: RepoKitCommand[];
  commands: Record<string, ICommand>;
  constructor({ project, commands = {}, thirdParty = [] }: IRepoKitConfig) {
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
