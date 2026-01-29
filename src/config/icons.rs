pub struct Icons {
    pub success: String,
    pub error: String,
    pub warning: String,
    pub info: String,
    pub link: String,
    pub unlink: String,
    pub delete: String,
}

impl Icons {
    pub fn new(style: &str) -> Self {
        match style.to_lowercase().as_str() {
            #[rustfmt::skip]
            "emoji" => Self {
                success : "✅ ".to_string(),
                error   : "❌ ".to_string(),
                warning : "⚠️ ".to_string(),
                info    : "ℹ️ ".to_string(),
                link    : "🔗 ".to_string(),
                unlink  : "💔 ".to_string(),
                delete  : "🗑️ ".to_string()
            },
            #[rustfmt::skip]
            "nerdfont" => Self {
                success : " ".to_string(),
                error   : " ".to_string(),
                warning : " ".to_string(),
                info    : " ".to_string(),
                link    : " ".to_string(),
                unlink  : " ".to_string(),
                delete  : " ".to_string(),
            },
            // default
            #[rustfmt::skip]
            "text" | _ => Self {
                success : "DONE  ".to_string(),
                error   : "ERROR ".to_string(),
                warning : "WARN  ".to_string(),
                info    : "INFO  ".to_string(),
                link    : "LINK  ".to_string(),
                unlink  : "MISS  ".to_string(),
                delete  : "DEL   ".to_string(),
            },
        }
    }
}
