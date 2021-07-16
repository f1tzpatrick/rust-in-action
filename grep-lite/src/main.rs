// grep very-lite

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use regex::Regex;
use clap::{App, Arg};

fn main() {

    let args = App::new("grep-lite")
        .version("0.1.0")
        .about("searches for terms in text")
        .arg(Arg::with_name("search-term")
            .help("A String pattern to search for")
            .long("search-term")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("Text to search through")
            .long("input")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("context-size")
            .help("The number of lines to display above and below a match")
            .short("C")
            .takes_value(true))
        .get_matches();

    let search_term = match args.value_of("search-term") {
        Some(input) => input,
        None => panic!("You need to provide a search term")
    };

    let input = match args.value_of("input") {
        Some(input) => input,
        None => panic!("You need to provide some text to search")
    };

    let context_lines = match args.value_of("context-size") {
        Some(input) => match input.parse::<usize>() {
            Ok(n) => n,
            _ => panic!("Invalid input for arg -C (Context size should be an integer")
        },
        None => 0,
    };

    println!("{}", grep(input, search_term, context_lines));
}

fn grep<'a>(
    source_text: &'a str,
    search_term: &'a str,
    context_size: usize
    ) -> String {
    
    let re = Regex::new(search_term).unwrap();
    let mut matching_lines: Vec<usize> = Vec::new();
    let mut output = String::new();
    
    // Create a mutable BufReader and iterate over it by reference, to support
    // reuse of the underlying File object
    let source_text = File::open(source_text).unwrap();
    let mut reader = BufReader::new(source_text);

    // First loop: Identify matching lines
    for (line_num, text) in reader.by_ref().lines().enumerate() {
        let text = text.unwrap();
        match re.find(&text) {
            Some(_) => matching_lines.push(line_num),
            None => (),
        }
    }

    if matching_lines.is_empty() {
        return "".to_string()
    }

    // At this point the reader has read through to the end of the file
    // In order to read it again, we must move the cursor back to the start
    reader.seek(SeekFrom::Start(0)).unwrap();

    // Second Loop: Grab content from source_text for output
    for (line_num, text) in reader.by_ref().lines().enumerate() {
        let text = text.unwrap();
        for target in matching_lines.iter() {
            
            let lower_bound = target.saturating_sub(context_size);
            let upper_bound = target + context_size;

            // if line_num is in the context bounds of the matched line
            // add the line to output
            if (line_num >= lower_bound) && (line_num <= upper_bound) {
                output += &text;
                output.push('\n');
            }
        }
    }
    output
}