extern crate mysql;
extern crate ring;

use ring::digest;
use ring::rand;

pub struct User {
    id: String,
    name: String,
    public_key: String,
    private_key: String,
}

impl User {
    pub fn new(name: String) -> User {
        let user_id = digest::digest(&digest::SHA512, b"");
    }

    pub fn create_user(&self) {

    }
}