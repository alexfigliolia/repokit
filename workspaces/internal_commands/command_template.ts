import { DevKitCommand } from "@devkit/DevKitCommand";

export const Commands = new DevKitCommand({
  name: "<Your Package Name>",
  description: "<Your Package Description>",
  commands: {
    "<your-first-command>": {
      "command": "<insert shell command here>",
      "description": "A description for using your command",
    },
    "<your-second-command>": {
      "command": "<insert shell command here>",
      "description": "A description for using your command",
    },
    "<your-third-command>": {
      "command": "<insert shell command here>",
      "description": "A description for using your command",
    }
  }
})