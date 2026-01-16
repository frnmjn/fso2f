# fso2f - GraphQL Federation Workshop

## Quick Start

### Using cargo-make (recommended)

Install cargo-make:
```bash
cargo install cargo-make
```

Run subgraphs with Docker:
```bash
cargo make docker-up
```

Run subgraphs locally:
```bash
# Start PostgreSQL first
cargo make db-up

# In separate terminals:
cargo make dev-products
cargo make dev-orders
```

### Available Commands

**Development:**
- `cargo make dev-products` - Run products subgraph
- `cargo make dev-orders` - Run orders subgraph
- `cargo make export-schemas` - Export GraphQL schemas

**Building:**
- `cargo make build-all` - Build both subgraphs
- `cargo make build-products` - Build products subgraph
- `cargo make build-orders` - Build orders subgraph

**Docker:**
- `cargo make docker-up` - Start all services
- `cargo make docker-up-detached` - Start in background
- `cargo make docker-down` - Stop all services
- `cargo make docker-restart-products` - Restart products
- `cargo make docker-restart-orders` - Restart orders
- `cargo make docker-logs` - View logs

**Code Quality:**
- `cargo make fmt` - Format code
- `cargo make check` - Check code
- `cargo make clippy` - Run lints
- `cargo make test` - Run tests

**Help:**
- `cargo make help` - Show all available tasks

## Endpoints

- **Products subgraph:** http://localhost:3001/graphql
- **Orders subgraph:** http://localhost:3002/graphql
- **PostgreSQL:** localhost:5432

## Manual Commands (without cargo-make)

```bash
# Run subgraphs
cargo run --bin products-subgraph
cargo run --bin orders-subgraph

# Export schemas
cargo run --bin export-schemas

# Docker
docker compose up
docker compose down
```
