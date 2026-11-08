use crate::{Person, get_person, insert_or_update_person, is_connection_open, remove_person};

#[must_use]
pub fn save_or_update(person_key: &str, person: &Person) -> bool {
    insert_or_update_person(person_key, person)
}

#[must_use]
pub fn remove(person_key: &str) -> bool {
    remove_person(person_key)
}

#[must_use]
pub fn choose(person_key: &str) -> Option<Person> {
    get_person(person_key)
}

#[must_use]
pub fn test_connection() -> bool {
    is_connection_open()
}
