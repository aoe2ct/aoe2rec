use binrw::binrw;
use serde::Serialize;

#[binrw]
#[derive(Clone, Default)]
pub struct DeString {
    #[brw(magic = b"\x60\x0A")]
    #[bw(try_calc = value.len().try_into())]
    #[br(temp)]
    length: u16,
    #[br(count = length, try_map = String::from_utf8)]
    #[bw(map = |s|s.as_bytes())]
    value: String,
}

impl From<&DeString> for String {
    fn from(value: &DeString) -> Self {
        value.value.clone()
    }
}

impl From<DeString> for String {
    fn from(value: DeString) -> Self {
        value.value
    }
}

impl From<&String> for DeString {
    fn from(value: &String) -> Self {
        Self {
            value: value.clone(),
        }
    }
}

impl From<String> for DeString {
    fn from(value: String) -> Self {
        Self {
            value: value,
        }
    }
}

impl std::fmt::Debug for DeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl Serialize for DeString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}
