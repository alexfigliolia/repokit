import { RepoKitConfig } from "@repokit/core";

/**
 * Please fill out this config file with your desired
 * repokit settings
 */
export const RepoKit = new RepoKitConfig({
  project: "Your Project Name",
  commands: {
    "<your-first-command>": {
      command: "<insert shell command here>",
      description: "A description for using your command",
    },
    "<your-second-command>": {
      command: "<insert shell command here>",
      description: "A description for using your command",
    },
    "<your-third-command>": {
      command: "<insert shell command here>",
      description: "A description for using your command",
    },
  },
});
