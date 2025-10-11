use plotpy::{Barplot, Plot};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
 Author Gaurav Sablok,
 Email: codeprog@icloud.com
 Date: 2025-10-11
*/

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FastaStruct {
    id: String,
    seq: String,
}

#[derive(Debug)]
pub struct FastaRecord {
    pub id: String,
    pub sequence: String,
}

pub fn read_fasta<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, FastaRecord>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut records = HashMap::new();
    let mut current_id = String::new();
    let mut current_sequence = String::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !current_id.is_empty() {
                records.insert(
                    current_id.clone(),
                    FastaRecord {
                        id: current_id.clone(),
                        sequence: current_sequence.clone(),
                    },
                );
                current_sequence.clear();
            }
            current_id = line[1..].to_string();
        } else {
            current_sequence.push_str(&line);
        }
    }

    if !current_id.is_empty() {
        records.insert(
            current_id.clone(),
            FastaRecord {
                id: current_id,
                sequence: current_sequence,
            },
        );
    }

    Ok(records)
}

#[tokio::main]
pub async fn cagplotmatch(filepath: &str, outputfile: &str) -> Result<String, Box<dyn Error>> {
    let fasta_records = read_fasta(filepath)?;
    let mut genomevec: Vec<FastaStruct> = Vec::new();
    for (_id, record) in fasta_records {
        genomevec.push(FastaStruct {
            id: record.id.clone().to_string(),
            seq: record.sequence.clone(),
        })
    }
    let mut tokensize: Vec<(String, Vec<&str>)> = Vec::new();

    for i in genomevec.iter() {
        let seqvec: Vec<&str> = i
            .seq
            .as_bytes()
            .windows(3)
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        let vecinput: (String, Vec<&str>) = (i.id.clone(), seqvec);
        tokensize.push(vecinput);
    }

    let mut genomevec_vec: Vec<(String, usize)> = Vec::new();
    for i in tokensize.iter() {
        let id: String = i.0.clone();
        let seq: Vec<&str> = i.1.clone();
        let mut count: usize = 0usize;
        for i in seq.iter() {
            if *i == "CAG" {
                count += 1
            }
        }
        let unitvector: (String, usize) = (id, count);
        genomevec_vec.push(unitvector);
    }

    let mut plottingnames: Vec<&str> = Vec::new();
    let mut plottingvalues: Vec<_> = Vec::new();

    for i in genomevec_vec.iter() {
        plottingnames.push(i.0.as_str());
        plottingvalues.push(i.1);
    }

    let mut bar = Barplot::new();
    bar.set_horizontal(true)
        .set_with_text("edge")
        .draw_with_str(&plottingnames.as_slice(), &plottingvalues);

    let mut plot = Plot::new();
    plot.set_inv_y()
        .add(&bar)
        .set_title("CAG Frequency")
        .set_label_x("Variant Count");

    let outputname = format!("{}.{}", outputfile, "svg");
    plot.save(&outputname)?;

    Ok("The cag repeats have been analyzed".to_string())
}
