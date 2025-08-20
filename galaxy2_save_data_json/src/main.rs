use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use galaxy2_save_data::save::SaveDataFile;

mod cli;

use cli::Args;

fn read_data_write_json<P: AsRef<Path> + ToString>(
    input_path: P,
    output_path: Option<String>,
    check: bool,
) {
    if check && let Err(error) = SaveDataFile::check_be_file(&input_path) {
        eprintln!("failed to validate file: {error}");
        return;
    }

    match SaveDataFile::read_be_file(&input_path) {
        Ok(save_data) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(input_path.to_string() + ".json"));
            let json = serde_json::to_string_pretty(&save_data).unwrap();

            fs::write(output_path, json).expect("failed to write JSON file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn read_json_write_data<P: AsRef<Path>>(input_path: P, output_path: Option<String>) {
    let json = fs::read_to_string(&input_path).unwrap();

    match serde_json::from_str::<SaveDataFile>(&json) {
        Ok(save_data) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_path.as_ref().with_extension("bin"));

            save_data
                .write_be_file(output_path)
                .expect("failed to write GameData.bin file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn main() {
    let args = Args::parse();

    match Path::new(&args.input)
        .extension()
        .expect("input file path should contain an extension")
        .to_str()
        .unwrap()
    {
        "json" => read_json_write_data(args.input, args.output),
        _ => read_data_write_json(args.input, args.output, !args.force),
    }
}
