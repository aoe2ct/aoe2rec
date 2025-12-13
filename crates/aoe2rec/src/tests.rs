#[cfg(test)]
mod tests {
    use binrw::{io::BufReader, BinReaderExt, BinWrite};
    use std::{
        fs::File,
        io::{Cursor, Read, Seek},
        path::Path,
    };

    use crate::minimal::MinimalSave;

    #[test]
    fn it_reads_minimal_save() {
        let result = MinimalSave::from_file(Path::new("./beargwyn_vs_kamlesh.aoe2record")).unwrap();
        assert_eq!(result.zheader.game.value.to_string(), "VER 9.4")
        // assert_eq!(result.zheader.game_settings.n_players, 2);
        // assert_eq!(
        //     String::from(&result.zheader.game_settings.players[0].name),
        //     "Beargwyn"
        // );
        // assert_eq!(
        //     String::from(&result.zheader.game_settings.players[1].name),
        //     "Kamlesh"
        // );
    }

    #[test]
    fn it_correctly_write_minimal_save() {
        let path = Path::new("./beargwyn_vs_kamlesh.aoe2record");
        let result = MinimalSave::from_file(path).unwrap();
        let mut output = Cursor::new(vec![]);
        result.write(&mut output).unwrap();
        let mut rewritten: Vec<u8> = vec![];
        output.read_to_end(&mut rewritten).unwrap();
        output.seek(std::io::SeekFrom::Start(0)).unwrap();
        let reparsed: MinimalSave = BufReader::new(output).read_le().unwrap();
        assert_eq!(reparsed.zheader.game.value.to_string(), "VER 9.4")
        // assert_eq!(
        //     String::from(&reparsed.zheader.game_settings.players[0].name),
        //     "Beargwyn"
        // );
        // assert_eq!(
        //     String::from(&reparsed.zheader.game_settings.players[1].name),
        //     "Kamlesh"
        // );
    }

    #[test]
    fn it_changes_the_names_of_the_players() {
        let path = Path::new("./beargwyn_vs_kamlesh.aoe2record");
        let mut result = MinimalSave::from_file(path).unwrap();
        let mut index = 1;
        for player in result.zheader.game_settings.players.iter_mut() {
            player.name = (&format!("Player {index}")).into();
            index += 1;
        }
        let mut output = Cursor::new(vec![]);
        result.write(&mut output).unwrap();
        let mut rewritten: Vec<u8> = vec![];
        output.read_to_end(&mut rewritten).unwrap();
        output.seek(std::io::SeekFrom::Start(0)).unwrap();
        let reparsed: MinimalSave = BufReader::new(output).read_le().unwrap();
        assert_eq!(
            String::from(&reparsed.zheader.game_settings.players.first().unwrap().name),
            "Player 1"
        );
        assert_eq!(
            String::from(&reparsed.zheader.game_settings.players.last().unwrap().name),
            "Player 2"
        )
    }

    #[test]
    fn write_file() {
        let path = Path::new("./beargwyn_vs_kamlesh.aoe2record");
        let mut result = MinimalSave::from_file(path).unwrap();
        let mut index = 1;
        for player in result.zheader.game_settings.players.iter_mut() {
            player.name = (&format!("Player {index}")).into();
            index += 1;
        }
        let mut file = File::create("./beargwyn_vs_kamlesh_mod.aoe2record").unwrap();
        result.write(&mut file).unwrap();
    }
}
