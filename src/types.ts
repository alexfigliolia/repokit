export interface IDevKitConfig {
  project: string;
  commands?: Record<string, ICommand>;
}

export interface IDevKitCommand {
  name: string;
  owner?: string;
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

export type AsyncTask<T> = () => Promise<T>;
