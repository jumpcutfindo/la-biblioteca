use biblioteca_backend::catalog::model::Book;
use hyper::{StatusCode, Body, Request, header};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{app::create_mock_app, db::{MockDatabaseBuilder, MockDatabaseQuerier}, catalog::MockCatalog};

#[tokio::test]
async fn update_book_correct_parameters_successful() {
    let database_path = "update_book_correct_parameters_successful.sqlite";

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
                .method("PUT")
                .uri(format!("/books/{}", book_b.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": book_a.name,
                        "description": book_a.description,
                        "language": book_a.language,
                        "author_id": author.id,
                    }))
                    .unwrap()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let expected_book = Book {
        id: book_b.id,
        name: book_a.name,
        description: book_a.description,
        language: book_a.language,
    };
    
    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "checking if response is OK"
    );

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        
        assert_eq!(
            querier.contains_book(&expected_book),
            true,
            "checking if book was updated correctly"
        )
    }
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn update_book_non_existent_book_failure() {
    let database_path = "update_book_non_existent_book_failure.sqlite";

    let author = MockCatalog::new_author().build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();

    let invalid_book_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/books/{}", invalid_book_id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": book_a.name,
                        "description": book_a.description,
                        "language": book_a.language,
                        "author_id": author.id,
                    }))
                    .unwrap()
                ))
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
            querier.contains_book(&book_a) && querier.contains_book(&book_b),
            true,
            "checking if book table was not affected"
        );
    }
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn update_book_incorrect_parameters_failure() {
    let database_path = "update_book_incorrect_parameters_failure.sqlite";

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
                .method("PUT")
                .uri(format!("/books/{}", book_b.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name_incorrect": book_a.name,
                        "description": book_a.description,
                        "language": book_a.language,
                        "author_id": author.id,
                    }))
                    .unwrap()
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "checking if response is OK"
    );
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn update_book_missing_parameters_failure() {
    let database_path = "update_book_missing_parameters_failure.sqlite";

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
                .method("PUT")
                .uri(format!("/books/{}", book_b.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "description": book_a.description,
                        "language": book_a.language,
                        "author_id": author.id,
                    }))
                    .unwrap()
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "checking if response is OK"
    );
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn update_book_existing_author_successful() {
    let database_path = "update_book_existing_author_successful.sqlite";

    let author_a = MockCatalog::new_author().build();
    let author_b = MockCatalog::new_author().build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .with_book(&book_a, &author_a.id)
        .with_book(&book_b, &author_b.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/books/{}", book_b.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": book_b.name,
                        "description": book_b.description,
                        "language": book_b.language,
                        "author_id": author_a.id,
                    }))
                    .unwrap()
                ))
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
            querier.contains_book(&book_b),
            true,
            "checking if book was not affected"
        );

        assert_eq!(
            querier.contains_book_author_mapping(&book_b.id, &author_a.id),
            true,
            "checking if book author mapping was updated correctly"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn update_book_non_existent_author_failure() {
    let database_path = "update_book_non_existent_author_failure.sqlite";

    let author = MockCatalog::new_author().build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();

    let invalid_author_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/books/{}", book_b.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": book_b.name,
                        "description": book_b.description,
                        "language": book_b.language,
                        "author_id": invalid_author_id,
                    }))
                    .unwrap()
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "checking if response is OK"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());

}