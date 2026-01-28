import { DevKitCommand } from "@devkit/core";

export const Commands = new DevKitCommand({
  name: "test",
  description: "<Your Package Description>",
  commands: {
    "install": {
      'command': "yarn install",
      "description": "Install all packages with yarn"
    }
  }
})