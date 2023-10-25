use std::path::PathBuf;
use clap::{Parser, Arg};

use clap::{arg, command, value_parser, ArgAction, Command};

use crate::args::{*};
use crate::commands::process;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let matches = command!()
    .subcommand(
        Command::new("encode")
        .about("Encodes a message in a png")
        .arg(Arg::new("path").required(true))
        .arg(Arg::new("chunk").required(true))
        .arg(Arg::new("message").required(true))
        .arg(Arg::new("output_file"))
    )
    .subcommand(
        Command::new("decode")
        .about("Decodes a message in a png")
        .arg(Arg::new("path").required(true))
        .arg(Arg::new("chunk").required(true))
    )
    .subcommand(
        Command::new("remove")
        .about("Removes a chunk in a png")
        .arg(Arg::new("path").required(true))
        .arg(Arg::new("chunk").required(true))
    )
    .subcommand(
        Command::new("print")
        .about("Prints a png")
        .arg(Arg::new("path").required(true))
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("encode") {
        let path = matches.get_one::<String>("path")
        .expect("'path' is required and parsing will fail if its missing")
        .clone();

        let chunk = matches.get_one::<String>("chunk")
        .expect("'chunk' is required and parsing will fail if its missing")
        .clone();
    
        let message = matches.get_one::<String>("message")
        .expect("'message' is required and parsing will fail if its missing")
        .clone();

        let output_file = matches.get_one::<String>("output_file");

        let encode_args = EncodeArgs {
            path: path,
            chunk: chunk,
            message: message,
            output_file: match output_file {
                Some(file) => Some(file.clone()),
                None => None,
            }
        };

        println!("{:?}", encode_args);

        process(PngMeArgs::Encode(encode_args));
        return Ok(());

    }

    if let Some(matches) = matches.subcommand_matches("decode") {
        let path = matches.get_one::<String>("path")
        .expect("'path' is required and parsing will fail if its missing")
        .clone();

        let chunk = matches.get_one::<String>("chunk")
        .expect("'chunk' is required and parsing will fail if its missing")
        .clone();

        let decode_args = DecodeArgs {
            path: path,
            chunk: chunk,
        };

        println!("{:?}", decode_args);

        process(PngMeArgs::Decode(decode_args));
        return Ok(());

    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let path = matches.get_one::<String>("path")
        .expect("'path' is required and parsing will fail if its missing")
        .clone();

        let chunk = matches.get_one::<String>("chunk")
        .expect("'chunk' is required and parsing will fail if its missing")
        .clone();

        let remove_args = RemoveArgs {
            path: path,
            chunk: chunk,
        };

        println!("{:?}", remove_args);

        process(PngMeArgs::Remove(remove_args));
        return Ok(());

    }

    if let Some(matches) = matches.subcommand_matches("print") {
        let path = matches.get_one::<String>("path")
        .expect("'path' is required and parsing will fail if its missing")
        .clone();

        let print_args = PrintArgs {
            path: path,
        };

        println!("{:?}", print_args);

        process(PngMeArgs::Print(print_args));
        return Ok(());

    }

    Ok(())
}
