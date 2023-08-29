use axum::{http::{Request, Method, header, StatusCode}, body::Body};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{db::MockDatabaseBuilder, catalog::MockCatalog, app::{create_mock_state, create_mock_app}};

mod mocker;

#[tokio::test]
async fn add_book_successful() {
    let author = MockCatalog::new_author();
    let book = MockCatalog::new_book();

    let mut db = MockDatabaseBuilder::create("mock_library.db".to_string())
        .with_author(&author)
        .build();
    let querier = MockDatabaseQuerier::of(&mut db);

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

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(querier.contains_book(&book), true);
    assert_eq!(querier.contains_book_author_mapping(&book.id, &author.id), true);

    MockDatabaseBuilder::teardown("mock_library.db".to_string());
}