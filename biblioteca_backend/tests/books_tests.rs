use axum::{http::{Request, Method, header, StatusCode}, body::Body};
use biblioteca_backend::catalog::model::Book;
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{db::{MockDatabaseBuilder, MockDatabaseQuerier}, catalog::MockCatalog, app::create_mock_app};

mod mocker;

#[tokio::test]
async fn add_book_correct_parameters_successful() {
    let database_path = "add_book_correct_parameters_successful.sqlite";

    let author = MockCatalog::new_author();
    let book = MockCatalog::new_book();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .build();

    let app = create_mock_app(db);

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
async fn add_book_wrong_parameters_unsuccessful() {
    let database_path = "add_book_wrong_parameters_unsuccessful.sqlite";

    let author = MockCatalog::new_author();
    let book = MockCatalog::new_book();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/books")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&json!({ 
                    "name_incorrect": book.name,
                    "description": book.description,
                    "language": book.language,
                    "author_id": author.id, 
                })).unwrap()))
                .unwrap()
        )
        .await
        .unwrap();

    
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY, "checking if response is correct (unprocessable)");

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn add_book_missing_parameters_unsuccessful() {
    let database_path = "add_book_missing_parameters_unsuccessful.sqlite";

    let author = MockCatalog::new_author();
    let book = MockCatalog::new_book();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/books")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&json!({ 
                    "name": book.name,
                    "language": book.language,
                    "author_id": author.id, 
                })).unwrap()))
                .unwrap()
        )
        .await
        .unwrap();

    
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY, "checking if response is correct (unprocessable)");
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn add_book_additional_parameters_successful() {
    let database_path = "add_book_additional_parameters_successful.sqlite";

    let author = MockCatalog::new_author();
    let book = MockCatalog::new_book();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .build();

    let app = create_mock_app(db);

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
                    "additional_parameter": "hello world".to_string()
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
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn add_book_invalid_author_unsuccessful() {
    let database_path = "add_book_invalid_author_unsuccessful.sqlite";
    let correct_uuid = Uuid::parse_str("1120489e-19a8-498a-a99d-63fc6b32769f").unwrap();
    let incorrect_uuid = Uuid::parse_str("1120489e-19a8-498a-a99d-63fc6b32769e").unwrap();

    let mut author = MockCatalog::new_author();
    author.id = correct_uuid;

    let book = MockCatalog::new_book();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .build();

    let app = create_mock_app(db);

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
                    "author_id": incorrect_uuid,
                })).unwrap()))
                .unwrap()
        )
        .await
        .unwrap();

    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST, "checking if response is correct (400)");

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

    let app = create_mock_app(db);

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
 
    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(querier.contains_num_books(created_books.len() as i32), true, "checking if book count is correct");
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}