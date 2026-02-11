use colored::Colorize;
use config::icons::{IconStyle, Icons};

#[derive(Debug, Clone)]
pub struct Message {
    icons: Icons,
}

impl Message {
    pub fn new(icon_style: IconStyle) -> Self {
        let icons = Icons::new(icon_style);
        Message { icons }
    }

    pub fn success(&self, msg: &str) {
        println!("{}{}", self.icons.success.green(), msg);
    }

    pub fn error(&self, msg: &str) {
        eprintln!("{}{}", self.icons.error.red(), msg);
    }

    pub fn warning(&self, msg: &str) {
        println!("{}{}", self.icons.warning.yellow(), msg);
    }

    pub fn info(&self, msg: &str) {
        println!("{}{}", self.icons.info.blue(), msg);
    }

    pub fn link(&self, msg: &str) {
        println!("{}{}", self.icons.link.cyan(), msg);
    }

    pub fn unlink(&self, msg: &str) {
        println!("{}{}", self.icons.unlink.purple(), msg);
    }

    pub fn delete(&self, msg: &str) {
        println!("{}{}", self.icons.delete.magenta(), msg);
    }
}
