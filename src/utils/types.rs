use bincode::{enc::Encoder, error::EncodeError, Encode};

pub type Padding<const N: usize> = [u8; N];

#[derive(Debug)]
pub struct Seq<T>(Vec<T>);

impl<T> Seq<T> {
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> Encode for Seq<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.iter().try_for_each(|item| item.encode(encoder))
    }
}

impl<T> FromIterator<T> for Seq<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T> IntoIterator for Seq<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> From<Vec<T>> for Seq<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub enum Opt<T> {
    Some(T),
    None,
}

impl<T> Opt<T> {
    pub fn as_option(&self) -> Option<&T> {
        match &self {
            Opt::Some(value) => Some(value),
            Opt::None => None,
        }
    }
}

impl<T> From<Option<T>> for Opt<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Opt::Some(value),
            None => Opt::None,
        }
    }
}

impl<T> Encode for Opt<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Opt::Some(value) => value.encode(encoder),
            Opt::None => Ok(()),
        }
    }
}
