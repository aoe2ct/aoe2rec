use binrw::binrw;

#[binrw]
#[derive(Clone, Default)]
pub struct LenString<L>
where
    usize: TryFrom<L>,
    for<'a> L:
        binrw::BinRead + binrw::BinWrite<Args<'a> = ()> + TryFrom<usize> + std::fmt::Debug + Copy,
    for<'a> <L as binrw::BinRead>::Args<'a>: std::default::Default,
    <L as TryFrom<usize>>::Error: Send + Sync + std::fmt::Display + std::fmt::Debug,
    for<'a> <L as TryFrom<usize>>::Error: 'a,
{
    _length: std::marker::PhantomData<L>,
    #[br(temp)]
    #[bw(try_calc = value.len().try_into())]
    length: L,
    #[br(count = length, try_map = String::from_utf8)]
    #[bw(map = |s|s.as_bytes())]
    value: String,
}

pub type LenString32 = LenString<u32>;
pub type LenString16 = LenString<u16>;

impl<L> From<&LenString<L>> for String
where
    usize: TryFrom<L>,
    for<'a> L:
        binrw::BinRead + binrw::BinWrite<Args<'a> = ()> + TryFrom<usize> + std::fmt::Debug + Copy,
    for<'a> <L as binrw::BinRead>::Args<'a>: std::default::Default,
    <L as TryFrom<usize>>::Error: Send + Sync + std::fmt::Display + std::fmt::Debug,
    for<'a> <L as TryFrom<usize>>::Error: 'a,
{
    fn from(value: &LenString<L>) -> Self {
        value.value.clone()
    }
}

impl<L> From<&String> for LenString<L>
where
    usize: TryFrom<L>,
    for<'a> L:
        binrw::BinRead + binrw::BinWrite<Args<'a> = ()> + TryFrom<usize> + std::fmt::Debug + Copy,
    for<'a> <L as binrw::BinRead>::Args<'a>: std::default::Default,
    <L as TryFrom<usize>>::Error: Send + Sync + std::fmt::Display + std::fmt::Debug,
    for<'a> <L as TryFrom<usize>>::Error: 'a,
{
    fn from(value: &String) -> Self {
        Self {
            _length: std::default::Default::default(),
            value: value.clone(),
        }
    }
}

impl<L> std::fmt::Debug for LenString<L>
where
    usize: TryFrom<L>,
    for<'a> L:
        binrw::BinRead + binrw::BinWrite<Args<'a> = ()> + TryFrom<usize> + std::fmt::Debug + Copy,
    for<'a> <L as binrw::BinRead>::Args<'a>: std::default::Default,
    <L as TryFrom<usize>>::Error: Send + Sync + std::fmt::Display + std::fmt::Debug,
    for<'a> <L as TryFrom<usize>>::Error: 'a,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl<L> serde::Serialize for LenString<L>
where
    usize: TryFrom<L>,
    for<'a> L:
        binrw::BinRead + binrw::BinWrite<Args<'a> = ()> + TryFrom<usize> + std::fmt::Debug + Copy,
    for<'a> <L as binrw::BinRead>::Args<'a>: std::default::Default,
    <L as TryFrom<usize>>::Error: Send + Sync + std::fmt::Display + std::fmt::Debug,
    for<'a> <L as TryFrom<usize>>::Error: 'a,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}
