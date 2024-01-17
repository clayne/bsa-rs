use crate::fo4::{Chunk, CompressionFormat, CompressionLevel, Format};
use core::{
    fmt::{self, Debug, Display, Formatter},
    ops::RangeBounds,
    result,
};
use std::error;

pub struct CapacityError<'bytes>(Chunk<'bytes>);

impl<'bytes> CapacityError<'bytes> {
    #[must_use]
    pub fn into_element(self) -> Chunk<'bytes> {
        self.0
    }
}

impl<'bytes> Debug for CapacityError<'bytes> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl<'bytes> Display for CapacityError<'bytes> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "could not insert another chunk because the file was already full"
        )
    }
}

impl<'bytes> error::Error for CapacityError<'bytes> {}

#[repr(transparent)]
pub struct ReadOptionsBuilder(ReadOptions);

impl ReadOptionsBuilder {
    #[must_use]
    pub fn build(self) -> ReadOptions {
        self.0
    }

    #[must_use]
    pub fn compression_format(mut self, compression_format: CompressionFormat) -> Self {
        self.0.compression_format = compression_format;
        self
    }

    #[must_use]
    pub fn compression_level(mut self, compression_level: CompressionLevel) -> Self {
        self.0.compression_level = compression_level;
        self
    }

    #[must_use]
    pub fn format(mut self, format: Format) -> Self {
        self.0.format = format;
        self
    }

    #[must_use]
    pub fn mip_chunk_height(mut self, mip_chunk_height: usize) -> Self {
        self.0.mip_chunk_height = mip_chunk_height;
        self
    }

