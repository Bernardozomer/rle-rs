use std::env;
use std::fs;
use std::process;

fn main() {
    let config;

    match Config::new(env::args().collect::<Vec<String>>().as_slice()) {
        Ok(c) => config = c,
        Err(e) => {
            let e_msg = format!(concat!(
                "invalid arguments: {}",
                "\nusage: [options] <filepath>",
                "\noptions:",
                "\n    d - decode"
            ), e);

            process::exit( bail(&e_msg));
        }
    }

    process::exit(
        match run(&config.path, config.do_encode) {
            Ok(_) => 0,
            Err(e) => { bail(&e.to_string()) }
        }
    )
}

/// Run the program.
///
/// * `path` - The path to the file.
/// * `do_encode` - Indicate whether the file should be encoded or decoded.
fn run(path: &str, do_encode: bool) -> Result<(), std::io::Error> {
    let func: fn(&[u8]) -> Vec<u8>;
    // The extension of the final file. Doesn't try to replace the previous one.
    // e.g.: encoding a .txt produces a .txt.rle, which then becomes
    // a .txt.rle.dat on decoding.
    let ext;

    if do_encode {
        func = encode;
        ext = "rle";
    } else {
        func = decode;
        ext = "dat";
    }
    
    fs::read(path)
        .map(|bytes| func(&bytes))
        .and_then(|result| {
            fs::write(format!("{}.{}", path, ext), result) }
        )
}

/// Read a byte vector and return its run-length encoding.
///
/// * `bytes` - The bytes to be encoded.
fn encode(bytes: &[u8]) -> Vec<u8> {
    let mut encoding;

    if bytes.first().is_none() {
        return vec![];
    } else {
        encoding = vec![*bytes.first().unwrap()];
    }

    let mut occurrences = 1;
    
    for byte in bytes.iter().skip(1) {
        if byte == encoding.last().unwrap() && occurrences < 255 {
            occurrences += 1;
        } else {
            encoding.extend(&[occurrences, *byte]);
            occurrences = 1;
        }
    }

    encoding.push(occurrences);

    encoding
} 

/// Read a run-length encoding and return its decoded contents.
///
/// * `bytes` - The bytes to be decoded.
fn decode(bytes: &[u8]) -> Vec<u8> {
    let mut decoding = Vec::<u8>::new();

    for (i, byte) in bytes.iter().enumerate() {
        if i % 2 != 0 {
            continue;
        }

        // Repeat bytes[i], bytes[i+1] times in a row.
        // e.g.: "!!" equals to 33 times "!" ("!" value in ASCII).
        for _j in 0..bytes[i+1] {
            decoding.push(*byte)
        }
    }

    decoding
}

/// Print an error message to stderr and return 1.
///
/// * `msg` - The message that will be printed.
fn bail(msg: &str) -> i32 {
    eprintln!("rle-rs: error: {}", msg);
    1
}

/// Hold configuration information needed for the program to run.
///
/// * `do_encode` - Wheter the file shall be encoded (true) or decoded (false).
/// * `path` - The filepath.
struct Config {
    do_encode: bool,
    path: String,
}

impl Config {
    /// Create a new Config struct based on user input.
    ///
    /// * `args` - The command-line arguments used to create the struct.
    ///     Usage: [options] <filepath>
    ///     Options:
    ///         d - decode
    fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 2 {
            return Err("no argument was specified")
        }

        if args[1] == "d" {
            if args.get(1).is_none() {
                return Err("no filepath was specified")
            }

            return Ok(Self { do_encode: false, path: args[2].clone() })
        }

        Ok(Self { do_encode: true, path: args[1].clone() })
    }
}
