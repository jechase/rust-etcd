// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    ID: ::std::option::Option<u64>,
    Method: ::protobuf::SingularField<::std::string::String>,
    Path: ::protobuf::SingularField<::std::string::String>,
    Val: ::protobuf::SingularField<::std::string::String>,
    Dir: ::std::option::Option<bool>,
    PrevValue: ::protobuf::SingularField<::std::string::String>,
    PrevIndex: ::std::option::Option<u64>,
    PrevExist: ::std::option::Option<bool>,
    Expiration: ::std::option::Option<i64>,
    Wait: ::std::option::Option<bool>,
    Since: ::std::option::Option<u64>,
    Recursive: ::std::option::Option<bool>,
    Sorted: ::std::option::Option<bool>,
    Quorum: ::std::option::Option<bool>,
    Time: ::std::option::Option<i64>,
    Stream: ::std::option::Option<bool>,
    Refresh: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // optional uint64 ID = 1;

    pub fn clear_ID(&mut self) {
        self.ID = ::std::option::Option::None;
    }

    pub fn has_ID(&self) -> bool {
        self.ID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: u64) {
        self.ID = ::std::option::Option::Some(v);
    }

    pub fn get_ID(&self) -> u64 {
        self.ID.unwrap_or(0)
    }

    fn get_ID_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ID
    }

    // optional string Method = 2;

    pub fn clear_Method(&mut self) {
        self.Method.clear();
    }

    pub fn has_Method(&self) -> bool {
        self.Method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Method(&mut self, v: ::std::string::String) {
        self.Method = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Method(&mut self) -> &mut ::std::string::String {
        if self.Method.is_none() {
            self.Method.set_default();
        }
        self.Method.as_mut().unwrap()
    }

    // Take field
    pub fn take_Method(&mut self) -> ::std::string::String {
        self.Method.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_Method(&self) -> &str {
        match self.Method.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_Method_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.Method
    }

    fn mut_Method_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.Method
    }

    // optional string Path = 3;

    pub fn clear_Path(&mut self) {
        self.Path.clear();
    }

    pub fn has_Path(&self) -> bool {
        self.Path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Path(&mut self, v: ::std::string::String) {
        self.Path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Path(&mut self) -> &mut ::std::string::String {
        if self.Path.is_none() {
            self.Path.set_default();
        }
        self.Path.as_mut().unwrap()
    }

    // Take field
    pub fn take_Path(&mut self) -> ::std::string::String {
        self.Path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_Path(&self) -> &str {
        match self.Path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_Path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.Path
    }

    fn mut_Path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.Path
    }

    // optional string Val = 4;

    pub fn clear_Val(&mut self) {
        self.Val.clear();
    }

    pub fn has_Val(&self) -> bool {
        self.Val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Val(&mut self, v: ::std::string::String) {
        self.Val = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Val(&mut self) -> &mut ::std::string::String {
        if self.Val.is_none() {
            self.Val.set_default();
        }
        self.Val.as_mut().unwrap()
    }

    // Take field
    pub fn take_Val(&mut self) -> ::std::string::String {
        self.Val.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_Val(&self) -> &str {
        match self.Val.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_Val_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.Val
    }

    fn mut_Val_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.Val
    }

    // optional bool Dir = 5;

    pub fn clear_Dir(&mut self) {
        self.Dir = ::std::option::Option::None;
    }

    pub fn has_Dir(&self) -> bool {
        self.Dir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Dir(&mut self, v: bool) {
        self.Dir = ::std::option::Option::Some(v);
    }

    pub fn get_Dir(&self) -> bool {
        self.Dir.unwrap_or(false)
    }

    fn get_Dir_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.Dir
    }

    fn mut_Dir_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.Dir
    }

    // optional string PrevValue = 6;

    pub fn clear_PrevValue(&mut self) {
        self.PrevValue.clear();
    }

    pub fn has_PrevValue(&self) -> bool {
        self.PrevValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_PrevValue(&mut self, v: ::std::string::String) {
        self.PrevValue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_PrevValue(&mut self) -> &mut ::std::string::String {
        if self.PrevValue.is_none() {
            self.PrevValue.set_default();
        }
        self.PrevValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_PrevValue(&mut self) -> ::std::string::String {
        self.PrevValue.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_PrevValue(&self) -> &str {
        match self.PrevValue.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_PrevValue_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.PrevValue
    }

    fn mut_PrevValue_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.PrevValue
    }

    // optional uint64 PrevIndex = 7;

    pub fn clear_PrevIndex(&mut self) {
        self.PrevIndex = ::std::option::Option::None;
    }

    pub fn has_PrevIndex(&self) -> bool {
        self.PrevIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_PrevIndex(&mut self, v: u64) {
        self.PrevIndex = ::std::option::Option::Some(v);
    }

    pub fn get_PrevIndex(&self) -> u64 {
        self.PrevIndex.unwrap_or(0)
    }

    fn get_PrevIndex_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.PrevIndex
    }

    fn mut_PrevIndex_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.PrevIndex
    }

    // optional bool PrevExist = 8;

    pub fn clear_PrevExist(&mut self) {
        self.PrevExist = ::std::option::Option::None;
    }

    pub fn has_PrevExist(&self) -> bool {
        self.PrevExist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_PrevExist(&mut self, v: bool) {
        self.PrevExist = ::std::option::Option::Some(v);
    }

    pub fn get_PrevExist(&self) -> bool {
        self.PrevExist.unwrap_or(false)
    }

    fn get_PrevExist_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.PrevExist
    }

    fn mut_PrevExist_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.PrevExist
    }

    // optional int64 Expiration = 9;

    pub fn clear_Expiration(&mut self) {
        self.Expiration = ::std::option::Option::None;
    }

    pub fn has_Expiration(&self) -> bool {
        self.Expiration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Expiration(&mut self, v: i64) {
        self.Expiration = ::std::option::Option::Some(v);
    }

    pub fn get_Expiration(&self) -> i64 {
        self.Expiration.unwrap_or(0)
    }

    fn get_Expiration_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.Expiration
    }

    fn mut_Expiration_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.Expiration
    }

    // optional bool Wait = 10;

    pub fn clear_Wait(&mut self) {
        self.Wait = ::std::option::Option::None;
    }

    pub fn has_Wait(&self) -> bool {
        self.Wait.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Wait(&mut self, v: bool) {
        self.Wait = ::std::option::Option::Some(v);
    }

    pub fn get_Wait(&self) -> bool {
        self.Wait.unwrap_or(false)
    }

    fn get_Wait_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.Wait
    }

    fn mut_Wait_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.Wait
    }

    // optional uint64 Since = 11;

    pub fn clear_Since(&mut self) {
        self.Since = ::std::option::Option::None;
    }

    pub fn has_Since(&self) -> bool {
        self.Since.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Since(&mut self, v: u64) {
        self.Since = ::std::option::Option::Some(v);
    }

    pub fn get_Since(&self) -> u64 {
        self.Since.unwrap_or(0)
    }

    fn get_Since_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.Since
    }

    fn mut_Since_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.Since
    }

    // optional bool Recursive = 12;

    pub fn clear_Recursive(&mut self) {
        self.Recursive = ::std::option::Option::None;
    }

    pub fn has_Recursive(&self) -> bool {
        self.Recursive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Recursive(&mut self, v: bool) {
        self.Recursive = ::std::option::Option::Some(v);
    }

    pub fn get_Recursive(&self) -> bool {
        self.Recursive.unwrap_or(false)
    }

    fn get_Recursive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.Recursive
    }

    fn mut_Recursive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.Recursive
    }

    // optional bool Sorted = 13;

    pub fn clear_Sorted(&mut self) {
        self.Sorted = ::std::option::Option::None;
    }

    pub fn has_Sorted(&self) -> bool {
        self.Sorted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Sorted(&mut self, v: bool) {
        self.Sorted = ::std::option::Option::Some(v);
    }

    pub fn get_Sorted(&self) -> bool {
        self.Sorted.unwrap_or(false)
    }

    fn get_Sorted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.Sorted
    }

    fn mut_Sorted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.Sorted
    }

    // optional bool Quorum = 14;

    pub fn clear_Quorum(&mut self) {
        self.Quorum = ::std::option::Option::None;
    }

    pub fn has_Quorum(&self) -> bool {
        self.Quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Quorum(&mut self, v: bool) {
        self.Quorum = ::std::option::Option::Some(v);
    }

    pub fn get_Quorum(&self) -> bool {
        self.Quorum.unwrap_or(false)
    }

    fn get_Quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.Quorum
    }

    fn mut_Quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.Quorum
    }

    // optional int64 Time = 15;

    pub fn clear_Time(&mut self) {
        self.Time = ::std::option::Option::None;
    }

    pub fn has_Time(&self) -> bool {
        self.Time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Time(&mut self, v: i64) {
        self.Time = ::std::option::Option::Some(v);
    }

    pub fn get_Time(&self) -> i64 {
        self.Time.unwrap_or(0)
    }

    fn get_Time_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.Time
    }

    fn mut_Time_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.Time
    }

    // optional bool Stream = 16;

    pub fn clear_Stream(&mut self) {
        self.Stream = ::std::option::Option::None;
    }

    pub fn has_Stream(&self) -> bool {
        self.Stream.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Stream(&mut self, v: bool) {
        self.Stream = ::std::option::Option::Some(v);
    }

    pub fn get_Stream(&self) -> bool {
        self.Stream.unwrap_or(false)
    }

    fn get_Stream_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.Stream
    }

    fn mut_Stream_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.Stream
    }

    // optional bool Refresh = 17;

    pub fn clear_Refresh(&mut self) {
        self.Refresh = ::std::option::Option::None;
    }

    pub fn has_Refresh(&self) -> bool {
        self.Refresh.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Refresh(&mut self, v: bool) {
        self.Refresh = ::std::option::Option::Some(v);
    }

    pub fn get_Refresh(&self) -> bool {
        self.Refresh.unwrap_or(false)
    }

    fn get_Refresh_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.Refresh
    }

    fn mut_Refresh_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.Refresh
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ID = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.Method)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.Path)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.Val)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.Dir = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.PrevValue)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.PrevIndex = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.PrevExist = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.Expiration = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.Wait = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.Since = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.Recursive = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.Sorted = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.Quorum = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.Time = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.Stream = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.Refresh = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ID {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.Method.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.Path.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.Val.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.Dir {
            my_size += 2;
        }
        if let Some(ref v) = self.PrevValue.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.PrevIndex {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.PrevExist {
            my_size += 2;
        }
        if let Some(v) = self.Expiration {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.Wait {
            my_size += 2;
        }
        if let Some(v) = self.Since {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.Recursive {
            my_size += 2;
        }
        if let Some(v) = self.Sorted {
            my_size += 2;
        }
        if let Some(v) = self.Quorum {
            my_size += 2;
        }
        if let Some(v) = self.Time {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.Stream {
            my_size += 3;
        }
        if let Some(v) = self.Refresh {
            my_size += 3;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ID {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.Method.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.Path.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.Val.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.Dir {
            os.write_bool(5, v)?;
        }
        if let Some(ref v) = self.PrevValue.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.PrevIndex {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.PrevExist {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.Expiration {
            os.write_int64(9, v)?;
        }
        if let Some(v) = self.Wait {
            os.write_bool(10, v)?;
        }
        if let Some(v) = self.Since {
            os.write_uint64(11, v)?;
        }
        if let Some(v) = self.Recursive {
            os.write_bool(12, v)?;
        }
        if let Some(v) = self.Sorted {
            os.write_bool(13, v)?;
        }
        if let Some(v) = self.Quorum {
            os.write_bool(14, v)?;
        }
        if let Some(v) = self.Time {
            os.write_int64(15, v)?;
        }
        if let Some(v) = self.Stream {
            os.write_bool(16, v)?;
        }
        if let Some(v) = self.Refresh {
            os.write_bool(17, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ID",
                    Request::get_ID_for_reflect,
                    Request::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Method",
                    Request::get_Method_for_reflect,
                    Request::mut_Method_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Path",
                    Request::get_Path_for_reflect,
                    Request::mut_Path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Val",
                    Request::get_Val_for_reflect,
                    Request::mut_Val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "Dir",
                    Request::get_Dir_for_reflect,
                    Request::mut_Dir_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "PrevValue",
                    Request::get_PrevValue_for_reflect,
                    Request::mut_PrevValue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "PrevIndex",
                    Request::get_PrevIndex_for_reflect,
                    Request::mut_PrevIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "PrevExist",
                    Request::get_PrevExist_for_reflect,
                    Request::mut_PrevExist_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "Expiration",
                    Request::get_Expiration_for_reflect,
                    Request::mut_Expiration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "Wait",
                    Request::get_Wait_for_reflect,
                    Request::mut_Wait_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "Since",
                    Request::get_Since_for_reflect,
                    Request::mut_Since_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "Recursive",
                    Request::get_Recursive_for_reflect,
                    Request::mut_Recursive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "Sorted",
                    Request::get_Sorted_for_reflect,
                    Request::mut_Sorted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "Quorum",
                    Request::get_Quorum_for_reflect,
                    Request::mut_Quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "Time",
                    Request::get_Time_for_reflect,
                    Request::mut_Time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "Stream",
                    Request::get_Stream_for_reflect,
                    Request::mut_Stream_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "Refresh",
                    Request::get_Refresh_for_reflect,
                    Request::mut_Refresh_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_ID();
        self.clear_Method();
        self.clear_Path();
        self.clear_Val();
        self.clear_Dir();
        self.clear_PrevValue();
        self.clear_PrevIndex();
        self.clear_PrevExist();
        self.clear_Expiration();
        self.clear_Wait();
        self.clear_Since();
        self.clear_Recursive();
        self.clear_Sorted();
        self.clear_Quorum();
        self.clear_Time();
        self.clear_Stream();
        self.clear_Refresh();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    NodeID: ::std::option::Option<u64>,
    ClusterID: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metadata {}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metadata {
        static mut instance: ::protobuf::lazy::Lazy<Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metadata,
        };
        unsafe {
            instance.get(Metadata::new)
        }
    }

    // optional uint64 NodeID = 1;

    pub fn clear_NodeID(&mut self) {
        self.NodeID = ::std::option::Option::None;
    }

    pub fn has_NodeID(&self) -> bool {
        self.NodeID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_NodeID(&mut self, v: u64) {
        self.NodeID = ::std::option::Option::Some(v);
    }

    pub fn get_NodeID(&self) -> u64 {
        self.NodeID.unwrap_or(0)
    }

    fn get_NodeID_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.NodeID
    }

    fn mut_NodeID_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.NodeID
    }

    // optional uint64 ClusterID = 2;

    pub fn clear_ClusterID(&mut self) {
        self.ClusterID = ::std::option::Option::None;
    }

    pub fn has_ClusterID(&self) -> bool {
        self.ClusterID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ClusterID(&mut self, v: u64) {
        self.ClusterID = ::std::option::Option::Some(v);
    }

    pub fn get_ClusterID(&self) -> u64 {
        self.ClusterID.unwrap_or(0)
    }

    fn get_ClusterID_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ClusterID
    }

    fn mut_ClusterID_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ClusterID
    }
}

impl ::protobuf::Message for Metadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.NodeID = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ClusterID = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.NodeID {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ClusterID {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.NodeID {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.ClusterID {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Metadata {
    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "NodeID",
                    Metadata::get_NodeID_for_reflect,
                    Metadata::mut_NodeID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ClusterID",
                    Metadata::get_ClusterID_for_reflect,
                    Metadata::mut_ClusterID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metadata>(
                    "Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.clear_NodeID();
        self.clear_ClusterID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10etcdserver.proto\x12\x0cetcdserverpb\x1a\x14gogoproto/gogo.proto\"\
    \x87\x04\n\x07Request\x12\x14\n\x02ID\x18\x01\x20\x01(\x04R\x02IDB\x04\
    \xc8\xde\x1f\0\x12\x1c\n\x06Method\x18\x02\x20\x01(\tR\x06MethodB\x04\
    \xc8\xde\x1f\0\x12\x18\n\x04Path\x18\x03\x20\x01(\tR\x04PathB\x04\xc8\
    \xde\x1f\0\x12\x16\n\x03Val\x18\x04\x20\x01(\tR\x03ValB\x04\xc8\xde\x1f\
    \0\x12\x16\n\x03Dir\x18\x05\x20\x01(\x08R\x03DirB\x04\xc8\xde\x1f\0\x12\
    \"\n\tPrevValue\x18\x06\x20\x01(\tR\tPrevValueB\x04\xc8\xde\x1f\0\x12\"\
    \n\tPrevIndex\x18\x07\x20\x01(\x04R\tPrevIndexB\x04\xc8\xde\x1f\0\x12\"\
    \n\tPrevExist\x18\x08\x20\x01(\x08R\tPrevExistB\x04\xc8\xde\x1f\x01\x12$\
    \n\nExpiration\x18\t\x20\x01(\x03R\nExpirationB\x04\xc8\xde\x1f\0\x12\
    \x18\n\x04Wait\x18\n\x20\x01(\x08R\x04WaitB\x04\xc8\xde\x1f\0\x12\x1a\n\
    \x05Since\x18\x0b\x20\x01(\x04R\x05SinceB\x04\xc8\xde\x1f\0\x12\"\n\tRec\
    ursive\x18\x0c\x20\x01(\x08R\tRecursiveB\x04\xc8\xde\x1f\0\x12\x1c\n\x06\
    Sorted\x18\r\x20\x01(\x08R\x06SortedB\x04\xc8\xde\x1f\0\x12\x1c\n\x06Quo\
    rum\x18\x0e\x20\x01(\x08R\x06QuorumB\x04\xc8\xde\x1f\0\x12\x18\n\x04Time\
    \x18\x0f\x20\x01(\x03R\x04TimeB\x04\xc8\xde\x1f\0\x12\x1c\n\x06Stream\
    \x18\x10\x20\x01(\x08R\x06StreamB\x04\xc8\xde\x1f\0\x12\x1e\n\x07Refresh\
    \x18\x11\x20\x01(\x08R\x07RefreshB\x04\xc8\xde\x1f\x01\"L\n\x08Metadata\
    \x12\x1c\n\x06NodeID\x18\x01\x20\x01(\x04R\x06NodeIDB\x04\xc8\xde\x1f\0\
    \x12\"\n\tClusterID\x18\x02\x20\x01(\x04R\tClusterIDB\x04\xc8\xde\x1f\0B\
    \x10\xc8\xe1\x1e\0\xc8\xe2\x1e\x01\xd0\xe2\x1e\x01\xe0\xe2\x1e\x01J\x9c\
    \x1d\n\x06\x12\x04\0\0!\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\
    \x02\x12\x03\x01\x08\x14\n\t\n\x02\x03\0\x12\x03\x03\x07\x1d\n\x08\n\x01\
    \x08\x12\x03\x05\0(\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x05\0(\n\x0c\n\x05\
    \x08\xe7\x07\0\x02\x12\x03\x05\x07\x20\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\
    \x03\x05\x07\x20\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x05\x08\x1f\
    \n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x05#'\n\x08\n\x01\x08\x12\x03\x06\
    \0$\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x06\0$\n\x0c\n\x05\x08\xe7\x07\
    \x01\x02\x12\x03\x06\x07\x1c\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x06\
    \x07\x1c\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x06\x08\x1b\n\x0c\
    \n\x05\x08\xe7\x07\x01\x03\x12\x03\x06\x1f#\n\x08\n\x01\x08\x12\x03\x07\
    \0*\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x07\0*\n\x0c\n\x05\x08\xe7\x07\
    \x02\x02\x12\x03\x07\x07\"\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x07\
    \x07\"\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x07\x08!\n\x0c\n\
    \x05\x08\xe7\x07\x02\x03\x12\x03\x07%)\n\x08\n\x01\x08\x12\x03\x08\0/\n\
    \x0b\n\x04\x08\xe7\x07\x03\x12\x03\x08\0/\n\x0c\n\x05\x08\xe7\x07\x03\
    \x02\x12\x03\x08\x07&\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x08\x07&\n\
    \x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x08\x08%\n\x0c\n\x05\x08\
    \xe7\x07\x03\x03\x12\x03\x08).\n\n\n\x02\x04\0\x12\x04\n\0\x1c\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\n\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0b\x08\
    G\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0b\x08\x10\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03\x0b\x11\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0b\x18\
    \x1a\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b&'\n\x0c\n\x05\x04\0\x02\0\
    \x08\x12\x03\x0b(F\n\x0f\n\x08\x04\0\x02\0\x08\xe7\x07\0\x12\x03\x0b)E\n\
    \x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x02\x12\x03\x0b)=\n\x11\n\n\x04\0\x02\
    \0\x08\xe7\x07\0\x02\0\x12\x03\x0b)=\n\x12\n\x0b\x04\0\x02\0\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x0b*<\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x03\x12\
    \x03\x0b@E\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x08G\n\x0c\n\x05\x04\0\
    \x02\x01\x04\x12\x03\x0c\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x0c\x11\x17\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\x18\x1e\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x0c&'\n\x0c\n\x05\x04\0\x02\x01\x08\x12\
    \x03\x0c(F\n\x0f\n\x08\x04\0\x02\x01\x08\xe7\x07\0\x12\x03\x0c)E\n\x10\n\
    \t\x04\0\x02\x01\x08\xe7\x07\0\x02\x12\x03\x0c)=\n\x11\n\n\x04\0\x02\x01\
    \x08\xe7\x07\0\x02\0\x12\x03\x0c)=\n\x12\n\x0b\x04\0\x02\x01\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x0c*<\n\x10\n\t\x04\0\x02\x01\x08\xe7\x07\0\x03\x12\
    \x03\x0c@E\n\x0b\n\x04\x04\0\x02\x02\x12\x03\r\x08G\n\x0c\n\x05\x04\0\
    \x02\x02\x04\x12\x03\r\x08\x10\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\r\
    \x11\x17\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\r\x18\x1c\n\x0c\n\x05\x04\
    \0\x02\x02\x03\x12\x03\r&'\n\x0c\n\x05\x04\0\x02\x02\x08\x12\x03\r(F\n\
    \x0f\n\x08\x04\0\x02\x02\x08\xe7\x07\0\x12\x03\r)E\n\x10\n\t\x04\0\x02\
    \x02\x08\xe7\x07\0\x02\x12\x03\r)=\n\x11\n\n\x04\0\x02\x02\x08\xe7\x07\0\
    \x02\0\x12\x03\r)=\n\x12\n\x0b\x04\0\x02\x02\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\r*<\n\x10\n\t\x04\0\x02\x02\x08\xe7\x07\0\x03\x12\x03\r@E\n\x0b\n\
    \x04\x04\0\x02\x03\x12\x03\x0e\x08G\n\x0c\n\x05\x04\0\x02\x03\x04\x12\
    \x03\x0e\x08\x10\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0e\x11\x17\n\x0c\
    \n\x05\x04\0\x02\x03\x01\x12\x03\x0e\x18\x1b\n\x0c\n\x05\x04\0\x02\x03\
    \x03\x12\x03\x0e&'\n\x0c\n\x05\x04\0\x02\x03\x08\x12\x03\x0e(F\n\x0f\n\
    \x08\x04\0\x02\x03\x08\xe7\x07\0\x12\x03\x0e)E\n\x10\n\t\x04\0\x02\x03\
    \x08\xe7\x07\0\x02\x12\x03\x0e)=\n\x11\n\n\x04\0\x02\x03\x08\xe7\x07\0\
    \x02\0\x12\x03\x0e)=\n\x12\n\x0b\x04\0\x02\x03\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x0e*<\n\x10\n\t\x04\0\x02\x03\x08\xe7\x07\0\x03\x12\x03\x0e@E\n\
    \x0b\n\x04\x04\0\x02\x04\x12\x03\x0f\x08G\n\x0c\n\x05\x04\0\x02\x04\x04\
    \x12\x03\x0f\x08\x10\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x0f\x11\x15\n\
    \x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x0f\x18\x1b\n\x0c\n\x05\x04\0\x02\
    \x04\x03\x12\x03\x0f&'\n\x0c\n\x05\x04\0\x02\x04\x08\x12\x03\x0f(F\n\x0f\
    \n\x08\x04\0\x02\x04\x08\xe7\x07\0\x12\x03\x0f)E\n\x10\n\t\x04\0\x02\x04\
    \x08\xe7\x07\0\x02\x12\x03\x0f)=\n\x11\n\n\x04\0\x02\x04\x08\xe7\x07\0\
    \x02\0\x12\x03\x0f)=\n\x12\n\x0b\x04\0\x02\x04\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x0f*<\n\x10\n\t\x04\0\x02\x04\x08\xe7\x07\0\x03\x12\x03\x0f@E\n\
    \x0b\n\x04\x04\0\x02\x05\x12\x03\x10\x08G\n\x0c\n\x05\x04\0\x02\x05\x04\
    \x12\x03\x10\x08\x10\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x10\x11\x17\n\
    \x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x10\x18!\n\x0c\n\x05\x04\0\x02\x05\
    \x03\x12\x03\x10&'\n\x0c\n\x05\x04\0\x02\x05\x08\x12\x03\x10(F\n\x0f\n\
    \x08\x04\0\x02\x05\x08\xe7\x07\0\x12\x03\x10)E\n\x10\n\t\x04\0\x02\x05\
    \x08\xe7\x07\0\x02\x12\x03\x10)=\n\x11\n\n\x04\0\x02\x05\x08\xe7\x07\0\
    \x02\0\x12\x03\x10)=\n\x12\n\x0b\x04\0\x02\x05\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x10*<\n\x10\n\t\x04\0\x02\x05\x08\xe7\x07\0\x03\x12\x03\x10@E\n\
    \x0b\n\x04\x04\0\x02\x06\x12\x03\x11\x08G\n\x0c\n\x05\x04\0\x02\x06\x04\
    \x12\x03\x11\x08\x10\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x11\x11\x17\n\
    \x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x11\x18!\n\x0c\n\x05\x04\0\x02\x06\
    \x03\x12\x03\x11&'\n\x0c\n\x05\x04\0\x02\x06\x08\x12\x03\x11(F\n\x0f\n\
    \x08\x04\0\x02\x06\x08\xe7\x07\0\x12\x03\x11)E\n\x10\n\t\x04\0\x02\x06\
    \x08\xe7\x07\0\x02\x12\x03\x11)=\n\x11\n\n\x04\0\x02\x06\x08\xe7\x07\0\
    \x02\0\x12\x03\x11)=\n\x12\n\x0b\x04\0\x02\x06\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x11*<\n\x10\n\t\x04\0\x02\x06\x08\xe7\x07\0\x03\x12\x03\x11@E\n\
    \x0b\n\x04\x04\0\x02\x07\x12\x03\x12\x08F\n\x0c\n\x05\x04\0\x02\x07\x04\
    \x12\x03\x12\x08\x10\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x12\x11\x15\n\
    \x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x12\x18!\n\x0c\n\x05\x04\0\x02\x07\
    \x03\x12\x03\x12&'\n\x0c\n\x05\x04\0\x02\x07\x08\x12\x03\x12(E\n\x0f\n\
    \x08\x04\0\x02\x07\x08\xe7\x07\0\x12\x03\x12)D\n\x10\n\t\x04\0\x02\x07\
    \x08\xe7\x07\0\x02\x12\x03\x12)=\n\x11\n\n\x04\0\x02\x07\x08\xe7\x07\0\
    \x02\0\x12\x03\x12)=\n\x12\n\x0b\x04\0\x02\x07\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x12*<\n\x10\n\t\x04\0\x02\x07\x08\xe7\x07\0\x03\x12\x03\x12@D\n\
    \x0b\n\x04\x04\0\x02\x08\x12\x03\x13\x08G\n\x0c\n\x05\x04\0\x02\x08\x04\
    \x12\x03\x13\x08\x10\n\x0c\n\x05\x04\0\x02\x08\x05\x12\x03\x13\x11\x16\n\
    \x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x13\x18\"\n\x0c\n\x05\x04\0\x02\x08\
    \x03\x12\x03\x13&'\n\x0c\n\x05\x04\0\x02\x08\x08\x12\x03\x13(F\n\x0f\n\
    \x08\x04\0\x02\x08\x08\xe7\x07\0\x12\x03\x13)E\n\x10\n\t\x04\0\x02\x08\
    \x08\xe7\x07\0\x02\x12\x03\x13)=\n\x11\n\n\x04\0\x02\x08\x08\xe7\x07\0\
    \x02\0\x12\x03\x13)=\n\x12\n\x0b\x04\0\x02\x08\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x13*<\n\x10\n\t\x04\0\x02\x08\x08\xe7\x07\0\x03\x12\x03\x13@E\n\
    \x0b\n\x04\x04\0\x02\t\x12\x03\x14\x08G\n\x0c\n\x05\x04\0\x02\t\x04\x12\
    \x03\x14\x08\x10\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\x14\x11\x15\n\x0c\n\
    \x05\x04\0\x02\t\x01\x12\x03\x14\x18\x1c\n\x0c\n\x05\x04\0\x02\t\x03\x12\
    \x03\x14%'\n\x0c\n\x05\x04\0\x02\t\x08\x12\x03\x14(F\n\x0f\n\x08\x04\0\
    \x02\t\x08\xe7\x07\0\x12\x03\x14)E\n\x10\n\t\x04\0\x02\t\x08\xe7\x07\0\
    \x02\x12\x03\x14)=\n\x11\n\n\x04\0\x02\t\x08\xe7\x07\0\x02\0\x12\x03\x14\
    )=\n\x12\n\x0b\x04\0\x02\t\x08\xe7\x07\0\x02\0\x01\x12\x03\x14*<\n\x10\n\
    \t\x04\0\x02\t\x08\xe7\x07\0\x03\x12\x03\x14@E\n\x0b\n\x04\x04\0\x02\n\
    \x12\x03\x15\x08G\n\x0c\n\x05\x04\0\x02\n\x04\x12\x03\x15\x08\x10\n\x0c\
    \n\x05\x04\0\x02\n\x05\x12\x03\x15\x11\x17\n\x0c\n\x05\x04\0\x02\n\x01\
    \x12\x03\x15\x18\x1d\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\x15%'\n\x0c\n\
    \x05\x04\0\x02\n\x08\x12\x03\x15(F\n\x0f\n\x08\x04\0\x02\n\x08\xe7\x07\0\
    \x12\x03\x15)E\n\x10\n\t\x04\0\x02\n\x08\xe7\x07\0\x02\x12\x03\x15)=\n\
    \x11\n\n\x04\0\x02\n\x08\xe7\x07\0\x02\0\x12\x03\x15)=\n\x12\n\x0b\x04\0\
    \x02\n\x08\xe7\x07\0\x02\0\x01\x12\x03\x15*<\n\x10\n\t\x04\0\x02\n\x08\
    \xe7\x07\0\x03\x12\x03\x15@E\n\x0b\n\x04\x04\0\x02\x0b\x12\x03\x16\x08G\
    \n\x0c\n\x05\x04\0\x02\x0b\x04\x12\x03\x16\x08\x10\n\x0c\n\x05\x04\0\x02\
    \x0b\x05\x12\x03\x16\x11\x15\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\x03\x16\
    \x18!\n\x0c\n\x05\x04\0\x02\x0b\x03\x12\x03\x16%'\n\x0c\n\x05\x04\0\x02\
    \x0b\x08\x12\x03\x16(F\n\x0f\n\x08\x04\0\x02\x0b\x08\xe7\x07\0\x12\x03\
    \x16)E\n\x10\n\t\x04\0\x02\x0b\x08\xe7\x07\0\x02\x12\x03\x16)=\n\x11\n\n\
    \x04\0\x02\x0b\x08\xe7\x07\0\x02\0\x12\x03\x16)=\n\x12\n\x0b\x04\0\x02\
    \x0b\x08\xe7\x07\0\x02\0\x01\x12\x03\x16*<\n\x10\n\t\x04\0\x02\x0b\x08\
    \xe7\x07\0\x03\x12\x03\x16@E\n\x0b\n\x04\x04\0\x02\x0c\x12\x03\x17\x08G\
    \n\x0c\n\x05\x04\0\x02\x0c\x04\x12\x03\x17\x08\x10\n\x0c\n\x05\x04\0\x02\
    \x0c\x05\x12\x03\x17\x11\x15\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03\x17\
    \x18\x1e\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03\x17%'\n\x0c\n\x05\x04\0\
    \x02\x0c\x08\x12\x03\x17(F\n\x0f\n\x08\x04\0\x02\x0c\x08\xe7\x07\0\x12\
    \x03\x17)E\n\x10\n\t\x04\0\x02\x0c\x08\xe7\x07\0\x02\x12\x03\x17)=\n\x11\
    \n\n\x04\0\x02\x0c\x08\xe7\x07\0\x02\0\x12\x03\x17)=\n\x12\n\x0b\x04\0\
    \x02\x0c\x08\xe7\x07\0\x02\0\x01\x12\x03\x17*<\n\x10\n\t\x04\0\x02\x0c\
    \x08\xe7\x07\0\x03\x12\x03\x17@E\n\x0b\n\x04\x04\0\x02\r\x12\x03\x18\x08\
    G\n\x0c\n\x05\x04\0\x02\r\x04\x12\x03\x18\x08\x10\n\x0c\n\x05\x04\0\x02\
    \r\x05\x12\x03\x18\x11\x15\n\x0c\n\x05\x04\0\x02\r\x01\x12\x03\x18\x18\
    \x1e\n\x0c\n\x05\x04\0\x02\r\x03\x12\x03\x18%'\n\x0c\n\x05\x04\0\x02\r\
    \x08\x12\x03\x18(F\n\x0f\n\x08\x04\0\x02\r\x08\xe7\x07\0\x12\x03\x18)E\n\
    \x10\n\t\x04\0\x02\r\x08\xe7\x07\0\x02\x12\x03\x18)=\n\x11\n\n\x04\0\x02\
    \r\x08\xe7\x07\0\x02\0\x12\x03\x18)=\n\x12\n\x0b\x04\0\x02\r\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x18*<\n\x10\n\t\x04\0\x02\r\x08\xe7\x07\0\x03\x12\
    \x03\x18@E\n\x0b\n\x04\x04\0\x02\x0e\x12\x03\x19\x08G\n\x0c\n\x05\x04\0\
    \x02\x0e\x04\x12\x03\x19\x08\x10\n\x0c\n\x05\x04\0\x02\x0e\x05\x12\x03\
    \x19\x11\x16\n\x0c\n\x05\x04\0\x02\x0e\x01\x12\x03\x19\x18\x1c\n\x0c\n\
    \x05\x04\0\x02\x0e\x03\x12\x03\x19%'\n\x0c\n\x05\x04\0\x02\x0e\x08\x12\
    \x03\x19(F\n\x0f\n\x08\x04\0\x02\x0e\x08\xe7\x07\0\x12\x03\x19)E\n\x10\n\
    \t\x04\0\x02\x0e\x08\xe7\x07\0\x02\x12\x03\x19)=\n\x11\n\n\x04\0\x02\x0e\
    \x08\xe7\x07\0\x02\0\x12\x03\x19)=\n\x12\n\x0b\x04\0\x02\x0e\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x19*<\n\x10\n\t\x04\0\x02\x0e\x08\xe7\x07\0\x03\x12\
    \x03\x19@E\n\x0b\n\x04\x04\0\x02\x0f\x12\x03\x1a\x08G\n\x0c\n\x05\x04\0\
    \x02\x0f\x04\x12\x03\x1a\x08\x10\n\x0c\n\x05\x04\0\x02\x0f\x05\x12\x03\
    \x1a\x11\x15\n\x0c\n\x05\x04\0\x02\x0f\x01\x12\x03\x1a\x18\x1e\n\x0c\n\
    \x05\x04\0\x02\x0f\x03\x12\x03\x1a%'\n\x0c\n\x05\x04\0\x02\x0f\x08\x12\
    \x03\x1a(F\n\x0f\n\x08\x04\0\x02\x0f\x08\xe7\x07\0\x12\x03\x1a)E\n\x10\n\
    \t\x04\0\x02\x0f\x08\xe7\x07\0\x02\x12\x03\x1a)=\n\x11\n\n\x04\0\x02\x0f\
    \x08\xe7\x07\0\x02\0\x12\x03\x1a)=\n\x12\n\x0b\x04\0\x02\x0f\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x1a*<\n\x10\n\t\x04\0\x02\x0f\x08\xe7\x07\0\x03\x12\
    \x03\x1a@E\n\x0b\n\x04\x04\0\x02\x10\x12\x03\x1b\x08F\n\x0c\n\x05\x04\0\
    \x02\x10\x04\x12\x03\x1b\x08\x10\n\x0c\n\x05\x04\0\x02\x10\x05\x12\x03\
    \x1b\x11\x15\n\x0c\n\x05\x04\0\x02\x10\x01\x12\x03\x1b\x18\x1f\n\x0c\n\
    \x05\x04\0\x02\x10\x03\x12\x03\x1b%'\n\x0c\n\x05\x04\0\x02\x10\x08\x12\
    \x03\x1b(E\n\x0f\n\x08\x04\0\x02\x10\x08\xe7\x07\0\x12\x03\x1b)D\n\x10\n\
    \t\x04\0\x02\x10\x08\xe7\x07\0\x02\x12\x03\x1b)=\n\x11\n\n\x04\0\x02\x10\
    \x08\xe7\x07\0\x02\0\x12\x03\x1b)=\n\x12\n\x0b\x04\0\x02\x10\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x1b*<\n\x10\n\t\x04\0\x02\x10\x08\xe7\x07\0\x03\x12\
    \x03\x1b@D\n\n\n\x02\x04\x01\x12\x04\x1e\0!\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x1e\x08\x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1f\x08E\n\x0c\n\
    \x05\x04\x01\x02\0\x04\x12\x03\x1f\x08\x10\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\x1f\x11\x17\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1f\x18\x1e\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1f$%\n\x0c\n\x05\x04\x01\x02\0\x08\
    \x12\x03\x1f&D\n\x0f\n\x08\x04\x01\x02\0\x08\xe7\x07\0\x12\x03\x1f'C\n\
    \x10\n\t\x04\x01\x02\0\x08\xe7\x07\0\x02\x12\x03\x1f';\n\x11\n\n\x04\x01\
    \x02\0\x08\xe7\x07\0\x02\0\x12\x03\x1f';\n\x12\n\x0b\x04\x01\x02\0\x08\
    \xe7\x07\0\x02\0\x01\x12\x03\x1f(:\n\x10\n\t\x04\x01\x02\0\x08\xe7\x07\0\
    \x03\x12\x03\x1f>C\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x20\x08E\n\x0c\n\
    \x05\x04\x01\x02\x01\x04\x12\x03\x20\x08\x10\n\x0c\n\x05\x04\x01\x02\x01\
    \x05\x12\x03\x20\x11\x17\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x20\x18\
    !\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x20$%\n\x0c\n\x05\x04\x01\x02\
    \x01\x08\x12\x03\x20&D\n\x0f\n\x08\x04\x01\x02\x01\x08\xe7\x07\0\x12\x03\
    \x20'C\n\x10\n\t\x04\x01\x02\x01\x08\xe7\x07\0\x02\x12\x03\x20';\n\x11\n\
    \n\x04\x01\x02\x01\x08\xe7\x07\0\x02\0\x12\x03\x20';\n\x12\n\x0b\x04\x01\
    \x02\x01\x08\xe7\x07\0\x02\0\x01\x12\x03\x20(:\n\x10\n\t\x04\x01\x02\x01\
    \x08\xe7\x07\0\x03\x12\x03\x20>C\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
