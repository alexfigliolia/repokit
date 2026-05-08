import { pipeline } from "node:stream/promises";
import { cwd } from "node:process";
import { join } from "node:path";
import { readFile, rm } from "node:fs/promises";
import { createReadStream, createWriteStream } from "node:fs";

import { Extract } from "unzipper";
import { ChildProcess } from "@figliolia/child-process";

import { Linker } from "./Linker";

export class Downloader extends Linker {
  public static async installBinary(name: string) {
    const version = await this.resolveVersion();
    const releaseUrl = `https://api.github.com/repos/alexfigliolia/repokit/releases/tags/${version}`;
    const response = await fetch(releaseUrl, {
      headers: {
        // Authorization: `Bearer ${this.config.token}`,
        Accept: "application/vnd.github+json",
      },
    });
    const release = (await response.json()) as ReleaseResponse;
    console.log(release.assets);
    const asset = release.assets.find(a => a.name === name);
    if (!asset) {
      throw new Error("Binary not found");
    }
    return this.downloadBinary(asset.id);
  }

  private static async downloadBinary(binaryID: number) {
    const downloadUrl = `https://api.github.com/repos/alexfigliolia/repokit/releases/assets/${binaryID}`;
    const assetResponse = await fetch(downloadUrl, {
      headers: {
        // Authorization: `Bearer ${this.config.token}`,
        Accept: "application/octet-stream",
      },
    });
    if (!assetResponse.ok || assetResponse.body === null) {
      throw new Error(`Download failed: ${assetResponse.statusText}`);
    }
    const tmpZip = join(cwd(), ".repokit.zip");
    const archive = join(cwd(), ".repokit_archive");
    await pipeline(assetResponse.body, createWriteStream(tmpZip));
    await pipeline(createReadStream(tmpZip), Extract({ path: archive }));
    await new ChildProcess(`ls ${archive}`).handler;
    const result = await this.moveAndMakeCallable(archive);
    await rm(tmpZip, { force: true, recursive: true });
    await rm(archive, { force: true, recursive: true });
    return result;
  }

  private static async resolveVersion() {
    const pkgFile = (await readFile(join(cwd(), "package.json"))).toString();
    const version = JSON.parse(pkgFile).version as string;
    return `v${version}`;
  }
}

interface ReleaseResponse {
  assets: { name: string; id: number }[];
}
