use hyper::{Request, Body, StatusCode};
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{db::{MockDatabaseBuilder, MockDatabaseQuerier}, app::create_mock_app, users::MockUserBase};

#[tokio::test]
async fn delete_user_role_existing_role_successful() {
    let database_path = "delete_user_role_existing_role_successful.sqlite";

    let user_a = MockUserBase::new_user().build();
    let user_b = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user_a, &user_role)
        .with_user(&user_b, &user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/users/roles/{}", user_role.id))
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
            querier.contains_user_role(&user_role),
            false,
            "checking if user_role was removed properly"
        );

        assert_eq!(
            querier.contains_user_user_role_mapping(&user_a.id, &user_role.id),
            false,
            "checking if user_a to user_role mapping was removed properly"
        );

        assert_eq!(
            querier.contains_user_user_role_mapping(&user_b.id, &user_role.id),
            false,
            "checking if user_b to user_role mapping was removed properly"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn delete_user_role_non_existent_role_successful() {
    let database_path = "delete_user_role_non_existent_role_successful.sqlite";

    let user_role_a = MockUserBase::new_user_role().build();
    let user_role_b = MockUserBase::new_user_role().build();

    let incorrect_uuid = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role_a)
        .with_user_role(&user_role_b)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/users/roles/{}", incorrect_uuid))
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
            querier.contains_user_role(&user_role_a) && querier.contains_user_role(&user_role_b),
            true,
            "checking if no user roles were removed"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}