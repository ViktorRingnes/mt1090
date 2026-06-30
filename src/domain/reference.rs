use uuid::Uuid;

pub fn generate_reference() -> String {
    Uuid::now_v7().to_string()
}
