#[flutter_rust_bridge::frb(sync)]
pub fn validate_email(value: &str) -> bool {
    validation::validate_email(value)
}

#[flutter_rust_bridge::frb(sync)]
pub fn validate_username(value: &str) -> bool {
    validation::validate_username(value)
}

#[flutter_rust_bridge::frb(sync)]
pub fn validate_password(value: &str) -> bool {
    validation::validate_password(value)
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
