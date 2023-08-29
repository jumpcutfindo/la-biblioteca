use axum::{http::{Request, Method, header, StatusCode}, body::Body};
use biblioteca_backend::catalog::model::Book;
use serde_json::json;
use tower::ServiceExt;

use crate::mocker::{db::{MockDatabaseBuilder, MockDatabaseQuerier}, catalog::MockCatalog, app::{create_mock_state, create_mock_app}};

mod mocker;

#[tokio::test]
async fn add_book_successful() {
    let author = MockCatalog::new_author();
    let book = MockCatalog::new_book();

    let db = MockDatabaseBuilder::create("mock_library.sqlite".to_string())
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
        let query_pool = MockDatabaseQuerier::new_pool("mock_library.sqlite".to_string());
        assert_eq!(MockDatabaseQuerier::contains_book(&query_pool, &created_book), true, "checking if book was added properly");
        assert_eq!(MockDatabaseQuerier::contains_author(&query_pool, &author), true, "checking if author was added properly");
        assert_eq!(MockDatabaseQuerier::contains_book_author_mapping(&query_pool, &created_book.id, &author.id), true, "checking if book to author mapping exists");
    }
    
    MockDatabaseBuilder::teardown("mock_library.sqlite".to_string());
}

// async fn get_all_books_successful() {
//     let author = MockCatalog::new_author();
//     let book_a = MockCatalog::new_book();
//     let book_b = MockCatalog::new_book();
//     let book_c = MockCatalog::new_book();

//     let db = MockDatabaseBuilder::create("mock_library.db".to_string())
//         .with_author(&author)
//         .with_book(&book_a, &author.id)
//         .with_book(&book_b, &author.id)
//         .with_book(&book_c, &author.id)
//         .build();

//     let state = create_mock_state(db);
//     let app = create_mock_app(state);

//     let response = app
//         .oneshot(
//             Request::builder()
//                     .uri("/books")
//                     .body(())
//                     .unwrap()
//         )
//         .await
//         .unwrap();

//     MockDatabaseBuilder::teardown("mock_library.db".to_string());
// }