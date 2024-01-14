use crate::containers::ByteContainer;
use core::{mem, ops::Range};
use memmap2::{Mmap, MmapOptions};
use std::{
    fs::File,
    io::{self, Write},
    sync::Arc,
};

pub enum Endian {
    Little,
    Big,
    #[allow(dead_code)]
    Native,
}

pub trait Source<'a> {
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()>;

    fn read_container(&mut self, len: usize) -> io::Result<ByteContainer<'a>>;

    #[must_use]
    fn read_to_end(&mut self) -> ByteContainer<'a>;

    fn seek_absolute(&mut self, pos: usize) -> io::Result<()>;

    #[must_use]
    fn stream_position(&self) -> usize;

    fn read<T>(&mut self, endian: Endian) -> io::Result<T>
    where
        T: BinaryStreamable<Item = T>,
    {
        T::from_stream(self, endian)
    }

    fn read_protocol<T>(&mut self, endian: Endian) -> io::Result<T::Item>
    where
        T: BinaryStreamable,
    {
        T::from_stream(self, endian)
    }

    fn save_restore_position<F, T>(&mut self, f: F) -> io::Result<T>
    where
        F: FnOnce(&mut Self) -> T,
    {
        let position = self.stream_position();
        let result = f(self);
        self.seek_absolute(position)?;
        Ok(result)
    }

    fn seek_relative(&mut self, offset: isize) -> io::Result<()> {
        if let Some(pos) = self.stream_position().checked_add_signed(offset) {
            self.seek_absolute(pos)
        } else {
            Err(io::ErrorKind::UnexpectedEof.into())
        }
    }
}

macro_rules! make_sourceable {
    ($this:ty, $container_lifetime:lifetime $(,$this_lifetime:lifetime)?) => {
        impl $(<$this_lifetime>)? Source<$container_lifetime> for $this {
            fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
                let len = buf.len();
                let start = self.pos;
                let stop = start + len;
                if stop > self.source.len() {
                    Err(io::ErrorKind::UnexpectedEof.into())
                } else {
                    self.pos += len;
                    buf.copy_from_slice(&self.source[start..stop]);
                    Ok(())
                }
            }

            fn read_container(&mut self, len: usize) -> io::Result<ByteContainer<$container_lifetime>> {
                let start = self.pos;
                let stop = start + len;
                if stop > self.source.len() {
                    Err(io::ErrorKind::UnexpectedEof.into())
                } else {
                    self.pos += len;
                    Ok(self.make_container(start..stop))
                }
            }

			fn read_to_end(&mut self) -> ByteContainer<$container_lifetime> {
				let len = self.source.len();
				let start = self.pos;
				let stop = len - start;
				self.make_container(start..stop)
			}

            fn seek_absolute(&mut self, pos: usize) -> io::Result<()> {
                if pos > self.source.len() {
                    Err(io::ErrorKind::UnexpectedEof.into())
                } else {
                    self.pos = pos;
                    Ok(())
                }
            }

            fn stream_position(&self) -> usize {
                self.pos
            }
        }
    };
}

pub struct BorrowedSource<'a> {
    source: &'a [u8],
    pos: usize,
}

impl<'a> BorrowedSource<'a> {
    #[must_use]
    fn make_container(&self, range: Range<usize>) -> ByteContainer<'a> {
        ByteContainer::from_borrowed(&self.source[range])
    }
}

impl<'a> From<&'a [u8]> for BorrowedSource<'a> {
    fn from(source: &'a [u8]) -> Self {
        Self { source, pos: 0 }
    }
}

make_sourceable!(BorrowedSource<'a>, 'a, 'a);

pub struct CopiedSource<'a> {
    source: &'a [u8],
    pos: usize,
}

impl<'a> CopiedSource<'a> {
    #[must_use]
    fn make_container(&self, range: Range<usize>) -> ByteContainer<'static> {
        ByteContainer::from_owned(self.source[range].to_vec())
    }
}

impl<'a> From<&'a [u8]> for CopiedSource<'a> {
    fn from(source: &'a [u8]) -> Self {
        Self { source, pos: 0 }
    }
}

make_sourceable!(CopiedSource<'a>, 'static, 'a);

pub struct MappedSource {
    source: Arc<Mmap>,
    pos: usize,
}

impl MappedSource {
    #[must_use]
    fn make_container(&self, range: Range<usize>) -> ByteContainer<'static> {
        ByteContainer::from_mapped(range.start, range.len(), self.source.clone())
    }
}

impl TryFrom<&File> for MappedSource {
    type Error = io::Error;

    fn try_from(value: &File) -> Result<Self, Self::Error> {
        let options = MmapOptions::new();
        let mapping = unsafe { options.map(value) }?;
        Ok(Self {
            source: Arc::new(mapping),
            pos: 0,
        })
    }
}

