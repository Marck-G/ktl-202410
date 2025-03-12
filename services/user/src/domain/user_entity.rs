pub struct Metadata {
    id: String,
    key: String,
    pub value: String,
    pub date_created: u64,
    pub date_modified: u64
}

impl Metadata {
    pub fn new (id: &str, key: &str, value: &str) -> Metadata{
        Metadata {
            id: id.to_string(),
            key: key.to_string(),
            value: value.to_string(),
            date_created: 0,
            date_modified: 0
        }
    }
}


pub struct UserEntity {
    id: String,
    email: String,
    pub verified: bool,
    pub person: String,
    pub date_created: u64,
    pub date_modified: u64,
}

impl UserEntity {
    pub fn new(id: &str, email: &str) -> UserEntity {
        UserEntity {
            id: id.to_string(),
            email: email.to_string(),
            verified: false,
            person: "".to_string(),
            date_created: 0,
            date_modified: 0
        }
    }
}
