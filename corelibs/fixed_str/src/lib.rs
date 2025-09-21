use anyhow::{Result, bail};
use serde::{
    Deserialize, Serialize,
    de::{self, Unexpected, Visitor},
};
use std::{fmt, marker::PhantomData, ops::Index, str::FromStr};

/// Short string value ,Copy type.
#[derive(Eq, PartialEq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct FixedStr<const N: usize>([u8; N]);

pub type Str8 = FixedStr<8>;
pub type Str16 = FixedStr<16>;
pub type Str24 = FixedStr<24>;
pub type Str32 = FixedStr<32>;

impl<const N: usize> FixedStr<N> {
    pub fn len(&self) -> usize {
        self.0.iter().filter(|&v| *v != 0u8).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0[..self.len()]).unwrap()
    }

    // for use by proc_macro.
    pub const fn from_bytes(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    pub const fn from_slice(bytes: &[u8; N]) -> Self {
        Self(*bytes)
    }

    pub fn into_array(self) -> [u8; N] {
        self.0
    }
}

impl<const N: usize> Into<[u8; N]> for FixedStr<N> {
    fn into(self) -> [u8; N] {
        self.0
    }
}

impl<const N: usize> Default for FixedStr<N>
where
    [u8; N]: Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<const N: usize> From<[u8; N]> for FixedStr<N> {
    fn from(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<&[u8; N]> for FixedStr<N> {
    fn from(value: &[u8; N]) -> Self {
        Self(*value)
    }
}

impl<const N: usize> std::fmt::Display for FixedStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> std::fmt::Debug for FixedStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<const N: usize> std::fmt::Write for FixedStr<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut i = self.len();
        for chr in s.bytes() {
            *self.0.get_mut(i).ok_or(fmt::Error)? = chr;
            i += 1;
        }
        Ok(())
    }
}

impl<const N: usize> FromStr for FixedStr<N> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let src = s.as_bytes();
        let mut bytes = [0u8; N];
        if src.len() > N {
            bail!("handle too long: {s}, must be <= {} characters", N)
        }
        let n = usize::min(src.len(), bytes.len());
        bytes[..n].clone_from_slice(&src[..n]);
        Ok(FixedStr(bytes))
    }
}

impl<const N: usize> AsRef<str> for FixedStr<N> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> Index<usize> for FixedStr<N> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[0]
    }
}

impl<const N: usize> Serialize for FixedStr<N> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedStr<N> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Vis(PhantomData))
    }
}

struct Vis<const N: usize>(PhantomData<[(); N]> /*hack to use N*/);

impl<const N: usize> Visitor<'_> for Vis<N> {
    type Value = FixedStr<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing at most {N} bytes")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        FixedStr::from_str(s).map_err(|_| de::Error::invalid_value(Unexpected::Str(s), &self))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_to_str() {
        let h = FixedStr::<16>::from_str("cube").unwrap();
        assert_eq!(h.as_str(), "cube");
    }

    #[test]
    fn test_write() {
        use std::fmt::Write;
        let mut s = FixedStr::<8>::from_str("0").unwrap();
        write!(&mut s, "123").unwrap();
        write!(&mut s, "45").unwrap();
        assert_eq!(s.as_str(), "012345");
        assert!(write!(&mut s, "6789").is_err());
    }
}
