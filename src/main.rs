use clap::{Parser, command};
use rand::{Rng, seq::SliceRandom};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of lottery to be generated.
    #[arg(default_value_t = 10)]
    n_lottos: u8,

    /// Number of lottery digits.
    #[arg(short, long, default_value_t = 6)]
    n_digits: u8,
}

fn main() {
    let args = Args::parse();

    let mut rng = rand::rng();
    let mut all_possible_nums = (0..(10_u32.pow(args.n_digits.into()))).collect::<Vec<u32>>();

    all_possible_nums.shuffle(&mut rng);

    let mut lotto_nums = all_possible_nums[0..(args.n_lottos.into())]
        .to_vec()
        .iter()
        .map(|val| format!("{val:0width$}", width = args.n_digits.into()))
        .collect::<Vec<String>>();
    lotto_nums.sort();

    println!("{}", lotto_nums.join("\n"))
}
