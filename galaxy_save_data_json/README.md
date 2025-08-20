# galaxy_save_data_json

A command-line program for creating and editing `GameData.bin` files from Super Mario Galaxy using JSON. Drag and drop a `GameData.bin` file onto the executable to create a JSON file. Drag and drop a properly structured JSON file onto the executable to create a `GameData.bin` file. JSON files are text files, so they can be viewed and edited in any text editor.

Sample output from a `GameData.bin` file:

```json
{
  "user_file_info": [
    {
      "name": "mario1",
      "user_file": {
        "GameData": [
          {
            "PlayerStatus": {
              "story_progress": 0,
              "stocked_star_piece_num": 0,
              "player_left": 4
            }
          },
```

## Usage

The latest version is available in [Releases](https://github.com/ThatNintendoNerd/galaxy_save_data/releases/latest).

For more information, run the following command:

```
galaxy_save_data_json --help
```

### Conversion

By default, galaxy_save_data_json assumes the source or target platform as the Wii.

```
galaxy_save_data_json <input> [output]
galaxy_save_data_json <input> [output] -p wii
galaxy_save_data_json <input> [output] --platform wii
```

```
galaxy_save_data_json GameData.bin GameData.json
galaxy_save_data_json GameData.json GameData.bin
```

```
galaxy_save_data_json GameData.bin GameData.json -p wii
galaxy_save_data_json GameData.json GameData.bin --platform wii
```

Conversion for NVIDIA Shield TV or Nintendo Switch will require specifying the platform.

```
galaxy_save_data_json <input> [output] -p shield-tv
galaxy_save_data_json <input> [output] --platform switch
```

```
galaxy_save_data_json GameData.bin GameData.json -p shield-tv
galaxy_save_data_json GameData.json GameData.bin --platform switch
```

### Skipping Validation

By default, galaxy_save_data_json will perform rudimentary checks on the header of the `GameData.bin` file to verify the saved data is not corrupt. To disable these checks, pass the `--force` option to the program.

```
galaxy_save_data_json GameData.bin GameData.json -f
galaxy_save_data_json GameData.bin GameData.json --force
```
