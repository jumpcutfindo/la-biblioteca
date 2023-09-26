use biblioteca_backend::users::model::UserRole;
use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;

use crate::mocker::{
    app::create_mock_app,
    db::{MockDatabaseBuilder, MockDatabaseQuerier},
    users::MockUserBase,
};

#[tokio::test]
async fn list_user_roles_successful() {
    let database_path = "list_user_roles_successful.sqlite";

    let user_role_a = MockUserBase::new_user_role().build();
    let user_role_b = MockUserBase::new_user_role().build();
    let user_role_c = MockUserBase::new_user_role().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role_a)
        .with_user_role(&user_role_b)
        .with_user_role(&user_role_c)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/roles")
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
    let returned_user_roles: Vec<UserRole> = serde_json::from_slice(&body).unwrap();

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert!(
            querier.contains_num_user_roles(returned_user_roles.len() as i32),
            "checking if user role count is correct"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}
