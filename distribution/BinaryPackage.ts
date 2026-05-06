import { cwd } from "node:process";
import { join } from "node:path";
import { mkdir, rename, writeFile } from "node:fs/promises";
import { existsSync } from "node:fs";

export class BinaryPackage {
  private packagePath?: string;
  public readonly packageName: string;
  public static readonly SET = new Set<string>();
  constructor(public readonly config: IBinaryPackage) {
    this.packageName = `${config.OS}-${config.CPU}-${config.FLAVOR}`;
    if (BinaryPackage.SET.has(this.packageName)) {
      throw new Error(`Duplicate package found: ${this.packageName}`);
    }
  }

  public async build(path = cwd()) {
    this.packagePath = join(path, this.packageName);
    await mkdir(this.packagePath, { recursive: true });
    await writeFile(
      join(this.packagePath, "package.json"),
      JSON.stringify(this.createPackageFile(), null, 2),
    );
    await writeFile(join(this.packagePath, "README.md"), this.createReadMe());
  }

  public async shipBinary(artifactsPath: string) {
    if (typeof this.packagePath === "undefined") {
      throw new Error("Package not yet scaffolded", { cause: this });
    }
    const binaryPath = join(artifactsPath, `repokit-${this.config.platform}`);
    if (!existsSync(binaryPath)) {
      throw new Error(
        `Binary path for ${this.config.platform} does not exist`,
        { cause: binaryPath },
      );
    }
    await rename(binaryPath, join(this.packagePath, this.config.binaryName));
  }

  private createPackageFile() {
    // oxlint-disable-next-line typescript-eslint(no-explicit-any)
    const JSON: Record<string, any> = {
      name: `@repokit/${this.packageName}`,
      version: this.config.version,
      cpu: [this.config.CPU],
      main: this.config.binaryName,
      files: [this.config.binaryName],
      description: "A knowledgebase for your repository - wrapped in a CLI",
      keywords: ["cli", "developer tool", "repository", "toolchain"],
      homepage: "https://github.com/alexfigliolia/repokit#readme",
      license: "MIT",
      engines: {
        node: ">= 10",
      },
      repository: {
        type: "git",
        url: "git+https://github.com/alexfigliolia/repokit.git",
      },
      bugs: {
        url: "https://github.com/alexfigliolia/repokit/issues",
      },
      publishConfig: {
        registry: "https://registry.npmjs.org/",
        access: "public",
      },
      os: [this.config.OS],
    };
    if (typeof this.config.LIBC === "string") {
      JSON["libc"] = [this.config.LIBC];
    }
    return JSON;
  }

  private createReadMe() {
    return `# ${`@repokit/${this.packageName}`}\n\nThis is the **${this.config.platform}** binary for ${`@repokit/core`}`;
  }
}

export interface IBinaryPackage {
  CPU: string;
  OS: string;
  LIBC?: string;
  FLAVOR: string;
  version: string;
  platform: string;
  binaryName: string;
}
