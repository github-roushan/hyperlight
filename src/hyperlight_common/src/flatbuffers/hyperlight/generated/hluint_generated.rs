// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem;

use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum hluintOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct hluint<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for hluint<'a> {
    type Inner = hluint<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> hluint<'a> {
    pub const VT_VALUE: flatbuffers::VOffsetT = 4;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        hluint { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args hluintArgs,
    ) -> flatbuffers::WIPOffset<hluint<'bldr>> {
        let mut builder = hluintBuilder::new(_fbb);
        builder.add_value(args.value);
        builder.finish()
    }

    #[inline]
    pub fn value(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u32>(hluint::VT_VALUE, Some(0)).unwrap() }
    }
}

impl flatbuffers::Verifiable for hluint<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u32>("value", Self::VT_VALUE, false)?
            .finish();
        Ok(())
    }
}
pub struct hluintArgs {
    pub value: u32,
}
impl<'a> Default for hluintArgs {
    #[inline]
    fn default() -> Self {
        hluintArgs { value: 0 }
    }
}

pub struct hluintBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> hluintBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_value(&mut self, value: u32) {
        self.fbb_.push_slot::<u32>(hluint::VT_VALUE, value, 0);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> hluintBuilder<'a, 'b, A> {
        let start = _fbb.start_table();
        hluintBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<hluint<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for hluint<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("hluint");
        ds.field("value", &self.value());
        ds.finish()
    }
}
