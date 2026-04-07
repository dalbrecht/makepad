use crate::makepad_live_id::LiveId;

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut, Index, IndexMut},
};

// Previously used HashMap with a custom zero-cost hasher (ValueHasher).
// Replaced with BTreeMap to work around an LLVM wasm32 codegen bug that
// miscompiles HashMap::insert, silently dropping keys during Makepad's
// DSL evaluation. BTreeMap uses comparison (Ord) instead of hashing,
// avoiding the buggy code path entirely. Performance is equivalent for
// the small maps used here (typically 3-57 entries).
// See: https://github.com/dalbrecht/makepad/issues/6

#[derive(Clone, Debug)]
pub struct ValueMap<K, V> {
    map: BTreeMap<K, V>,
}

impl<K, V> Default for ValueMap<K, V>
where
    K: Ord + Copy + From<LiveId> + std::fmt::Debug,
{
    fn default() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl<K, V> Deref for ValueMap<K, V>
where
    K: Ord + Copy + From<LiveId>,
{
    type Target = BTreeMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K, V> DerefMut for ValueMap<K, V>
where
    K: Ord + Copy + From<LiveId>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<K, V> Index<K> for ValueMap<K, V>
where
    K: Ord + Copy + From<LiveId>,
{
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        self.map.get(&index).unwrap()
    }
}

impl<K, V> IndexMut<K> for ValueMap<K, V>
where
    K: Ord + Copy + From<LiveId>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.map.get_mut(&index).unwrap()
    }
}
