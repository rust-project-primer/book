use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Person {
    name: String,
    id: Uuid,
    age: u16,
}
