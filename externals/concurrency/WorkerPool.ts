import { availableParallelism } from "node:os";

import { Thread } from "./Thread";

export class WorkerPool<
  A extends Record<string, any>,
  R extends Record<string, any>,
> {
  private lastUsedIndex = -1;
  public readonly maxThreads: number;
  public readonly maxConcurrency: number;
  public readonly workerScript: string | URL;
  private readonly POOL: (Thread<A, R> | undefined)[];
  public static readonly CORES = availableParallelism();
  constructor({
    workerScript,
    maxUtilization = 0.5,
    maxConcurrency = Infinity,
  }: IWorkerPool) {
    this.validateUtilization(maxUtilization);
    this.workerScript = workerScript;
    this.maxConcurrency = maxConcurrency;
    this.maxThreads = Math.trunc(WorkerPool.CORES * maxUtilization);
    this.POOL = Array.from({ length: this.maxThreads }, () => undefined);
  }

  public async enqueue(args: A) {
    this.lastUsedIndex = this.getIdolThread();
    if (this.currentLoad > this.maxConcurrency) {
      await this.waitOnMaxConcurrency();
    }
    const position = this.lastUsedIndex;
    const worker =
      this.POOL[position] ??
      new Thread(this.workerScript, () => (this.POOL[position] = undefined));
    const result = worker.enqueue(args);
    if (!this.POOL[position]) {
      this.POOL[position] = worker;
    }
    return result;
  }

  public shutDown() {
    return Promise.all(
      this.POOL.map(thread => Promise.resolve(thread?.kill?.())),
    );
  }

  public shutDownBackground() {
    return Promise.all(
      this.POOL.map(thread => Promise.resolve(thread?.killBackground?.())),
    );
  }

  private getIdolThread() {
    let minIndex = this.maxThreads - 1;
    let minLoad = Infinity;
    let pointer = -1;
    for (const thread of this.POOL) {
      ++pointer;
      const threadLoad = thread?.outstandingTasks?.size ?? 0;
      if (threadLoad === 0) {
        return pointer;
      }
      if (threadLoad < minLoad) {
        minLoad = threadLoad;
        minIndex = pointer;
      }
    }
    return minIndex;
  }

  private get currentLoad() {
    let load = 0;
    for (const thread of this.POOL) {
      load += thread?.outstandingTasks?.size ?? 0;
    }
    return load;
  }

  private async waitOnMaxConcurrency() {
    while (true) {
      if (this.currentLoad < this.maxConcurrency) {
        break;
      }
      await Promise.race(
        this.POOL.flatMap(thread =>
          Array.from(thread?.outstandingTasks.values?.() ?? { length: 0 }),
        ),
      );
    }
  }

  private validateUtilization(utilization: number) {
    if (utilization > 1 || utilization < 0) {
      throw new Error(
        `Utilitization Error: Specifying a utilization requires a decimal value between 0 and 1`,
      );
    }
  }
}

interface IWorkerPool {
  workerScript: string | URL;
  maxConcurrency?: number;
  maxUtilization?: number;
}
