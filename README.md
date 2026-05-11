# fso2f - GraphQL Federation Workshop

<p align="center">
  <img src="assets/logo.svg" alt="fso2f logo" width="400"/>
</p>

## Prerequisite

### Cargo-make installed

Install cargo-make:
```bash
cargo install cargo-make
```

### Docker installed

Follow your operating system to [install Docker](https://docs.docker.com/engine/install/)

### Cosmo Wundergraph Account

Create an account on [cosmo wundergraph](https://cosmo.wundergraph.com/)

## Workshop Runner

The workshop is structured as a series of exercises. A small CLI tool (`fso2f`) manages your progress:

- `cargo make task` — Shows the instructions for the current exercise and checks out the corresponding branch
- `cargo make test` — Runs the tests for the current exercise. If all pass, it asks whether to advance to the next one
- `cargo make solution` — Checks out the branch with the solution for the current exercise

Your current exercise is stored in the `.fso2f` file (git-ignored). The exercise definitions (instructions, test name, solution branch) live in `fso2f.json`.

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

