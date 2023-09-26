use biblioteca_backend::catalog::model::Book;
use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;

use crate::mocker::{
    app::create_mock_app,
    catalog::MockCatalog,
    db::{MockDatabaseBuilder, MockDatabaseQuerier},
};

#[tokio::test]
async fn list_books_successful() {
    let database_path = "list_books_successful.sqlite";

    let author = MockCatalog::new_author().build();
    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();
    let book_c = MockCatalog::new_book().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .with_book(&book_c, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/books")
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
    let returned_books: Vec<Book> = serde_json::from_slice(&body).unwrap();

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert!(
            querier.contains_num_books(returned_books.len() as i32),
            "checking if book count is correct"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn list_books_with_name_search_successful() {
    let database_path = "list_books_with_name_search_successful.sqlite";

    let author = MockCatalog::new_author().build();

    let book_a = MockCatalog::new_book()
        .name("Alice in Wonderland".to_string())
        .build();

    let book_b = MockCatalog::new_book()
        .name("Alice in Wonderland 2".to_string())
        .build();

    let book_c = MockCatalog::new_book()
        .name("Harry Potter and the Philosopher's Stone".to_string())
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .with_book(&book_c, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/books?name=Alice")
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
    let returned_books: Vec<Book> = serde_json::from_slice(&body).unwrap();

    {
        assert!(returned_books.len() == 2);
        for book in returned_books.iter() {
            assert!(book.name.contains("Alice"));
        }
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn list_books_with_language_search_successful() {
    let database_path = "list_books_with_language_search_successful.sqlite";

    let author = MockCatalog::new_author().build();

    let book_a = MockCatalog::new_book()
        .language("English".to_string())
        .build();

    let book_b = MockCatalog::new_book()
        .language("English".to_string())
        .build();

    let book_c = MockCatalog::new_book()
        .language("Chinese".to_string())
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .with_book(&book_c, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/books?language=English")
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
    let returned_books: Vec<Book> = serde_json::from_slice(&body).unwrap();

    {
        assert!(returned_books.len() == 2);
        for book in returned_books.iter() {
            assert!(book.language.contains("English"));
        }
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn list_books_with_search_wrong_params_successful() {
    let database_path = "list_books_with_search_wrong_params_successful.sqlite";

    let author = MockCatalog::new_author().build();

    let book_a = MockCatalog::new_book().build();
    let book_b = MockCatalog::new_book().build();
    let book_c = MockCatalog::new_book().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author)
        .with_book(&book_a, &author.id)
        .with_book(&book_b, &author.id)
        .with_book(&book_c, &author.id)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/books?unsupported_params=Test")
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
    let returned_books: Vec<Book> = serde_json::from_slice(&body).unwrap();

    {
        assert!(returned_books.len() == 3);
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}
