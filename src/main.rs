use colored::Colorize;

mod parser;
mod validator;

fn main() {
    println!("\n🔍 DevGuard - scanning .env...\n");
    let lines_map = parser::parser_env();
    let valid = validator::validate_env(lines_map);

    for valid_error in &valid {
        println!("❌ {} -> {}", valid_error.key.red(), valid_error.message)
    }
    if valid.is_empty() {
        println!("✅ All checks passed! Your .env looks good!");
    } else {
        println!("\n⚠️  {} issue(s) found", valid.len());
    }
}
