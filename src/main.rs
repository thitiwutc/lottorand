use std::{
    collections::HashMap,
    io::{self, IsTerminal},
};

use clap::{Parser, command};
use rand::{Rng, seq::SliceRandom};
use terminal_size::{Width, terminal_size};

const MAXIMUM_LOTTO_NUM_PER_LINE: u8 = 25;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// Number of lottery to be generated.
    #[arg(default_value_t = 10)]
    n_lottos: u8,

    /// Number of lottery digits.
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..=19))]
    n_digits: u8,
}

fn main() {
    let args = Args::parse();

    let mut rng = rand::rng();
    let mut lotto_nums: Vec<String>;

    // For smaller n_digits use Fisher–Yates shuffle
    if args.n_digits <= 6 {
        let mut all_possible_nums = (0..(10_u32.pow(args.n_digits.into()))).collect::<Vec<u32>>();

        all_possible_nums.shuffle(&mut rng);

        lotto_nums = all_possible_nums[0..(args.n_lottos.into())]
            .to_vec()
            .iter()
            .map(|val| format!("{val:0width$}", width = args.n_digits.into()))
            .collect::<Vec<String>>();
    } else {
        // For bigger n_digits use hash map.
        let mut generated: HashMap<u64, bool> = HashMap::new();

        while generated.len() < args.n_lottos.into() {
            let rand_num = rng.random_range(0..=(10_u64.pow(args.n_digits.into())));
            generated.insert(rand_num, true);
        }

        lotto_nums = generated
            .keys()
            .map(|val| format!("{val:0width$}", width = args.n_digits.into()))
            .collect::<Vec<String>>();
    }

    lotto_nums.sort();

    let stdout = io::stdout();
    if stdout.is_terminal()
        && let Some((Width(w), _)) = terminal_size()
    {
        // Calculate number of lotto num per line but not greater than the maximum.
        let items_per_line =
            ((Into::<f32>::into(w / Into::<u16>::into(args.n_digits)) * 0.6).ceil() as usize)
                .min(MAXIMUM_LOTTO_NUM_PER_LINE.into());

        // Create string with capacity of lottery numbers, spaces, and newlines.
        let mut sb = String::with_capacity(
            Into::<usize>::into(args.n_lottos * args.n_digits) // Lotto number count.
                + Into::<usize>::into(args.n_lottos - 1), // Space count and new line.
        );

        for (i, lotto_num) in lotto_nums.iter().enumerate() {
            if i > 0 {
                // Add newline to before the first lotto num of each line.
                if i % items_per_line == 0 {
                    sb.push('\n');
                } else {
                    // Add space after each lotto num.
                    sb.push(' ');
                }
            }

            sb.push_str(&lotto_num);
        }

        println!("{sb}")
    } else {
        println!("{}", lotto_nums.join("\n"))
    }
}
