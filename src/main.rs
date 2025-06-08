use clap::Parser;
mod encode;
mod decode;

#[derive(Parser, Debug)]
#[command(about = "Simple binary file to video encoder")]
struct Params {
    #[arg(short, long, value_name = "OPERATION TYPE")]
    /// Operation type E(ncode)/D(ecode)
    optype: String,
    #[arg(short, long, value_name = "FILE")]
    /// Path to the file to process
    path: std::path::PathBuf,
    #[arg(long, value_name = "FOLDER", default_value="./")]
    /// Output folder
    output: std::path::PathBuf
}

fn main() {
    let args: Params = Params::parse();
    let optype: String = args.optype.to_uppercase();
    let path: std::path::PathBuf = args.path;
    let output: std::path::PathBuf = args.output;
    if optype != "D" && optype != "E" {
        println!("Operation type must be E or D, not {optype}.");
        return;
    }
    if !path.exists() {
        println!("The path doesn't exist");
        return;
    }
    if !path.is_file(){
        println!("The file doesn't exist");
        return;
    }

    if !output.is_dir() {
        println!("The output isn't a directory.")
    }

    if optype  == "E"{
        encode::encode(path.clone(), output.clone());
    }
    
    if optype  == "D"{
        decode::decode(path.clone(), output.clone());
    }
}
