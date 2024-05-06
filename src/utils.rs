use std::{io::{self, Write}, str::FromStr};

pub fn input<T: FromStr>(prompt: Option<String>) -> T {
    let prompt: String = match prompt {
        Some(s) => s,
        None => String::new()
    };
    loop {
        let mut input: String = String::new();
        print!("{} ", prompt);
        io::stdout().flush().expect("Failed to write to stdout.");
        io::stdin().read_line(&mut input).expect("Error reading from stdin.");
        let ans: T = match input.trim().parse() {
            Ok(r) => r,
            Err(_) => {
                println!("PARSE ERROR.");
                continue;
            }
        };
        return ans;
    }
}

pub fn clear(){
    print!("\x1b[2J\x1b[0;0H");
    io::stdout().flush().expect("Error writting to the stdout.");
}

pub fn position(x: usize, y: usize) {
    print!("\x1b[{};{}H", y, x);
    io::stdout().flush().expect("Error writting to the stdout.");
}

pub fn pause() {
    let _ = input::<String>(Some("Press ENTER to continue...".to_string()));
}


