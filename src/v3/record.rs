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
pub struct Record {
    // message fields
    field_type: ::std::option::Option<i64>,
    crc: ::std::option::Option<u32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Record {}

impl Record {
    pub fn new() -> Record {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Record {
        static mut instance: ::protobuf::lazy::Lazy<Record> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Record,
        };
        unsafe {
            instance.get(Record::new)
        }
    }

    // optional int64 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i64) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i64 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.field_type
    }

    // optional uint32 crc = 2;

    pub fn clear_crc(&mut self) {
        self.crc = ::std::option::Option::None;
    }

    pub fn has_crc(&self) -> bool {
        self.crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crc(&mut self, v: u32) {
        self.crc = ::std::option::Option::Some(v);
    }

    pub fn get_crc(&self) -> u32 {
        self.crc.unwrap_or(0)
    }

    fn get_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.crc
    }

    fn mut_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.crc
    }

    // optional bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for Record {
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
                    let tmp = is.read_int64()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.crc = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.crc {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.crc {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for Record {
    fn new() -> Record {
        Record::new()
    }

    fn descriptor_static(_: ::std::option::Option<Record>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "type",
                    Record::get_field_type_for_reflect,
                    Record::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "crc",
                    Record::get_crc_for_reflect,
                    Record::mut_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Record::get_data_for_reflect,
                    Record::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Record>(
                    "Record",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Record {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_crc();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Record {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Record {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Snapshot {
    // message fields
    index: ::std::option::Option<u64>,
    term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Snapshot {}

impl Snapshot {
    pub fn new() -> Snapshot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Snapshot {
        static mut instance: ::protobuf::lazy::Lazy<Snapshot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snapshot,
        };
        unsafe {
            instance.get(Snapshot::new)
        }
    }

    // optional uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.index
    }

    // optional uint64 term = 2;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    fn get_term_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.term
    }
}

impl ::protobuf::Message for Snapshot {
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
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.term = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.term {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.term {
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

impl ::protobuf::MessageStatic for Snapshot {
    fn new() -> Snapshot {
        Snapshot::new()
    }

    fn descriptor_static(_: ::std::option::Option<Snapshot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    Snapshot::get_index_for_reflect,
                    Snapshot::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    Snapshot::get_term_for_reflect,
                    Snapshot::mut_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Snapshot>(
                    "Snapshot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Snapshot {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Snapshot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0crecord.proto\x12\x05walpb\x1a\x14gogoproto/gogo.proto\"N\n\x06Reco\
    rd\x12\x18\n\x04type\x18\x01\x20\x01(\x03R\x04typeB\x04\xc8\xde\x1f\0\
    \x12\x16\n\x03crc\x18\x02\x20\x01(\rR\x03crcB\x04\xc8\xde\x1f\0\x12\x12\
    \n\x04data\x18\x03\x20\x01(\x0cR\x04data\"@\n\x08Snapshot\x12\x1a\n\x05i\
    ndex\x18\x01\x20\x01(\x04R\x05indexB\x04\xc8\xde\x1f\0\x12\x18\n\x04term\
    \x18\x02\x20\x01(\x04R\x04termB\x04\xc8\xde\x1f\0B\x10\xc8\xe2\x1e\x01\
    \xd0\xe2\x1e\x01\xc8\xe1\x1e\0\xe0\xe2\x1e\x01J\xa0\t\n\x06\x12\x04\0\0\
    \x13\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\
    \r\n\t\n\x02\x03\0\x12\x03\x03\x07\x1d\n\x08\n\x01\x08\x12\x03\x05\0(\n\
    \x0b\n\x04\x08\xe7\x07\0\x12\x03\x05\0(\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x05\x07\x20\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x05\x07\x20\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x05\x08\x1f\n\x0c\n\x05\x08\
    \xe7\x07\0\x03\x12\x03\x05#'\n\x08\n\x01\x08\x12\x03\x06\0$\n\x0b\n\x04\
    \x08\xe7\x07\x01\x12\x03\x06\0$\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\
    \x06\x07\x1c\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x06\x07\x1c\n\x0e\n\
    \x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x06\x08\x1b\n\x0c\n\x05\x08\xe7\
    \x07\x01\x03\x12\x03\x06\x1f#\n\x08\n\x01\x08\x12\x03\x07\0*\n\x0b\n\x04\
    \x08\xe7\x07\x02\x12\x03\x07\0*\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\
    \x07\x07\"\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x07\x07\"\n\x0e\n\x07\
    \x08\xe7\x07\x02\x02\0\x01\x12\x03\x07\x08!\n\x0c\n\x05\x08\xe7\x07\x02\
    \x03\x12\x03\x07%)\n\x08\n\x01\x08\x12\x03\x08\0/\n\x0b\n\x04\x08\xe7\
    \x07\x03\x12\x03\x08\0/\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x08\x07&\
    \n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x08\x07&\n\x0e\n\x07\x08\xe7\
    \x07\x03\x02\0\x01\x12\x03\x08\x08%\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\
    \x03\x08).\n\n\n\x02\x04\0\x12\x04\n\0\x0e\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\n\x08\x0e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0b\x08@\n\x0c\n\x05\x04\
    \0\x02\0\x04\x12\x03\x0b\x08\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0b\
    \x11\x16\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0b\x17\x1b\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03\x0b\x1f\x20\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\x0b\
    !?\n\x0f\n\x08\x04\0\x02\0\x08\xe7\x07\0\x12\x03\x0b\">\n\x10\n\t\x04\0\
    \x02\0\x08\xe7\x07\0\x02\x12\x03\x0b\"6\n\x11\n\n\x04\0\x02\0\x08\xe7\
    \x07\0\x02\0\x12\x03\x0b\"6\n\x12\n\x0b\x04\0\x02\0\x08\xe7\x07\0\x02\0\
    \x01\x12\x03\x0b#5\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x03\x12\x03\x0b9>\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x08@\n\x0c\n\x05\x04\0\x02\x01\
    \x04\x12\x03\x0c\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0c\x11\
    \x17\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\x18\x1b\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x0c\x1f\x20\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\
    \x0c!?\n\x0f\n\x08\x04\0\x02\x01\x08\xe7\x07\0\x12\x03\x0c\">\n\x10\n\t\
    \x04\0\x02\x01\x08\xe7\x07\0\x02\x12\x03\x0c\"6\n\x11\n\n\x04\0\x02\x01\
    \x08\xe7\x07\0\x02\0\x12\x03\x0c\"6\n\x12\n\x0b\x04\0\x02\x01\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\x0c#5\n\x10\n\t\x04\0\x02\x01\x08\xe7\x07\0\x03\
    \x12\x03\x0c9>\n\x0b\n\x04\x04\0\x02\x02\x12\x03\r\x08!\n\x0c\n\x05\x04\
    \0\x02\x02\x04\x12\x03\r\x08\x10\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\r\
    \x11\x16\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\r\x17\x1b\n\x0c\n\x05\x04\
    \0\x02\x02\x03\x12\x03\r\x1f\x20\n\n\n\x02\x04\x01\x12\x04\x10\0\x13\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x10\x08\x10\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03\x11\x08A\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x11\x08\x10\n\x0c\n\
    \x05\x04\x01\x02\0\x05\x12\x03\x11\x11\x17\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x11\x18\x1d\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x11\x20!\n\
    \x0c\n\x05\x04\x01\x02\0\x08\x12\x03\x11\"@\n\x0f\n\x08\x04\x01\x02\0\
    \x08\xe7\x07\0\x12\x03\x11#?\n\x10\n\t\x04\x01\x02\0\x08\xe7\x07\0\x02\
    \x12\x03\x11#7\n\x11\n\n\x04\x01\x02\0\x08\xe7\x07\0\x02\0\x12\x03\x11#7\
    \n\x12\n\x0b\x04\x01\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03\x11$6\n\x10\n\
    \t\x04\x01\x02\0\x08\xe7\x07\0\x03\x12\x03\x11:?\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03\x12\x08A\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x12\x08\
    \x10\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x12\x11\x17\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03\x12\x18\x1c\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03\x12\x20!\n\x0c\n\x05\x04\x01\x02\x01\x08\x12\x03\x12\"@\n\x0f\n\x08\
    \x04\x01\x02\x01\x08\xe7\x07\0\x12\x03\x12#?\n\x10\n\t\x04\x01\x02\x01\
    \x08\xe7\x07\0\x02\x12\x03\x12#7\n\x11\n\n\x04\x01\x02\x01\x08\xe7\x07\0\
    \x02\0\x12\x03\x12#7\n\x12\n\x0b\x04\x01\x02\x01\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x12$6\n\x10\n\t\x04\x01\x02\x01\x08\xe7\x07\0\x03\x12\x03\x12:?\
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
