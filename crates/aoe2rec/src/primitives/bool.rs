use binrw::binrw;

#[binrw]
#[derive(Copy, Clone)]
pub struct Bool {
    #[br(map = |x: u8| x == 1)]
    #[bw(map = |x: &bool| match x { true => 1u8, false => 0u8})]
    value: bool,
}

impl std::fmt::Debug for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl serde::Serialize for Bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.value)
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Bool { value }
    }
}
