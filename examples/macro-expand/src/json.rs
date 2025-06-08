use serde_json::json;
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    let person = json!({
        "name": "Jeff",
        "age": 24,
        "interests": ["guns", "trucks", "bbq"],
        "nationality": "us",
        "state": "tx",
        "id": id.to_string()
    });
}
