use anyhow::ensure;
use rustc_hash::FxHashMap;

use crate::{Analytics, Key, Storage, Value};

pub struct KeyStore<const N: usize>(FxHashMap<[Key; N], Value>);

impl<const N: usize> KeyStore<N> {
    pub fn new() -> Self {
        Self(FxHashMap::default())
    }
}

impl<const N: usize> Default for KeyStore<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Storage<N> for KeyStore<N> {
    fn increment(&mut self, key: [String; N]) -> anyhow::Result<()> {
        let len = 2usize.pow(N as u32);

        for mask in 0..len {
            let key = apply_mask(&key, mask)?;
            let value = self.0.entry(key).or_insert(Value(0));
            value.0 += 1;
        }

        Ok(())
    }

    fn get(&self, key: [Key; N]) -> anyhow::Result<Value> {
        self.0
            .get(&key)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("key not found"))
    }
}

impl<const N: usize> Analytics for KeyStore<N> {
    fn space_occupied(&self) -> usize {
        let mut bytes = 0;
        for (key, _) in self.0.iter() {
            bytes += std::mem::size_of::<Value>();

            for k in key.iter() {
                bytes += match k {
                    Key::Wildcard => std::mem::size_of::<Key>(),
                    Key::String(s) => s.len(),
                };
            }
        }

        bytes
    }
}

fn apply_mask<const N: usize>(key: &[String; N], mask: usize) -> anyhow::Result<[Key; N]> {
    let mut result = [const { Key::Wildcard }; N];

    let max_mask = 2usize.pow(N as u32) - 1;

    ensure!(
        mask <= max_mask,
        "mask is too large: max_mask={},mask={}",
        max_mask,
        mask
    );

    for i in 0..N {
        if mask & (1 << i) == 0 {
            result[i] = Key::String(key[i].clone());
        }
    }

    Ok(result)
}
