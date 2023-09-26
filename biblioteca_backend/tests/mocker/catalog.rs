use biblioteca_backend::catalog::model::{Author, Book};
use rand::Rng;
use random_string::generate;
use uuid::Uuid;

pub struct MockCatalog {}

pub struct MockBookBuilder {
    id: Uuid,
    name: String,
    description: String,
    language: String,
}

impl MockBookBuilder {
    pub fn id(mut self, id: Uuid) -> MockBookBuilder {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> MockBookBuilder {
        self.name = name;
        self
    }

    pub fn description(mut self, description: String) -> MockBookBuilder {
        self.description = description;
        self
    }

    pub fn language(mut self, language: String) -> MockBookBuilder {
        self.language = language;
        self
    }

    pub fn build(self) -> Book {
        return Book {
            id: self.id,
            name: self.name,
            description: self.description,
            language: self.language,
        };
    }
}

pub struct MockAuthorBuilder {
    id: Uuid,
    name: String,
    description: String,
    country: String,
}

impl MockAuthorBuilder {
    pub fn id(mut self, id: Uuid) -> MockAuthorBuilder {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> MockAuthorBuilder {
        self.name = name;
        self
    }

    pub fn description(mut self, description: String) -> MockAuthorBuilder {
        self.description = description;
        self
    }

    pub fn country(mut self, country: String) -> MockAuthorBuilder {
        self.country = country;
        self
    }

    pub fn build(self) -> Author {
        return Author {
            id: self.id,
            name: self.name,
            description: self.description,
            country: self.country,
        };
    }
}

impl MockCatalog {
    pub fn new_book() -> MockBookBuilder {
        return MockBookBuilder {
            id: Uuid::new_v4(),
            name: Self::random_string(8, 24),
            description: Self::random_string(32, 64),
            language: Self::random_string(32, 64),
        };
    }

    pub fn new_author() -> MockAuthorBuilder {
        return MockAuthorBuilder {
            id: Uuid::new_v4(),
            name: Self::random_string(16, 24),
            description: Self::random_string(32, 64),
            country: Self::random_string(8, 16),
        };
    }

    fn random_string(min: usize, max: usize) -> String {
        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ".to_string();
        let mut rng = rand::thread_rng();
        return generate(rng.gen_range(min..max), charset);
    }
}
