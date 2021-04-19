use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/// Internal struct for storing key-value pairs
/// along with their hash so that we don't
/// need to recompute the hashes for all our
/// keys while expanding the hashtable
#[derive(Debug, Clone)]
struct HashNode<K, V>
where
    K: Hash + PartialEq + Clone + Debug,
    V: Clone + Debug,
{
    key: K,
    value: V,
    hash: u64,
}

/// Public facing HashTable struct containing
/// a vector of node buckets while keeping
/// track of a target max load factor as
/// well as the amount of entries
#[derive(Debug)]
pub struct HashTable<K, V>
where
    K: Hash + PartialEq + Clone + Debug,
    V: Clone + Debug,
{
    buckets: Vec<Vec<HashNode<K, V>>>,
    load_factor: f64,
    pub size: usize,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + PartialEq + Clone + Debug,
    V: Clone + Debug,
{
    /// Computes and returns the hash of
    /// a key using the stdlib default
    /// hasher
    fn hash_of(&self, key: &K) -> u64 {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish()
    }

    /// Hashes a key and mods it by the length of the buckets
    /// vector to get the index of the bucket a key should be
    /// placed in
    fn index_of(&self, key: &K) -> usize {
        (self.hash_of(key) % (self.buckets.len() as u64)) as usize
    }

    /// Initializes and returns a new HashTable
    /// with an initial capacity of 16 buckets
    /// and a target max load factor of 1.0
    pub fn new() -> Self {
        HashTable {
            buckets: vec![vec![]; 16],
            load_factor: 1.0,
            size: 0,
        }
    }

    /// Initializes and returns a new HashTable
    /// with the specified amount of buckets
    /// and target max load factor
    pub fn with(buckets: usize, load_factor: f64) -> Self {
        HashTable {
            buckets: vec![vec![]; buckets],
            load_factor,
            size: 0,
        }
    }

    /// Internal put method that accepts a HashNode instead
    /// of a regular key-value pair. The reason for implementing
    /// this separately is that when rehashing the table, we can
    /// simply pass all the old nodes into this method internally
    /// so that hashcodes don't need to be recomputed.
    fn _put(&mut self, n: HashNode<K, V>) {
        let index = (n.hash % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[index];

        for node in bucket.iter_mut() {
            if n.key == node.key {
                node.value = n.value;
                return;
            }
        }

        bucket.push(HashNode {
            key: n.key,
            value: n.value,
            hash: n.hash,
        });

        self.size += 1;
    }

    /// Inserts a key-value pair into the HashTable and
    /// returns the updated HashTable. If the target
    /// maximum load factor is surpassed, rehashing
    /// automatically occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rds::hashtable::*;
    ///
    /// let mut table = HashTable::new();
    /// table = table.put('a', 1);
    /// let retrieved_value = table.get('a');
    /// assert_eq!(retrieved_value, Some(&1))
    /// ```
    pub fn put(mut self, key: K, value: V) -> Self {
        let hash = self.hash_of(&key);

        let hash_node = HashNode { key, value, hash };

        self._put(hash_node);

        // Rehashing if the max load factor is surpassed
        if (self.size as f64 / self.buckets.len() as f64) >= self.load_factor {
            // Create a new bucket vector that's twice as large as the previous one
            let new_buckets = vec![vec![]; self.buckets.len() * 2];
            // Take ownership of all the old nodes. This is why this method
            // requires ownership of self.
            let old = self.buckets.into_iter().flat_map(Vec::into_iter);

            self.buckets = new_buckets;
            self.size = 0;

            for node in old {
                self._put(node);
            }
        }

        self
    }

    /// Retrieves an optional read-only reference to a value from
    /// the HashTable corresponding to the given key. If the key
    /// doesn't exist in the table, None is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rds::hashtable::*;
    ///
    /// let mut table = HashTable::new();
    /// table = table.put('a', 1);
    /// let retrieved_value = table.get('a');
    /// assert_eq!(retrieved_value, Some(&1));
    /// assert_eq!(table.get('b'), None);
    /// ```
    pub fn get(&self, key: K) -> Option<&V> {
        let index = self.index_of(&key);
        let bucket = &self.buckets[index];

        for n in bucket.iter() {
            if n.key == key {
                return Some(&n.value);
            }
        }

        None
    }

    /// Produces and returns a String representing the HashTable,
    /// displaying all key-value pairs.
    pub fn to_string(&self) -> String {
        let mut s = String::new();

        for n in self.buckets.iter().flatten() {
            s = format!("{}\n{:?} : {:?}", s, n.key, n.value);
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put() {
        let mut m: HashTable<char, i32> = HashTable::new();
        m = m.put('a', 1);
        println!("{:?}", m);
        assert_eq!(m.get('a'), Some(&1));
    }

    #[test]
    fn test_rehashing() {
        let mut m: HashTable<char, i32> = HashTable::new();
        let values: Vec<(i32, char)> = (1..26).into_iter().zip('a'..'z').collect();

        for (i, c) in values.into_iter() {
            m = m.put(c, i);
        }

        assert_eq!(m.size, 25);
    }
}
