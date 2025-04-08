//! This mod holds all the general interfaces used in the core crate.
use crate::MemoryChuckSize;

pub trait MemoryInterface {
    /// This function reads a word from the memory
    /// It returns the value if the read was successful
    fn read_mem(&self, addr: u32, size: MemoryChuckSize) -> Option<u32>;
    /// This function writes a word to the memory
    /// It returns true if the write was successful
    fn write_mem(&mut self, addr: u32, size: MemoryChuckSize, value: u32) -> bool;
}
