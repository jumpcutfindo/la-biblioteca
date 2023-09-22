use biblioteca_backend::users::model::UserRole;
use hyper::{Request, Method, header, Body, StatusCode};
use serde_json::json;
use tower::ServiceExt;

use crate::mocker::{db::{MockDatabaseBuilder, MockDatabaseQuerier}, app::create_mock_app, users::MockUserBase};

#[tokio::test]
async fn create_user_role_correct_parameters_successful() {
    let database_path = "create_user_role_correct_parameters_successful.sqlite";

    let user_role = MockUserBase::new_user_role().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users/roles")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": user_role.name,
                        "num_borrowable_books": user_role.num_borrowable_books,
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
    let created_user_role: UserRole = serde_json::from_slice(&body).unwrap();
    let expected_user_role = UserRole {
        id: created_user_role.id,
        name: user_role.name,
        num_borrowable_books: user_role.num_borrowable_books,
    };

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_user_role(&expected_user_role),
            true,
            "check if user role was added properly",
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_user_role_wrong_parameters_failure() {
    let database_path = "create_user_role_wrong_parameters_failure.sqlite";

    let user_role = MockUserBase::new_user_role().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users/roles")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name_invalid": user_role.name,
                        "num_borrowable_books": user_role.num_borrowable_books,
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
async fn create_user_role_missing_parameters_failure() {
    let database_path = "create_user_role_missing_parameters_failure.sqlite";

    let user_role = MockUserBase::new_user_role().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users/roles")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "num_borrowable_books": user_role.num_borrowable_books,
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
async fn create_user_role_additional_parameters_successful() {
    let database_path = "create_user_role_additional_parameters_successful.sqlite";

    let user_role = MockUserBase::new_user_role().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users/roles")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": user_role.name,
                        "num_borrowable_books": user_role.num_borrowable_books,
                        "additional_parameter": "hello_world".to_string(),
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
    let created_user_role: UserRole = serde_json::from_slice(&body).unwrap();
    let expected_user_role = UserRole {
        id: created_user_role.id,
        name: user_role.name,
        num_borrowable_books: user_role.num_borrowable_books,
    };

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_user_role(&expected_user_role),
            true,
            "check if user role was added properly",
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_user_role_negative_borrowable_books_failure() {
    let database_path = "create_user_role_negative_borrowable_books_failure.sqlite";

    let user_role = MockUserBase::new_user_role()
        .num_borrowable_books(-100)
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users/roles")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "name": user_role.name,
                        "num_borrowable_books": user_role.num_borrowable_books,
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
        "checking if response is correct (unprocessable)"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}