use colored::Colorize;

pub fn success(msg: &str) {
    println!("{}: {}", "[DONE]".green(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{}: {}", "[ERROR]".red(), msg);
}

pub fn warning(msg: &str) {
    println!("{}: {}", "[WARN]".yellow(), msg);
}

pub fn info(msg: &str) {
    println!("{}: {}", "[INFO]".blue(), msg);
}

pub fn link(msg: &str) {
    println!("{}: {}", "[LINK]".cyan(), msg);
}

pub fn delete(msg: &str) {
    println!("{}: {}", "[DELETE]".red(), msg);
}
