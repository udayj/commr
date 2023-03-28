use clap::{App, Arg};
use std::{
    error::Error,
    fs:: File,
    io::{self, BufRead, BufReader}
};

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
    let _file1=open(file1)?;
    let _file2 = open(file2)?;

    println!("Opened {} and {}", file1, file2);

    let mut lines1 = _file1.lines();
    let mut lines2 = _file2.lines();

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();
    loop {

        let mut result: [&str;3] = ["","",""];
        let temp: String;
        
        if line1.as_ref().is_none() && line2.as_ref().is_none() {
            break;
        }
        match line1 {

            None => {

                let temp:String = line2.unwrap().unwrap();
                result[1] = &temp;
                line2 = lines2.next();
                if config.show_col2 {
                    println!("{}", result.join(&config.delimiter));
                    
                }
                continue;
            },
            Some(val1) =>  {

                if line2.is_some() {

                    let actual_val2 = String::from("");
                    let actual_val1 = val1.unwrap();
                    //let new_result:[&str;3] = put_in_result(&actual_val1.as_str(), &actual_val2.as_str(), &config);
                    if actual_val1 < actual_val2 {

                        if config.show_col1{
                            temp = actual_val1.clone();
                            result[0]=&temp;
                            
                        }
                        line1 = lines1.next();

                    }
                    else if actual_val1 > actual_val2 {

                        if config.show_col2 {

                            temp = actual_val2.clone();
                            result[1] = &temp;
                            
                        }
                        line2 = lines2.next();
                    }
                
                    else {

                        if config.show_col3 {
                
                            temp = actual_val1.clone();
                            result[2] = &temp;
                           
                        }
                        line1 = lines1.next();
                        line2 = lines2.next();
                    }
                    println!("{}", result.join(&config.delimiter));
                    
                    continue;

                } 
                else {

                    let temp: String = val1.unwrap();
                    result[0] = &temp;
                    line1 = lines1.next();
                    if config.show_col1 {
                        
                        println!("{}", result.join(&config.delimiter));
                       
                    }
                    continue;
                }
            }
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