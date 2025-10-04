# Oxidizer

A hybrid web application that combines the power of **Rust** for backend processing with **Nitro** (TypeScript) for modern web server capabilities.

## Overview

Oxidizer is a monorepo project that demonstrates seamless integration between Rust and Node.js/TypeScript. The project uses Nitro as a web server that executes Rust binaries and serves the results as web content.

## Architecture

- **Nitro Server** (`/nitro`): TypeScript-based web server using Nitro framework
- **Rust Backend** (`/rust`): Rust application for core processing logic
- **Workspace Management**: PNPM workspace for efficient dependency management

## Project Structure

```text
oxidizer/
├── nitro/                    # Nitro web server
│   ├── server/
│   │   └── routes/
│   │       └── index.ts      # Main API route that calls Rust
│   └── package.json
├── rust/                     # Rust application
│   ├── src/
│   │   └── main.rs          # Rust main application
│   └── Cargo.toml
├── package.json             # Root workspace configuration
├── pnpm-workspace.yaml      # PNPM workspace setup
└── README.md
```

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [PNPM](https://pnpm.io/) (v8 or higher)
- [Rust](https://rustup.rs/) (latest stable)

## Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd oxidizer
   ```

2. Install dependencies:

   ```bash
   pnpm install
   ```

3. Build the Rust application:

   ```bash
   pnpm rust:build
   ```

## Development

### Starting the Development Server

To start the Nitro development server:

```bash
pnpm nitro:dev
```

The server will be available at `http://localhost:3000`

### Available Scripts

#### Nitro (Web Server)

- `pnpm nitro:dev` - Start development server
- `pnpm nitro:build` - Build for production

#### Rust (Backend)

- `pnpm rust:build` - Compile Rust application
- `pnpm rust:run` - Run Rust application directly
- `pnpm rust:check` - Check Rust code without building
- `pnpm rust:test` - Run Rust tests

## How It Works

1. The Nitro server (`nitro/server/routes/index.ts`) handles HTTP requests
2. When a request comes in, it executes the compiled Rust binary (`rust/target/debug/rust.exe`)
3. The Rust application generates HTML content and outputs it to stdout
4. The Nitro server captures this output and serves it as the HTTP response

## Production Build

To build the entire project for production:

```bash
# Build Rust application
pnpm rust:build

# Build Nitro server
pnpm nitro:build
```

The built Nitro server will be available in `nitro/.output/server/`

To run the production server:

```bash
cd nitro
node .output/server/index.mjs
```

## Technologies Used

- **[Nitro](https://nitro.build/)** - Universal web server framework
- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[TypeScript](https://www.typescriptlang.org/)** - Type-safe JavaScript
- **[PNPM](https://pnpm.io/)** - Fast, disk space efficient package manager
- **[H3](https://github.com/unjs/h3)** - HTTP framework for Nitro

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the ISC License.

## Author

[@xcvzmoon](https://github.com/xcvzmoon)
