<p align="center">
  <img src="assets/logo.svg" alt="fso2f logo" width="800"/>
</p>

## Intro

Welcome! This hands-on workshop teaches you how to build scalable, distributed GraphQL APIs using federation patterns with Rust.

We will walk through the Async-GraphQL [book](https://async-graphql.github.io/async-graphql/en/index.html), starting with simple resolvers and ending with schema evolution using feature flags.

With the help of the workshop runner, you will receive each exercise task, and to move forward you need to make its related test pass.


## Prerequisite

### Rust toolchain

Official guide: [Install Rust](https://www.rust-lang.org/tools/install)

### Cargo Make (task runner)

```bash
cargo install cargo-make
```

On Ubuntu, install the following packages:

```bash
sudo apt install build-essential cmake clang pkg-config
```

### Docker installed

Follow your operating system instructions to [install Docker](https://docs.docker.com/engine/install/)

### jq installed

Follow your operating system instructions to [install jq](https://jqlang.org/)

### WunderGraph CLI (wgc)

Requires Node.js (npm). Install it with:

```bash
cargo make install
```

### Cosmo Wundergraph Account

Create an account on [cosmo wundergraph](https://cosmo.wundergraph.com/). Required by the cloud WunderGraph tasks in `cargo make help` (`federated-graph-create`, `subgraph-publish-*`, `control-plane`).

## How to start the Workshop

Spin up the containers with: 
```bash
docker compose up -d
```

The workshop is structured as a series of exercises, and the following tasks will help you progress. 
The actual exercise is stored into the file .fso2f. Delete it to restart

- `cargo make task` — Shows the instructions for the current exercise (starts postgres if not running)
- `cargo make test` — Runs the test for the current exercise.

if you get stuck you can commit your progress and move on a new branch with all the exercises until the current one is solved.

`cargo make solution` - Checkout the `solution/<exercise>` branch for the current exercise (e.g. `solution/07`)


## Available Commands

```bash
cargo make help
```

## Endpoints

- **Federated graph** http://localhost:5000/graphql
- **Products subgraph:** http://localhost:3001/graphql
- **Orders subgraph:** http://localhost:3002/graphql
- **Customers subgraph** http://localhost:3003/graphql
- **PostgreSQL:** jdbc:postgresql://localhost:5432/fso2f

