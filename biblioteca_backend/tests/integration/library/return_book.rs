use chrono::Duration;
use hyper::{Request, Body, StatusCode, header};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::mocker::{users::MockUserBase, catalog::MockCatalog, db::{MockDatabaseBuilder, MockDatabaseQuerier}, app::create_mock_app, library::MockLibrary, api::BibliotecaApiResponse};

#[tokio::test]
async fn return_book_can_return_successful() {
    let database_path = "return_book_can_return_successful.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let book = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();
    let borrow_entry = MockLibrary::new_borrow_entry()
        .user_id(user.id)
        .book_id(book.id)
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .with_book(&book, &author.id)
        .with_library_entry(&borrow_entry)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/return", book.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": user.id,
                    }))
                    .unwrap()
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
        assert_eq!(
            querier.is_book_returned(&book.id),
            true,
            "checking if book is returned",
        )
    }
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn return_book_user_not_same_failure() {
    let database_path = "return_book_user_not_same_failure.sqlite";

    let user_a = MockUserBase::new_user().build();
    let user_b = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let book = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();
    let borrow_entry = MockLibrary::new_borrow_entry()
        .user_id(user_a.id)
        .book_id(book.id)
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user_a, &user_role)
        .with_user(&user_b, &user_role)
        .with_author(&author)
        .with_book(&book, &author.id)
        .with_library_entry(&borrow_entry)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/return", book.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": user_b.id,
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
        "checking if response is correct (bad request)"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let api_response: BibliotecaApiResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        api_response.is_correct(40001, "book was not borrowed by given user".to_string()),
        true,
        "checking if API response message is correct"
    );
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn return_book_book_non_existent_failure() {
    let database_path = "return_book_book_non_existent_failure.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let author = MockCatalog::new_author().build();

    let incorrect_book_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/return", incorrect_book_id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": user.id,
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
        "checking if response is correct (bad request)"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let api_response: BibliotecaApiResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        api_response.is_correct(40001, "book does not exist".to_string()),
        true,
        "checking if API response message is correct"
    );
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn return_book_user_non_existent_failure() {
    let database_path = "return_book_user_non_existent_failure.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let book = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();
    let borrow_entry = MockLibrary::new_borrow_entry()
        .user_id(user.id)
        .book_id(book.id)
        .build();

    let incorrect_user_id = Uuid::new_v4();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .with_book(&book, &author.id)
        .with_library_entry(&borrow_entry)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/return", book.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": incorrect_user_id,
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
        "checking if response is correct (bad request)"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let api_response: BibliotecaApiResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        api_response.is_correct(40001, "user does not exist".to_string()),
        true,
        "checking if API response message is correct"
    );
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn return_book_book_already_returned_failure() {
    let database_path = "return_book_book_already_returned_failure.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role().build();
    let book = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();
    let borrow_entry = MockLibrary::new_borrow_entry()
        .user_id(user.id)
        .book_id(book.id)
        .build();
    let return_entry = MockLibrary::new_return_entry()
        .user_id(user.id)
        .book_id(book.id)
        .timestamp(borrow_entry.timestamp + Duration::days(1))
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .with_book(&book, &author.id)
        .with_library_entry(&borrow_entry)
        .with_library_entry(&return_entry)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/books/{}/return", book.id))
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "user_id": user.id,
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
        "checking if response is correct (bad request)"
    );

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let api_response: BibliotecaApiResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        api_response.is_correct(40001, "book has already been returned".to_string()),
        true,
        "checking if API response message is correct"
    );
    
    MockDatabaseBuilder::teardown(database_path.to_string());
}