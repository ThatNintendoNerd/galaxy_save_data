# galaxy2_save_data_json

A command-line program for creating and editing `GameData.bin` files from Super Mario Galaxy 2 using JSON. Drag and drop a `GameData.bin` file onto the executable to create a JSON file. Drag and drop a properly structured JSON file onto the executable to create a `GameData.bin` file. JSON files are text files, so they can be viewed and edited in any text editor.

Sample output from a `GameData.bin` file:

```json
{
  "user_file_info": [
    {
      "name": "user1",
      "user_file": {
        "GameData": [
          {
            "PlayerStatus": {
              "player_left": 4,
              "stocked_star_piece_num": 0,
              "stocked_coin_num": 0,
              "last_1up_coin_num": 0,
              "flag": {
                "player_luigi": false
              }
            }
          },
```

## Usage

The latest version is available in [Releases](https://github.com/ThatNintendoNerd/galaxy_save_data/releases/latest).

For more information, run the following command:

```
galaxy2_save_data_json --help
```

### Conversion

```
galaxy2_save_data_json <input> [output]
```

```
galaxy2_save_data_json GameData.bin GameData.json
galaxy2_save_data_json GameData.json GameData.bin
```

### Skipping Validation

By default, galaxy2_save_data_json will perform rudimentary checks on the header of the `GameData.bin` file to verify the saved data is not corrupt. To disable these checks, pass the `--force` option to the program.

```
galaxy2_save_data_json GameData.bin GameData.json -f
galaxy2_save_data_json GameData.bin GameData.json --force
```
