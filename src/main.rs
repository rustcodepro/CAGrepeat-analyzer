mod args;
mod cag;
mod plotcag;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::cag::caganalyzer;
use crate::plotcag::cagplotmatch;
use clap::Parser;
use figlet_rs::FIGfont;

/*
 Author Gaurav Sablok,
 Email: codeprog@icloud.com
 Date: 2025-8-29
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("CAGRepeat");
    println!("{}", repgenerate.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::CAGRepeat {
            filepath,
            outputfile,
        } => {
            let command = caganalyzer(filepath, outputfile).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::CAGPlot {
            filepath,
            outputfile,
        } => {
            let command = cagplotmatch(filepath, outputfile).unwrap();
            println!("The command has completed: {}", command);
        }
    }
}
