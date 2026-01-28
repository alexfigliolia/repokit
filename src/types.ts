export interface IDevKitConfig {
  project: string;
  workspaces: string[];
  commands?: Record<string, ICommand>;
}

export interface IDevKitCommand {
  name: string;
  description: string;
  commands: Record<string, ICommand>;
}

export interface ICommand {
  command: string;
  description: string;
}

export interface ILocatedCommand extends IDevKitCommand {
  location: string;
}
