use clap::{crate_authors, crate_name, crate_version, ArgAction, Parser};
use tabular::{row, Table};

mod algo;
mod file;
mod hash;
mod trans;

use algo::HashAlgo;

#[derive(Parser)]
#[command(
    name = crate_name!(),
    author = crate_authors!(", "),
    version = crate_version!(),
)]
/// Compute hashes of file with alternate line endings
struct Cli {
    /// File path to transform and compute
    #[arg(
        action = ArgAction::Set,
        num_args = 1,
        value_name = "file path",
    )]
    file_path: String,

    /// The hashing algorithm to use for the resulting hash
    #[clap(value_enum)]
    #[arg(
        long = "hash",
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "hashing algorithm",
        default_value_t = HashAlgo::Sha256,
    )]
    hash: HashAlgo,
}

fn main() {
    let cli = Cli::parse();

    let path = file::Path::from(&cli.file_path);

    if !path.validity.exists {
        eprintln!("[ERROR: File does not exist]");
        std::process::exit(1)
    }

    let mut table = Table::new("{:>}  {:<}");

    for transformation in &path.trans {
        let hash = hash::get_hash_with_transformation(&path, transformation, &cli.hash);
        table.add_row(row!(hash, transformation));
    }

    print!("{table}");

    std::process::exit(0);
}
