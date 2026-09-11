import { ThreadPoolWorker } from "@figliolia/thread-pool";

import { CommandParser } from "../CommandParser";

new ThreadPoolWorker(({ root, path }: Args) => {
  return CommandParser.parseCommand(root, path);
});

interface Args {
  root: string;
  path: string;
}
