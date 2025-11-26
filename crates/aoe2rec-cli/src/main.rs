use aoe2rec;

use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let test_path1 = Path::new("testdata/Silk_vs_CEOSnipezz_G2.aoe2record");
    let replay = aoe2rec::Savegame::from_file(test_path1)?;

    Ok(())
}
