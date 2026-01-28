import { DevKitCommand } from "@devkit/core";

/**
 * Please fill out this command file with your desired settings
 */
export const Commands = new DevKitCommand({
  name: "<Your Package Name>",
  description: "<Your Package Description>",
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
