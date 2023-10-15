# Overview

La Biblioteca (also known as "the library" in Spanish) is a simple fullstack application that will provide a "proof-of-concept" for a system expected to handle most capabilities of your typical library.

It will contain a React frontend, Rust backend and use MongoDB for it's data storage.

## Considerations

For the sake of simplicity, we will omit important features that are typically important for the functioning of such a system:

- User and request authentication

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

## User management

| API                   | Functionality                          |
| --------------------- | -------------------------------------- |
| `GET /users`          | Retrieves all users in the system      |
| `GET /user/:id`       | Retrieves specific user in the system  |
| `POST /users`         | Adds a user to the system              |
| `DELETE /users`       | Removes a user from the system         |
| `GET /users/roles`    | Retrieves all user roles in the system |
| `GET /user/roles/:id` | Retrieves a specific user role         |
| `POST /users/roles`   | Adds a user role to the system         |
| `DELETE /users/roles` | Deletes a user role from the system    |

## Library management

| API                             | Functionality                                               |
| ------------------------------- | ----------------------------------------------------------- |
| `GET /library/book/:id`         | Retrieves the borrow and return history of a specified book |
| `GET /library/user/:id`         | Retrieves the borrow and return history of a specified user |
| `POST /library/book/:id/borrow` | Borrows a specified book from the catalog                   |
| `POST /library/book/:id/return` | Returns a specified book from the catalog                   |
