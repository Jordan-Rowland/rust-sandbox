use std::env;

#[derive(Debug)]
struct CoinSort {
    q: u8,
    d: u8,
    n: u8,
    p: u8,
}

impl CoinSort {
    fn new() -> Self {
        CoinSort {
            q: 0,
            d: 0,
            n: 0,
            p: 0,
        }
    }

    fn sort(&mut self, total: u8) {
        let mut total = total;
        while total > 0 {
            if total / 25 >= 1 {
                self.q += 1;
                total -= 25;
            } else if total / 10 >= 1 {
                self.d += 1;
                total -= 10;
            } else if total / 5 >= 1 {
                self.n += 1;
                total -= 5;
            } else if total / 1 >= 1 {
                self.p += 1;
                total -= 1;
            }
        }
        println!("{:?}", self);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    CoinSort::new().sort(args[1].parse().unwrap());
}
