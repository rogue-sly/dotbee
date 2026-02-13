use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum IconStyle {
    #[default]
    Text,
    Emoji,
    NerdFont,
}

#[derive(Debug, Clone)]
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
    pub fn new(style: IconStyle) -> Self {
        match style {
            #[rustfmt::skip]
            IconStyle::Emoji => Self {
                success : "✅ ".to_string(),
                error   : "❌ ".to_string(),
                warning : "⚠️ ".to_string(),
                info    : "ℹ️ ".to_string(),
                link    : "🔗 ".to_string(),
                unlink  : "💔 ".to_string(),
                delete  : "🗑️ ".to_string()
            },
            #[rustfmt::skip]
            IconStyle::NerdFont => Self {
                success : " ".to_string(),
                error   : " ".to_string(),
                warning : " ".to_string(),
                info    : " ".to_string(),
                link    : " ".to_string(),
                unlink  : " ".to_string(),
                delete  : " ".to_string(),
            },
            #[rustfmt::skip]
            IconStyle::Text => Self {
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
