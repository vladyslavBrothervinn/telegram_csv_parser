use anyhow::anyhow;
use clap::*;
use std::{fs, ops::Add};
use telegram_csv_parser::*;

fn main() -> anyhow::Result<()> {
    let matches = App::new("Csv-Telegram-Parser")
        .version("0.1")
        .author("Vladyslav Bezborodov")
        .about("A simple parser of csv_telegram files")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input file you want to parse")
                .takes_value(true),
        ).arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output file")
                .takes_value(true),
        )
        .get_matches();

    let path_to_file = matches
        .value_of("file")
        .ok_or_else(|| anyhow!("No input file specified"))?;

    let output_file = matches.value_of("output").unwrap_or("default_output.txt");

    parse_csv(&path_to_file, &output_file)?;

    Ok(())
}

fn parse_csv(path: &str, output_file: &str) -> anyhow::Result<()> {
    let unparsed_file = fs::read_to_string(path)?;

    let file = CSVParser::parse(Rule::file, &unparsed_file)?
        .next()
        .ok_or_else(|| anyhow!("No pairs found"))?;

    //println!("{:?}", file);

    let mut word_count: usize = 0;
    let mut message_count: u64 = 0;

    for row in file.into_inner() {
        match row.as_rule() {
            Rule::row => {
                if row.as_str().contains("PeerUser(user_id=") {
                    // Containing "PeerUser(user_id=" means that it is the user collected the telegram data
                    message_count += 1; // and it displays the messages user sent

                    for value in row.into_inner() {
                        match value.as_rule() {
                            Rule::value => {
                                //println!("{}", value.as_str());
                                for quoted_string in value.into_inner() {
                                    match quoted_string.as_rule() {
                                        Rule::quoted_string => {
                                            //println!("{}", quoted_string.as_str());
                                            if quoted_string.as_str().len() != 2 {
                                                //println!("{}", quoted_string.as_str().len());
                                                word_count += quoted_string
                                                    .as_str()
                                                    .split_whitespace()
                                                    .count();
                                                //println!("{}",word_count);
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                            }

                            _ => unreachable!(),
                        }
                    }
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("Number of messages: {}", message_count);
    println!("Number of words from all messages: {}", word_count);

    write_to_file(output_file, (String::from("User's message count: ").add(&message_count.to_string()).add("\n")
                                                    .add("User's word count: ").add(&word_count.to_string())).as_str()).unwrap();

    Ok(())
}

fn write_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    fs::write(file_path, content)
}
