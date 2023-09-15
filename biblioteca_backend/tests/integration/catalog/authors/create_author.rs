use biblioteca_backend::catalog::model::Author;
use hyper::{Request, Method, header, Body, StatusCode};
use serde_json::json;
use tower::ServiceExt;

use crate::mocker::{catalog::MockCatalog, db::{MockDatabaseBuilder, MockDatabaseQuerier}, app::create_mock_app};

#[tokio::test]
async fn create_author_correct_parameters_successful() {
    let database_path = "create_author_correct_parameters_successful.sqlite";

    let original_author = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/authors")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": original_author.name,
                        "description": original_author.description,
                        "country": original_author.country
                    }))
                    .unwrap()
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
    let created_author: Author = serde_json::from_slice(&body).unwrap();
    let expected_author = Author {
        id: created_author.id,
        name: original_author.name,
        description: original_author.description,
        country: original_author.country,
    };

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_author(&expected_author),
            true,
            "checking if the author was added properly"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_author_wrong_parameters_failure() {
    let database_path = "create_author_wrong_parameters_failure.sqlite";

    let original_author = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/authors")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name_incorrect": original_author.name,
                        "description": original_author.description,
                        "country": original_author.country
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
        "checking if response is correct (unprocessable)"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_author_missing_paramaters_failure() {
    let database_path = "create_author_missing_paramaters_failure.sqlite";

    let original_author = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/authors")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "description": original_author.description,
                        "country": original_author.country
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
        "checking if response is correct (unprocessable)"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_author_additional_parameters_successful() {
    let database_path = "create_author_additional_parameters_successful.sqlite";

    let original_author = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/authors")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": original_author.name,
                        "description": original_author.description,
                        "country": original_author.country,
                        "additional_parameter": "hello world".to_string(),
                    }))
                    .unwrap()
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
    let created_author: Author = serde_json::from_slice(&body).unwrap();
    let expected_author = Author {
        id: created_author.id,
        name: original_author.name,
        description: original_author.description,
        country: original_author.country,
    };

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_author(&expected_author),
            true,
            "checking if the author was added properly"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}