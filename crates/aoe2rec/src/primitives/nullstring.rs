use binrw::binrw;

use binrw::NullString;

#[binrw]
#[derive(Debug, Clone)]
pub struct MyNullString {
    #[br(try_map = |val: NullString| std::string::String::from_utf8(val.0))]
    #[bw(map = |val| NullString::from(val.clone()))]
    pub value: String,
}

impl From<String> for MyNullString {
    fn from(value: String) -> Self {
        MyNullString { value: value.into() }
    }
}

impl From<MyNullString> for String {
    fn from(value: MyNullString) -> Self {
        value.value
    }
}

impl serde::Serialize for MyNullString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}
