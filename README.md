# Rocket, Diesel, MySQL, Rest API Tutorial

Build a RESTful CRUD API for managing heroes with Rocket and Diesel.

## Requirements

1. Rust and Cargo — current stable ([rustup](https://rustup.rs/))
2. Diesel CLI — current, with MySQL support (`cargo install diesel_cli --no-default-features --features mysql`)
3. MySQL — 5.7+ or 8.x, including the MySQL client library and headers (`libmysqlclient`) so Diesel can compile

## Steps to Setup

**1. Clone the application**

```bash
git clone https://github.com/sean3z/rocket-diesel-rest-api-example.git
cd rocket-diesel-rest-api-example
```

**2. Create DATABASE_URL environment variable**

Create the `heroes` database, then point Diesel and the app at it:

```bash
export DATABASE_URL=mysql://user:pass@localhost/heroes
```

**3. Run Database migration**

```bash
diesel migration run
```

**4. Build and run the app using cargo**

```bash
cargo run
```

The app listens at <http://127.0.0.1:8000>. `DATABASE_URL` must be set in the environment when the process starts. Change the bind address or port with `ROCKET_ADDRESS` and `ROCKET_PORT`.

A release binary is built the same way:

```bash
cargo build --release
./target/release/hero-api
```

## Explore Rest APIs

The app defines the following CRUD APIs.

    GET /heroes

    POST /hero

    PUT /hero/{heroId}

    DELETE /hero/{heroId}

You can test them using postman or any other rest client.

## Learn more

You can find the tutorial for this application on my blog —

<https://medium.com/p/1867308352d8/>
