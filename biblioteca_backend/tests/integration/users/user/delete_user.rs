use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{
    app::create_mock_app,
    db::{MockDatabaseBuilder, MockDatabaseQuerier},
    users::MockUserBase,
};

#[tokio::test]
async fn delete_user_existing_user_successful() {
    let database_path = "delete_user_existing_user_successful.sqlite";

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
                .uri(format!("/users/{}", user_a.id))
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
        assert!(
            !querier.contains_user(&user_a),
            "checking if user was removed properly"
        );

        assert!(
            !querier.contains_user_user_role_mapping(&user_a.id, &user_role.id),
            "checking if user_a to user_role mapping was removed properly"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn delete_user_non_existent_user_successful() {
    let database_path = "delete_user_non_existent_user_successful.sqlite";

    let user_role = MockUserBase::new_user_role().build();
    let user = MockUserBase::new_user().build();

    let incorrect_uuid = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/users/{}", incorrect_uuid))
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
        assert!(
            querier.contains_user(&user),
            "checking if no users were removed"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}
