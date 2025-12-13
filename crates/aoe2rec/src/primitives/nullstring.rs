use binrw::binrw;

use binrw::NullString;

#[binrw]
#[derive(Debug, Clone)]
pub struct MyNullString {
    pub text: NullString,
}

impl From<String> for MyNullString {
    fn from(value: String) -> Self {
        MyNullString { text: value.into() }
    }
}

impl serde::Serialize for MyNullString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let strvalue = std::string::String::from_utf8_lossy(&self.text);
        serializer.serialize_str(&strvalue)
    }
}
