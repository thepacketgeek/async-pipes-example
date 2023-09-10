use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Phrase to output, can include {{i}} to insert the iteration number
    phrase: String,
    #[clap(long, short = 'i', default_value_t = 5)]
    iterations: usize,
    #[clap(long, short = 'w', default_value = "1s")]
    wait_interval: humantime::Duration,
}

fn main() {
    let args = Args::parse();

    for i in 0..args.iterations {
        println!("{}", args.phrase.replace("{{i}}", &i.to_string()));
        std::thread::sleep(args.wait_interval.try_into().unwrap());
    }
}
