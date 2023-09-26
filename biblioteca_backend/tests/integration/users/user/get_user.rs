use biblioteca_backend::users::model::User;
use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{app::create_mock_app, db::MockDatabaseBuilder, users::MockUserBase};

#[tokio::test]
async fn get_user_user_exists_successful() {
    let database_path = "get_user_user_exists_successful.sqlite";

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
                .method("GET")
                .uri(format!("/users/{}", user.id))
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
    let returned_user: User = serde_json::from_slice(&body).unwrap();

    {
        assert!(returned_user.id == user.id);
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn get_user_non_existent_user_failure() {
    let database_path = "get_user_non_existent_user_failure.sqlite";

    let user_role = MockUserBase::new_user_role().build();
    let user = MockUserBase::new_user().build();

    let incorrect_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/users/{}", incorrect_id))
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
