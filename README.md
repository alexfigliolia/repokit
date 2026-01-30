<img src="media/devkit.webp" alt="Alt text" width="150px" />

# Devkit

A knowledgebase for your repository - wrapped in a CLI.

Devkit is designed for large teams in complex codebases to publish self-documenting commands, API's, and workflows to a central CLI.

The Devkit CLI exists as a living source of documentation and knowledge - growing alongside your team.

## Getting Started

### Installation

If you don't have `typescript` setup already in your repository, you can run:

```bash
npm i -D typescript && tsc --init
```

Next, install devkit:

```bash
npm i -D @devkit/core
```

Devkit will automatically create a config file named `devkit.ts` for you upon installing. Fill out this file with your desired settings.

Here's an example of what Devkit's internal config looks like:

```typescript
import { DevKitConfig } from "@devkit/core";

export const DevKit = new DevKitConfig({
  project: "Devkit",
  commands: {
    "lint:rust": {
      command: "cargo clippy",
      description: "Lints rust files",
    },
    "format:rust": {
      command: "cargo clippy --fix",
      description: "Formats rust files",
    },
    "build:rust": {
      command: "cargo build --release",
      description: "Build CLI in production mode",
    },
    "install:rust": {
      command: "cargo install --path .",
      description: "Installs the production CLI and adds it to your path",
    },
    "lint:ts": {
      command:
        "yarn oxlint --type-aware --type-check --report-unused-disable-directives --fix && yarn oxfmt",
      description: "Lints typescript files using oxc",
    },
    "build:ts": {
      command: "yarn ts-packager -e src",
      description: "Builds the typescript package",
    },
  },
});
```

To verify your configuration, run

```bash
devkit
```

The CLI will list out its internal commands as well as any commands you registered in your config file.

Next run:

```bash
devkit onboard
```

### Building Your CLI

To begin building your CLI, you can run:

```bash
devkit register-command ./path/to/your/feature
```

This command generates a tool definition for your feature that you can fill out using your tool's API's. When complete, save the file and run:

```bash
devkit <your-tool-name>
```

The CLI will list out your new tool's API's. To invoke any of them, run:

```bash
devkit <your-tool-name> <your-command-name>
```

### Best Practices

Use verbose descriptions. Document flags, positionals, and environment variables required to invoke your tool.

When possible abstract common combinations of arguments into their own command definitions. For example, instead of a single `build` command requiring flags to configure it, create abstractions for each target environment:

```typescript
import { DevKitCommand } from "@devkit/core";

export const Commands = new DevKitCommand({
  name: "example-package",
  owner: "Name of Team or Individual"
  description: "A package designed to do important things",
  commands: {
    "build:local": {
      command: "bazel build --env development",
      description: "Builds example-package in development mode",
    },
    "build:production": {
      command: "bazel build --progress --stats --env production",
      description: "Builds example-package in production mode",
    },
  },
});
```

### Registered Commands

The commands you register onto the devkit toolchain will always be invoked using the working directory of the command's definition.

If your command needs to reason about the file system, keep this in mind.

## Motivation

I worked in a codebase at Google that used just about every programming language in existence. Each team had their own methodology for exposing commands, scripts, and API's for their team's day-to-day development needs.

Some teams used shell scripts, some used a tool called `bazel`, and some relied on good old `python ./path/to/my-script.py` or something similar.

For engineers new and old to onboard to new features, they were often left stuck combing through these undocumented scripts and tools - tracking down environment variables, positionals, and flags to get necessary commands to succeed.

Most of the time landing them in GChat asking for help.

During my time there, I never met an engineer with a fully functioning local environment.

It was there that I designed an early version **devkit.**
