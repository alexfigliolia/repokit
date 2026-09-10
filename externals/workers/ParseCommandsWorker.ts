import { parentPort } from "node:worker_threads";

import type { WorkerArgs } from "../concurrency";
import { CommandParser } from "../CommandParser";

parentPort?.on(
  "message",
  (data: WorkerArgs<{ root: string; path: string }>) => {
    console.log(data);
    parentPort?.postMessage({
      __WORKER_POOL_ID__: data.__WORKER_POOL_ID__,
      result: CommandParser.parseCommand(data.root, data.path),
    });
  },
);
