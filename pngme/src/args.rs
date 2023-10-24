pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug)]
pub struct EncodeArgs {
    pub path: String,
    pub chunk: String,
    pub message: String,
    pub output_file: Option<String>
}

#[derive(Debug)]
pub struct DecodeArgs {
    pub path: String,
    pub chunk: String,
}

#[derive(Debug)]
pub struct RemoveArgs {
    pub path: String,
    pub chunk: String,
}

#[derive(Debug)]
pub struct PrintArgs {
    pub path: String,
}