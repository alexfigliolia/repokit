import type { ICommand, IDevKitCommand } from "./types";

export class DevKitCommand implements IDevKitCommand {
  name: string;
  description: string;
  commands: Record<string, ICommand>;
  constructor({ name, description, commands = {} }: IDevKitCommand) {
    this.name = name;
    this.commands = commands;
    this.description = description;
  }

  public toJSON() {
    const { name, commands, description } = this;
    return { name, commands, description };
  }
}
