use std::ffi::c_void;

pub(crate) type Context = *const c_void;
pub(crate) type PersistentValue = *const c_void;

#[allow(dead_code)]
#[repr(u32)]
#[derive(Copy, Clone)]
pub(crate) enum ValueTag {
    Null = 0,
    Undefined = 1,
    Number = 2,
    Boolean = 3,
    Array = 4,
    Function = 5,
    Date = 6,
    Object = 7,
    String = 8,
}

#[repr(C)]
pub(crate) union ValueInner {
    pub(crate) empty: u8,
    pub(crate) number: f64,
    pub(crate) boolean: u8,
    pub(crate) value: PersistentValue,
}

#[repr(C)]
pub(crate) struct Value {
    pub(crate) tag: ValueTag,
    pub(crate) inner: ValueInner,
}

impl Value {
    pub(crate) fn new(tag: ValueTag, inner: ValueInner) -> Value {
        Value { tag, inner }
    }
}

#[repr(C)]
pub(crate) struct EvalResult {
    pub(crate) exception: u8,
    pub(crate) value: Value,
}

#[repr(C)]
pub(crate) struct Utf8Value {
    pub(crate) data: *const u8,
    pub(crate) length: i32,
    src: *mut c_void,
}

extern "C" {
    pub(crate) fn context_new() -> Context;
    pub(crate) fn context_eval(ctx: Context, data: *const u8, length: usize) -> EvalResult;
    pub(crate) fn context_drop(ctx: Context);
    pub(crate) fn value_clone(ctx: Context, value: PersistentValue) -> PersistentValue;
    pub(crate) fn value_drop(value: PersistentValue);
    pub(crate) fn string_create(ctx: Context, data: *const u8, length: usize) -> PersistentValue;
    pub(crate) fn string_to_utf8_value(ctx: Context, value: PersistentValue) -> Utf8Value;
    pub(crate) fn utf8_value_drop(utf8_value: Utf8Value);
    pub(crate) fn array_length(ctx: Context, object: PersistentValue) -> u32;
    pub(crate) fn object_create(ctx: Context) -> PersistentValue;
    pub(crate) fn array_create(ctx: Context) -> PersistentValue;
    pub(crate) fn object_get(ctx: Context, object: PersistentValue, key: Value) -> EvalResult;
    pub(crate) fn object_set(ctx: Context, object: PersistentValue, key: Value, value: Value)
        -> EvalResult;
    pub(crate) fn object_get_index(ctx: Context, object: PersistentValue, index: u32) -> Value;
    pub(crate) fn object_set_index(ctx: Context, object: PersistentValue, index: u32, value: Value);
    pub(crate) fn coerce_boolean(ctx: Context, value: Value) -> u8;
    pub(crate) fn coerce_number(ctx: Context, value: Value) -> EvalResult;
    pub(crate) fn coerce_string(ctx: Context, value: Value) -> EvalResult;
}
