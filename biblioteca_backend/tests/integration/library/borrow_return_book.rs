use axum::Router;
use hyper::{header, Body, Request};
use serde_json::json;
use tower::{Service, ServiceExt};
use uuid::Uuid;

use crate::mocker::{
    app::create_mock_app,
    catalog::MockCatalog,
    db::{MockDatabaseBuilder, MockDatabaseQuerier},
    users::MockUserBase,
};

#[tokio::test]
async fn borrow_book_return_book_successful() {
    let database_path = "borrow_book_return_book_successful.sqlite";

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

    let mut app = create_mock_app(db);

    borrow_book_with_api(&mut app, user.id, book.id).await;
    return_book_with_api(&mut app, user.id, book.id).await;

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert!(
            querier.is_book_returned(&book.id),
            "checking if book is returned",
        );
    }

    app.oneshot(Request::get("/").body(Body::empty()).unwrap());
    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn borrow_multiple_books_return_multiple_books_successful() {
    let database_path = "borrow_multiple_books_return_multiple_books_successful.sqlite";

    let user = MockUserBase::new_user().build();
    let user_role = MockUserBase::new_user_role()
        .num_borrowable_books(4)
        .build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();
    let book_c = MockCatalog::new_book().build();
    let book_d = MockCatalog::new_book().build();
    let author = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_user_role(&user_role)
        .with_user(&user, &user_role)
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .with_book(&book_c, &author.id)
        .with_book(&book_d, &author.id)
        .build();

    let mut app = create_mock_app(db);

    borrow_book_with_api(&mut app, user.id, book_a.id).await;
    borrow_book_with_api(&mut app, user.id, book_b.id).await;
    borrow_book_with_api(&mut app, user.id, book_c.id).await;
    return_book_with_api(&mut app, user.id, book_c.id).await;
    return_book_with_api(&mut app, user.id, book_b.id).await;
    return_book_with_api(&mut app, user.id, book_a.id).await;
    borrow_book_with_api(&mut app, user.id, book_d.id).await;
    borrow_book_with_api(&mut app, user.id, book_a.id).await;

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert!(
            querier.is_book_borrowed(&book_a.id)
                && querier.is_book_borrowed(&book_d.id)
                && querier.is_book_returned(&book_b.id)
                && querier.is_book_returned(&book_c.id),
            "checking if book is returned",
        );
    }

    app.oneshot(Request::get("/").body(Body::empty()).unwrap());
    MockDatabaseBuilder::teardown(database_path.to_string());
}

async fn borrow_book_with_api(app: &mut Router, user_id: Uuid, book_id: Uuid) {
    app.call(
        Request::builder()
            .method("POST")
            .uri(format!("/borrow/books/{}", book_id))
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_string(&json!({
                    "user_id": user_id,
                }))
                .unwrap(),
            ))
            .unwrap(),
    )
    .await
    .unwrap();
}

async fn return_book_with_api(app: &mut Router, user_id: Uuid, book_id: Uuid) {
    app.call(
        Request::builder()
            .method("POST")
            .uri(format!("/return/books/{}", book_id))
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_string(&json!({
                    "user_id": user_id,
                }))
                .unwrap(),
            ))
            .unwrap(),
    )
    .await
    .unwrap();
}
