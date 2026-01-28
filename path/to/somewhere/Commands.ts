import { DevKitCommand } from "@devkit/core";

export const Commands = new DevKitCommand({
  name: "test-package",
  description: "A package designed to do important things",
  commands: {
    install: {
      command: "yarn install",
      description: "Installs package dependencies using yarn",
    },
    test: {
      command: "yarn test",
      description: "Runs all tests",
    },
  },
});
