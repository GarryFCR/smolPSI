use crate::utils::elligator::{keygen, Key};
use curve25519_elligator2::EdwardsPoint;

pub struct Party {
    pub key: Key,
    pub interests: Vec<String>,
    pub friends: Vec<EdwardsPoint>,
    pub strangers: Vec<EdwardsPoint>,
}

impl Party {
    pub fn new(list: Vec<String>) -> Party {
        Party {
            key: keygen(),
            interests: list,
            friends: vec![],
            strangers: vec![],
        }
    }
}
