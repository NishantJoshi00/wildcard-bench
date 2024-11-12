use nanoid::nanoid;

use crate::Key;

// if the value of N is too large, the number of possible keys will be too large
const UNIQUENESS_THRESHOLD: usize = 3;

pub fn generate_key<const N: usize>() -> [String; N] {
    let mut key = [const { String::new() }; N];

    key.iter_mut().for_each(|k| {
        *k = nanoid!(UNIQUENESS_THRESHOLD);
    });

    key
}

pub fn generate_find_key<const N: usize>() -> [Key; N] {
    let mut key = [const { Key::Wildcard }; N];

    key.iter_mut().for_each(|k| {
        if rand::random() {
            *k = Key::String(nanoid!(UNIQUENESS_THRESHOLD));
        }
    });

    key
}
