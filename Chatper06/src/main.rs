use std::path::PathBuf;

use rstat::errors::StatsError;
use rstat::rstats::get_summary_src_stats;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rstat",
    about = "This is a tool to generate statistics on Rust projects"
)]
struct Opt {
    #[structopt(name = "source directory", parse(from_os_str))]
    in_dir: PathBuf,
    #[structopt(name = "mode", short)]
    mode: String,
}
fn main() -> Result<(), StatsError> {
    let opt = Opt::from_args();
    let mode = &opt.mode[..];
    match mode {
        "src" => {
            let stats = get_summary_src_stats(&opt.in_dir)?;
            println!("Summary stats: {:?}", stats);
        }
        _ => println!("Sorry, no stats"),
    }
    Ok(())
}
