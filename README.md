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



## Starting the Teacher Service

1. Navigate to the project directory:
cd ws/webservice

2. Start the service using cargo:
cargo run --bin teacher-service


The service will start on `http://127.0.0.1:3000` by default.

### Available Endpoints

- Health Check: `GET /health`
- Create Course: `POST /courses/`
  ```json
  {
    "teacher_id": 1,
    "name": "Course Name"
  }
  ```


