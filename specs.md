# Overview

La Biblioteca (also known as "the library" in Spanish) is a simple fullstack application that will provide a "proof-of-concept" for a system expected to handle most capabilities of your typical library.

It will contain a React frontend, Rust backend and use MongoDB for it's data storage.

# APIs

The following are the expected set of APIs to be used:

## Catalog management

### Book management

| API                 | Functionality                                  |
| ------------------- | ---------------------------------------------- |
| `GET /books`        | Retrieves all the books present in the catalog |
| `GET /books/:id`    | Retrieves the full details of a specified book |
| `POST /books`       | Adds a book to the catalog                     |
| `PUT /books/:id`    | Updates an existing book in the catalog        |
| `DELETE /books/:id` | Deletes a specified book from the catalog      |

### Author management

| API                      | Functionality                                    |
| ------------------------ | ------------------------------------------------ |
| `GET /authors`           | Retrieves all the authors present in the catalog |
| `GET /authors/:id`       | Retrieves the full details of an author          |
| `GET /authors/:id/books` | Retrieves the books written by that author       |
| `POST /authors`          | Adds an author to the catalog                    |
| `PUT /authors/:id`       | Updates the author's information in the catalog  |
| `DELETE /authors/:id`    | Deletes a specified author from the catalog      |
