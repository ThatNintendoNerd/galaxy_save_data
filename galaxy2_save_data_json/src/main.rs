use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use galaxy_save_core::hash::{HashCodeMap, ParseLabelError};
use galaxy2_save_data::save::SaveDataFile;

mod cli;

use cli::Args;

fn read_data_write_json<P: AsRef<Path> + ToString>(
    input_path: P,
    output_path: Option<String>,
    check: bool,
) {
    if check && let Err(error) = SaveDataFile::check_be_file(&input_path) {
        eprintln!("Failed to validate save file: {error}");
        return;
    }

    match SaveDataFile::read_be_file(&input_path) {
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

fn read_json_write_data<P: AsRef<Path>>(input_path: P, output_path: Option<String>) {
    let json = fs::read_to_string(&input_path).unwrap();

    match serde_json::from_str::<SaveDataFile>(&json) {
        Ok(save_data) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_path.as_ref().with_extension("bin"));

            if let Err(error) = save_data.write_be_file(output_path) {
                eprintln!("Failed to write save file: {error}");
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}

fn try_read_labels(labels_path: Option<String>, strict: bool) -> Result<(), ParseLabelError> {
    let labels_path = PathBuf::from(labels_path.unwrap_or("labels.txt".into()));
    let label_map_binding = HashCodeMap::get();
    let mut label_map = label_map_binding.lock();

    label_map.read_shift_jis(labels_path)?;
    label_map.set_strict(strict);

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(error) = try_read_labels(args.labels, args.strict) {
        eprintln!("Failed to read labels file: {error}");
    }

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
