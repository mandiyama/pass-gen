use rand::Rng;

fn generate_password(length: usize) -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
                    
    let mut rng = rand::thread_rng();

    let mut password = String::new();
    let mut prev_char: Option<char> = None;

    while password.len() < length {
        let idx = rng.gen_range(0..charset.len());
        let current_char = charset[idx] as char;

        if let Some(prev) = prev_char {
            if prev.is_digit(10) && current_char.is_digit(10) {
                continue;
            }

            if prev.is_ascii_lowercase() && current_char.is_ascii_lowercase() {
                continue;
            }
            if prev.is_ascii_uppercase() && current_char.is_ascii_uppercase() {
                continue;
            }
        }

        password.push(current_char);
        prev_char = Some(current_char);
    }

    password
}

fn main() {
    let password_length = 24; 
    let password = generate_password(password_length);
    println!("Сгенерированный пароль: {}", password);
}
