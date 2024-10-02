use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short = 'e', long = "encode")]
    encode: bool,
    #[arg(short = 'd', long = "decode")]
    decode: bool,
    #[arg(short = 'i', long = "info")]
    info: bool,

    message: String,
}

pub fn main() {
    let args: Args = Args::parse();
    if args.encode && args.decode {
        eprintln!("You can't encode and decode at the same time");
        std::process::exit(1);
    }
    println!("Message:\t{}", args.message);
}
