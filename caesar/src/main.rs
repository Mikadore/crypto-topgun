#![feature(io_read_to_string)]
use caesar::{caesar_decode, caesar_encode, cz_caesar_decode, cz_caesar_encode, caesar_rot};
use clap::{Arg, ArgGroup, Command};

fn main() {
    let matches = Command::new("caesar")
        .about("caesar is a cli utility for encoding/decoding text using the caesar cipher")
        .arg(
            Arg::new("Key")
                .long("key")
                .short('k')
                .help("The shift amount for the caesar cipher")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("Czech")
                .long("czech")
                .short('c')
                .help("Use the czech alphabet"),
        )
        .arg(
            Arg::new("Input File")
                .long("in")
                .short('i')
                .help("Use file as input")
                .takes_value(true),
        )
        .arg(
            Arg::new("Output File")
                .long("out")
                .short('o')
                .help("Write output to file instead of stdout")
                .takes_value(true),
        )
        .arg(
            Arg::new("Decode")
                .long("decode")
                .short('d')
                .help("Decode the input"),
        )
        .arg(
            Arg::new("Encode")
                .long("encode")
                .short('e')
                .help("Encode the input")
        )
        .arg(
            Arg::new("Binary")
                .long("binary")
                .short('b')
                .help("Shift the input's bits by {Key}")
                .conflicts_with("Czech")
        )
        .group(
            ArgGroup::new("Mode")
                .args(&["Decode", "Encode", "Binary"])
                .required(true),
        )
        .after_help(
            "The program encodes/decodes valid utf8 using the caesar cipher.\
             You need to provide a key, i.e. the amount to shift by.\
             By default, the program reads from stdin and writes to stdout.\
             Any non-alphabetic character will not be changed by either decoding or encoding.\
             Lowercase and uppercase characters are encoded separately, i.e. with key = 1,\
             'A' becomes 'B' and 'a' becomes 'b'.\
             Use -c to enable the czech alphabet. This will use czech letters in the order 'AÁBCČDĎEÉĚFGHIÍJKLMNŇOÓPQRŘSŠTŤUÚŮVWXYÝZŽ'\
             (as defined in unicode, same order for lowercase), exluding the letter 'ch'.\n\n\
             Alternatively, use -b to shift the inputs bytes by the amount given with -k")
        .get_matches();

    let key = matches.value_of("Key").unwrap().parse::<u32>().expect("Key must be a positive number");
    let key = (key % 26) as u8;
        
    if matches.is_present("Binary") {
        use std::io::{Read, Write};
        let input = if let Some(file) = matches.value_of("Input File") {
            std::fs::read(file).expect("Couldn't read inout file")
        } else {
            let mut v = vec![];
            std::io::stdin()
                .read_to_end(&mut v)
                .expect("I/O error reading stdin");
            v
        };

        let out = caesar_rot(&input, key);

        if let Some(outfile) = matches.value_of("Output File") {
            std::fs::write(outfile, &out).expect("Error writing output file");
        } else {
            std::io::stdout()
                .write_all(&out)
                .expect("I/O error writing to stdout");
        }

        std::process::exit(0);
    }

    let input = if let Some(file) = matches.value_of("Input File") {
        std::fs::read_to_string(file).expect("Couldn't read input file")
    } else {
        std::io::read_to_string(std::io::stdin()).expect("I/O error reading stdin")
    };

    let function = if matches.is_present("Decode") {
        if matches.is_present("Czech") {
            cz_caesar_decode
        } else {
            caesar_decode
        }
    } else {
        if matches.is_present("Czech") {
            cz_caesar_encode
        } else {
            caesar_encode
        }
    };

    let output = function(&input, key);

    if let Some(file) = matches.value_of("Output File") {
        std::fs::write(file, output).expect("Couldn't write output file")
    } else {
        print!("{}", output)
    }
}