make_sourceable!(MappedSource, 'static);

pub trait BinaryStreamable {
    type Item;

    fn from_be_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
    where
        I: ?Sized + Source<'a>;

    fn from_le_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
    where
        I: ?Sized + Source<'a>;

    fn from_ne_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
    where
        I: ?Sized + Source<'a>;

    fn from_stream<'a, I>(stream: &mut I, endian: Endian) -> io::Result<Self::Item>
    where
        I: ?Sized + Source<'a>,
    {
        match endian {
            Endian::Big => Self::from_be_stream(stream),
            Endian::Little => Self::from_le_stream(stream),
            Endian::Native => Self::from_ne_stream(stream),
        }
    }

    fn to_be_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
    where
        O: ?Sized + Write;

    fn to_le_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
    where
        O: ?Sized + Write;

    fn to_ne_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
    where
        O: ?Sized + Write;

    fn to_stream<O>(stream: &mut O, item: &Self::Item, endian: Endian) -> io::Result<()>
    where
        Self: Sized,
        O: ?Sized + Write,
    {
        match endian {
            Endian::Big => Self::to_be_stream(stream, item),
            Endian::Little => Self::to_le_stream(stream, item),
            Endian::Native => Self::to_ne_stream(stream, item),
        }
    }
}

macro_rules! make_binary_streamable {
    ($t:ty) => {
        impl BinaryStreamable for $t {
            type Item = $t;

            fn from_be_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
            where
                I: ?Sized + Source<'a>,
            {
                let mut bytes = [0u8; mem::size_of::<Self::Item>()];
                stream.read_bytes(&mut bytes)?;
                Ok(Self::from_be_bytes(bytes))
            }

            fn from_le_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
            where
                I: ?Sized + Source<'a>,
            {
                let mut bytes = [0u8; mem::size_of::<Self::Item>()];
                stream.read_bytes(&mut bytes)?;
                Ok(Self::from_le_bytes(bytes))
            }

            fn from_ne_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
            where
                I: ?Sized + Source<'a>,
            {
                let mut bytes = [0u8; mem::size_of::<Self::Item>()];
                stream.read_bytes(&mut bytes)?;
                Ok(Self::from_ne_bytes(bytes))
            }

            fn to_be_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
            where
                O: ?Sized + Write,
            {
                let mut bytes = item.to_be_bytes();
                stream.write_all(&mut bytes)
            }

            fn to_le_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
            where
                O: ?Sized + Write,
            {
                let mut bytes = item.to_le_bytes();
                stream.write_all(&mut bytes)
            }

            fn to_ne_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
            where
                O: ?Sized + Write,
            {
                let mut bytes = item.to_ne_bytes();
                stream.write_all(&mut bytes)
            }
        }
    };
}

make_binary_streamable!(u8);
make_binary_streamable!(u16);
make_binary_streamable!(u32);
make_binary_streamable!(u64);

make_binary_streamable!(i8);
make_binary_streamable!(i16);
make_binary_streamable!(i32);
make_binary_streamable!(i64);

macro_rules! make_binary_streamable_tuple {
    ($($idx:tt $t:ident),+) => {
        impl<$($t,)+> BinaryStreamable for ($($t,)+)
        where
            $($t: BinaryStreamable,)+
        {
            type Item = ($($t::Item,)+);

            fn from_be_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
			where
				I: ?Sized + Source<'a>,
			{
                Ok(($(
                    $t::from_be_stream(stream)?,
                )+))
            }

            fn from_le_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
			where
				I: ?Sized + Source<'a>,
			{
                Ok(($(
                    $t::from_le_stream(stream)?,
                )+))
            }

            fn from_ne_stream<'a, I>(stream: &mut I) -> io::Result<Self::Item>
			where
				I: ?Sized + Source<'a>,
			{
                Ok(($(
                    $t::from_ne_stream(stream)?,
                )+))
            }

            fn to_be_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
			where
				O: ?Sized + Write,
			{
                $(
                    $t::to_be_stream(stream, &item.$idx)?;
                )+
                Ok(())
            }

            fn to_le_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
			where
				O: ?Sized + Write,
			{
                $(
                    $t::to_le_stream(stream, &item.$idx)?;
                )+
                Ok(())
            }

            fn to_ne_stream<O>(stream: &mut O, item: &Self::Item) -> io::Result<()>
			where
				O: ?Sized + Write,
			{
                $(
                    $t::to_ne_stream(stream, &item.$idx)?;
                )+
                Ok(())
            }
        }
    };
}

make_binary_streamable_tuple!(0 T0);
make_binary_streamable_tuple!(0 T0, 1 T1);
make_binary_streamable_tuple!(0 T0, 1 T1, 2 T2);
make_binary_streamable_tuple!(0 T0, 1 T1, 2 T2, 3 T3);
make_binary_streamable_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
make_binary_streamable_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
make_binary_streamable_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
make_binary_streamable_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7);
make_binary_streamable_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8);
make_binary_streamable_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9);

pub struct Sink<'a, R>
where
    R: Write,
{
    stream: &'a mut R,
}

impl<'a, R> Sink<'a, R>
where
    R: Write,
{
    #[must_use]
    pub fn new(stream: &'a mut R) -> Self {
        Self { stream }
    }

    pub fn write<T>(&mut self, item: &T, endian: Endian) -> io::Result<()>
    where
        T: BinaryStreamable<Item = T>,
    {
        T::to_stream(&mut self.stream, item, endian)
    }

    pub fn write_protocol<T>(&mut self, item: &T::Item, endian: Endian) -> io::Result<()>
    where
        T: BinaryStreamable,
    {
        T::to_stream(&mut self.stream, item, endian)
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.stream.write_all(bytes)
    }
}
