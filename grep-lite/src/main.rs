// grep very-lite

use regex::Regex;

fn main() {
    let context_lines = 1;
    let search_term = "oo";
    let quote = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";
  
    
    println!("{}", grep(quote, search_term, context_lines));
}

fn grep<'a>(
    source_text: &'a str,
    search_term: &'a str,
    context_size: usize
    ) -> String {

    let re = Regex::new(search_term).unwrap();
    let mut matching_lines: Vec<usize> = Vec::new();
    let mut output = String::new();

    // First loop: Identify matching lines
    for (line_num, text) in source_text.lines().enumerate() {
        match re.find(text) {
            Some(_) => matching_lines.push(line_num),
            None => (),
        }
    }

    if matching_lines.is_empty() {
        return "".to_string()
    }

    // Second Loop: Grab content from source_text for output
    for (line_num, text) in source_text.lines().enumerate() {
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