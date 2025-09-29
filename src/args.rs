use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "CAGrepeat",
    version = "1.0",
    about = "CAG repeat pattern.
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// CAG repeat pattern
    CAGRepeat {
        /// path to the sequence file
        filepath: String,
        /// path to the output file
        outputfile: String,
    },
}
