# Overview

La Biblioteca (also known as "the library" in Spanish) is a simple fullstack application that will provide a "proof-of-concept" for a system expected to handle most capabilities of your typical library.

It will contain a React frontend, Rust backend and use MongoDB for it's data storage.

# APIs

The following are the expected set of APIs to be used:

## Catalog management

| API                 | Functionality                                  |
| ------------------- | ---------------------------------------------- |
| `GET /books`        | Retrieves all the books present in the catalog |
| `GET /books/:id`    | Retrieves the full details of a specified book |
| `POST /books`       | Adds a book to the catalog                     |
| `PUT /books/:id`    | Updates an existing book in the catalog        |
| `DELETE /books/:id` | Deletes a specified book from the catalog      |
