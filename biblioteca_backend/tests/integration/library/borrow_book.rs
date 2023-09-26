use hyper::{header, Body, Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{
    api::BibliotecaApiResponse,
    app::create_mock_app,
    catalog::MockCatalog,
    db::{MockDatabaseBuilder, MockDatabaseQuerier},
    library::MockLibrary,
    users::MockUserBase,
};

#[tokio::test]
async fn borrow_book_can_borrow_successful() {
    let database_path = "borrow_book_can_borrow_successful.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let book = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .with_book(&book, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/borrow", book.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": user.id,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::ACCEPTED,
        "checking if response is OK"
    );

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert!(
            querier.is_book_borrowed(&book.id),
            "checking if book is borrowed",
        )
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn borrow_book_book_non_existent_failure() {
    let database_path = "borrow_book_book_non_existent_failure.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let book = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();

    let incorrect_book_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .with_book(&book, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/borrow", incorrect_book_id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": user.id,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "checking if response is correct (bad request)"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let api_response: BibliotecaApiResponse = serde_json::from_slice(&body).unwrap();

    assert!(
        api_response.is_correct(40001, "book does not exist".to_string()),
        "checking if API response message is correct"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn borrow_book_user_non_existent_failure() {
    let database_path = "borrow_book_user_non_existent_failure.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let book = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();

    let incorrect_user_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .with_book(&book, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/borrow", book.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": incorrect_user_id,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "checking if response is correct (bad request)"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let api_response: BibliotecaApiResponse = serde_json::from_slice(&body).unwrap();

    assert!(
        api_response.is_correct(40001, "user does not exist".to_string()),
        "checking if API response message is correct"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn borrow_book_book_already_borrowed_failure() {
    let database_path = "borrow_book_book_already_borrowed_failure.sqlite";

    let user_a = MockUserBase::new_user().build();
    let user_b = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let book = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();
    let user_a_borrow_entry = MockLibrary::new_borrow_entry()
        .book_id(book.id)
        .user_id(user_a.id)
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user_a, &user_role)
        .with_user(&user_b, &user_role)
        .with_author(&author)
        .with_book(&book, &author.id)
        .with_library_entry(&user_a_borrow_entry)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/borrow", book.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": user_b.id,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "checking if response is correct (bad request)"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let api_response: BibliotecaApiResponse = serde_json::from_slice(&body).unwrap();

    assert!(
        api_response.is_correct(40001, "already been borrowed".to_string()),
        "checking if API response message is correct"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn borrow_book_user_borrowed_too_many_failure() {
    let database_path = "borrow_book_user_borrowed_too_many_failure.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role()
        .num_borrowable_books(1)
        .build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();
    let user_borrow_entry = MockLibrary::new_borrow_entry()
        .book_id(book_a.id)
        .user_id(user.id)
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .with_library_entry(&user_borrow_entry)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/borrow", book_b.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": user.id,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "checking if response is correct (bad request)"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let api_response: BibliotecaApiResponse = serde_json::from_slice(&body).unwrap();

    assert!(
        api_response.is_correct(
            40001,
            "user has reached max num of borrowable books".to_string()
        ),
        "checking if API response message is correct"
    );

    MockDatabaseBuilder::teardown(database_path.to_string());
}
