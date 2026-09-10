import { Worker } from "node:worker_threads";

import { AutoIncrementingID } from "@figliolia/event-emitter";

export class Thread<
  A extends Record<string, any>,
  R extends Record<string, any>,
> {
  private isShuttingDown = false;
  private readonly Worker: Worker;
  private idleKillListener?: Promise<void>;
  private readonly idleCallbacks: (() => void)[] = [];
  private static readonly IDs = new AutoIncrementingID();
  private timer: ReturnType<typeof setTimeout> | null = null;
  private pendingTasks = new Map<string, Promise<WorkerArgs<R>>>();
  constructor(
    workerScript: string | URL,
    public readonly onDestroy: () => void,
  ) {
    this.Worker = new Worker(workerScript);
  }

  public enqueue(args: A) {
    this.clearIdleTimer();
    const workerArgs = this.toWorkerArgs(args);
    const { resolve, reject, promise } = Promise.withResolvers<WorkerArgs<R>>();
    this.pendingTasks.set(workerArgs.__WORKER_POOL_ID__, promise);
    const onMessage = this.onMessage(workerArgs, resolve);
    const onError = this.onError(workerArgs, reject);
    this.Worker.on("message", onMessage);
    this.Worker.on("error", onError);
    const OFF = () => {
      this.pendingTasks.delete(workerArgs.__WORKER_POOL_ID__);
      this.Worker.off("message", onMessage);
      this.Worker.off("error", onError);
      if (this.pendingTasks.size === 0) {
        this.deferKill();
      }
    };
    this.Worker.postMessage(workerArgs);
    return promise.finally(OFF);
  }

  public kill() {
    if (this.isShuttingDown) {
      return;
    }
    this.isShuttingDown = true;
    this.clearIdleTimer();
    return this.Worker.terminate().then(() => this.onDestroy());
  }

  public killBackground() {
    if (this.isIdol) {
      if (this.isShuttingDown) {
        return Promise.resolve();
      }
      return this.kill();
    }
    if (!this.idleKillListener) {
      const { resolve, promise } = Promise.withResolvers<void>();
      this.idleKillListener = promise.then(() => this.kill());
      this.idleCallbacks.push(resolve);
    }
    return this.idleKillListener;
  }

  public get isIdol() {
    return this.pendingTasks.size === 0;
  }

  public get outstandingTasks() {
    return this.pendingTasks;
  }

  private deferKill() {
    this.clearIdleTimer();
    this.timer = setTimeout(() => {
      void this.kill();
    }, 2000);
  }

  private clearIdleTimer() {
    if (this.timer) {
      clearTimeout(this.timer);
      this.timer = null;
    }
  }

  private toWorkerArgs(args: A) {
    return { ...args, __WORKER_POOL_ID__: Thread.IDs.get() };
  }

  private onMessage(
    workerArgs: WorkerArgs<A>,
    resolve: (value: WorkerArgs<R> | PromiseLike<WorkerArgs<R>>) => void,
  ) {
    return (data: WorkerArgs<R>) => {
      if (data.__WORKER_POOL_ID__ === workerArgs.__WORKER_POOL_ID__) {
        resolve(data);
      }
    };
  }

  private onError(workerArgs: WorkerArgs<A>, reject: (reason?: any) => void) {
    return (error: WorkerError) => {
      if (error.__WORKER_POOL_ID__ === workerArgs.__WORKER_POOL_ID__) {
        reject(error);
      }
    };
  }
}

export type WorkerArgs<T extends Record<string, any>> = T & {
  __WORKER_POOL_ID__: string;
};

export type WorkerError = WorkerArgs<{ reason: string }>;
