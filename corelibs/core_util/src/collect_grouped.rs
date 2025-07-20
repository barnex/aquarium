use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Extension trait to collect `(K, V)` items into `HashMap<K, Vec<V>>`.
///
/// ```
/// # use core_util::*;
/// use std::collections::HashMap;
/// let items = [("bob", 1), ("bob", 2), ("alice", 3)];
/// let grouped: HashMap<_, _> = items.into_iter().collect_grouped();
/// assert_eq!(grouped.get("bob"), Some(&vec![1, 2]));
/// assert_eq!(grouped.get("alice"), Some(&vec![3]));
///
/// ```
pub trait CollectGrouped {
	type Key;
	type Value;
	fn collect_grouped<H>(self) -> HashMap<Self::Key, Vec<Self::Value>, H>
	where
		H: BuildHasher + Default;
}

impl<I, K, V> CollectGrouped for I
where
	I: Iterator<Item = (K, V)>,
	K: Eq + Hash,
{
	type Key = K;
	type Value = V;

	fn collect_grouped<H>(self) -> HashMap<Self::Key, Vec<Self::Value>, H>
	where
		H: BuildHasher + Default,
	{
		let mut result = HashMap::<K, Vec<V>, H>::default();
		for (k, v) in self {
			result.entry(k).or_default().push(v);
		}
		result
	}
}
