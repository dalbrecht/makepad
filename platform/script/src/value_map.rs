use crate::makepad_live_id::LiveId;

use std::ops::{Index, IndexMut};

// Replaced HashMap with Vec to work around an LLVM wasm32 codegen bug that
// miscompiles both HashMap::insert and BTreeMap internal navigation, silently
// dropping keys during Makepad's DSL evaluation. Vec + linear scan is immune
// because it uses only array indexing and equality comparison — operations too
// simple for LLVM to miscompile. Performance is equivalent for the small maps
// used here (typically 3-57 entries).
// See: https://github.com/dalbrecht/makepad/issues/6

#[derive(Clone, Debug)]
pub struct ValueMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> Default for ValueMap<K, V>
where
    K: Eq + Copy + From<LiveId> + std::fmt::Debug,
{
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl<K: Eq + Copy, V> ValueMap<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        for (k, v) in &self.entries {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for (k, v) in &mut self.entries {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        for (k, v) in &mut self.entries {
            if *k == key {
                let old = std::mem::replace(v, value);
                return Some(old);
            }
        }
        self.entries.push((key, value));
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
            Some(self.entries.swap_remove(pos).1)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.entries.iter_mut().map(|(k, v)| (&*k, v))
    }

    pub fn entry(&mut self, key: K) -> VecMapEntry<'_, K, V> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| *k == key) {
            VecMapEntry::Occupied(OccupiedEntry {
                entries: &mut self.entries,
                index: pos,
            })
        } else {
            VecMapEntry::Vacant(VacantEntry {
                entries: &mut self.entries,
                key,
            })
        }
    }
}

pub enum VecMapEntry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub struct OccupiedEntry<'a, K, V> {
    entries: &'a mut Vec<(K, V)>,
    index: usize,
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.entries[self.index].1
    }

    pub fn remove(self) -> V {
        self.entries.swap_remove(self.index).1
    }
}

pub struct VacantEntry<'a, K, V> {
    entries: &'a mut Vec<(K, V)>,
    key: K,
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V {
        self.entries.push((self.key, value));
        let len = self.entries.len();
        &mut self.entries[len - 1].1
    }
}

impl<K, V> Index<K> for ValueMap<K, V>
where
    K: Eq + Copy + From<LiveId>,
{
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<K, V> IndexMut<K> for ValueMap<K, V>
where
    K: Eq + Copy + From<LiveId>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}
