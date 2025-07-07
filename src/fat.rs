use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};


fn verify_fat_internal(fat_image: &mut BufReader<File>, offset: u64) -> Result<bool, std::io::Error> {
    let mut buffer = [0; 3];

    fat_image.seek(SeekFrom::Start(offset))?;
    fat_image.read_exact(&mut buffer)?;

    // Fat magic (kind of)
    if buffer[0] != 0xEB || buffer[2] != 0x90 {
        return Ok(false);
    }

    Ok(true)
}

/// Wraps internal verify fat function turning its return into a bool
pub fn verify_fat (fat_image: &mut BufReader<File>, offset: u64) -> bool {
    if let Ok(result) = verify_fat_internal(fat_image, offset) {
        result
    } else {
        false
    }
}
