import { Release } from "./Release";

void (async () => {
  const release = new Release();
  await release.run();
})();
