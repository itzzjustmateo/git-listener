use poise::serenity_prelude as serenity;

pub struct EmbedBuilder {
    embed: serenity::CreateEmbed,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self {
            embed: serenity::CreateEmbed::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.embed = self.embed.title(title);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.embed = self.embed.description(description);
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.embed = self.embed.url(url);
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.embed = self.embed.color(color);
        self
    }

    pub fn author(
        mut self,
        name: impl Into<String>,
        icon_url: Option<String>,
        url: Option<String>,
    ) -> Self {
        let mut author = serenity::CreateEmbedAuthor::new(name);
        if let Some(icon) = icon_url {
            author = author.icon_url(icon);
        }
        if let Some(u) = url {
            author = author.url(u);
        }
        self.embed = self.embed.author(author);
        self
    }

    pub fn field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        inline: bool,
    ) -> Self {
        self.embed = self.embed.field(name, value, inline);
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<serenity::Timestamp>) -> Self {
        self.embed = self.embed.timestamp(timestamp);
        self
    }

    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        self.embed = self.embed.thumbnail(url);
        self
    }

    pub fn footer(mut self, text: impl Into<String>, icon_url: Option<String>) -> Self {
        let mut footer = serenity::CreateEmbedFooter::new(text);
        if let Some(icon) = icon_url {
            footer = footer.icon_url(icon);
        }
        self.embed = self.embed.footer(footer);
        self
    }

    pub fn build(self) -> serenity::CreateEmbed {
        self.embed
    }
}

impl Default for EmbedBuilder {
    fn default() -> Self {
        Self::new()
    }
}
