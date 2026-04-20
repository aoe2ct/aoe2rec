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
        assert_eq!(result.zheader.game.text.to_string(), "VER 9.4")
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
        assert_eq!(reparsed.zheader.game.text.to_string(), "VER 9.4")
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

#[cfg(test)]
mod robust_tests {
    use crate::{actions::ActionData, parse_operations, Operation};
    use binrw::BinReaderExt;
    use std::io::Cursor;

    #[test]
    fn test_parse_ai_operation() {
        let mut data = Cursor::new(vec![
            0x07, 0x00, 0x00, 0x00, // Magic 7 (AI)
            0x04, 0x00, 0x00, 0x00, // Length 4
            0xDE, 0xAD, 0xBE, 0xEF, // AI Data
        ]);
        let op: Operation = data.read_le_args((1u16,)).unwrap();
        if let Operation::Ai { length, data } = op {
            assert_eq!(length, 4);
            assert_eq!(data, vec![0xDE, 0xAD, 0xBE, 0xEF]);
        } else {
            panic!("Expected AI operation");
        }
    }

    #[test]
    fn test_parse_operations_resync() {
        // parse_operations stops at magic 6.
        let mut data = Cursor::new(vec![
            0x99, 0x99, 0x99, // 3 bytes of garbage
            0x08, 0x00, 0x00, 0x00, // Magic 8 (MapNote)
            0x02, 0x00, 0x00, 0x00, // Length 2
            0x01, 0x02, // Data
            0x06, 0x00, 0x00, 0x00, // Magic 6 (PostGame)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, // Padding for PostGame seeks
            0xCE, 0xA4, 0x59, 0xB1, 0x05, 0xDB, 0x7B, 0x43, // End bit
        ]);

        let ops = parse_operations(&mut data, binrw::Endian::Little, (1u16,)).unwrap();
        assert_eq!(ops.len(), 2);
        if let Operation::MapNote { length, .. } = &ops[0] {
            assert_eq!(*length, 2);
        } else {
            panic!("Expected MapNote operation");
        }
    }

    #[test]
    fn test_parse_action_with_failed_data() {
        // Action magic = 1
        // Action header: length (u32), data (Vec<u8>), world_time (u32)
        let mut data = Cursor::new(vec![
            0x01, 0x00, 0x00, 0x00, // Magic 1 (Action)
            0x04, 0x00, 0x00, 0x00, // Length 4
            0x00, 0x00, 0x00, 0x00, // Garbage action data
            0x00, 0x00, 0x00, 0x00, // World time
        ]);
        let op: Operation = data.read_le_args((1u16,)).unwrap();
        if let Operation::Action { action_data, .. } = op {
            assert!(action_data.is_none());
        } else {
            panic!("Expected Action operation");
        }
    }

    #[test]
    fn test_parse_ai_research_action_without_selected_buildings() {
        let mut data = Cursor::new(vec![
            0x01, 0x00, 0x00, 0x00, // Magic 1 (Action)
            0x11, 0x00, 0x00, 0x00, // Payload length 17
            0x65, // Action type 101 (Research)
            0x02, // Player 2
            0x0d, 0x00, // Action length 13
            0x84, 0x0b, 0x00, 0x00, // Building ID 2948
            0x01, 0x00, // Selected 1
            0x16, 0x00, // Technology 22
            0xff, 0xff, 0xff, 0xff, 0x00, // Unknown trailer
            0xbd, 0x6a, 0x02, 0x00, // World time 158189
        ]);

        let op: Operation = data.read_le_args((66u16,)).unwrap();
        match op {
            Operation::Action {
                action_data:
                    Some(ActionData::Research {
                        player_id,
                        action_length,
                        building_id,
                        selected,
                        technology_type,
                        building_ids,
                        ..
                    }),
                ..
            } => {
                assert_eq!(player_id, 2);
                assert_eq!(action_length, 13);
                assert_eq!(building_id, 2948);
                assert_eq!(selected, 1);
                assert_eq!(technology_type, 22);
                assert!(building_ids.is_empty());
            }
            _ => panic!("Expected parsed AI research action"),
        }
    }

    #[test]
    fn test_parse_research_action_with_selected_buildings() {
        let mut data = Cursor::new(vec![
            0x01, 0x00, 0x00, 0x00, // Magic 1 (Action)
            0x15, 0x00, 0x00, 0x00, // Payload length 21
            0x65, // Action type 101 (Research)
            0x01, // Player 1
            0x11, 0x00, // Action length 17
            0x7d, 0x0b, 0x00, 0x00, // Building ID 2941
            0x01, 0x00, // Selected 1
            0x16, 0x00, // Technology 22
            0xff, 0xff, 0xff, 0xff, 0x00, // Unknown trailer
            0x7d, 0x0b, 0x00, 0x00, // Selected building ID
            0xce, 0xbe, 0x07, 0x00, // World time 507470
        ]);

        let op: Operation = data.read_le_args((66u16,)).unwrap();
        match op {
            Operation::Action {
                action_data:
                    Some(ActionData::Research {
                        player_id,
                        action_length,
                        building_id,
                        selected,
                        technology_type,
                        building_ids,
                        ..
                    }),
                ..
            } => {
                assert_eq!(player_id, 1);
                assert_eq!(action_length, 17);
                assert_eq!(building_id, 2941);
                assert_eq!(selected, 1);
                assert_eq!(technology_type, 22);
                assert_eq!(building_ids, vec![2941]);
            }
            _ => panic!("Expected parsed research action with selected buildings"),
        }
    }
}
