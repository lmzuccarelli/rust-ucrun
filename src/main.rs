use crate::commands::{create, delete, start};
use crate::runtime::crun::*;
use anyhow::{bail, Result};
use clap::Parser;
use std::ffi::OsStr;

mod commands;
mod runtime;

#[derive(Parser, Debug)]
#[clap(no_binary_name = true)]
#[command(name = "rust-ucrun")]
#[command(author = "Luigi Mario Zuccarelli <luzuccar@redhat.com>")]
#[command(version = "0.0.1")]
#[command(about = "OCI runtime for ops nanvm ukernel", long_about = None)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    global: liboci_cli::GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

// Adapted from https://github.com/containers/youki/blob/main/crates/youki/src/main.rs
#[derive(Parser, Debug)]
enum Command {
    #[clap(flatten)]
    Standard(Box<liboci_cli::StandardCmd>),

    #[clap(flatten)]
    Common(Box<liboci_cli::CommonCmd>),
}

pub fn entry_point(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let raw_args = args
        .into_iter()
        .map(|a| a.as_ref().to_os_string())
        .collect::<Vec<_>>();

    let parsed_args = Args::parse_from(&raw_args);

    match parsed_args.command {
        Command::Standard(cmd) => {
            match *cmd {
                liboci_cli::StandardCmd::Create(args) => create::create(&args, &raw_args),
                liboci_cli::StandardCmd::Delete(args) => delete::delete(&args, &raw_args),
                liboci_cli::StandardCmd::Start(args) => start::start(&args, &raw_args),
                liboci_cli::StandardCmd::State(_) | liboci_cli::StandardCmd::Kill(_) => {
                    // not a command we implement ourselves, pass it on to crun
                    println!("passing command to crun {:#?}", raw_args);
                    crun(&raw_args)
                }
            }
        }
        Command::Common(cmd) => {
            match *cmd {
                liboci_cli::CommonCmd::Exec(_)
                | liboci_cli::CommonCmd::Checkpointt(_)
                | liboci_cli::CommonCmd::Events(_)
                | liboci_cli::CommonCmd::Features(_)
                | liboci_cli::CommonCmd::List(_)
                | liboci_cli::CommonCmd::Pause(_)
                | liboci_cli::CommonCmd::Ps(_)
                | liboci_cli::CommonCmd::Resume(_)
                | liboci_cli::CommonCmd::Run(_)
                | liboci_cli::CommonCmd::Update(_)
                | liboci_cli::CommonCmd::Spec(_) => {
                    // not a command we support
                    bail!("unknown command")
                }
            }
        }
    }
}

fn main() {
    if let Err(e) = entry_point(std::env::args_os().skip(1)) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
