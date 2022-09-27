use rand::prelude::*;

fn main() {
    let zobrist = Zobrist::new(vec![1, 2, 3, 4]);
    println!("{:?}", zobrist);
}

#[derive(Debug, Clone)]
struct Zobrist<T> {
    items: Vec<T>,
    hashes: Vec<u64>,
}

impl<T> Zobrist<T> {
    fn new(items: Vec<T>) -> Zobrist<T> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        let hashes = (0..items.len())
            .map(|_| rng.gen::<u64>())
            .collect::<Vec<u64>>();
        Zobrist { items, hashes }
    }
}
