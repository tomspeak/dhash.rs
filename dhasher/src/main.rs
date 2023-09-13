use std::vec;

use clap::Parser;
use dhash;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_parser, value_delimiter = ' ', num_args = 1..=50)]
    files: Vec<String>,

    #[arg(short, long)]
    pretty: bool,
}

#[derive(Clone)]
struct HashResult {
    name: String,
    hash: u64,
}

struct HashDistanceResult {
    hash: HashResult,
    distance: u32,
}

fn main() {
    let args = Cli::parse();

    let mut hashes = vec![];
    for file in args.files {
        hashes.push(HashResult {
            name: file.to_string(),
            hash: dhash::from_file(&file),
        });
    }

    let base_hash = hashes.first().expect("base_hash does not exist");
    let base_hash_binary_string = format!("{:b}", base_hash.hash);

    let mut hash_distances = vec![];

    if hashes.len() > 1 {
        for hash in hashes.iter().skip(1) {
            hash_distances.push(HashDistanceResult {
                hash: hash.clone(),
                distance: dhash::calculate_distance(base_hash.hash, hash.hash),
            })
        }
    } else {
        if args.pretty {
            println!("{:?}", pretty_print(&base_hash_binary_string));
        }
        println!("dHash: {}", base_hash.hash);
        return;
    }

    println!("{:<22} :: {}", base_hash.name, base_hash.hash);
    for hds in hash_distances {
        println!(
            "{:<22} :: {} :: {}",
            hds.hash.name, hds.hash.hash, hds.distance
        );
    }
}

fn pretty_print(s: &String) {
    for (i, c) in s.chars().enumerate() {
        print!("{}", c);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }
}
