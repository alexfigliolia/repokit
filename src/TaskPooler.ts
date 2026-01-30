import { AutoIncrementingID } from "@figliolia/event-emitter";

import type { AsyncTask } from "./types";

export class TaskPooler<T> {
  private readonly IDs = new AutoIncrementingID();
  private readonly runningTasks = new Map<string, Promise<T>>();
  constructor(public maxSize = 10) {}

  public enqueue(task: AsyncTask<T>) {
    return new Promise<T>(resolve => {
      if (this.runningTasks.size < 10) {
        return resolve(this.indexRunningTask(task));
      }
      resolve(this.indexBehindNextOpening(task));
    });
  }

  private indexRunningTask(task: AsyncTask<T>) {
    const ID = this.IDs.get();
    const promise = task();
    this.runningTasks.set(ID, promise);
    void promise.finally(() => this.runningTasks.delete(ID));
    return promise;
  }

  private async indexBehindNextOpening(task: AsyncTask<T>) {
    await Promise.race(this.runningTasks.values());
    return this.indexRunningTask(task);
  }
}
