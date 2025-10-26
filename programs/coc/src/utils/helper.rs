// All the utility functions that support the main logic of your program is here

use anchor_lang::prelude::*;

/// Helper function to check if a user has a specific permission
/// This is a placeholder - implement with remaining_accounts when needed
#[allow(dead_code)]
pub fn has_permission(_user_roles: &[Pubkey], _user: &Pubkey, _action: &str) -> bool {
    // For now, return false - only default_admin can perform actions
    false
}
