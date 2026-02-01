<img src="media/devkit.webp" alt="Alt text" width="150px" />

# Devkit

A knowledgebase for your repository - wrapped in a CLI.

Devkit is designed for large teams in complex codebases to publish self-documenting commands, API's, and workflows to a central CLI.

The Devkit CLI exists as a living source of documentation and knowledge - growing alongside your team.

## Getting Started

### Installation

If you do not have node.js setup in your repository, you'll first want to install node.js.

[NVM is a populat posix compliant installer](https://github.com/nvm-sh/nvm)

Once installed, you can run the following in the root of your repository

```bash
npm init
```

If you don't have `typescript` already setup in your repository, you can run:

```bash
npm i -D typescript && tsc --init
```

Next, install devkit:

```bash
npm i -D @devkit/core
# or
yarn add -D @devkit/core
# or
pnpm add -D @devkit/core
```

Devkit will automatically create a config file named `devkit.ts` for you upon installing. Fill out this file with your desired settings.

Here's an example of what Devkit's internal config looks like:

```typescript
import { DevKitConfig } from "@devkit/core";

export const DevKit = new DevKitConfig({
  project: "Devkit",
  commands: {
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

To begin building your CLI, run:

```bash
devkit register ./path/to/your/feature
```

This command generates a tool definition for your feature that you can fill out using your tool's API's. When complete, save the file and run:

```bash
devkit <your-tool-name>
```

The CLI will list out your new tool's API's. To invoke any of them, run:

```bash
devkit <your-tool-name> <your-command-name>
```

### Reasoning about your toolchain

As your toolchain grows it's possible to find yourself with hundreds, if not thousands of registered commands.

To make reasoning about your commands easier, there are a few internal commands worth getting to know

#### `devkit search`

`devkit search` is a blanket search over all command definitions. Using it you can search for commands by name, owner, definition, location, or even the tools that it invokes.

For example, let's say you wanted to list all commands that invoke `cargo`, you could run

```bash
devkit search cargo
```

If you wanted to search for all commands owned by an individual or team you could run

```bash
devkit search <person or team name>
```

If you wanted to search for commands under a given path you could run

```bash
devkit search path/within/your/codebase
```

You can query for just about anything you can imagine

#### `devkit locate`

Code changes can sometimes require updating command definitions. Devkit can easily locate any command's definition by name:

```bash
devkit locate <your-tool-name>
```

#### `devkit owners`

If your team makes use of the `owners` attribute when defining your commands, you can easily list all commands owned by an individual or team

```bash
devkit list <owner>
```

`devkit list` can also accept `internal | registered | root` as an argument.

`internal` will cause devkit to list out all of its internal commands

`registered` will cause devkit to list out all of the commands your team has defined around your codebase

`root` will cause devkit to list out all commands in your `devkit.ts` config

### Best Practices for Registering Commands

First and most simply - use verbose descriptions. Document flags, positionals, and environment variables required to invoke your tool.

If your tool requires arguments, abstract common combinations of arguments into their own sub-commands. For example, instead of a single `build` command requiring flags to configure it, create sub commands that abstract commonly used combinations of parameters:

```typescript
import { DevKitCommand } from "@devkit/core";

export const Commands = new DevKitCommand({
  // ... command definition
  commands: {
    "build:local": {
      command: "bazel build --env development",
      description: "Builds in development mode",
    },
    "build:production": {
      command: "bazel build --progress --stats --env production",
      description: "Builds in production mode",
    },
  },
});
```

When possible, prefer flags and positionals over environment variables. Often times your argv parsers will provide some out-of-the-box validations for free that environment variables simply don't get.

#### Working Directories

The commands you register onto the devkit toolchain will always be invoked using the working directory of the command's definition.

If your command needs to reason about the file system, keep this in mind.

## Motivation

I worked in a codebase at Google that used just about every programming language in existence. Each team had their own methodology for exposing commands, scripts, and API's for their team's day-to-day development needs.

Some teams used shell scripts, some used a tool called `bazel`, and some relied on good old `python ./path/to/my-script.py` or something similar.

For engineers new and old to onboard to new features, they were often left stuck combing through these undocumented scripts and tools - tracking down environment variables, positionals, and flags to get necessary commands to succeed.

Most of the time landing them in GChat asking for help.

During my time there, I never met an engineer with a fully functioning local environment.

It was there that I designed an early version **devkit.**
