use aoe2rec;

use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    //let test_path1 = Path::new("testdata/Silk_vs_CEOSnipezz_G2.aoe2record");
    let test_path1 = Path::new("testdata/SP Replay v101.103.28990.0 @2025.11.25 205627.aoe2record");

    let replay = aoe2rec::Savegame::from_file(test_path1)?;

    let mut i = 0;
    for op in replay.operations() {
        println!("{:?}", op);
        i += 1;
    }
    println!("Counted ops: {i}");

    Ok(())
}
