use biblioteca_backend::users::model::FullUser;
use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;

use crate::mocker::{
    app::create_mock_app,
    db::{MockDatabaseBuilder, MockDatabaseQuerier},
    users::MockUserBase,
};

#[tokio::test]
async fn list_users_successful() {
    let database_path = "list_users_successful.sqlite";

    let user_role = MockUserBase::new_user_role().build();
    let user_a = MockUserBase::new_user().build();
    let user_b = MockUserBase::new_user().build();
    let user_c = MockUserBase::new_user().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user_a, &user_role)
        .with_user(&user_b, &user_role)
        .with_user(&user_c, &user_role)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users")
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
    let returned_users: Vec<FullUser> = serde_json::from_slice(&body).unwrap();

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert!(
            querier.contains_num_users(returned_users.len() as i32),
            "checking if user count is correct"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}
