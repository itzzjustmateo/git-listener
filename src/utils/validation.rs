pub fn validate_url(s: &str) -> bool {
    url::Url::parse(s).is_ok()
}

pub fn validate_discord_channel_id(s: &str) -> bool {
    s.parse::<u64>().is_ok()
}

pub fn validate_webhook_secret(secret: &str) -> Result<(), String> {
    if secret.len() < 16 {
        return Err("webhook secret must be at least 16 characters".to_owned());
    }
    if secret.len() > 256 {
        return Err("webhook secret must be at most 256 characters".to_owned());
    }
    Ok(())
}
