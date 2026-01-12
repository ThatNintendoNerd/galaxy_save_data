use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use galaxy_save_core::hash::{HashCodeMap, ParseLabelError};
use galaxy_save_data::save::SaveDataFile;

mod cli;

use cli::{Args, LabelEncoding, Platform};

fn read_data_write_json<P: AsRef<Path> + ToString>(
    input_path: P,
    output_path: Option<String>,
    platform: Platform,
    check: bool,
) {
    let result = match platform {
        Platform::Wii | Platform::ShieldTv => {
            if check && let Err(error) = SaveDataFile::check_be_file(&input_path) {
                eprintln!("Failed to validate save file: {error}");
                return;
            }

            SaveDataFile::read_be_file(&input_path)
        }
        Platform::Switch => {
            if check && let Err(error) = SaveDataFile::check_le_file(&input_path) {
                eprintln!("Failed to validate save file: {error}");
                return;
            }

            SaveDataFile::read_le_file(&input_path)
        }
    };

    match result {
        Ok(save_data) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_path.as_ref().with_added_extension("json"));
            let json = serde_json::to_string_pretty(&save_data).unwrap();

            if let Err(error) = fs::write(output_path, json) {
                eprintln!("Failed to write JSON file: {error}");
            }
        }
        Err(error) => eprintln!("{error}"),
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

            if let Err(error) = result {
                eprintln!("Failed to write save file: {error}");
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}

fn try_read_labels(
    labels_path: Option<String>,
    platform: Platform,
    strict: bool,
) -> Result<(), ParseLabelError> {
    let labels_path = PathBuf::from(labels_path.unwrap_or("labels.txt".into()));
    let label_map_binding = HashCodeMap::get();
    let mut label_map = label_map_binding.lock();

    match LabelEncoding::from(platform) {
        LabelEncoding::ShiftJis => label_map.read_shift_jis(labels_path)?,
        LabelEncoding::Utf8 => label_map.read_utf8(labels_path)?,
    }

    label_map.set_strict(strict);

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(error) = try_read_labels(args.labels, args.platform, args.strict) {
        eprintln!("Failed to read labels file: {error}");
    }

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
