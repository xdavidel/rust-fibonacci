static ERROR_TO_SMALL: &'static str = "Error: The number must be greater than 0.";

struct Fib {
    c: u64,
    n: u64,
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib { c: 1, n: 1 }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let number = args[1].parse::<usize>();
        match number {
            Ok(res) => match res {
                0 => println!("{}", ERROR_TO_SMALL),
                1 => println!("1"),
                _ => println!("{}", fib().skip(res - 2).next().unwrap()),
            },
            _ => println!("{}", ERROR_TO_SMALL),
        }
    } else {
        println!("Usage: {} <fibonacci offset>", args[0]);
    }
}
