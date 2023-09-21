use biblioteca_backend::users::model::User;
use rand::Rng;
use random_string::generate;
use uuid::Uuid;

pub struct MockUserBase {}

pub struct MockUserBuilder {
    id: Uuid,
    username: String,
}

impl MockUserBase {
    pub fn new_user() -> MockUserBuilder {
        return MockUserBuilder {
            id: Uuid::new_v4(),
            username: Self::random_string(8, 16),
        }
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
        }
    }
}