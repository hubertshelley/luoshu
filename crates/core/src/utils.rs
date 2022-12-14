use uuid::Uuid;

/// generate default uuid4
pub fn get_default_uuid4() -> String {
    Uuid::new_v4().to_string()
}

/// get default namespace
pub fn default_namespace() -> String {
    "default".to_string()
}
