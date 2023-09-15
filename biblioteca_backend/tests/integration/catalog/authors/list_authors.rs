use biblioteca_backend::catalog::model::Author;
use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;

use crate::mocker::{
    app::create_mock_app,
    catalog::MockCatalog,
    db::{MockDatabaseBuilder, MockDatabaseQuerier},
};

#[tokio::test]
async fn list_authors_successful() {
    let database_path = "list_authors_successful.sqlite";
    
    let author_a = MockCatalog::new_author().build();
    let author_b = MockCatalog::new_author().build();
    let author_c = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .with_author(&author_c)
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
    let returned_authors: Vec<Author> = serde_json::from_slice(&body).unwrap();

    {
        let querier = MockDatabaseQuerier::create(database_path.to_string());
        assert_eq!(
            querier.contains_num_books(returned_authors.len() as i32),
            true,
            "checking if book count is correct"
        );
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn list_authors_with_name_search_successful() {
    let database_path = "list_authors_with_name_search_successful.sqlite";
    
    let author_a = MockCatalog::new_author()
        .name("Alex Adamson".to_string())
        .build();
    let author_b = MockCatalog::new_author()
        .name("Alex Barberson".to_string())
        .build();
    let author_c = MockCatalog::new_author()
        .name("Bob Hudson".to_string())
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .with_author(&author_c)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/authors?name=Alex")
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
    let returned_authors: Vec<Author> = serde_json::from_slice(&body).unwrap();

    {
        assert_eq!(returned_authors.len() == 2, true);

        for author in returned_authors.iter() {
            assert_eq!(author.name.contains("Alex"), true);
        }
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn list_authors_with_country_search_successful() {
    let database_path = "list_authors_with_country_search_successful.sqlite";
    
    let author_a = MockCatalog::new_author()
        .country("Singapore".to_string())
        .build();
    let author_b = MockCatalog::new_author()
        .country("Singapore".to_string())
        .build();
    let author_c = MockCatalog::new_author()
        .country("United Kingdom".to_string())
        .build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .with_author(&author_c)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/authors?country=Singapore")
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
    let returned_authors: Vec<Author> = serde_json::from_slice(&body).unwrap();

    {
        assert_eq!(returned_authors.len() == 2, true);

        for author in returned_authors.iter() {
            assert_eq!(author.country.contains("Singapore"), true);
        }
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}

#[tokio::test]
async fn list_authors_with_search_wrong_params_successful() {
    let database_path = "list_authors_with_search_wrong_params_successful.sqlite";
    
    let author_a = MockCatalog::new_author().build();
    let author_b = MockCatalog::new_author().build();
    let author_c = MockCatalog::new_author().build();

    let db = MockDatabaseBuilder::create(database_path.to_string())
        .with_author(&author_a)
        .with_author(&author_b)
        .with_author(&author_c)
        .build();

    let app = create_mock_app(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/authors?unsupported_params=Test")
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
    let returned_authors: Vec<Author> = serde_json::from_slice(&body).unwrap();

    {
        assert_eq!(returned_authors.len() == 3, true);
    }

    MockDatabaseBuilder::teardown(database_path.to_string());
}