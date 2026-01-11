use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use galaxy_save_data::save::SaveDataFile;

mod cli;

use cli::{Args, Platform};

fn read_data_write_json<P: AsRef<Path> + ToString>(
    input_path: P,
    output_path: Option<String>,
    platform: Platform,
    check: bool,
) {
    let result = match platform {
        Platform::Wii | Platform::ShieldTv => {
            if check && let Err(error) = SaveDataFile::check_be_file(&input_path) {
                eprintln!("failed to validate file: {error}");
                return;
            }

            SaveDataFile::read_be_file(&input_path)
        }
        Platform::Switch => {
            if check && let Err(error) = SaveDataFile::check_le_file(&input_path) {
                eprintln!("failed to validate file: {error}");
                return;
            }

            SaveDataFile::read_le_file(&input_path)
        }
    };

    match result {
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

fn read_json_write_data<P: AsRef<Path>>(
    input_path: P,
    output_path: Option<String>,
    platform: Platform,
) {
    let json = fs::read_to_string(&input_path).unwrap();

    match serde_json::from_str::<SaveDataFile>(&json) {
        Ok(save_data) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_path.as_ref().with_extension("bin"));
            let result = match platform {
                Platform::Wii | Platform::ShieldTv => save_data.write_be_file(output_path),
                Platform::Switch => save_data.write_le_file(output_path),
            };

            result.expect("failed to write GameData.bin file");
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
        "json" => read_json_write_data(args.input, args.output, args.platform),
        _ => read_data_write_json(args.input, args.output, args.platform, !args.force),
    }
}
