use uuid::Uuid;

pub fn new() -> String {
    let my_uuid = Uuid::new_v4();
    my_uuid.to_simple().to_string()
}