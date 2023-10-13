# La Biblioteca

La Biblioteca, also known as "The Library" in Spanish, is an implementation of what I would expect a library's API to be like in Rust. The application is built on top of the [Axum](https://github.com/tokio-rs/axum) framework and exposes a set of APIs for users or applications to interact with.

This application makes use of Rust's testing framework to carry out integration tests on the APIs, primarily focusing on testing the correctness of the APIs available.

There is also a GitHub workflow in place that checks all commits for linting and formatting issues, as well as running the aforementioned set of integration tests to prevent any regression.

## API Specifications

To see the API specifications, please refer to the [specs.md](https://github.com/jumpcutfindo/la-biblioteca/blob/master/specs.md) document, which briefly describes available and accessible APIs.

## Building the application

To build the application, please clone the repository and install as you would a Rust application.
