<p align="center">
  <img src="assets/logo.svg" alt="fso2f logo" width="800"/>
</p>

## Intro

Welcome! This hands-on workshop teaches you how to build scalable, distributed GraphQL APIs in Rust using federation.

Following the [async-graphql book](https://async-graphql.github.io/async-graphql/en/index.html), you start from simple resolvers and end with schema evolution behind feature flags.

The workshop runner gives you one exercise at a time: make its test pass to move on to the next one.

## Prerequisites

### Rust toolchain

[Install Rust](https://www.rust-lang.org/tools/install), then the task runner:

```bash
cargo install cargo-make
```

A C toolchain is needed to compile some dependencies:

- **macOS:** `xcode-select --install`
- **Ubuntu:** `sudo apt install build-essential cmake clang pkg-config`
- **Windows:** the [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) requested by `rustup` and [CMake](https://cmake.org/download/). Every `cargo make` task works from PowerShell or cmd.

### Docker

[Install Docker](https://docs.docker.com/engine/install/) (on macOS and Windows Docker Desktop, or an alternative such as OrbStack, Rancher Desktop or colima). On Windows use Linux containers.

### WunderGraph CLI (wgc)

Requires [Node.js](https://nodejs.org). The tasks that need `wgc` install it when it is missing, or you can install it upfront:

```bash
cargo make wgc-install
```

### Cosmo account (optional)

The cloud tasks of exercise 10 (`federated-graph-create`, `subgraph-publish-*`, `control-plane`) need a [Cosmo](https://cosmo.wundergraph.com/) account.
Without one, run exercise 10 locally with `cargo make local-feature-flag`.

### Check your setup

Before the workshop, run:

```bash
cargo make check-setup
```

It checks the tools above, pulls and builds the Docker images, starts Postgres and runs the exercise 00 test.

## Workshop

- `cargo make start` shows the instructions of the current exercise, unlocks its test and binaries and asks whether to run the test. If it fails, it asks whether to run it again; when it passes, it asks to move on to the next exercise.
- `cargo make solution` replaces the code of the current exercise (e.g. `src/ex07_mutation/`) with its solution from the `main` branch, so you can move on if you get stuck. It only touches that folder: commit your progress first, then `git diff` shows the solution.
- `cargo make help` lists all the tasks.

`start` starts Postgres when it is not running. The current exercise is stored in `.fso2f`: delete it to start over.

## Postman

The [FSO2F](postman/collections/FSO2F/) collection contains the GraphQL requests of the exercise tests, with the same variables.
It uses Postman's file-based format (GraphQL requests can't be shipped as a JSON collection), so it is opened as a local folder, not imported.

To load it in Postman (v12 or later):

1. Open a workspace and click **Local Files → Open Folder** (or **Switch to Local**). Alternatively create a new workspace and choose **From a local folder → Choose a folder**.
2. Select the root of this repository (the folder with `Cargo.toml` and `postman/`), not a subfolder such as `schemas/`. Postman writes its workspace settings in `.postman/`, which is git-ignored.
3. If Postman asks to **Connect to workspace**, confirm.
4. The FSO2F collection appears under `postman/collections/`.

Don't use **Import**: it rejects this format with "Incorrect format".
If the collection is empty, the workspace is connected to the wrong folder: disconnect it (**File viewer options → Disconnect**) and open the repository root.

Every request has its own variable `exNN` = `http://localhost:40NN/graphql`:

- `cargo make servers` serves exercise 00 and every unlocked exercise among 01-08, 11 and 12 (`cargo make init` serves only exercise 00). An exercise that does not compile yet is skipped: solve it and run `cargo make servers` again.
- Exercises 09 and 10 go through the router (`cargo make local`, `cargo make local-feature-flag` or `cargo make control-plane`), published on 4009/4010 as well as on 5000.
