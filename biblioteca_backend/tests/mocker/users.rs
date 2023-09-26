use biblioteca_backend::users::model::{User, UserRole};
use rand::Rng;
use random_string::generate;
use uuid::Uuid;

pub struct MockUserBase {}

pub struct MockUserBuilder {
    id: Uuid,
    username: String,
}

pub struct MockUserRoleBuilder {
    id: Uuid,
    name: String,
    num_borrowable_books: i32,
}

impl MockUserBase {
    pub fn new_user() -> MockUserBuilder {
        return MockUserBuilder {
            id: Uuid::new_v4(),
            username: Self::random_string(8, 16),
        };
    }

    pub fn new_user_role() -> MockUserRoleBuilder {
        let mut rng = rand::thread_rng();
        
        return MockUserRoleBuilder {
            id: Uuid::new_v4(),
            name: Self::random_string(8, 16),
            num_borrowable_books: rng.gen_range(2..4),
        };
    }

    fn random_string(min: usize, max: usize) -> String {
        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ".to_string();
        let mut rng = rand::thread_rng();
        return generate(rng.gen_range(min..max), charset);
    }
}

impl MockUserBuilder {
    pub fn id(mut self, id: Uuid) -> MockUserBuilder {
        self.id = id;
        self
    }

    pub fn username(mut self, username: String) -> MockUserBuilder {
        self.username = username;
        self
    }

    pub fn build(self) -> User {
        return User {
            id: self.id,
            username: self.username,
        };
    }
}

impl MockUserRoleBuilder {
    pub fn id(mut self, id: Uuid) -> MockUserRoleBuilder {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> MockUserRoleBuilder {
        self.name = name;
        self
    }

    pub fn num_borrowable_books(mut self, num: i32) -> MockUserRoleBuilder {
        self.num_borrowable_books = num;
        self
    }

    pub fn build(self) -> UserRole {
        return UserRole {
            id: self.id,
            name: self.name,
            num_borrowable_books: self.num_borrowable_books,
        };
    }
}
