
pub fn format_err(message: String) -> String {
    format!("\x1b[91m{}\x1b[0m", message)
}
pub fn format_succ(message: String) -> String {
    format!("\x1b[92m{}\x1b[0m", message)
}
pub fn format_warning(message: String) -> String {
    format!("\x1b[93m{}\x1b[0m", message)
}
