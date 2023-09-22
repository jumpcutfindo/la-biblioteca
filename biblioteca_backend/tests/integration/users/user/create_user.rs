use biblioteca_backend::users::model::User;
use hyper::{Request, Method, header, Body, StatusCode};
use serde_json::json;
use tower::ServiceExt;

use crate::mocker::{db::{MockDatabaseBuilder, MockDatabaseQuerier}, app::create_mock_app, users::MockUserBase};

#[tokio::test]
async fn create_user_correct_parameters_successful() {
    let database_path = "create_user_correct_parameters_successful.sqlite";

    let user_role = MockUserBase::new_user_role().build();
    let user = MockUserBase::new_user().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": user.username,
                        "user_role_id": user_role.id,
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
    let created_user: User = serde_json::from_slice(&body).unwrap();
    let expected_user = User {
        id: created_user.id,
        username: user.username,
    };

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_user(&expected_user),
            true,
            "check if user was added properly",
        );

        assert_eq!(
            querier.contains_user_user_role_mapping(&created_user.id, &user_role.id),
            true,
            "check if user to user role mapping was added properly"
        )
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_user_wrong_parameters_failure() {
    let database_path = "create_user_wrong_parameters_failure.sqlite";

    let user_role = MockUserBase::new_user_role().build();
    let user = MockUserBase::new_user().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username_incorrect": user.username,
                        "user_role_id": user_role.id,
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
async fn create_user_missing_parameters_failure() {
    let database_path = "create_user_missing_parameters_failure.sqlite";

    let user_role = MockUserBase::new_user_role().build();
    let user = MockUserBase::new_user().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": user.username,
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
async fn create_user_additional_parameters_successful() {
    let database_path = "create_user_additional_parameters_successful.sqlite";

    let user_role = MockUserBase::new_user_role().build();
    let user = MockUserBase::new_user().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": user.username,
                        "user_role_id": user_role.id,
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
    let created_user: User = serde_json::from_slice(&body).unwrap();
    let expected_user = User {
        id: created_user.id,
        username: user.username,
    };

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_user(&expected_user),
            true,
            "check if user was added properly",
        );

        assert_eq!(
            querier.contains_user_user_role_mapping(&created_user.id, &user_role.id),
            true,
            "check if user to user role mapping was added properly"
        )
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn create_user_existing_username_failure() {
    let database_path = "create_user_existing_username_failure.sqlite";

    let user_role = MockUserBase::new_user_role().build();
    let user = MockUserBase::new_user().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": user.username,
                        "user_role_id": user_role.id,
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