import { DevKitConfig } from "./src/DevKitConfig";

export const Config = new DevKitConfig({
  project: 'Test',
  workspaces: ['./packages/*', ...["./another-path/*", "./an/additional/path/*"]],
  commands: {
    test: {
      "command": 'npx tsc --noEmit',
      "description": "Typecheck"
    }
  }
});