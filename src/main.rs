use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}

fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let person1 = Person {
        id: 5,
        name: "Janet".to_string(),
        phone: 555_666_7777,
    };
    let person2 = Person {
        id: 5,
        name: "Bob".to_string(),
        phone: 555_666_7777,
    };

    println!("{}", my_hash(person1));
    println!("{}", my_hash(person2));
}
