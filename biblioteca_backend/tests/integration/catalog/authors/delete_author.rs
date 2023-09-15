use hyper::{Request, Body, StatusCode};
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{catalog::MockCatalog, db::{MockDatabaseBuilder, MockDatabaseQuerier}, app::create_mock_app};


#[tokio::test]
async fn delete_author_existing_author_successful() {
    let database_path = "delete_author_existing_author_successful.sqlite";

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
                .uri(format!("/authors/{}", author.id))
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
            querier.contains_author(&author),
            false,
            "checking if author was removed properly"
        );
        assert_eq!(
            querier.contains_book_author_mapping(&book_a.id, &author.id),
            false,
            "checking if book_a to author mapping doesn't exist"
        );
        assert_eq!(
            querier.contains_book_author_mapping(&book_b.id, &author.id),
            false,
            "checking if book_b to author mapping doesn't exist"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn delete_author_non_existent_author_successful() {
    let database_path = "delete_author_non_existent_author_successful.sqlite";

    let author_a = MockCatalog::new_author().build();
    let author_b = MockCatalog::new_author().build();

    let incorrect_uuid = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .build();
    
    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/authors/{}", incorrect_uuid))
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
            querier.contains_num_authors(2),
            true,
            "checking if no authors were deleted"
        );
    }
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}