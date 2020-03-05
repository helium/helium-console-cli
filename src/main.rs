use std::process;
use structopt::StructOpt;
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(StructOpt, Debug)]
enum DeviceCmd {
    List,
    Get,
    Post {
        name: String,
    },
}

/// Interact with Helium API via CLI
#[derive(Debug, StructOpt)]
enum Cli {
    /// Get wallet information
    Device {
        #[structopt(subcommand)] 
        cmd: DeviceCmd,
    },

}
fn main() {
    let cli = Cli::from_args();
    if let Err(e) = run(cli) {
        println!("error: {}", e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result {
    match cli {
        Cli::Device { cmd } => {

            match cmd {
                DeviceCmd::List => {

                },
                DeviceCmd::Get => {
                }
                DeviceCmd::Post { name } => {
                    println!("{:?}", name);
                }
            }
            Ok(())
        }
    }
}
