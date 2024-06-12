// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub struct StructWithZeroFieldTwinSse {}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_struct_with_zero_field_twin_sse(
    arg: StructWithZeroFieldTwinSse,
) -> StructWithZeroFieldTwinSse {
    arg
}

pub struct StructWithOneFieldTwinSse {
    pub a: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_struct_with_one_field_twin_sse(
    arg: StructWithOneFieldTwinSse,
) -> StructWithOneFieldTwinSse {
    arg
}

pub struct StructWithTwoFieldTwinSse {
    pub a: i32,
    pub b: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_struct_with_two_field_twin_sse(
    arg: StructWithTwoFieldTwinSse,
) -> StructWithTwoFieldTwinSse {
    arg
}

pub struct TupleStructWithOneFieldTwinSse(pub i32);

#[flutter_rust_bridge::frb(serialize)]
pub fn func_tuple_struct_with_one_field_twin_sse(
    arg: TupleStructWithOneFieldTwinSse,
) -> TupleStructWithOneFieldTwinSse {
    arg
}

pub struct TupleStructWithTwoFieldTwinSse(pub i32, pub i32);

#[flutter_rust_bridge::frb(serialize)]
pub fn func_tuple_struct_with_two_field_twin_sse(
    arg: TupleStructWithTwoFieldTwinSse,
) -> TupleStructWithTwoFieldTwinSse {
    arg
}
