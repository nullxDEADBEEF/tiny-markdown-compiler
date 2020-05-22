use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, Error};

fn parse_markdown_file(filename: &str) -> Result<(), Error> {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);

    // create a path variable based on filename
    let input_filename = Path::new(filename);

    // try to open file
    let file = File::open(&input_filename)?;

    let mut ptag: bool = false; // keep track of paragraph tag
    let mut htag: bool = false; // keep track of h1 tag

    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line?;
        let mut first_char: Vec<char> = line_content.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if ptag {
                    ptag = false;
                    output_line.push_str("</p>\n");
                }

                if htag {
                    htag = false;
                    output_line.push_str("</h1>\n");
                }

                htag = true;
                output_line.push_str("<h1>");
                output_line.push_str(&line_content[2..]); // get all but first two characters
            },
            _ => {
                if !ptag {
                    ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&line_content);
            }
        };
        if ptag {
            ptag = false;
            output_line.push_str("</p>\n");
        }

        if htag {
            htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n".to_string() {
            tokens.push(output_line);
        }
    }
    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");
    let mut output_file = File::create(output_filename)?;
    for line in &tokens {
        output_file.write_all(line.as_bytes())?;
    }
    println!("[ INFO ] Parsing complete!");
    Ok(())
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    let the_version = env!("CARGO_PKG_VERSION");
    let the_description = env!("CARGO_PKG_DESCRIPTION");
    the_title.push_str(" (v");
    the_title.push_str(the_version);
    the_title.push_str("), ");
    the_title.push_str(the_description);
    the_title
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Usage: tinymd <somefile>.md");
}

fn usage() {
}

fn main() -> Result<(), Error> {
    // collect command-line arguments
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1])?,
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            print_long_banner();
        }
    }

    Ok(())
}