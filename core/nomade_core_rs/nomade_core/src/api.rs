use flutter_rust_bridge::frb;

#[frb(sync)]
pub fn process_message(input: String) -> String {
    format!("Echo from Nomade Core: {}", input)
}

#[frb(init)]
pub fn init_app() {
    // Default utilities - Flutter Rust Bridge
    flutter_rust_bridge::setup_default_user_utils();
}
