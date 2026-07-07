use super::PermissionLevel;

pub const COMMAND_PERMISSIONS: &[(&str, PermissionLevel)] = &[
    ("setup", PermissionLevel::Administrator),
    ("repository", PermissionLevel::Manager),
    ("provider", PermissionLevel::Manager),
    ("channel", PermissionLevel::Manager),
    ("events", PermissionLevel::Manager),
    ("filters", PermissionLevel::Manager),
    ("templates", PermissionLevel::Manager),
    ("permissions", PermissionLevel::Administrator),
    ("reload", PermissionLevel::Administrator),
    ("test", PermissionLevel::Manager),
    ("status", PermissionLevel::Everyone),
    ("stats", PermissionLevel::Everyone),
    ("ping", PermissionLevel::Everyone),
    ("help", PermissionLevel::Everyone),
    ("about", PermissionLevel::Everyone),
    ("preview", PermissionLevel::Manager),
];
