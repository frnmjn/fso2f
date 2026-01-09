# Copilot Instructions for fso2f

## Project Overview
This is a **workshop project** designed to demonstrate how **GraphQL Federation** works. It's built as an educational/demonstration tool to showcase federation concepts and patterns.

### Tech Stack
This is a Rust-based GraphQL API subgraph built with:
- **Axum** for web framework
- **async-graphql** for GraphQL implementation (with federation support)
- **SQLx** for PostgreSQL database interactions
- **Tokio** for async runtime
- **tracing** for logging

### Workshop Purpose
The project serves as a hands-on example for understanding:
- GraphQL subgraph implementation
- Federation schema design and composition
- Entity references and relationships across subgraphs
- Query planning and execution in a federated architecture

## Code Style & Conventions

### General Rust Guidelines
- Use Rust 2024 edition features
- Follow idiomatic Rust patterns and naming conventions
- Prefer `Result` and `Option` types over panics
- Use descriptive variable names
- Add `tracing::info!`, `tracing::debug!`, or `tracing::error!` for important operations

### GraphQL Schema Design
- Use `async-graphql` derive macros: `#[Object]`, `#[SimpleObject]`, `#[ComplexObject]`, `#[Interface]`, `#[Enum]`, `#[InputObject]`
- Separate queries, mutations, and subscriptions into logical groups
- Use `MergedObject` to combine multiple query/mutation roots
- For polymorphic types, use `#[derive(Interface)]` with `#[graphql(field(...))]` attributes
- Always provide `input_name` for types that need both output and input variants
- Use custom scalars for domain-specific types (e.g., `OrderId`)

### GraphQL Federation Patterns
- This is a **subgraph** in a federated architecture - design with composition in mind
- Use entities to represent shared types across subgraphs
- Consider which fields are owned by this subgraph vs referenced from others
- Keep subgraph boundaries clear and focused on specific domains (e.g., orders, products)
- Design schemas that can be extended by other subgraphs
- Document entity keys and external references for federation composition

### Database Patterns
- Use SQLx with PostgreSQL
- Connection pool should be passed via `State<Pool<Postgres>>`
- Use prepared statements with `sqlx::query!` or `sqlx::query_as!` macros
- Handle database errors with proper error propagation
- Use migrations for schema changes

### Async Patterns
- All handlers and resolvers should be `async`
- Use `tokio::spawn` for background tasks
- For GraphQL subscriptions, use `tokio_stream::Stream`
- Prefer `tokio` ecosystem tools (e.g., `tokio::net::TcpListener`)

### Type Safety
- Use `uuid::Uuid` for unique identifiers
- Use `chrono::DateTime<Utc>` for timestamps
- Enable serde `derive` feature for serialization
- Wrap domain types in newtypes when appropriate (e.g., `OrderId(Uuid)`)

### Server Configuration
- Default server port: 3000
- GraphQL endpoint: `/graphql` (POST for queries/mutations, GET for GraphiQL)
- WebSocket endpoint: `/ws` (for subscriptions)
- Database connection: `postgres://fso2f:fso2f@localhost/fso2f`

## File Organization
- `src/main.rs`: Server initialization, routing, and startup
- `src/lib.rs`: Public module exports
- `src/schema.rs`: GraphQL schema definitions (queries, mutations, subscriptions, types)

## Dependencies Management
- Keep dependencies updated regularly
- Prefer stable crate versions
- Enable only necessary features to reduce compilation time
- Document why optional features are enabled

## Testing Guidelines
- Write unit tests for business logic
- Use `#[tokio::test]` for async tests
- Mock database connections when appropriate
- Test GraphQL queries with `async-graphql::Schema::execute`

## Error Handling
- Use `sqlx::Error` for database operations
- Implement custom error types for domain logic
- Return meaningful error messages in GraphQL responses
- Use `tracing::error!` for unexpected errors

## Performance Considerations
- Configure appropriate connection pool sizes (currently 5)
- Use database indexes for frequently queried fields
- Batch database operations when possible
- Consider using DataLoader pattern for N+1 query problems

## Common Tasks
When adding new features:
1. **New GraphQL Type**: Add to `schema.rs` with appropriate derive macros
2. **New Query**: Extend an existing query root or create new one and merge
3. **New Mutation**: Add to `MutationRoot` similar to queries
4. **New Subscription**: Use `Stream` return type in `SubscriptionRoot`
5. **Database Schema**: Create migration in `migrations/` directory
6. **New Route**: Add to router in `main.rs`
