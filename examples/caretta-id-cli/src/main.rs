mod command;
mod option;

use clap::{Parser, Subcommand};

use crate::command::{DecodeArgs, EncodeArgs, RandomCommandArgs, TimestampArgs};

#[derive(Debug, Parser)]
#[command(version, about, long_about, infer_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    command: CliSubcommand,
}

impl Cli {
    pub fn run(self) {
        self.command.run()
    }
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    Decode(DecodeArgs),
    Encode(EncodeArgs),
    Timestamp(TimestampArgs),
    Random(RandomCommandArgs),
}

impl CliSubcommand {
    pub fn run(self) {
        match self {
            CliSubcommand::Decode(decode_args) => decode_args.run(),
            CliSubcommand::Encode(encode_args) => encode_args.run(),
            CliSubcommand::Timestamp(timestamp_args) => timestamp_args.run(),
            CliSubcommand::Random(random_command_args) => random_command_args.run(),
        }
    }
}

fn main() {
    let args = Cli::parse();
    args.run();
}
