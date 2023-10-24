use crate::args::{*};

pub fn process(args: PngMeArgs) {
    match args {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print(args),
    }
}

fn encode(args: EncodeArgs) {
    
}

fn decode(args: DecodeArgs) {
    
}

fn remove(args: RemoveArgs) {
    
}

fn print(args: PrintArgs) {

}