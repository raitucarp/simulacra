//! Which thoughts surface as sparks: older thoughts of the active network, a different
//! handful each day, never the newest ones the feed already shows at its top.

use crate::domain::{NetworkId, Post};

/// The newest thoughts of a network are on screen in the feed already.
const SKIP_NEWEST: usize = 6;
/// FNV-1a offset basis and prime: a stable, dependency-free hash for the daily shuffle.
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;

/// Up to `count` thoughts of `network` from `posts`, shuffled by `day` (same day, same picks).
/// Instant reposts carry no thought of their own and never surface.
pub fn pick_sparks<'a>(posts: &'a [Post], network: &NetworkId, day: i64, count: usize) -> Vec<&'a Post> {
    let mut pool: Vec<&Post> = posts.iter().filter(|post| &post.network_id == network && !post.is_instant_repost()).collect();
    pool.sort_by_key(|post| std::cmp::Reverse(post.created_at));
    if pool.len() > SKIP_NEWEST + count {
        pool.drain(..SKIP_NEWEST);
    }
    pool.sort_by_key(|post| shuffle_key(post.id.as_str(), day));
    pool.truncate(count);
    pool
}

fn shuffle_key(id: &str, day: i64) -> u64 {
    id.bytes().chain(day.to_le_bytes()).fold(FNV_OFFSET, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME))
}
