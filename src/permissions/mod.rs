pub mod checks;
pub mod levels;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermissionLevel {
    Everyone,
    Contributor,
    Manager,
    Administrator,
    Owner,
}

impl PermissionLevel {
    pub fn requires_administrator(self) -> bool {
        self >= Self::Administrator
    }

    pub fn requires_manager(self) -> bool {
        self >= Self::Manager
    }
}
