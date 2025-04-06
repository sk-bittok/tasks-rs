# Tasks Authenticated

A robust backend service for task management built with Rust and the Axum web framework.

## Overview

Tasks Authenticated is a RESTful API service that allows users to manage tasks with authentication and authorization using JWT. The service is built with performance and security in mind, leveraging Rust's safety guarantees and the Axum framework's modern async approach.
Screenshots of the application running on Scalar, Swagger-UI, Redoc and Rapidoc OpenAPI can be found in assets folder.

## Features

- ✅ RESTful API for task management
- ✅ JWT-based authentication and authorization
- ✅ API documentation with Swagger UI, ReDoc, and RapiDoc
- ✅ Comprehensive error handling
- ✅ Logging and request tracing
- ✅ Environment-specific configuration
- ✅ PostgreSQL database integration (planned)

## Tech Stack

- **Rust** - For safety, performance, and reliability
- **Axum** - Ergonomic and modular web framework
- **Tokio** - Asynchronous runtime
- **Tower-HTTP** - Middleware for HTTP services
- **Tracing** - Application-level tracing
- **Utoipa** - OpenAPI documentation generator
- **PostgreSQL** - Database (planned)
- **JWT** - Authentication and authorization (planned)

## Project Structure

```
.
├── config
│   ├── development.yaml    # Development environment configuration
│   └── production.yaml     # Production environment configuration
├── src
│   ├── bin
│   │   └── main.rs         # Application entry point
│   ├── config              # Configuration management
│   │   ├── logger.rs
│   │   └── mod.rs
│   ├── errors              # Error handling
│   │   ├── mod.rs
│   │   └── response.rs
│   ├── middlewares         # HTTP middlewares
│   │   ├── mod.rs
│   │   └── trace.rs
│   ├── app.rs              # Application setup
│   ├── lib.rs              # Library entry point
│   └── router.rs           # API routes definition
├── tests                   # Test suite
│   ├── config
│   │   └── mod.rs
│   └── mod.rs
├── Cargo.lock
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL (optional, for development)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/tasks-authenticated.git
   cd tasks-authenticated
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

### Configuration

The application uses configuration files located in the `config` directory:
- `development.yaml` - Used in development environment
- `production.yaml` - Used in production environment

You can specify which configuration to use with the cli options:
```bash
cargo run -- --env <OPTIONS>
```

## API Documentation

API documentation is available through multiple interfaces:
- Swagger UI: `/swagger-ui`
- ReDoc: `/redoc`
- RapiDoc: `/rapidoc`

## Testing

Run the test suite with:
```bash
cargo test
```

## Future Enhancements

- PostgreSQL database integration
- JWT authentication and authorization
- User management
- Task categorization and filtering
- Performance metrics and monitoring

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[MIT](LICENSE) Yet to decide 

## Contact

Simon Bittok - bittokks@gmail.com
