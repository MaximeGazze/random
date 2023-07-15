use clap::Parser;
use clio::Input;
use rand::seq::SliceRandom;

/// Simple program to query a recipe bank file
#[derive(Parser)]
struct Args {
    /// File to read from
    #[clap(short, long, value_parser, default_value = "-")]
    file: Input,

    /// Delemiter to use when parsing
    #[clap(short, long, value_parser, default_value = ",")]
    delimiter: String,
}

fn slice(buffer: &str, delimiter: &str) -> Vec<String> {
    buffer
        .replace(['\n', '\r'], "")
        .split(delimiter)
        .map(|it| it.trim().to_string())
        .collect()
}

fn get_random_elem<T>(elems: &[T]) -> &T {
    elems
        .choose(&mut rand::thread_rng())
        .expect("unable to choose a random element")
}

fn main() {
    let mut args = Args::parse();
    let mut buffer = String::new();

    if args.file.is_std() {
        args.file
            .lock()
            .read_line(&mut buffer)
            .expect("unable to read file");
    } else {
        args.file
            .lock()
            .read_to_string(&mut buffer)
            .expect("unable to read file");
    }

    let elems = slice(&buffer, &args.delimiter);
    let rand_elem = get_random_elem(&elems);

    println!("{rand_elem}");
}
