use poise::serenity_prelude as serenity;

use super::PermissionLevel;

pub async fn require_level(
    ctx: &serenity::Context,
    guild_id: serenity::GuildId,
    user_id: serenity::UserId,
    required: PermissionLevel,
) -> Result<(), String> {
    let member = guild_id.member(ctx, user_id).await.map_err(|_| {
        "could not fetch member information".to_owned()
    })?;

    let level = member_permission_level(ctx, &member, guild_id).await;

    if level >= required {
        Ok(())
    } else {
        Err("insufficient permissions for this command".to_owned())
    }
}

pub async fn member_permission_level(
    ctx: &serenity::Context,
    member: &serenity::Member,
    guild_id: serenity::GuildId,
) -> PermissionLevel {
    let guild = guild_id.to_partial_guild(ctx).await.ok();

    if let Some(ref guild) = guild {
        if member.user.id == guild.owner_id {
            return PermissionLevel::Owner;
        }
    }

    #[allow(deprecated)]
    if let Ok(perms) = member.permissions(ctx) {
        if perms.contains(serenity::Permissions::ADMINISTRATOR) {
            return PermissionLevel::Administrator;
        }
        if perms.contains(serenity::Permissions::MANAGE_GUILD) {
            return PermissionLevel::Manager;
        }
        if perms.contains(serenity::Permissions::MANAGE_WEBHOOKS) {
            return PermissionLevel::Manager;
        }
    }

    PermissionLevel::Everyone
}
