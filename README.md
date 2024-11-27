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


  ## PostgreSQL Setup

### Installation (Mac)
brew install postgresql


### Verify Installation
ps aux | grep postgres
postgres --version


### Database Setup

# Create the database
createdb tutorial

# use whoami and update .env variable for DATABASE_URL
DATABASE_URL = postgres://username@localhost:5432/tutorial

# Verify database exists by connecting to it
psql tutorial


# Inside psql, you can:
\l        # List all databases
\dt       # List all tables
\q        # Quit psql



## Database Operations

### Using DataGrip

1. **Connect to Database**
   ```
   Host: localhost
   Port: 5432
   Database: tutorial
   User: [your_username]
   Password: [leave empty if not set]
   ```

2. **Create Tables**
   ```sql
   CREATE TABLE course (
       id SERIAL PRIMARY KEY,
       teacher_id INT NOT NULL,
       name VARCHAR(140) NOT NULL,
       time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
   );
   ```

3. **Insert Test Data**
   ```sql
   INSERT INTO course (teacher_id, name) 
   VALUES 
       (1, 'First Course'),
       (1, 'Second Course'),
       (2, 'Another Course');
   ```

4. **Common Queries**
   ```sql
   -- Get all courses
   SELECT * FROM course;

   -- Get courses for specific teacher
   SELECT * FROM course WHERE teacher_id = 1;

   -- Get course by ID
   SELECT * FROM course WHERE id = 1;
   ```

### DataGrip Tips

1. **Execute Query**: 
   - Use `Ctrl + Enter` (Windows/Linux) or `Cmd + Enter` (Mac) to execute
   - Use `Ctrl + Shift + Enter` to execute the entire script

2. **Navigation**:
   - Database Explorer: View all tables in left panel
   - Double-click table to see data
   - Right-click table for options (Export Data, Diagrams, etc.)

3. **Useful Features**:
   - Table Editor: Double-click table name
   - Query Console: `Ctrl/Cmd + N` for new console
   - Data Export: Right-click results → Export Data

4. **View Table Structure**:
   ```sql
   -- In query console:
   postgres=# \d course
                           Table "public.course"
   Column   |            Type             | Collation | Nullable | Default 
------------+-----------------------------+-----------+----------+---------
 name       | character varying           |           |          | 
 teacher_id | integer                     |           |          | 
 time       | timestamp without time zone |           |          | now()
 id         | integer                     |           |          | 
   
   -- Or right-click table → Diagrams → Show Visualization

postgres=# SELECT * FROM course WHERE teacher_id = 1;
 name | teacher_id |        time         | id 
------+------------+---------------------+----
 c1   |          1 | 2024-11-26 17:00:10 |  1
 c2   |          1 | 2024-08-26 17:00:14 |  2
 c3   |          1 | 2024-11-26 17:00:18 |  3
(3 rows)
   ```

### Query url
http://localhost:3000/courses/1/2
teacher id = 1, course id = 2