use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input PAF file
    #[clap(short, long)]
    input: String,

    /// Calculate coverage based on total overlap
    #[clap(short, long)]
    overlap: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.input).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut coverage_map: HashMap<String, Vec<u32>> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let (target_name, target_length, start, end, cigar) = parse_paf_line(&line);

        let coverage_vector = coverage_map.entry(target_name).or_insert(vec![0; target_length]);

        if args.overlap {
            update_coverage_vector_total_overlap(coverage_vector, start, end);
        } else {
            update_coverage_vector(coverage_vector, &cigar, start);
        }
    }

    // Output in BED format
    let mut output = io::stdout();
    for (target, coverage) in coverage_map {
        for (pos, count) in coverage.iter().enumerate() {
            writeln!(output, "{}\t{}\t{}\t{}", target, pos, pos + 1, count).expect("Unable to write to output");
        }
    }
}

fn parse_paf_line(line: &str) -> (String, usize, usize, usize, String) {
    let fields: Vec<&str> = line.split('\t').collect();
    let target_name = fields[5].to_string();
    let target_length: usize = fields[6].parse().expect("Invalid target length");
    let start: usize = fields[7].parse().unwrap();
    let end: usize = fields[8].parse().unwrap();
    let cigar = fields.iter().find(|&f| f.starts_with("cg:z:")).expect("CIGAR string not found").split(':').nth(2).unwrap().to_string();

    (target_name, target_length, start, end, cigar)
}

fn update_coverage_vector(coverage_vector: &mut Vec<u32>, cigar: &str, start_pos: usize) {
    let cigar_regex = Regex::new(r"(\d+)([MX=])").unwrap();
    let mut pos = start_pos;

    for cap in cigar_regex.captures_iter(cigar) {
        let length: usize = cap[1].parse().unwrap();
        let op = &cap[2];

        if op == "M" || op == "=" {
            for i in pos..(pos + length) {
                if i < coverage_vector.len() {
                    coverage_vector[i] += 1;
                }
            }
        }
        pos += length;
    }
}

fn update_coverage_vector_total_overlap(coverage_vector: &mut Vec<u32>, start: usize, end: usize) {
    for i in start..end {
        if i < coverage_vector.len() {
            coverage_vector[i] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paf_line() {
        let line = "query1\t100\t0\t100\t+\ttarget1\t1000\t100\t200\tcg:z:10M10X10=\tmore\tfields";
        let (target_name, target_length, start, end, cigar) = parse_paf_line(line);
        assert_eq!(target_name, "target1");
        assert_eq!(target_length, 1000);
        assert_eq!(start, 100);
        assert_eq!(end, 200);
        assert_eq!(cigar, "10M10X10=");
    }

    #[test]
    fn test_update_coverage_vector() {
        let mut coverage = vec![0; 300];
        update_coverage_vector(&mut coverage, "10M10X10=", 100);
        for i in 100..109 {
            assert_eq!(coverage[i], 1);
        }
        for i in 110..119 {
            assert_eq!(coverage[i], 0);
        }
        for i in 120..130 {
            assert_eq!(coverage[i], 1);
        }
    }

    #[test]
    fn test_update_coverage_vector_total_overlap() {
        let mut coverage = vec![0; 300];
        update_coverage_vector_total_overlap(&mut coverage, 100, 200);
        for i in 100..200 {
            assert_eq!(coverage[i], 1);
        }
        for i in 200..300 {
            assert_eq!(coverage[i], 0);
        }
    }
}


