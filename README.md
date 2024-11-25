# Rusty Web Server

A simple web server built with Actix-web framework in Rust.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

You can install Rust and Cargo from [https://rustup.rs/](https://rustup.rs/)


## Dependencies

- actix-web: Web framework for Rust
- actix-rt: Actix runtime

These dependencies are managed in the `Cargo.toml` file.


## Running the Server

1. Clone the repository: 
git clone <repository-url>
cd ws

2. Run the server using cargo:
cargo run -p webservice --bin server1


The server will start on `http://localhost:3000`

## Testing the Server

You can test the server using curl or your web browser:

curl http://localhost:3000/health