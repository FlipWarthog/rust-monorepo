[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Actions CI](https://github.com/primefaces/primefaces/workflows/CI/badge.svg)](https://github.com/FlipWarthog/rust-monorepo/actions/workflows/build.yml)
# Rust Monorepo

<img src="https://github.com/FlipWarthog/rust-monorepo/blob/main/frontend/src/main/webapp/public/static/images/rust.svg" height="50" ><img src="https://github.com/FlipWarthog/rust-monorepo/blob/main/frontend/src/main/webapp/public/static/images/plus-sign.svg" height="50" ><img src="https://github.com/FlipWarthog/rust-monorepo/blob/main/frontend/src/main/webapp/public/static/images/primereact-dark.svg" height="50" >

This [monorepo](https://en.wikipedia.org/wiki/Monorepo) is a minimal CRUD service exposing a couple of endpoints over REST,
with a front-end based on React so you can play with it from your browser.

While the code is surprisingly simple, under the hood this is using:

- Actix Web to expose the REST endpoints
- Diesel ORM to perform the CRUD operations on the database to automatically update database
- PostgreSQL database; see below to run one via Docker
- React + PrimeReact for a top notch user interface including lazy datatable
- TanStack Query for powerful asynchronous state management for TypeScript
- Orval to generate TanStack Query client Typescript from OpenAPI definition
- React Hook Forms to validate user input data

## Requirements

To compile and run this demo you will need:

- Rust/Cargo
- Docker

In addition, you will need either a PostgreSQL database, or Docker to run one.

## Developing

### Building frontend

The UI static assets are configured to be served up from the backend, along with the exposed API that the SPA uses to retrieve data.

```bash
$ npm install
$ npm run build
```
### Running backend

With Rust/Cargo installed, you can build and run the backend with:

```bash
$ cargo r
```

Now open your web browser to http://localhost:8080/ to see it in action.

[![Quarkus Monorepo](https://github.com/melloware/quarkus-monorepo/blob/main/src/test/resources/quarkus-monorepo-screen.png)](https://github.com/melloware/quarkus-monorepo)
