use biblioteca_backend::catalog::model::{Book, Author};
use rand::{rngs::ThreadRng, Rng};
use mime::CHARSET;
use random_string::generate;
use uuid::Uuid;

pub struct MockCatalog {}

impl MockCatalog {

    pub fn new_book() -> Book {
        return Book {
            id: Uuid::new_v4(),
            name: Self::random_string(8, 24),
            description: Self::random_string(32, 64),
            language: Self::random_string(32, 64),
        }
    }

    pub fn new_author() -> Author {
        return Author {
            id: Uuid::new_v4(),
            name: Self::random_string(16, 24),
            description: Self::random_string(32, 64),
            country: Self::random_string(8, 16),
        }
    }

    fn random_string(min: usize, max: usize) -> String {
        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ".to_string();
        let mut rng = rand::thread_rng();
        return generate(rng.gen_range(min..max), charset);
    }
}