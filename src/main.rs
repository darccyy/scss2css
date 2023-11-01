use std::{fs, path::Path};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value = "css")]
    pub input: String,

    #[arg(short, long, default_value = "scss")]
    pub out_dir: String,

    #[arg(short, long)]
    pub minify: bool,
    // #[arg(short, long)]
    // pub watch: bool,
}

fn main() {
    let args = Args::parse();

    let in_path = Path::new(&args.input);
    let out_dir = Path::new(&args.out_dir);

    let options = grass::Options::default().style(if args.minify {
        grass::OutputStyle::Compressed
    } else {
        grass::OutputStyle::Expanded
    });

    if in_path.is_dir() {
        let dir = fs::read_dir(in_path).expect("Failed to read directory");
        for child in dir {
            let child = child.expect("Failed to read child of directory");
            let child_path = child.path();
            let child_path = Path::new(&child_path);

            if child_path.is_dir() {
                unimplemented!("recursive compile");
            }
            convert_file(child_path, &out_dir, &options);
        }
    } else {
        convert_file(in_path, &out_dir, &options);
    }
}

fn convert_file(file: &Path, out_dir: &Path, options: &grass::Options) {
    let filename = get_filename(&file).expect("Failed to read css file name");
    let out_dir_str = out_dir
        .to_str()
        .expect("Failed to read output directory name");
    let out_file = format!("{}/{}.css", out_dir_str, filename);

    let css = fs::read_to_string(file).expect("Failed to read css file");
    let scss = grass::from_path(css, options).expect("Failed to convert");
    fs::write(out_file, scss).expect("Failed to write output file");
}

fn get_filename(path: &Path) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}
