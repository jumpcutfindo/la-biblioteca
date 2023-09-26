use serde::Deserialize;

#[derive(Deserialize)]
pub struct BibliotecaApiResponse {
    code: u16,
    message: String,
}

impl BibliotecaApiResponse {
    pub fn is_correct(&self, expected_code: u16, expected_substring: String) -> bool {
        return self.code == expected_code && self.message.contains(&expected_substring);
    }
}