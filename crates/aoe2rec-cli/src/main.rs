use aoe2rec::Savegame;
use std::path::Path;
use std::{error::Error, path::PathBuf};
use tracing::{debug, error, info, warn};
use zip::ZipArchive;

use clap::Parser;
use std::io::Read;

#[derive(Debug, Parser)]
#[clap(name = "aoe2rec-tool", version)]
pub struct App {
    //#[clap(flatten)]
    //global_opts: GlobalOpts,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Unpack directory of zip archives and filter dummy recs.
    UnpackDirectory {
        /// Input directory
        #[clap(long, short = 'i')]
        input_directory: String,

        /// Input directory
        #[clap(long, short = 'o')]
        output_directory: String,
    },
}

fn process_replay_zip(zip_path: &Path) -> Result<(), Box<dyn Error>> {
    let zip_file = std::fs::File::open(zip_path)?;
    let mut zip_archive = zip::ZipArchive::new(zip_file)?;
    for (name, buffer) in replay_files_iter(&mut zip_archive) {
        if buffer.ends_with(b"DUMMYREC") {
            debug!("skipping dummy rec");
            continue;
        }

        let replay = match aoe2rec::Savegame::from_slice(&buffer) {
            Ok(replay) => replay,
            Err(_err) => {
                error!("failed to parse {:?}", name);
                continue;
            }
        };

        process_replay(&replay);
    }

    Ok(())
}

fn replay_files_iter(
    archive: &mut ZipArchive<std::fs::File>,
) -> impl Iterator<Item = (PathBuf, Vec<u8>)> {
    (0..archive.len()).filter_map(|index| {
        let mut entry = match archive.by_index(index) {
            Ok(entry) => entry,
            Err(err) => {
                warn!("failed to get entry: {:?}", err);
                return None;
            }
        };

        let entry_name = match entry.enclosed_name() {
            Some(path) => path,
            None => {
                debug!("skipping entry {:?}", entry.name());
                return None;
            }
        };
        let extension = match entry_name.extension() {
            Some(extension) => extension,
            None => {
                warn!("failed to get file extension: {:?}", entry_name);
                return None;
            }
        };
        if extension != "aoe2record" {
            debug!("skipping entry {:?}", entry_name);
            return None;
        }

        if !entry.is_file() {
            debug!("skipping non-file entry {:?}", entry_name);
            return None;
        }

        let mut buffer = Vec::new();
        match entry.read_to_end(&mut buffer) {
            Ok(_) => Some((entry_name, buffer)),
            Err(err) => {
                warn!("failed to extract replay: {:?}", err);
                return None;
            }
        }
    })
}

fn process_replay(replay: &Savegame) {
    let mut i = 0;
    for op in replay.operations() {
        //println!("{:?}", op);
        i += 1;
    }
    println!("Counted ops: {i}");
}

fn unpack_directory(indir: String, outdir: String) -> Result<(), Box<dyn Error>> {
    let outdir = Path::new(&outdir);
    for entry in std::fs::read_dir(indir)? {
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

        let zip_file = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                error!("failed to open file {:?}: {:?}", path, err);
                continue;
            }
        };
        let mut zip_archive = match zip::ZipArchive::new(zip_file) {
            Ok(zip) => zip,
            Err(err) => {
                error!("failed to open zip {:?}: {:?}", path, err);
                continue;
            }
        };
        for (name, buffer) in replay_files_iter(&mut zip_archive) {
            if buffer.ends_with(b"DUMMYREC") {
                debug!("skipping dummy rec");
                continue;
            }

            match std::fs::write(outdir.join(&name), buffer) {
                Ok(_) => {}
                Err(err) => {
                    error!("failed to write {:?}: {:?}", name, err);
                    continue;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let args = App::parse();

    match args.command {
        Command::UnpackDirectory {
            input_directory,
            output_directory,
        } => unpack_directory(input_directory, output_directory),
    }

    //let test_path1 = Path::new("testdata/Silk_vs_CEOSnipezz_G2.aoe2record");
    //let test_path1 = Path::new("testdata/SP Replay v101.103.28990.0 @2025.11.25 205627.aoe2record");

    /*let test_path1 = Path::new("testdata2/Frozen-HotPepperPizza_vs_Mavinga_G1.aoe2record");
    let replay = Savegame::from_file(test_path1)?;
    process_replay(&replay);*/

    /*let recs_dir = Path::new("recs");
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

    Ok(())*/
}
