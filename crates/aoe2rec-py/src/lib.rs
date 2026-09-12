use pyo3::prelude::*;

#[pymodule]
mod aoe2rec_py {
    use std::sync::Arc;

    use chrono::{DateTime, Utc};
    use pyo3::{pyclass, pyfunction, pymethods, Bound, PyAny, PyResult, Python};
    use pythonize::pythonize;

    #[pyfunction]
    #[pyo3(signature = (data: "bytes") -> "Any")]
    fn parse_rec(py: Python<'_>, data: Vec<u8>) -> PyResult<Bound<'_, PyAny>> {
        let rec = aoe2rec::Savegame::from_bytes(data.into()).unwrap();
        let pyrec = pythonize(py, &rec).unwrap();
        Ok(pyrec)
    }

    #[pyclass]
    struct Savegame {
        rec_info: Arc<aoe2rec::Savegame>,
    }
    #[pymethods]
    impl Savegame {
        #[new]
        fn from_bytes(data: Vec<u8>) -> PyResult<Self> {
            let savegame = Arc::new(aoe2rec::Savegame::from_bytes(data.into()).unwrap());
            Ok(Savegame { rec_info: savegame })
        }
        #[getter]
        fn header(&self) -> Header {
            Header {
                rec_info: self.rec_info.clone(),
            }
        }

        #[getter]
        fn played_at(&self) -> DateTime<Utc> {
            self.rec_info.played_at()
        }

        #[getter]
        fn is_restored(&self) -> bool {
            self.rec_info.is_restored()
        }

        #[getter]
        fn world_time(&self) -> u32 {
            self.rec_info.world_time()
        }
    }

    #[pyclass]
    struct Header {
        rec_info: Arc<aoe2rec::Savegame>,
    }

    #[pymethods]
    impl Header {
        #[getter]
        fn players(&self) -> Vec<Player> {
            self.rec_info
                .header()
                .unwrap()
                .players()
                .iter()
                .enumerate()
                .map(|(index, _)| Player {
                    rec_info: self.rec_info.clone(),
                    index,
                })
                .collect()
        }

        #[getter]
        fn save_version(&self) -> String {
            let header = self.rec_info.header().unwrap();
            format!("{}.{}", header.version_major, header.version_minor)
        }

        #[getter]
        fn map_info(&self) -> MapInfo {
            MapInfo {
                rec_info: self.rec_info.clone(),
            }
        }

        #[getter]
        fn game_settings(&self) -> GameSettings {
            GameSettings {
                rec_info: self.rec_info.clone(),
            }
        }
    }

    #[pyclass]
    struct Player {
        rec_info: Arc<aoe2rec::Savegame>,
        index: usize,
    }

    #[pymethods]
    impl Player {
        fn __str__(&self) -> std::string::String {
            self.name()
        }
        fn __repr__(&self) -> std::string::String {
            format!("<Player: {}>", self.__str__())
        }
        #[getter]
        fn name(&self) -> std::string::String {
            String::from(&self.rec_info.players()[self.index].name)
        }
    }

    #[pyclass]
    struct MapInfo {
        rec_info: Arc<aoe2rec::Savegame>,
    }

    #[pymethods]
    impl MapInfo {
        #[getter]
        fn size_x(&self) -> u32 {
            self.rec_info.header().unwrap().map_info.size_x
        }

        #[getter]
        fn size_y(&self) -> u32 {
            self.rec_info.header().unwrap().map_info.size_y
        }

        #[getter]
        fn all_visible(&self) -> bool {
            self.rec_info.header().unwrap().map_info.all_visible.into()
        }

        #[getter]
        fn fog_of_war(&self) -> bool {
            self.rec_info.header().unwrap().map_info.fog_of_war.into()
        }

        #[getter]
        fn tiles(&self) -> Vec<Tile> {
            self.rec_info
                .header()
                .unwrap()
                .map_info
                .tiles
                .iter()
                .map(|t| Tile {
                    terrain_type: t.terrain_type,
                    terrain_subtype: t.terrain_type_2,
                    elevation: t.elevation,
                })
                .collect()
        }
    }

    #[pyclass]
    struct Tile {
        #[pyo3(get)]
        terrain_type: u8,
        #[pyo3(get)]
        terrain_subtype: u8,
        #[pyo3(get)]
        elevation: u8,
    }

    #[pyclass]
    struct GameSettings {
        rec_info: Arc<aoe2rec::Savegame>,
    }

    #[pymethods]
    impl GameSettings {
        #[getter]
        fn lobby_name(&self) -> String {
            String::from(&self.rec_info.header().unwrap().game_settings.lobby_name)
        }
    }
}
