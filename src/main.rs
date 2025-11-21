use base64::{Engine, engine::GeneralPurpose};
use base64::{engine, prelude::BASE64_STANDARD};
use clap::Parser;
use std::path::PathBuf;
use std::process::exit;
use windows_dpapi::{Scope, decrypte_data, encrypt_data};

#[derive(Parser, Debug)]
#[command(version, about, long_about = "decrypt dpapi stuff, but rusty!")]
struct Args {
    #[arg(short, long)]
    blob: Option<String>,

    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long)]
    outfile: Option<PathBuf>,
}

fn decrypt_blob(blob: String) {
    let data = engine::general_purpose::STANDARD.decode(blob);
    let decyrpt_res = decrypte_data(&data, Scope::User);
    if decyrpt_res
}

fn decyrpt_file(input: PathBuf, output: PathBuf) {}

fn main() {
    let args = Args::parse();
    if args.blob.is_none() && args.file.is_none() {
        eprintln!("we need either a blob or a file path to decrypt dummy!");
        eprintln!("try again...");
        exit(1);
    }
    if args.blob.is_some() {
        decrypt_blob(args.blob.unwrap());
    } else if args.file.is_some() {
        if args.outfile.is_none() {
            eprintln!("we need an output file to decyrpt a file dummy!");
            eprintln!("try again...");
            exit(1);
        }
        decyrpt_file(args.file.unwrap(), args.outfile.unwrap());
    }
}
