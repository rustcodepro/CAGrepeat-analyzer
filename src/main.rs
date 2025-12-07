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
Gaurav Sablok,
codeprog@icloud.com
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
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = caganalyzer(filepath, outputfile).unwrap();
                println!("The command has been finished:{}", command);
            });
        }
        Commands::CAGPlot {
            filepath,
            outputfile,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = cagplotmatch(filepath, outputfile).unwrap();
                println!("The command has completed: {}", command);
            });
        }
    }
}