    #[must_use]
    pub fn mip_chunk_width(mut self, mip_chunk_width: usize) -> Self {
        self.0.mip_chunk_width = mip_chunk_width;
        self
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ReadOptionsBuilder {
    fn default() -> Self {
        Self(ReadOptions {
            format: Format::GNRL,
            mip_chunk_width: 0,
            mip_chunk_height: 0,
            compression_format: CompressionFormat::Zip,
            compression_level: CompressionLevel::FO4,
        })
    }
}

#[derive(Clone, Copy)]
pub struct ReadOptions {
    format: Format,
    mip_chunk_width: usize,
    mip_chunk_height: usize,
    compression_format: CompressionFormat,
    compression_level: CompressionLevel,
}

impl ReadOptions {
    #[must_use]
    pub fn builder() -> ReadOptionsBuilder {
        ReadOptionsBuilder::new()
    }

    #[must_use]
    pub fn compression_format(&self) -> CompressionFormat {
        self.compression_format
    }

    #[must_use]
    pub fn compression_level(&self) -> CompressionLevel {
        self.compression_level
    }

    #[must_use]
    pub fn format(&self) -> Format {
        self.format
    }

    #[must_use]
    pub fn mip_chunk_height(&self) -> usize {
        self.mip_chunk_height
    }

    #[must_use]
    pub fn mip_chunk_width(&self) -> usize {
        self.mip_chunk_width
    }
}

#[repr(transparent)]
pub struct WriteOptionsBuilder(WriteOptions);

impl WriteOptionsBuilder {
    #[must_use]
    pub fn build(self) -> WriteOptions {
        self.0
    }

    #[must_use]
    pub fn compression_format(mut self, compression_format: CompressionFormat) -> Self {
        self.0.compression_format = compression_format;
        self
    }

    #[must_use]
    pub fn format(mut self, format: Format) -> Self {
        self.0.format = format;
        self
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for WriteOptionsBuilder {
    fn default() -> Self {
        Self(WriteOptions {
            format: Format::GNRL,
            compression_format: CompressionFormat::Zip,
        })
    }
}

#[derive(Clone, Copy)]
pub struct WriteOptions {
    format: Format,
    compression_format: CompressionFormat,
}

impl WriteOptions {
    #[must_use]
    pub fn builder() -> WriteOptionsBuilder {
        WriteOptionsBuilder::new()
    }

    #[must_use]
    pub fn compression_format(&self) -> CompressionFormat {
        self.compression_format
    }

    #[must_use]
    pub fn format(&self) -> Format {
        self.format
    }
}

#[derive(Default)]
pub struct File<'bytes> {
    pub(crate) chunks: Vec<Chunk<'bytes>>,
}

impl<'bytes> File<'bytes> {
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut Chunk<'bytes> {
        self.chunks.as_mut_ptr()
    }

    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [Chunk<'bytes>] {
        self.chunks.as_mut_slice()
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const Chunk<'bytes> {
        self.chunks.as_ptr()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[Chunk<'bytes>] {
        self.chunks.as_slice()
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        4
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
    }

    /// # Panics
    ///
    /// Panics if [`start_bound`](RangeBounds::start_bound) exceeds [`end_bound`](RangeBounds::end_bound), or if [`end_bound`](RangeBounds::end_bound) exceeds [`len`](Self::len).
    pub fn drain<R>(&mut self, range: R) -> impl Iterator<Item = Chunk<'bytes>> + '_
    where
        R: RangeBounds<usize>,
    {
        self.chunks.drain(range)
    }

    /// # Panics
    ///
    /// Panics if `index` exceeds [`len`](Self::len), or [`is_full`](Self::is_full).
    pub fn insert(&mut self, index: usize, element: Chunk<'bytes>) {
        self.try_insert(index, element).unwrap();
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pop(&mut self) -> Option<Chunk<'bytes>> {
        self.chunks.pop()
    }

    /// # Panics
    ///
    /// Panics if [`is_full`](Self::is_full).
    pub fn push(&mut self, element: Chunk<'bytes>) {
        self.try_push(element).unwrap();
    }

    #[must_use]
    pub fn remaining_capacity(&self) -> usize {
        self.capacity() - self.len()
    }

    /// # Panics
    ///
    /// Panics if `index` exceeds [`len`](Self::len).
    pub fn remove(&mut self, index: usize) -> Chunk<'bytes> {
        self.chunks.remove(index)
    }

    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut Chunk<'bytes>) -> bool,
    {
        self.chunks.retain_mut(f);
    }

    /// # Panics
    ///
    /// Panics if `index` exceeds [`len`](Self::len), or [`is_empty`](Self::is_empty).
    pub fn swap_remove(&mut self, index: usize) -> Chunk<'bytes> {
        self.try_swap_remove(index).unwrap()
    }

    pub fn truncate(&mut self, len: usize) {
        self.chunks.truncate(len);
    }

    /// # Panics
    ///
    /// Panics if `index` exceeds [`len`](Self::len).
    pub fn try_insert(
        &mut self,
        index: usize,
        element: Chunk<'bytes>,
    ) -> result::Result<(), CapacityError<'bytes>> {
        if self.is_full() {
            Err(CapacityError(element))
        } else {
            self.do_reserve();
            self.chunks.insert(index, element);
            Ok(())
        }
    }

    pub fn try_push(
        &mut self,
        element: Chunk<'bytes>,
    ) -> result::Result<(), CapacityError<'bytes>> {
        if self.is_full() {
            Err(CapacityError(element))
        } else {
            self.do_reserve();
            self.chunks.push(element);
            Ok(())
        }
    }

    /// # Panics
    ///
    /// Panics if `index` exceeds [`len`](Self::len).
    pub fn try_swap_remove(&mut self, index: usize) -> Option<Chunk<'bytes>> {
        if index < self.len() {
            Some(self.chunks.swap_remove(index))
        } else {
            None
        }
    }

    fn do_reserve(&mut self) {
        match self.len() {
            0 | 3 => self.chunks.reserve_exact(1),
            1 => self.chunks.reserve_exact(3),
            2 => self.chunks.reserve_exact(2),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fo4::File;

    #[test]
    fn default_state() {
        let f = File::default();
        assert!(f.is_empty());
        assert!(f.as_slice().is_empty());
        assert!(!f.is_full());
        assert_eq!(f.capacity(), 4);
    }
}
