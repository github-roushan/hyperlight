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
pub enum FunctionCallResultOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct FunctionCallResult<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for FunctionCallResult<'a> {
    type Inner = FunctionCallResult<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> FunctionCallResult<'a> {
    pub const VT_RETURN_VALUE_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_RETURN_VALUE: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        FunctionCallResult { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args FunctionCallResultArgs,
    ) -> flatbuffers::WIPOffset<FunctionCallResult<'bldr>> {
        let mut builder = FunctionCallResultBuilder::new(_fbb);
        if let Some(x) = args.return_value {
            builder.add_return_value(x);
        }
        builder.add_return_value_type(args.return_value_type);
        builder.finish()
    }

    #[inline]
    pub fn return_value_type(&self) -> ReturnValue {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<ReturnValue>(
                    FunctionCallResult::VT_RETURN_VALUE_TYPE,
                    Some(ReturnValue::NONE),
                )
                .unwrap()
        }
    }
    #[inline]
    pub fn return_value(&self) -> flatbuffers::Table<'a> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(
                    FunctionCallResult::VT_RETURN_VALUE,
                    None,
                )
                .unwrap()
        }
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hlint(&self) -> Option<hlint<'a>> {
        if self.return_value_type() == ReturnValue::hlint {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hlint::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hluint(&self) -> Option<hluint<'a>> {
        if self.return_value_type() == ReturnValue::hluint {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hluint::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hllong(&self) -> Option<hllong<'a>> {
        if self.return_value_type() == ReturnValue::hllong {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hllong::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hlulong(&self) -> Option<hlulong<'a>> {
        if self.return_value_type() == ReturnValue::hlulong {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hlulong::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hlfloat(&self) -> Option<hlfloat<'a>> {
        if self.return_value_type() == ReturnValue::hlfloat {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hlfloat::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hldouble(&self) -> Option<hldouble<'a>> {
        if self.return_value_type() == ReturnValue::hldouble {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hldouble::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hlstring(&self) -> Option<hlstring<'a>> {
        if self.return_value_type() == ReturnValue::hlstring {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hlstring::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hlbool(&self) -> Option<hlbool<'a>> {
        if self.return_value_type() == ReturnValue::hlbool {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hlbool::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hlvoid(&self) -> Option<hlvoid<'a>> {
        if self.return_value_type() == ReturnValue::hlvoid {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hlvoid::init_from_table(u) })
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn return_value_as_hlsizeprefixedbuffer(&self) -> Option<hlsizeprefixedbuffer<'a>> {
        if self.return_value_type() == ReturnValue::hlsizeprefixedbuffer {
            let u = self.return_value();
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid union in this slot
            Some(unsafe { hlsizeprefixedbuffer::init_from_table(u) })
        } else {
            None
        }
    }
}

impl flatbuffers::Verifiable for FunctionCallResult<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_union::<ReturnValue, _>(
                "return_value_type",
                Self::VT_RETURN_VALUE_TYPE,
                "return_value",
                Self::VT_RETURN_VALUE,
                true,
                |key, v, pos| match key {
                    ReturnValue::hlint => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hlint>>(
                            "ReturnValue::hlint",
                            pos,
                        ),
                    ReturnValue::hluint => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hluint>>(
                            "ReturnValue::hluint",
                            pos,
                        ),
                    ReturnValue::hllong => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hllong>>(
                            "ReturnValue::hllong",
                            pos,
                        ),
                    ReturnValue::hlulong => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hlulong>>(
                            "ReturnValue::hlulong",
                            pos,
                        ),
                    ReturnValue::hlfloat => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hlfloat>>(
                            "ReturnValue::hlfloat",
                            pos,
                        ),
                    ReturnValue::hldouble => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hldouble>>(
                            "ReturnValue::hldouble",
                            pos,
                        ),
                    ReturnValue::hlstring => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hlstring>>(
                            "ReturnValue::hlstring",
                            pos,
                        ),
                    ReturnValue::hlbool => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hlbool>>(
                            "ReturnValue::hlbool",
                            pos,
                        ),
                    ReturnValue::hlvoid => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hlvoid>>(
                            "ReturnValue::hlvoid",
                            pos,
                        ),
                    ReturnValue::hlsizeprefixedbuffer => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<hlsizeprefixedbuffer>>(
                            "ReturnValue::hlsizeprefixedbuffer",
                            pos,
                        ),
                    _ => Ok(()),
                },
            )?
            .finish();
        Ok(())
    }
}
pub struct FunctionCallResultArgs {
    pub return_value_type: ReturnValue,
    pub return_value: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for FunctionCallResultArgs {
    #[inline]
    fn default() -> Self {
        FunctionCallResultArgs {
            return_value_type: ReturnValue::NONE,
            return_value: None, // required field
        }
    }
}

pub struct FunctionCallResultBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> FunctionCallResultBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_return_value_type(&mut self, return_value_type: ReturnValue) {
        self.fbb_.push_slot::<ReturnValue>(
            FunctionCallResult::VT_RETURN_VALUE_TYPE,
            return_value_type,
            ReturnValue::NONE,
        );
    }
    #[inline]
    pub fn add_return_value(
        &mut self,
        return_value: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            FunctionCallResult::VT_RETURN_VALUE,
            return_value,
        );
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    ) -> FunctionCallResultBuilder<'a, 'b, A> {
        let start = _fbb.start_table();
        FunctionCallResultBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<FunctionCallResult<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_
            .required(o, FunctionCallResult::VT_RETURN_VALUE, "return_value");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for FunctionCallResult<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("FunctionCallResult");
        ds.field("return_value_type", &self.return_value_type());
        match self.return_value_type() {
            ReturnValue::hlint => {
                if let Some(x) = self.return_value_as_hlint() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hluint => {
                if let Some(x) = self.return_value_as_hluint() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hllong => {
                if let Some(x) = self.return_value_as_hllong() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hlulong => {
                if let Some(x) = self.return_value_as_hlulong() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hlfloat => {
                if let Some(x) = self.return_value_as_hlfloat() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hldouble => {
                if let Some(x) = self.return_value_as_hldouble() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hlstring => {
                if let Some(x) = self.return_value_as_hlstring() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hlbool => {
                if let Some(x) = self.return_value_as_hlbool() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hlvoid => {
                if let Some(x) = self.return_value_as_hlvoid() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            ReturnValue::hlsizeprefixedbuffer => {
                if let Some(x) = self.return_value_as_hlsizeprefixedbuffer() {
                    ds.field("return_value", &x)
                } else {
                    ds.field(
                        "return_value",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            _ => {
                let x: Option<()> = None;
                ds.field("return_value", &x)
            }
        };
        ds.finish()
    }
}
#[inline]
/// Verifies that a buffer of bytes contains a `FunctionCallResult`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_function_call_result_unchecked`.
pub fn root_as_function_call_result(
    buf: &[u8],
) -> Result<FunctionCallResult, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<FunctionCallResult>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `FunctionCallResult` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_function_call_result_unchecked`.
pub fn size_prefixed_root_as_function_call_result(
    buf: &[u8],
) -> Result<FunctionCallResult, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<FunctionCallResult>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `FunctionCallResult` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_function_call_result_unchecked`.
pub fn root_as_function_call_result_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<FunctionCallResult<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<FunctionCallResult<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `FunctionCallResult` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_function_call_result_unchecked`.
pub fn size_prefixed_root_as_function_call_result_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<FunctionCallResult<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<FunctionCallResult<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a FunctionCallResult and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `FunctionCallResult`.
pub unsafe fn root_as_function_call_result_unchecked(buf: &[u8]) -> FunctionCallResult {
    flatbuffers::root_unchecked::<FunctionCallResult>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed FunctionCallResult and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `FunctionCallResult`.
pub unsafe fn size_prefixed_root_as_function_call_result_unchecked(
    buf: &[u8],
) -> FunctionCallResult {
    flatbuffers::size_prefixed_root_unchecked::<FunctionCallResult>(buf)
}
#[inline]
pub fn finish_function_call_result_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<FunctionCallResult<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_function_call_result_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<FunctionCallResult<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
