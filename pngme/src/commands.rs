use std::{str::FromStr, path, fs::File, io::Write};

use crate::{args::{*}, png::Png, chunk::{Chunk, self}, chunk_type::{ChunkType, self}};

pub fn process(args: PngMeArgs) {
    match args {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print(args),
    }
}

fn encode(args: EncodeArgs) {
    let png = Png::from_file(args.path);

    if let Ok(mut png) = png {
        let chunk = png.chunk_by_type(&args.chunk);

        if let Some(chunk) = chunk {
            chunk.set_data(args.message.as_bytes().to_vec());
        }
        else {
            png.append_chunk(Chunk::new(
                ChunkType::from_str(&args.chunk).unwrap(),
                args.message.as_bytes().to_vec(),
            ));
        }

        if let Some(output_file) = args.output_file {
            let file = File::create(output_file);
    
            if let Ok(mut file) = file {
                file.write_all(&png.as_bytes());
            }
            
        }

    }

    
}

fn decode(args: DecodeArgs) {
    let png = Png::from_file(args.path);

    if let Ok(mut png) = png {
        let chunk = png.chunk_by_type(&args.chunk);
        if let Some(chunk) = chunk {
            println!("Message is: {:?}", chunk.data_as_string().unwrap());
        }
        else {
            println!("Chunk not found");
        }
    }
}

fn remove(args: RemoveArgs) {
    let png = Png::from_file(args.path);

    if let Ok(mut png) = png {
        png.remove_chunk(&args.chunk);
    }

}

fn print(args: PrintArgs) {
    let png = Png::from_file(args.path);

    if let Ok(png) = png {
        for chunk in png.chunks() {
            println!("{}", chunk.chunk_type());
        }
    }

}