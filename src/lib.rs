use clap::{App, Arg};
use std::{
    error::Error,
    fs:: File,
    io::{self, BufRead, BufReader}
};
use std::cmp::Ordering::*;

use crate::Column::*;

enum Column<'a> {

    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str)
}

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {

    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}


pub fn run(config: Config) -> MyResult<()> {

    let file1 = &config.file1;
    let file2 = &config.file2;


    if file1=="-" && file2=="-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let case = |line: String| {

        if config.insensitive {
            line.to_lowercase()
        }
        else {
            line
        }
    };

    let print = |col: Column| {

        let mut columns = vec![];

        match col {

            Col1(val) => {

                if config.show_col1 {
                    columns.push(val);
                }
            },
            Col2(val) => {

                if config.show_col2 {

                    if config.show_col1 {

                        columns.push("");
                    }
                    columns.push(val);
                }
            },

            Col3(val) => {

                if config.show_col3 {

                    if config.show_col1 {

                        columns.push("");
                    }
                    if config.show_col2 {

                        columns.push("");
                    }

                    columns.push(val);
                }
            }
        };
        if !columns.is_empty() {

            println!("{}", columns.join(&config.delimiter));
        }

    };

    let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {

        match (&line1, &line2) {

            (Some(val1), Some(val2)) => {

                match val1.cmp(val2) {

                    Equal => {
                        print(Col3(val1));
                        line1 = lines1.next();
                        line2 = lines2.next();
                    },
                    Less => {
                        print(Col1(val1));
                        line1 = lines1.next();
                    },

                    Greater => {
                        print(Col2(val2));
                        line2 = lines2.next();
                    }
                }

                
            },
            (Some(val1), None) => {
                print(Col1(val1));
                line1 = lines1.next();
            },
            (None, Some(val2)) => {

                print(Col2(val2));
                line2 = lines2.next();
            },

            _ => ()


        }
    }

    

   
    Ok(()) 
}

fn put_in_result<'a> (line1: &'a str, line2: &'a str, config: &Config) ->  [&'a str;3]{

    let mut result:[&str;3] = ["","",""];
    if line1 < line2 && config.show_col1{
        result[0]=&line1.clone();
    }
    if line1 > line2 && config.show_col2 {
        result[1] = &line2.clone();
    }

    if line1 == line2 && config.show_col3 {

        result[2] = &line1.clone();
    }
    result

}
pub fn get_args() -> MyResult<Config> {

    let matches = App::new("commr")
                    .version("0.1.0")
                    .author("udayj")
                    .about("Rust comm")
                    .arg(

                        Arg::with_name("file1")
                            .value_name("FILE1")
                            .help("Input file 1")
                            .takes_value(true)
                            .required(true)

                    )
                    .arg(
                        Arg::with_name("file2")
                            .value_name("FILE2")
                            .help("Input file 2")
                            .takes_value(true)
                            .required(true)
                    )
                    .arg(
                        Arg::with_name("suppress_col1")
                            .short("1")
                            .help("Suppress Column 1 ")
                            .takes_value(false)
                    )
                    .arg(

                        Arg::with_name("suppress_col2")
                            .short("2")
                            .help("Suppress Column 2")
                            .takes_value(false)
                    )
                    .arg(
                        Arg::with_name("suppress_col3")
                            .short("3")
                            .help("Suppress Column 3")
                            .takes_value(false)
                    )
                    .arg(

                        Arg::with_name("insensitive")
                            .short("i")
                            .help("Case insensitive comparison")
                            .takes_value(false)
                    )
                    .arg(
                        Arg::with_name("delimiter")
                            .short("d")
                            .long("output-delimiter")
                            .takes_value(true)
                            .help("Output delimiter")
                            .default_value("\t")

                    ).get_matches();
                    
    
    Ok(

        Config {

            file1: matches.value_of("file1").unwrap().to_string(),
            file2: matches.value_of("file2").unwrap().to_string(),
            show_col1: !matches.is_present("suppress_col1"),
            show_col2: !matches.is_present("suppress_col2"),
            show_col3: !matches.is_present("suppress_col3"),
            insensitive: matches.is_present("insensitive"),
            delimiter: matches.value_of("delimiter").unwrap().to_string()

        }
    )
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {

    match filename {

        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)
                                                    .map_err(|e| format!("{}: {}", filename, e))?,
            )))
    }
}