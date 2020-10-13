extern crate regex;

#[macro_use]
extern crate clap;

#[path = "gfa/gfa.rs"]
mod gfa; 
#[path = "gfa2/gfa2.rs"]
mod gfa2;

#[path = "gfa/parser.rs"]
mod parser_gfa; 
#[path = "gfa2/parser.rs"]
mod parser_gfa2;

#[path = "gfa/test.rs"]
mod test_gfa; 
#[path = "gfa2/test.rs"]
mod test_gfa2; 

fn main() {

    // REMEMBER! ./target/debug/rs-gfa2 --help
    // USAGE! ./target/debug/rs-gfa2 <INPUT> <FORMAT>

    // use clap as a macro to handle the argument passed as parameters via command line
   let matches = clap_app!(rs_gfa2 =>
    (version: "1.0")
    (author: "Stievano Matteo <m.stievano1@campus.unimib.it>")
    (about: "The point of this project is to extend the rs-gfa library developed by \n\
        Christian Fischer, so to make possible the use of the parser either on \n\
        GFA files and GFA2 files.\n")
    (@arg INPUT: +required "Set the input file to read.")
    (@arg FORMAT: +required "Set the right file format in order to use the parser.\n\
        > gfa: tells the parser that the file passed as INPUT argument \n\
            it's accordant to the GFA1 format\n\
        > gfa2: tells the parser that the file passed as INPUT argument \n\
            it's accordant to the GFA2 format")).get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let file = matches.value_of("INPUT").unwrap();

    let filename = std::path::Path::new(file.clone()).file_name().unwrap();

    match matches.value_of("FORMAT").unwrap() {
        "gfa" => {
            println!("Checking the file: {:?}", filename);
            
            let gfa = parser_gfa::parse_gfa(&std::path::PathBuf::from(file));
            match gfa {
                // TODO: FIX ME SENPAI! I AM NEVER USED :( 
                None => println!("Error parsing the file {:?} as a GFA1 file", filename),
                Some(_g) => println!("Success! the file {:?} is accordant to the GFA1 format", filename),
            }
        },
        "gfa2" => {
            println!("Checking the file: {:?}", filename);
            
            let gfa = parser_gfa2::parse_gfa(&std::path::PathBuf::from(file));
            match gfa {
                // TODO: FIX ME SENPAI! I AM NEVER USED :(
                None => println!("Error parsing the file {:?} as a GFA2 file", filename),
                Some(_g) => println!("Success! the file {:?} is accordant to the GFA2 format", filename),
            }
        },
        _ => eprintln!("Error! the argument passed as FORMAT is not valid\n\
                        Run application.exe --help or application.exe -h to show how to use \
                        the application properly\n"),
    }
}