# Soulana Backend

A Rust-based REST API backend service with PostgreSQL database integration.

## Features

- User management (CRUD operations)
- Health check endpoint
- PostgreSQL database integration
- Actix-web framework

## Database Setup

Connect to PostgreSQL and create the database:

```sql
postgres=# CREATE DATABASE soulana_db;
CREATE DATABASE
postgres=# \c soulana_db
You are now connected to database `soulana_db` as user `postgres`.
soulana_db=# CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);
```

## API Endpoints

### Health Check
- GET `/health` - Check API health status

### Users
- GET `/users` - Retrieve all users
- POST `/users` - Create a new user

## Environment Variables

Create a `.env` file with:

```plaintext
DATABASE_URL=postgresql://username:password@localhost/soulana_db
```

## Running the Server

The server will start at `http://127.0.0.1:8080`