# data_to_avi

### Just a simple CLI tool to convert files to avi videos and be able to convert it back to the original file.

## Building

```sh
git clone https://github.com/BrewTheFox/data_to_avi.git
cd ./data_to_avi
cargo build
```

## Usage

### Help

```
Usage: data_to_avi [OPTIONS] --optype <OPERATION TYPE> --path <FILE>
Options:
  -o, --optype <OPERATION TYPE>  Operation type E(ncode)/D(ecode)
  -p, --path <FILE>              Path to the file to process
      --output <FOLDER>          Output folder [default: ./]
  -h, --help                     Print help

```

### Example Commands

- Decode Video 
    ```sh 
    data_to_avi -o D --path myvideo.avi
    ```

- Encode Video 
    ```sh 
    data_to_avi -o E --path file.zip
    ```

### Example Output (Converted to GIF so can't be decoded)

![video](./resources/output.gif)
