use super::*;
pub(crate) mod token_admin;
pub(crate) mod token_requester;

pub fn default_max_count() -> Option<u64> {
    Some(5)
}
