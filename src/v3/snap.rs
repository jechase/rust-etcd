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
pub struct snapshot {
    // message fields
    crc: ::std::option::Option<u32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for snapshot {}

impl snapshot {
    pub fn new() -> snapshot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static snapshot {
        static mut instance: ::protobuf::lazy::Lazy<snapshot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const snapshot,
        };
        unsafe {
            instance.get(snapshot::new)
        }
    }

    // optional uint32 crc = 1;

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

    // optional bytes data = 2;

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

impl ::protobuf::Message for snapshot {
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
                    let tmp = is.read_uint32()?;
                    self.crc = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        if let Some(v) = self.crc {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.crc {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for snapshot {
    fn new() -> snapshot {
        snapshot::new()
    }

    fn descriptor_static(_: ::std::option::Option<snapshot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "crc",
                    snapshot::get_crc_for_reflect,
                    snapshot::mut_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    snapshot::get_data_for_reflect,
                    snapshot::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<snapshot>(
                    "snapshot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for snapshot {
    fn clear(&mut self) {
        self.clear_crc();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for snapshot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for snapshot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nsnap.proto\x12\x06snappb\x1a\x14gogoproto/gogo.proto\"6\n\x08snapsho\
    t\x12\x16\n\x03crc\x18\x01\x20\x01(\rR\x03crcB\x04\xc8\xde\x1f\0\x12\x12\
    \n\x04data\x18\x02\x20\x01(\x0cR\x04dataB\x10\xe0\xe2\x1e\x01\xd0\xe2\
    \x1e\x01\xc8\xe1\x1e\0\xc8\xe2\x1e\x01J\xfb\x04\n\x06\x12\x04\0\0\r\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0e\n\t\
    \n\x02\x03\0\x12\x03\x03\x07\x1d\n\x08\n\x01\x08\x12\x03\x05\0(\n\x0b\n\
    \x04\x08\xe7\x07\0\x12\x03\x05\0(\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\
    \x05\x07\x20\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x05\x07\x20\n\x0e\n\
    \x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x05\x08\x1f\n\x0c\n\x05\x08\xe7\x07\
    \0\x03\x12\x03\x05#'\n\x08\n\x01\x08\x12\x03\x06\0$\n\x0b\n\x04\x08\xe7\
    \x07\x01\x12\x03\x06\0$\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x06\x07\
    \x1c\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x06\x07\x1c\n\x0e\n\x07\x08\
    \xe7\x07\x01\x02\0\x01\x12\x03\x06\x08\x1b\n\x0c\n\x05\x08\xe7\x07\x01\
    \x03\x12\x03\x06\x1f#\n\x08\n\x01\x08\x12\x03\x07\0*\n\x0b\n\x04\x08\xe7\
    \x07\x02\x12\x03\x07\0*\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x07\x07\
    \"\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x07\x07\"\n\x0e\n\x07\x08\xe7\
    \x07\x02\x02\0\x01\x12\x03\x07\x08!\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\
    \x03\x07%)\n\x08\n\x01\x08\x12\x03\x08\0/\n\x0b\n\x04\x08\xe7\x07\x03\
    \x12\x03\x08\0/\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x08\x07&\n\r\n\
    \x06\x08\xe7\x07\x03\x02\0\x12\x03\x08\x07&\n\x0e\n\x07\x08\xe7\x07\x03\
    \x02\0\x01\x12\x03\x08\x08%\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x08)\
    .\n\n\n\x02\x04\0\x12\x04\n\0\r\x01\n\n\n\x03\x04\0\x01\x12\x03\n\x08\
    \x10\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0b\x08@\n\x0c\n\x05\x04\0\x02\0\
    \x04\x12\x03\x0b\x08\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0b\x11\x17\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0b\x18\x1b\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x0b\x1f\x20\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\x0b!?\n\x0f\
    \n\x08\x04\0\x02\0\x08\xe7\x07\0\x12\x03\x0b\">\n\x10\n\t\x04\0\x02\0\
    \x08\xe7\x07\0\x02\x12\x03\x0b\"6\n\x11\n\n\x04\0\x02\0\x08\xe7\x07\0\
    \x02\0\x12\x03\x0b\"6\n\x12\n\x0b\x04\0\x02\0\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x0b#5\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x03\x12\x03\x0b9>\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x08!\n\x0c\n\x05\x04\0\x02\x01\x04\
    \x12\x03\x0c\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0c\x11\x16\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\x17\x1b\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x0c\x1f\x20\
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
