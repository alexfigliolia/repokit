import { DevKitConfig } from "@devkit/core";

/**
 * Please fill out this config file with your desired
 * devkit settings
 */
export const DevKit = new DevKitConfig({
  project: "Your Project Name",
  workspaces: ["./workspace-1/*", "./workspace-2/*"],
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
