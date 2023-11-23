use super::*;

// Section: impl_wire2api

impl Wire2Api<String> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<EnumWithItemMixed> for *mut wire_enum_with_item_mixed {
    fn wire2api(self) -> EnumWithItemMixed {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemMixed>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumWithItemStruct> for *mut wire_enum_with_item_struct {
    fn wire2api(self) -> EnumWithItemStruct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumWithItemTuple> for *mut wire_enum_with_item_tuple {
    fn wire2api(self) -> EnumWithItemTuple {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemTuple>::wire2api(*wrap).into()
    }
}
impl Wire2Api<f32> for *mut f32 {
    fn wire2api(self) -> f32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i16> for *mut i16 {
    fn wire2api(self) -> i16 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i8> for *mut i8 {
    fn wire2api(self) -> i8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<StructWithCommentsTwinNormal> for *mut wire_struct_with_comments_twin_normal {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithCommentsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithCommentsTwinSync> for *mut wire_struct_with_comments_twin_sync {
    fn wire2api(self) -> StructWithCommentsTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithCommentsTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithOneField> for *mut wire_struct_with_one_field {
    fn wire2api(self) -> StructWithOneField {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithOneField>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithTwoField> for *mut wire_struct_with_two_field {
    fn wire2api(self) -> StructWithTwoField {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithTwoField>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithZeroField> for *mut wire_struct_with_zero_field {
    fn wire2api(self) -> StructWithZeroField {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithZeroField>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TupleStructWithOneField> for *mut wire_tuple_struct_with_one_field {
    fn wire2api(self) -> TupleStructWithOneField {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TupleStructWithOneField>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TupleStructWithTwoField> for *mut wire_tuple_struct_with_two_field {
    fn wire2api(self) -> TupleStructWithTwoField {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TupleStructWithTwoField>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u16> for *mut u16 {
    fn wire2api(self) -> u16 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u64> for *mut u64 {
    fn wire2api(self) -> u64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u8> for *mut u8 {
    fn wire2api(self) -> u8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<EnumWithItemMixed> for wire_enum_with_item_mixed {
    fn wire2api(self) -> EnumWithItemMixed {
        match self.tag {
            0 => EnumWithItemMixed::A,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemMixed::B(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.C);
                EnumWithItemMixed::C {
                    c_field: ans.c_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemStruct> for wire_enum_with_item_struct {
    fn wire2api(self) -> EnumWithItemStruct {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.A);
                EnumWithItemStruct::A {
                    a_field: ans.a_field.wire2api(),
                }
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemStruct::B {
                    b_field: ans.b_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemTuple> for wire_enum_with_item_tuple {
    fn wire2api(self) -> EnumWithItemTuple {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.A);
                EnumWithItemTuple::A(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemTuple::B(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Vec<bool>> for *mut wire_list_bool {
    fn wire2api(self) -> Vec<bool> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<f32>> for *mut wire_list_prim_f_32 {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<f64>> for *mut wire_list_prim_f_64 {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i16>> for *mut wire_list_prim_i_16 {
    fn wire2api(self) -> Vec<i16> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i32>> for *mut wire_list_prim_i_32 {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i64>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<i64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i8>> for *mut wire_list_prim_i_8 {
    fn wire2api(self) -> Vec<i8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u16>> for *mut wire_list_prim_u_16 {
    fn wire2api(self) -> Vec<u16> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u32>> for *mut wire_list_prim_u_32 {
    fn wire2api(self) -> Vec<u32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u64>> for *mut wire_list_prim_u_64 {
    fn wire2api(self) -> Vec<u64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<StructWithCommentsTwinNormal> for wire_struct_with_comments_twin_normal {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        StructWithCommentsTwinNormal {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<StructWithCommentsTwinSync> for wire_struct_with_comments_twin_sync {
    fn wire2api(self) -> StructWithCommentsTwinSync {
        StructWithCommentsTwinSync {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<StructWithOneField> for wire_struct_with_one_field {
    fn wire2api(self) -> StructWithOneField {
        StructWithOneField {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<StructWithTwoField> for wire_struct_with_two_field {
    fn wire2api(self) -> StructWithTwoField {
        StructWithTwoField {
            a: self.a.wire2api(),
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<StructWithZeroField> for wire_struct_with_zero_field {
    fn wire2api(self) -> StructWithZeroField {
        StructWithZeroField {}
    }
}
impl Wire2Api<TupleStructWithOneField> for wire_tuple_struct_with_one_field {
    fn wire2api(self) -> TupleStructWithOneField {
        TupleStructWithOneField(self.field0.wire2api())
    }
}
impl Wire2Api<TupleStructWithTwoField> for wire_tuple_struct_with_two_field {
    fn wire2api(self) -> TupleStructWithTwoField {
        TupleStructWithTwoField(self.field0.wire2api(), self.field1.wire2api())
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_mixed {
    tag: i32,
    kind: *mut EnumWithItemMixedKind,
}

#[repr(C)]
pub union EnumWithItemMixedKind {
    A: *mut wire_EnumWithItemMixed_A,
    B: *mut wire_EnumWithItemMixed_B,
    C: *mut wire_EnumWithItemMixed_C,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixed_A {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixed_B {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixed_C {
    c_field: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_struct {
    tag: i32,
    kind: *mut EnumWithItemStructKind,
}

#[repr(C)]
pub union EnumWithItemStructKind {
    A: *mut wire_EnumWithItemStruct_A,
    B: *mut wire_EnumWithItemStruct_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStruct_A {
    a_field: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStruct_B {
    b_field: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_tuple {
    tag: i32,
    kind: *mut EnumWithItemTupleKind,
}

#[repr(C)]
pub union EnumWithItemTupleKind {
    A: *mut wire_EnumWithItemTuple_A,
    B: *mut wire_EnumWithItemTuple_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTuple_A {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTuple_B {
    field0: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_list_bool {
    ptr: *mut bool,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_f_32 {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_f_64 {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_16 {
    ptr: *mut i16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_32 {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_64 {
    ptr: *mut i64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_8 {
    ptr: *mut i8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_16 {
    ptr: *mut u16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_32 {
    ptr: *mut u32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_64 {
    ptr: *mut u64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_8 {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_normal {
    field_with_comments: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_sync {
    field_with_comments: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_one_field {
    a: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_two_field {
    a: i32,
    b: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_zero_field {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_one_field {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_two_field {
    field0: i32,
    field1: i32,
}

// Section: impl_new_with_nullptr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}
impl NewWithNullPtr for wire_enum_with_item_mixed {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_mixed {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_struct {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_struct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_tuple {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_tuple {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_comments_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_comments_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_one_field {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
        }
    }
}
impl Default for wire_struct_with_one_field {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_two_field {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
        }
    }
}
impl Default for wire_struct_with_two_field {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_zero_field {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}
impl Default for wire_struct_with_zero_field {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_one_field {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_one_field {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_two_field {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_two_field {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    port_: i64,
    that: *mut wire_struct_with_comments_twin_normal,
) {
    wire_StructWithCommentsTwinNormal_instance_method_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_static_method_twin_normal(port_: i64) {
    wire_StructWithCommentsTwinNormal_static_method_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_normal(port_: i64) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_normal(port_: i64) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_normal(port_: i64) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_simple_twin_normal(port_: i64, arg: i32) {
    wire_func_enum_simple_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_mixed_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_mixed,
) {
    wire_func_enum_with_item_mixed_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_struct_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_struct,
) {
    wire_func_enum_with_item_struct_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_tuple_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_tuple,
) {
    wire_func_enum_with_item_tuple_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: *mut wire_struct_with_comments_twin_sync,
) -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_static_method_twin_sync(
) -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_sync() -> support::WireSyncReturn
{
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_sync(
) -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_sync(
) -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_bool_twin_normal(
    port_: i64,
    arg: *mut bool,
) {
    wire_example_optional_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_normal(port_: i64, arg: *mut f32) {
    wire_example_optional_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_normal(port_: i64, arg: *mut f64) {
    wire_example_optional_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_normal(port_: i64, arg: *mut i16) {
    wire_example_optional_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_normal(port_: i64, arg: *mut i32) {
    wire_example_optional_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_normal(port_: i64, arg: *mut i64) {
    wire_example_optional_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_normal(port_: i64, arg: *mut i8) {
    wire_example_optional_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_normal(port_: i64, arg: *mut u16) {
    wire_example_optional_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_normal(port_: i64, arg: *mut u32) {
    wire_example_optional_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_normal(port_: i64, arg: *mut u64) {
    wire_example_optional_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_normal(port_: i64, arg: *mut u8) {
    wire_example_optional_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_bool_twin_sync(
    arg: *mut bool,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_sync(
    arg: *mut f32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_sync(
    arg: *mut f64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_sync(
    arg: *mut i16,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_sync(
    arg: *mut i32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_sync(
    arg: *mut i64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_sync(
    arg: *mut i8,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_sync(
    arg: *mut u16,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_sync(
    arg: *mut u32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_sync(
    arg: *mut u64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_sync(
    arg: *mut u8,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_normal(port_: i64, arg: bool) {
    wire_example_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_normal(port_: i64, arg: f32) {
    wire_example_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_normal(port_: i64, arg: f64) {
    wire_example_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_normal(port_: i64, arg: i16) {
    wire_example_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_normal(port_: i64, arg: i32) {
    wire_example_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_normal(port_: i64, arg: i64) {
    wire_example_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_normal(port_: i64, arg: i8) {
    wire_example_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_normal(port_: i64, arg: u16) {
    wire_example_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_normal(port_: i64, arg: u32) {
    wire_example_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_normal(port_: i64, arg: u64) {
    wire_example_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_normal(port_: i64, arg: u8) {
    wire_example_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_bool_twin_normal(
    port_: i64,
    arg: *mut wire_list_bool,
) {
    wire_example_primitive_list_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_f_32,
) {
    wire_example_primitive_list_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_f_64,
) {
    wire_example_primitive_list_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_16,
) {
    wire_example_primitive_list_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_32,
) {
    wire_example_primitive_list_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_64,
) {
    wire_example_primitive_list_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_8,
) {
    wire_example_primitive_list_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_16,
) {
    wire_example_primitive_list_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_32,
) {
    wire_example_primitive_list_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_64,
) {
    wire_example_primitive_list_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_8,
) {
    wire_example_primitive_list_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_bool_twin_sync(
    arg: *mut wire_list_bool,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_sync(
    arg: *mut wire_list_prim_f_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_sync(
    arg: *mut wire_list_prim_f_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_sync(
    arg: *mut wire_list_prim_i_16,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_sync(
    arg: *mut wire_list_prim_i_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_sync(
    arg: *mut wire_list_prim_i_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_sync(
    arg: *mut wire_list_prim_i_8,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_sync(
    arg: *mut wire_list_prim_u_16,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_sync(
    arg: *mut wire_list_prim_u_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_sync(
    arg: *mut wire_list_prim_u_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_sync(
    arg: *mut wire_list_prim_u_8,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_sync(arg: bool) -> support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_sync(arg: f32) -> support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_sync(arg: f64) -> support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_sync(arg: i16) -> support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_sync(arg: i32) -> support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_sync(arg: i64) -> support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_sync(arg: i8) -> support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_sync(arg: u16) -> support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_sync(arg: u32) -> support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_sync(arg: u64) -> support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_sync(arg: u8) -> support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_sync(a: i32, b: i32) -> support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_normal(port_: i64, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_one_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_one_field,
) {
    wire_func_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_two_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_two_field,
) {
    wire_func_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_zero_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_zero_field,
) {
    wire_func_struct_with_zero_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_one_field_twin_normal(
    port_: i64,
    arg: *mut wire_tuple_struct_with_one_field,
) {
    wire_func_tuple_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_two_field_twin_normal(
    port_: i64,
    arg: *mut wire_tuple_struct_with_two_field,
) {
    wire_func_tuple_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_mixed() -> *mut wire_enum_with_item_mixed {
    support::new_leak_box_ptr(wire_enum_with_item_mixed::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_struct() -> *mut wire_enum_with_item_struct {
    support::new_leak_box_ptr(wire_enum_with_item_struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_tuple() -> *mut wire_enum_with_item_tuple {
    support::new_leak_box_ptr(wire_enum_with_item_tuple::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_32(value: f32) -> *mut f32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_64(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_16(value: i16) -> *mut i16 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_32(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_64(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_8(value: i8) -> *mut i8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_normal(
) -> *mut wire_struct_with_comments_twin_normal {
    support::new_leak_box_ptr(wire_struct_with_comments_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_sync(
) -> *mut wire_struct_with_comments_twin_sync {
    support::new_leak_box_ptr(wire_struct_with_comments_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_one_field() -> *mut wire_struct_with_one_field {
    support::new_leak_box_ptr(wire_struct_with_one_field::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_two_field() -> *mut wire_struct_with_two_field {
    support::new_leak_box_ptr(wire_struct_with_two_field::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_zero_field() -> *mut wire_struct_with_zero_field {
    support::new_leak_box_ptr(wire_struct_with_zero_field::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_one_field(
) -> *mut wire_tuple_struct_with_one_field {
    support::new_leak_box_ptr(wire_tuple_struct_with_one_field::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_two_field(
) -> *mut wire_tuple_struct_with_two_field {
    support::new_leak_box_ptr(wire_tuple_struct_with_two_field::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_16(value: u16) -> *mut u16 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_32(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_64(value: u64) -> *mut u64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_8(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_list_bool(len: i32) -> *mut wire_list_bool {
    let wrap = wire_list_bool {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_32(len: i32) -> *mut wire_list_prim_f_32 {
    let ans = wire_list_prim_f_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_64(len: i32) -> *mut wire_list_prim_f_64 {
    let ans = wire_list_prim_f_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_16(len: i32) -> *mut wire_list_prim_i_16 {
    let ans = wire_list_prim_i_16 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_32(len: i32) -> *mut wire_list_prim_i_32 {
    let ans = wire_list_prim_i_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_64(len: i32) -> *mut wire_list_prim_i_64 {
    let ans = wire_list_prim_i_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_8(len: i32) -> *mut wire_list_prim_i_8 {
    let ans = wire_list_prim_i_8 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_16(len: i32) -> *mut wire_list_prim_u_16 {
    let ans = wire_list_prim_u_16 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_32(len: i32) -> *mut wire_list_prim_u_32 {
    let ans = wire_list_prim_u_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_64(len: i32) -> *mut wire_list_prim_u_64 {
    let ans = wire_list_prim_u_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_8(len: i32) -> *mut wire_list_prim_u_8 {
    let ans = wire_list_prim_u_8 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixed_B() -> *mut EnumWithItemMixedKind {
    support::new_leak_box_ptr(EnumWithItemMixedKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemMixed_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixed_C() -> *mut EnumWithItemMixedKind {
    support::new_leak_box_ptr(EnumWithItemMixedKind {
        C: support::new_leak_box_ptr(wire_EnumWithItemMixed_C {
            c_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStruct_A() -> *mut EnumWithItemStructKind {
    support::new_leak_box_ptr(EnumWithItemStructKind {
        A: support::new_leak_box_ptr(wire_EnumWithItemStruct_A {
            a_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStruct_B() -> *mut EnumWithItemStructKind {
    support::new_leak_box_ptr(EnumWithItemStructKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemStruct_B {
            b_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTuple_A() -> *mut EnumWithItemTupleKind {
    support::new_leak_box_ptr(EnumWithItemTupleKind {
        A: support::new_leak_box_ptr(wire_EnumWithItemTuple_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTuple_B() -> *mut EnumWithItemTupleKind {
    support::new_leak_box_ptr(EnumWithItemTupleKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemTuple_B {
            field0: core::ptr::null_mut(),
        }),
    })
}
