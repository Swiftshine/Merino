pub struct EmojiMessage;

macro_rules! emoji_msg {
    // both standalone and message
    ($name:ident, $emoji:expr, both) => {
        pub fn $name() -> &'static str {
            $emoji
        }
        paste::paste! {
            pub fn [<$name _msg>](msg: &str) -> String { format!("{} {}", $emoji, msg) }
        }
    };
    // standalone only
    ($name:ident, $emoji:expr, icon) => {
        pub fn $name() -> &'static str {
            $emoji
        }
    };
    // message only
    ($name:ident, $emoji:expr, message) => {
        paste::paste! {
            pub fn [<$name _msg>](msg: &str) -> String { format!("{} {}", $emoji, msg) }
        }
    };
}

impl EmojiMessage {
    // emoji_msg!(add, "➕", both);
    // emoji_msg!(check, "✔", message);
    // emoji_msg!(cross, "❌", both);
    // emoji_msg!(discard, "🗑", icon);
    // emoji_msg!(burger, "☰", message);
    emoji_msg!(folder, "📁", message);
    emoji_msg!(memo, "📝", message);
    // emoji_msg!(palette, "🎨", message);
    // emoji_msg!(target, "🎯", both);
    // emoji_msg!(warning, "⚠", message);
}
