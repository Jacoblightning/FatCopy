/*
    FatCopy Copyright (C) 2025  Jacob Freeman

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod fat;

use std::fs;
use std::path::PathBuf;
use clap::Parser;
use std::io::{BufReader, BufWriter};

#[derive(Parser)]
#[command(name = "FatCopy")]
#[command(version = "1.0")]
#[command(about = "Copies files to fat filesystems.")]
#[command(long_about = None)]
struct Cli {
    /* TODO: Support multiple sources. this would require a re-work of destinations as a positional
        vec must be last or second to last. One way this could be done is combining destination and
        image into one arg with the format "image:destination"
     */

    // TODO: Copying files from FAT as well

    /// File or Directory to copy
    source: PathBuf,

    /// Where to copy the files or directories to in the image
    destination: PathBuf,

    /// The fat filesystem image or an image containing one
    image: PathBuf,

    /// Offset into the image where the filesystem is located. Accepts common suffixes
    #[arg(short, long, default_value_t = String::from("0"))]
    offset: String,

    /// Length of the filesystem. Accepts common suffixes.
    #[arg(short, long)]
    length: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let source = args.source;
    let destination = args.destination;
    let image = args.image;

    let offset = match prefix_parser::parse_prefixes(&args.offset) {
        Ok(_tmp) => {
            _tmp
        }
        Err(err) => {
            eprintln!("Failed parsing offset: {}", err);
            return;
        }
    };

    // Calculate length if it is not given
    let length;
    if let Some(_length) = args.length {
        match prefix_parser::parse_prefixes(&_length) {
            Ok(__length) => {
                length = __length;
            }
            Err(err) => {
                eprintln!("Failed parsing length: {}", err);
                return;
            }
        }
    } else {
        match fs::metadata(&image) {
            Ok(metadata) => {
                length = metadata.len() as i128
            }
            Err(err) => {
                eprintln!("Error opening image: {}", err);
                return;
            }
        }
    }

    let mut fat_file_reader;
    let mut fat_file_writer;

    match fs::File::options().read(true).write(true).open(&image) {
        Ok(_tmp) => {
            match _tmp.try_clone() {
                Ok(_tmp2) => {
                    // We now have 2 Identical file handles. _tmp and _tmp2
                    fat_file_reader = BufReader::new(_tmp);
                    fat_file_writer = BufWriter::new(_tmp2);
                }
                Err(err) => {
                    eprintln!("Failed cloning file handle: {}", err);
                    return;
                }
            }
        }
        Err(err) => {
            eprintln!("Error opening image: {}", err);
            return;
        }
    };

    if fat::verify_fat(&mut fat_file_reader, offset as u64) {

    } else {
        eprintln!("Invalid image {} at offset {}", image.display(), args.offset);
    }
}
