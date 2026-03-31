<img src="media/repokit.webp" alt="Alt text" width="150px" />

# repokit

A knowledgebase for your repository - wrapped in a CLI.

Repokit is designed for large teams in complex codebases to publish self-documenting commands, API's, and workflows to a central CLI.

The Repokit CLI exists as a living source of documentation and knowledge - growing alongside your team.

## Getting Started

### Installation

If you do not have node.js setup in your repository, you'll first want to install node.js.

[NVM is a popular posix compliant installer](https://github.com/nvm-sh/nvm)

Once installed, you can run the following in the root of your repository

```bash
npm init
```

If you don't have `typescript` already setup in your repository, you can run:

```bash
npm i -D typescript && tsc --init
```

Next, install repokit:

```bash
npm i -D @repokit/core
# or
yarn add -D @repokit/core
# or
pnpm add -D @repokit/core
```

Once installed, run

```bash
repokit
```

Repokit will create a config file named `repokit.ts` on your first run. Fill out this file with your desired settings.

Here's an example of what our intenral Repokit config looks like:

```typescript
import { RepoKitConfig } from "@repokit/core";

export const RepoKit = new RepoKitConfig({
  project: "Repokit",
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
  },
});
```

Commands in your config file are optional, but in your `repokit.ts` config is a good place to store any commonly run commands that aren't specific to any library or package in your repository.

To verify your configuration, run

```bash
repokit
```

The CLI will list out its internal commands as well as any commands you registered in your config file.

<img src="media/post-init.webp" alt="Help screen after initialization" width="100%" />

Next run:

```bash
repokit onboard
```

### Building Your CLI

To begin building your CLI, run:

```bash
repokit register ./path/to/your/feature
```

This command generates a tool definition for your feature that you can fill out using your tool's API's. A tool definition might look something like the following:

```typescript
import { RepoKitCommand } from "@repokit/core";

export const Commands = new RepoKitCommand({
  name: "user-interface",
  description: "Build commands for the UI",
  commands: {
    "build:production": {
      command: "vite build",
      description: "Build the UI for production",
      args: {
        "(--optimization | -o)":
          "Run post-build optimizers such as compression and css purging",
      },
    },
    "run:development": {
      command: "vite",
      description: "Run the UI in development mode",
      args: {
        "(--port | -p)":
          "Specifies the port number to run the development server on",
        "(--open | -o)": "Opens your OS' preferred browser",
      },
    },
  },
});
```

When finished with your definition, save the file and run:

```bash
repokit user-interface
```

The CLI will list out your new tool's API's:
<img src="media/new-command.webp" alt="Command definition" width="100%" />

To invoke any of them, run:

```bash
repokit user-interface <sub-command> <args>
```

### Reasoning about your toolchain

As your toolchain grows it's possible to find yourself with hundreds, if not thousands of registered commands.

To make reasoning about your commands easier, there are a few internal commands worth getting to know

#### `repokit search`

`repokit search` is a blanket search over all command definitions. Using it you can search for commands by name, owner, definition, location, or even the tools that it invokes.

For example, let's say you wanted to list all commands that invoke `cargo`, you could run

```bash
repokit search cargo
```

If you wanted to search for all commands owned by an individual or team you could run

```bash
repokit search <person or team name>
```

If you wanted to search for commands under a given path you could run

```bash
repokit search path/within/your/codebase
```

You can query for just about anything you can imagine

#### `repokit locate`

Code changes can sometimes require updating command definitions. Repokit can easily locate any command's definition by name:

```bash
repokit locate <your-tool-name>
```

#### `repokit owners`

If your team makes use of the `owners` attribute when defining your commands, you can easily list all commands owned by an individual or team

```bash
repokit list <owner>
```

`repokit list` can also accept `internal | registered | root` as an argument.

`internal` will cause repokit to list out all of its internal commands

`registered` will cause repokit to list out all of the commands your team has defined around your codebase

`root` will cause repokit to list out all commands in your `repokit.ts` config

### Best Practices for Registering Commands

First and most simply - use verbose descriptions. Document flags, positionals, and environment variables required to invoke your tool.

If your tool requires arguments, abstract common combinations of arguments into their own sub-commands. For example, instead of a single `build` command requiring flags to configure it, create sub commands that abstract commonly used combinations of parameters:

```typescript
import { RepoKitCommand } from "@repokit/core";

export const Commands = new RepoKitCommand({
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

The commands you register onto the repokit toolchain will always be invoked using the working directory of the command's definition.

If your command needs to reason about the file system, keep this in mind.

### Themes

The Repokit CLI can be customized to support themes that better match your team's visual preferences.

Repokit comes with 4 pre-built themes that you view by running

```bash
repokit themes
```

To Set your desired theme, you can run

```bash
repokit themes --set <theme-name>
```

RepoKit also allows you to design your own themes using the `themes` property on your `RepoKitConfig`:

```typescript
import { RepoKitConfig, RepoKitTheme } from "@repokit/core";

export const Kit = new RepoKitConfig({
  project: "My Project",
  themes: [
    new RepoKitTheme({
      name: "my-awesome-theme",
      colors: {
        prefixColor: "rgb(220, 36, 91)",
        commandColor: "rgb(220, 36, 36)",
        subcommandColor: "rgb(220, 131, 36)",
        argColor: "rgb(220, 205, 36)",
        descriptionColor: "rgb(179, 100, 151)",
        errorPrefixColor: "rgb(220, 36, 39)",
        highlightColor: "rgb(237, 175, 41)",
      },
    }),
  ],
});
```

All properties on the `RepoKitTheme.colors` are optional overrides for `RepoKit`'s default styling. A color can be any CSS-valid `rgb()` string.

<img src="media/seeing-red.webp" width="100%" alt="seeing red theme" />

Once a theme is added to your `RepoKitConfig`, it becomes available by running

```bash
repokit themes
```

<img src="media/list-themes.webp" width="100%" alt="listing-themes" />

## Motivation

I worked in a codebase at Google that used just about every programming language in existence. Each team had their own methodology for exposing commands, scripts, and API's for their team's day-to-day development needs.

Some teams used shell scripts, some used a tool called `bazel`, and some relied on good old `python ./path/to/my-script.py` (or something similar).

For engineers new and old to onboard to new features, they were often left stuck combing through these undocumented scripts and tools - tracking down environment variables, positionals, and flags to get necessary commands to succeed.

Most of the time landing them in GChat asking for help.

During my time there, I never met an engineer with a fully functioning local environment.

It was there that I designed an early version **repokit.**
