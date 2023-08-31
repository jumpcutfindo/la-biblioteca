use biblioteca_backend::catalog::model::Book;
use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{app::create_mock_app, catalog::MockCatalog, db::MockDatabaseBuilder};

#[tokio::test]
async fn get_book_book_exists_successful() {
    let database_path = "get_book_book_exists_successful.sqlite";

    let author = MockCatalog::new_author().build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/books/{}", book_a.id.to_string()))
                .body(Body::empty())
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
    let returned_book: Book = serde_json::from_slice(&body).unwrap();

    {
        assert_eq!(returned_book.id == book_a.id, true);
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn get_book_book_no_exists_failure() {
    let database_path = "get_book_book_no_exists_failure.sqlite";

    let author = MockCatalog::new_author().build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/books/{}", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "checking if response is correct"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}