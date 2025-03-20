use hashbrown::HashSet;
use std::hash::Hash;

pub fn remove_random<T>(set: &mut HashSet<T>) -> Option<T>
where
    T: Eq + PartialEq + Hash,
{
    // Found on the internet
    if set.is_empty() {
        return None;
    }
    // If load factor is under 25%, shrink to fit.
    // We need a high load factor to ensure that the sampling succeeds in a reasonable time,
    // and the table doesn't rebalance on removals.
    // Insertions can only cause the load factor to reach as low as 50%,
    // so it's safe to shrink at 25%.
    if set.capacity() >= 8 && set.len() < set.capacity() / 4 {
        set.shrink_to_fit();
    }
    let raw_table = set.raw_table_mut();
    let num_buckets = raw_table.buckets();
    // Perform rejection sampling: Pick a random bucket, check if it's full,
    // repeat until a full bucket is found.
    loop {
        let bucket_index = fastrand::usize(0..num_buckets);
        // Safety: bucket_index is less than the number of buckets.
        // Note that we return the first time we modify the table,
        // so raw_table.buckets() never changes.
        // Also, the table has been allocated, because set is a HashSet.
        unsafe {
            if raw_table.is_bucket_full(bucket_index) {
                let bucket = raw_table.bucket(bucket_index);
                let ((element, ()), _insert_slot) = raw_table.remove(bucket);
                return Some(element);
            }
        }
    }
}
