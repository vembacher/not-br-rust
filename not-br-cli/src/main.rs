use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Read, stdin, stdout, Write};
use clap::{Arg, Command};
use not_br_lib::not_br;
use not_br_lib::not_br::OutputType;

fn main() {
    let m = Command::new("notbr-cli")
        .author("Victor Embacher, victor@embacher.xyz")
        .version("0.1.0")
        .about("CLI for notbr.")
        .arg(
            Arg::new("input")
                .help("File which should be used as input.")
                .long("input")
                .short('i')
                .takes_value(true)
        )
        .arg(
            Arg::new("output")
                .help("File which should be used as output")
                .long("output")
                .short('o')
                .takes_value(true)
        )
        .arg(
            Arg::new("frequency")
                .help("Frequency of bolded words.")
                .long("frequency")
                .short('f')
                .takes_value(true)
        )
        .arg(
            Arg::new("bold_percentage")
                .help("Percentage of word to be bolded.")
                .long("bold_percentage")
                .short('b')
                .takes_value(true)
        )
        .arg(
            Arg::new("output_type")
                .help("Which kind of output should be produced.")
                .long("output_type")
                .short('t')
                .short('t')
                .takes_value(true)
                .required(true)
                .possible_values(["HTML", "html", "Markdown", "markdown", "md"])
        )
        .arg(
            Arg::new("out_file")
        )
        .get_matches();

    let reader: Box<dyn BufRead> = match m.value_of("input") {
        None => Box::new(BufReader::new(stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };
    let writer: Box<dyn Write> = match m.value_of("output") {
        None => { Box::new(BufWriter::new(stdout())) }
        Some(filename) => Box::new(BufWriter::new(
            OpenOptions::new().write(true)
                .create_new(true)
                .open(filename)
                .unwrap()
        ))
    };

    let frequency = m.value_of("frequency").map_or(1, |f| f.parse::<u64>().unwrap());
    let bold_percentage = m.value_of("bold_percentage").map_or(50, |f| f.parse::<u64>().unwrap());
    if bold_percentage > 100 {
        eprintln!("bold_percentage has to be in range [0,100].")
    }
    let output_type = m.value_of("output_type").unwrap().parse::<OutputType>().unwrap();

    process_cli(reader, writer, frequency, (bold_percentage as f64) / 100., output_type);
}

fn process_cli(mut input: Box<dyn BufRead>, mut output: Box<dyn Write>, frequency: u64, bold_percentage: f64, output_type: OutputType) {
    loop {
        let mut buf = String::new();
        match input.read_to_string( &mut buf) {
            Ok(size) => {
                if size == 0 { break; }
                // let partial_input = String::from_utf8(buf).unwrap();
                match not_br::process(buf.as_str(), frequency, bold_percentage, output_type) {
                    Ok(output_partial) => {
                        output.write(output_partial.as_bytes()).expect("Could not write output.");
                    }
                    Err(_) => {}
                }
            }
            Err(err) => {
                eprintln!("{err}")
            }
        }
    }
}