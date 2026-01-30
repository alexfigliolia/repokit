import type { ICommand, IDevKitCommand } from "./types";

export class DevKitCommand {
  name: string;
  owner: string;
  description: string;
  commands: Record<string, ICommand>;
  constructor({
    name,
    description,
    owner = "",
    commands = {},
  }: IDevKitCommand) {
    this.name = name;
    this.owner = owner;
    this.commands = commands;
    this.description = description;
  }

  public toJSON() {
    const { name, owner, commands, description } = this;
    return { name, owner, commands, description };
  }
}
