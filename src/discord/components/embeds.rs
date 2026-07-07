use poise::serenity_prelude as serenity;
use serenity::CreateEmbed;

pub fn base_embed() -> CreateEmbed {
    CreateEmbed::default()
}

pub fn error_embed(message: impl Into<String>) -> CreateEmbed {
    CreateEmbed::default()
        .title("Error")
        .description(message)
        .color(0xED4245)
}

pub fn success_embed(message: impl Into<String>) -> CreateEmbed {
    CreateEmbed::default()
        .title("Success")
        .description(message)
        .color(0x57F287)
}

pub fn info_embed(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
    CreateEmbed::default()
        .title(title)
        .description(description)
        .color(0x5865F2)
}
