use biblioteca_backend::catalog::model::Book;
use hyper::{header, Body, Method, Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{
    app::create_mock_app,
    catalog::MockCatalog,
    db::{MockDatabaseBuilder, MockDatabaseQuerier},
};

#[tokio::test]
async fn create_book_correct_parameters_successful() {
    let database_path = "create_book_correct_parameters_successful.sqlite";

    let author = MockCatalog::new_author().build();
    let original_book = MockCatalog::new_book().build();

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
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": original_book.name,
                        "description": original_book.description,
                        "language": original_book.language,
                        "author_id": author.id,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "checking if response is OK"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let created_book: Book = serde_json::from_slice(&body).unwrap();
    let expected_book = Book {
        id: created_book.id,
        name: original_book.name,
        description: original_book.description,
        language: original_book.language,
    };

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_book(&expected_book),
            true,
            "checking if book was added properly"
        );
        assert_eq!(
            querier.contains_book_author_mapping(&expected_book.id, &author.id),
            true,
            "checking if book to author mapping exists"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_book_wrong_parameters_failure() {
    let database_path = "create_book_wrong_parameters_failure.sqlite";

    let author = MockCatalog::new_author().build();
    let original_book = MockCatalog::new_book().build();

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
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name_incorrect": original_book.name,
                        "description": original_book.description,
                        "language": original_book.language,
                        "author_id": author.id,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "checking if response is correct (unprocessable)"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_book_missing_parameters_failure() {
    let database_path = "create_book_missing_parameters_failure.sqlite";

    let author = MockCatalog::new_author().build();
    let original_book = MockCatalog::new_book().build();

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
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": original_book.name,
                        "language": original_book.language,
                        "author_id": author.id,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "checking if response is correct (unprocessable)"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_book_additional_parameters_successful() {
    let database_path = "create_book_additional_parameters_successful.sqlite";

    let author = MockCatalog::new_author().build();
    let original_book = MockCatalog::new_book().build();

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
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": original_book.name,
                        "description": original_book.description,
                        "language": original_book.language,
                        "author_id": author.id,
                        "additional_parameter": "hello world".to_string()
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "checking if response is OK"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let created_book: Book = serde_json::from_slice(&body).unwrap();
    let expected_book = Book {
        id: created_book.id,
        name: original_book.name,
        description: original_book.description,
        language: original_book.language,
    };

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_book(&expected_book),
            true,
            "checking if book was added properly"
        );
        assert_eq!(
            querier.contains_book_author_mapping(&expected_book.id, &author.id),
            true,
            "checking if book to author mapping exists"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_book_invalid_author_failure() {
    let database_path = "create_book_invalid_author_failure.sqlite";
    let correct_uuid = Uuid::parse_str("1120489e-19a8-498a-a99d-63fc6b32769f").unwrap();
    let incorrect_uuid = Uuid::parse_str("1120489e-19a8-498a-a99d-63fc6b32769e").unwrap();

    let author = MockCatalog::new_author().id(correct_uuid).build();

    let original_book = MockCatalog::new_book().build();

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
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": original_book.name,
                        "description": original_book.description,
                        "language": original_book.language,
                        "author_id": incorrect_uuid,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "checking if response is correct (400)"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}
