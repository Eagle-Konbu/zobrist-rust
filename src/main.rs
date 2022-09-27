use proconio::input;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        q: usize
    };

    let mut items = a.clone();
    items.extend(b.clone());

    items.sort_unstable();
    items.dedup();

    let zobrist = Zobrist::new(items);

    let mut hash_a = vec![0; n];
    let mut set_a: HashSet<i64> = std::collections::HashSet::new();
    for (i, &a_item) in a.iter().enumerate() {
        if set_a.contains(&a_item) {
            if i > 0 {
                hash_a[i] = hash_a[i - 1];
            }
        } else {
            set_a.insert(a_item);
            if i > 0 {
                hash_a[i] = hash_a[i - 1] ^ zobrist.hash_table.get(&a_item).unwrap();
            } else {
                let &item_hash = zobrist.hash_table.get(&a_item).unwrap();
                hash_a[i] = item_hash;
            }
        }
    }

    let mut hash_b = vec![0; n];
    let mut set_b: HashSet<i64> = std::collections::HashSet::new();
    for (i, &b_item) in b.iter().enumerate() {
        if set_b.contains(&b_item) {
            if i > 0 {
                hash_b[i] = hash_b[i - 1];
            }
        } else {
            set_b.insert(b_item);
            if i > 0 {
                hash_b[i] = hash_b[i - 1] ^ zobrist.hash_table.get(&b_item).unwrap();
            } else {
                let &item_hash = zobrist.hash_table.get(&b_item).unwrap();
                hash_b[i] = item_hash;
            }
        }
    }

    for _ in 0..q {
        input! {
            x: usize,
            y: usize
        };
        if hash_a[x - 1] == hash_b[y - 1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

#[derive(Debug, Clone)]
struct Zobrist<T: std::hash::Hash + std::cmp::Eq + Copy> {
    items: Vec<T>,
    hash_table: HashMap<T, u64>,
}

impl<T: std::hash::Hash + std::cmp::Eq + Copy> Zobrist<T> {
    fn new(items: Vec<T>) -> Zobrist<T> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        let mut hash_table = HashMap::new();
        for &item in items.iter() {
            hash_table.insert(item, rng.gen::<u64>());
        }
        Zobrist { items, hash_table }
    }
}
