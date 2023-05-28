use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::io::Error;

fn main() -> Result<(), Error> {
    let  mut signals = Signals::new(&[SIGTERM, SIGINT])?;
    'signal_loop: loop {
        // Pick up signals that arrived since last time
        for signal in signals.pending() {
            match signal {
                SIGINT => {
                    println!("Received signal SIGINT");
                }
                SIGTERM => {
                    println!("Received signal SIGTERM");
                    break 'signal_loop;
                }
                _ => unreachable!(),
            }
        }
    }
    println!("Terminating program");
    Ok(())
}
