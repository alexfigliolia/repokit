import { AutoIncrementingID } from "@figliolia/event-emitter";
import type { AsyncTask } from "./types";

export class ConcurrencyPool<T> {
  private readonly IDs = new AutoIncrementingID();
  private readonly activeTasks = new Map<string, Promise<T>>();
  constructor(public readonly maxConcurrency = 10) {}

  public async enqueue(task: AsyncTask<T>) {
    if (this.activeTasks.size === this.maxConcurrency) {
      await Promise.race(Array.from(this.activeTasks.values()));
    }
    return this.executeTask(task);
  }

  private executeTask(task: AsyncTask<T>) {
    const ID = this.IDs.get();
    const promise = task();
    this.activeTasks.set(ID, promise);
    void promise.finally(() => this.activeTasks.delete(ID));
    return promise;
  }
}
