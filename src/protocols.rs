use crate::io::{BinaryReadable, BinaryWriteable, Endian, Source};
use bstr::{BStr as ByteStr, BString as ByteString};
use std::io::{self, Write};

#[derive(Debug, thiserror::Error)]
enum MalformedStringError {
    #[error("postfix null-terminator was missing from read string")]
    MissingNullTerminator,

    #[error("a string is too large to be written without data loss")]
    StringTooLarge,
}

impl From<MalformedStringError> for io::Error {
    fn from(value: MalformedStringError) -> Self {
        Self::new(io::ErrorKind::InvalidData, value)
    }
}

pub struct BString;

impl BinaryReadable for BString {
    type Item = ByteString;

    fn from_ne_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
    where
        I: ?Sized + Source<'a>,
    {
        let len: u8 = stream.read(Endian::Native)?;
        let mut result = Vec::new();
        result.resize_with(usize::from(len), Default::default);
        stream.read_bytes(&mut result[..])?;
        result.shrink_to_fit();
        Ok(result.into())
    }
}

impl BinaryWriteable for BString {
    type Item = ByteStr;

    fn to_ne_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
    where
        O: ?Sized + Write,
    {
        let len: Result<u8, _> = item.len().try_into();
        match len {
            Ok(len) => {
                stream.write_all(&len.to_ne_bytes())?;
                stream.write_all(item)?;
                Ok(())
            }
            Err(_) => Err(MalformedStringError::StringTooLarge.into()),
        }
    }
}

pub struct ZString;

impl BinaryReadable for ZString {
    type Item = ByteString;

    fn from_ne_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
    where
        I: ?Sized + Source<'a>,
    {
        let mut result = Vec::new();
        loop {
            let byte: u8 = stream.read(Endian::Native)?;
            match byte {
                0 => break,
                byte => result.push(byte),
            };
        }

        result.shrink_to_fit();
        Ok(result.into())
    }
}

impl BinaryWriteable for ZString {
    type Item = ByteStr;

    fn to_ne_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
    where
        O: ?Sized + Write,
    {
        stream.write_all(item)?;
        stream.write_all(b"\0")?;
        Ok(())
    }
}

pub struct BZString;

impl BinaryReadable for BZString {
    type Item = ByteString;

    fn from_ne_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
    where
        I: ?Sized + Source<'a>,
    {
        let len: u8 = stream.read(Endian::Native)?;
        if len > 0 {
            let mut result = Vec::new();
            result.resize_with(usize::from(len), Default::default);
            stream.read_bytes(&mut result[..])?;
            match result.pop() {
                Some(b'\0') => {
                    result.shrink_to_fit();
                    Ok(result.into())
                }
                _ => Err(MalformedStringError::MissingNullTerminator.into()),
            }
        } else {
            Ok(Self::Item::default())
        }
    }
}

impl BinaryWriteable for BZString {
    type Item = ByteStr;

    fn to_ne_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
    where
        O: ?Sized + Write,
    {
        let len: Result<u8, _> = (item.len() + 1).try_into();
        match len {
            Ok(len) => {
                stream.write_all(&len.to_ne_bytes())?;
                stream.write_all(item)?;
                stream.write_all(b"\0")?;
                Ok(())
            }
            Err(_) => Err(MalformedStringError::StringTooLarge.into()),
        }
    }
}