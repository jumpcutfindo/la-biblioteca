use hyper::{StatusCode, Body, Request};
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{db::{MockDatabaseBuilder, MockDatabaseQuerier}, catalog::MockCatalog, app::create_mock_app};

#[tokio::test]
async fn delete_book_existing_book_successful() {
    let database_path = "delete_book_existing_book_successful.sqlite";

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
                .method("DELETE")
                .uri(format!("/books/{}", book_a.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "checking if response is OK"
    );

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_book(&book_a),
            false,
            "checking if book was removed properly"
        );
        assert_eq!(
            querier.contains_book_author_mapping(&book_a.id, &author.id),
            false,
            "checking if book to author mapping doesn't exist"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn delete_book_non_existent_book_successful() {
    let database_path = "delete_book_non_existent_book_successful.sqlite";

    let author = MockCatalog::new_author().build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();

    let incorrect_uuid = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .build();
    
    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/books/{}", incorrect_uuid))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "checking if response is OK"
    );

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_num_books(2),
            true,
            "checking if no books were deleted"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());

}