import type { ICommand, IRepoKitCommand } from "./types";

export class RepoKitCommand {
  name: string;
  owner: string;
  description: string;
  commands: Record<string, ICommand>;
  constructor({
    name,
    description,
    owner = "",
    commands = {},
  }: IRepoKitCommand) {
    this.name = name;
    this.owner = owner;
    this.commands = commands;
    this.description = description;
  }
}
