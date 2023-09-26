use biblioteca_backend::users::model::UserRole;
use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{app::create_mock_app, db::MockDatabaseBuilder, users::MockUserBase};

#[tokio::test]
async fn get_user_role_role_exists_successful() {
    let database_path = "get_user_role_role_exists_successful.sqlite";

    let user_role_a = MockUserBase::new_user_role().build();
    let user_role_b = MockUserBase::new_user_role().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role_a)
        .with_user_role(&user_role_b)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/users/roles/{}", user_role_a.id))
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
    let returned_user_role: UserRole = serde_json::from_slice(&body).unwrap();

    {
        assert!(returned_user_role.id == user_role_a.id);
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn get_user_role_non_existent_role_failure() {
    let database_path = "get_user_role_non_existent_role_failure.sqlite";

    let user_role_a = MockUserBase::new_user_role().build();
    let user_role_b = MockUserBase::new_user_role().build();

    let incorrect_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role_a)
        .with_user_role(&user_role_b)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/users/roles/{}", incorrect_id))
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
