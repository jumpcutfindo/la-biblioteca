use axum::{http::{Request, Method, header, StatusCode}, body::Body};
use biblioteca_backend::catalog::model::Book;
use serde_json::json;
use tower::ServiceExt;

use crate::mocker::{db::{MockDatabaseBuilder, MockDatabaseQuerier}, catalog::MockCatalog, app::{create_mock_state, create_mock_app}};

mod mocker;

#[tokio::test]
async fn add_book_correct_parameters_successful() {
    let database_path = "add_book_successful.sqlite";

    let author = MockCatalog::new_author();
    let book = MockCatalog::new_book();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .build();

    let state = create_mock_state(db);
    let app = create_mock_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/books")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&json!({ 
                    "name": book.name,
                    "description": book.description,
                    "language": book.language,
                    "author_id": author.id, 
                })).unwrap()))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK, "checking if response is OK");

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let created_book: Book = serde_json::from_slice(&body).unwrap();

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(querier.contains_book(&created_book), true, "checking if book was added properly");
        assert_eq!(querier.contains_author(&author), true, "checking if author was added properly");
        assert_eq!(querier.contains_book_author_mapping(&created_book.id, &author.id), true, "checking if book to author mapping exists");
    }
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn get_all_books_successful() {
    let database_path = "get_all_books_successful.sqlite";
    
    let author = MockCatalog::new_author();
    let book_a = MockCatalog::new_book();
    let book_b = MockCatalog::new_book();
    let book_c = MockCatalog::new_book();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .with_book(&book_c, &author.id)
        .build();

    let state = create_mock_state(db);
    let app = create_mock_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/books")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK, "checking if response is OK");

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let created_books: Vec<Book> = serde_json::from_slice(&body).unwrap();

    println!("{:?}", created_books);

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(querier.contains_num_books(created_books.len() as i32), true, "checking if book count is correct");
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}