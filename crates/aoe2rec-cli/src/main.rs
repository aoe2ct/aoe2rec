use aoe2rec::Savegame;
use std::error::Error;
use std::path::Path;
use tracing::{debug, error, info};

use std::io::Read;

fn process_replay_zip(zip_path: &Path) -> Result<(), Box<dyn Error>> {
    let zip_file = std::fs::File::open(zip_path)?;
    let mut zip_archive = zip::ZipArchive::new(zip_file)?;
    for file_number in 0..zip_archive.len() {
        let mut archive_entry = zip_archive.by_index(file_number)?;

        let entry_name = match archive_entry.enclosed_name() {
            Some(path) => path,
            None => {
                debug!("skipping entry {:?}", archive_entry.name());
                continue;
            }
        };
        let extension = match entry_name.extension() {
            Some(extension) => extension,
            None => {
                continue;
            }
        };
        if extension != "aoe2record" {
            debug!("skipping entry {:?}", entry_name);
            continue;
        }

        if !archive_entry.is_file() {
            continue;
        }

        /*let replay_data = match archive_entry {
            Some(data) => data,
            None => continue,
        };*/

        let mut buffer = Vec::new();
        archive_entry.read_to_end(&mut buffer)?;

        if buffer.ends_with(b"DUMMYREC") {
            debug!("skipping dummy rec");
            continue;
        }

        let replay = match aoe2rec::Savegame::from_slice(&buffer) {
            Ok(replay) => replay,
            Err(_err) => {
                error!("failed to parse {:?}", entry_name);
                continue;
            }
        };

        process_replay(&replay);
    }

    Ok(())
}

fn process_replay(replay: &Savegame) {
    let mut i = 0;
    for op in replay.operations() {
        //println!("{:?}", op);
        i += 1;
    }
    println!("Counted ops: {i}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    //let test_path1 = Path::new("testdata/Silk_vs_CEOSnipezz_G2.aoe2record");
    //let test_path1 = Path::new("testdata/SP Replay v101.103.28990.0 @2025.11.25 205627.aoe2record");

    /*let test_path1 = Path::new("testdata2/Frozen-HotPepperPizza_vs_Mavinga_G1.aoe2record");
    let replay = Savegame::from_file(test_path1)?;
    process_replay(&replay);*/

    let recs_dir = Path::new("recs");
    for entry in std::fs::read_dir(recs_dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = match path.extension() {
            Some(extension) => extension,
            None => {
                continue;
            }
        };
        if extension != "zip" {
            debug!("skipping file {:?}", path);
            continue;
        }
        info!("processing archive {:?}", path);

        match process_replay_zip(&path) {
            Ok(_) => {}
            Err(err) => {
                error!("error processing zip {:?}", err);
            }
        }
    }

    Ok(())
}
