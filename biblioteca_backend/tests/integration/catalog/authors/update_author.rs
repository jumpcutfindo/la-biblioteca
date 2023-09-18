use biblioteca_backend::catalog::model::Author;
use hyper::{StatusCode, Body, Request, header};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{app::create_mock_app, db::{MockDatabaseBuilder, MockDatabaseQuerier}, catalog::MockCatalog};


#[tokio::test]
async fn update_author_correct_parameters_successful() {
    let database_path = "update_book_correct_parameters_successful.sqlite";

    let author = MockCatalog::new_author().build();

    let new_author = MockCatalog::new_author()
        .name("New author name".to_string())
        .description("New author description".to_string())
        .country("New country".to_string())
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/authors/{}", author.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": new_author.name,
                        "description": new_author.description,
                        "country": new_author.country,
                    }))
                    .unwrap()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let expected_author = Author {
        id: author.id,
        name: new_author.name,
        description: new_author.description,
        country: new_author.country
    };

    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "checking if response is OK"
    );

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        
        assert_eq!(
            querier.contains_author(&expected_author),
            true,
            "checking if author was updated correctly"
        )
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn update_author_non_existent_author_failure() {
    let database_path = "update_author_non_existent_author_failure.sqlite";

    let author_a = MockCatalog::new_author().build();
    let author_b = MockCatalog::new_author().build();

    let invalid_author_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/authors/{}", invalid_author_id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": author_a.name,
                        "description": author_a.description,
                        "country": author_a.country,
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
            querier.contains_author(&author_a) && querier.contains_author(&author_b),
            true,
            "checking if author table was not affected"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn update_author_incorrect_parameters_failure() {
    let database_path = "update_author_incorrect_parameters_failure.sqlite";

    let author_a = MockCatalog::new_author().build();
    let author_b = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/authors/{}", author_b.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name_incorrect": author_a.name,
                        "description": author_a.description,
                        "country": author_a.country,
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
async fn update_author_missing_parameters_failure() {
    let database_path = "update_author_missing_parameters_failure.sqlite";

    let author_a = MockCatalog::new_author().build();
    let author_b = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/authors/{}", author_b.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": author_a.name,
                        "description": author_a.description,
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