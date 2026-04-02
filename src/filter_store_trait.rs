use std::io;
use std::path::Path;

/// Trait abstracting over compact block filter storage.
pub trait FilterStoreTrait: Sized {
    /// Load the store, from `path`.
    fn open<P: AsRef<Path>>(path: P) -> io::Result<Self>;

    /// Append a filter for `height`.
    ///
    /// Implementations may skip storing the filter if it is not buried deeply
    /// enough (relative to `header_tip_height`), and should return an error if
    /// `height` already exists.
    fn add(&mut self, hash: &[u8], height: u32, header_tip_height: u32, filter: &[u8]) -> io::Result<()>;

    /// Retrieve the block hash and filter data for `height`, or `None` if not present.
    fn get(&mut self, height: u32) -> io::Result<Option<([u8; 32], Vec<u8>)>>;

    /// Number of filters currently stored.
    fn count(&self) -> usize;

    /// Total size in bytes of all stored filter payloads.
    fn total_size(&self) -> u64;
}

/// Default "dummy" storage with no storage.
pub struct NoStorageFilterStore {}

impl FilterStoreTrait for NoStorageFilterStore {
    fn open<P: AsRef<Path>>(_path: P) -> io::Result<Self> { Ok(Self{}) }

    fn add(&mut self, _hash: &[u8], _height: u32, _header_tip_height: u32, _filter: &[u8]) -> io::Result<()> { Ok(()) }

    fn get(&mut self, _height: u32) -> io::Result<Option<([u8; 32], Vec<u8>)>> { Ok(None) }

    fn count(&self) -> usize { 0 }

    fn total_size(&self) -> u64 { 0 }
}
