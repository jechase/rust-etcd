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
pub struct LockRequest {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub lease: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LockRequest {}

impl LockRequest {
    pub fn new() -> LockRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LockRequest {
        static mut instance: ::protobuf::lazy::Lazy<LockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LockRequest,
        };
        unsafe {
            instance.get(LockRequest::new)
        }
    }

    // bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // int64 lease = 2;

    pub fn clear_lease(&mut self) {
        self.lease = 0;
    }

    // Param is passed by value, moved
    pub fn set_lease(&mut self, v: i64) {
        self.lease = v;
    }

    pub fn get_lease(&self) -> i64 {
        self.lease
    }

    fn get_lease_for_reflect(&self) -> &i64 {
        &self.lease
    }

    fn mut_lease_for_reflect(&mut self) -> &mut i64 {
        &mut self.lease
    }
}

impl ::protobuf::Message for LockRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lease = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.name);
        }
        if self.lease != 0 {
            my_size += ::protobuf::rt::value_size(2, self.lease, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        }
        if self.lease != 0 {
            os.write_int64(2, self.lease)?;
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

impl ::protobuf::MessageStatic for LockRequest {
    fn new() -> LockRequest {
        LockRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LockRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    LockRequest::get_name_for_reflect,
                    LockRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lease",
                    LockRequest::get_lease_for_reflect,
                    LockRequest::mut_lease_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LockRequest>(
                    "LockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LockRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_lease();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LockResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::rpc::ResponseHeader>,
    pub key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LockResponse {}

impl LockResponse {
    pub fn new() -> LockResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LockResponse {
        static mut instance: ::protobuf::lazy::Lazy<LockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LockResponse,
        };
        unsafe {
            instance.get(LockResponse::new)
        }
    }

    // .etcdserverpb.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::rpc::ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::rpc::ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::rpc::ResponseHeader {
        self.header.take().unwrap_or_else(|| super::rpc::ResponseHeader::new())
    }

    pub fn get_header(&self) -> &super::rpc::ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| super::rpc::ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::ResponseHeader> {
        &mut self.header
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }
}

impl ::protobuf::Message for LockResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
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

impl ::protobuf::MessageStatic for LockResponse {
    fn new() -> LockResponse {
        LockResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LockResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::ResponseHeader>>(
                    "header",
                    LockResponse::get_header_for_reflect,
                    LockResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    LockResponse::get_key_for_reflect,
                    LockResponse::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LockResponse>(
                    "LockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LockResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnlockRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnlockRequest {}

impl UnlockRequest {
    pub fn new() -> UnlockRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnlockRequest {
        static mut instance: ::protobuf::lazy::Lazy<UnlockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnlockRequest,
        };
        unsafe {
            instance.get(UnlockRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }
}

impl ::protobuf::Message for UnlockRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
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

impl ::protobuf::MessageStatic for UnlockRequest {
    fn new() -> UnlockRequest {
        UnlockRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnlockRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    UnlockRequest::get_key_for_reflect,
                    UnlockRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnlockRequest>(
                    "UnlockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnlockRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnlockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnlockResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::rpc::ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnlockResponse {}

impl UnlockResponse {
    pub fn new() -> UnlockResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnlockResponse {
        static mut instance: ::protobuf::lazy::Lazy<UnlockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnlockResponse,
        };
        unsafe {
            instance.get(UnlockResponse::new)
        }
    }

    // .etcdserverpb.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::rpc::ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::rpc::ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::rpc::ResponseHeader {
        self.header.take().unwrap_or_else(|| super::rpc::ResponseHeader::new())
    }

    pub fn get_header(&self) -> &super::rpc::ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| super::rpc::ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for UnlockResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for UnlockResponse {
    fn new() -> UnlockResponse {
        UnlockResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnlockResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::ResponseHeader>>(
                    "header",
                    UnlockResponse::get_header_for_reflect,
                    UnlockResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnlockResponse>(
                    "UnlockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnlockResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnlockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cv3lock.proto\x12\x08v3lockpb\x1a\x14gogoproto/gogo.proto\x1a&etcd/\
    etcdserver/etcdserverpb/rpc.proto\x1a\x1cgoogle/api/annotations.proto\"7\
    \n\x0bLockRequest\x12\x12\n\x04name\x18\x01\x20\x01(\x0cR\x04name\x12\
    \x14\n\x05lease\x18\x02\x20\x01(\x03R\x05lease\"V\n\x0cLockResponse\x124\
    \n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06h\
    eader\x12\x10\n\x03key\x18\x02\x20\x01(\x0cR\x03key\"!\n\rUnlockRequest\
    \x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\"F\n\x0eUnlockResponse\
    \x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\
    \x06header2\xb8\x01\n\x04Lock\x12S\n\x04Lock\x12\x15.v3lockpb.LockReques\
    t\x1a\x16.v3lockpb.LockResponse\"\x1c\x82\xd3\xe4\x93\x02\x16\"\x11/v3be\
    ta/lock/lock:\x01*\x12[\n\x06Unlock\x12\x17.v3lockpb.UnlockRequest\x1a\
    \x18.v3lockpb.UnlockResponse\"\x1e\x82\xd3\xe4\x93\x02\x18\"\x13/v3beta/\
    lock/unlock:\x01*B\x08\xd0\xe2\x1e\x01\xc8\xe2\x1e\x01J\xbd\x12\n\x06\
    \x12\x04\0\0@\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\
    \x03\x01\x08\x10\n\t\n\x02\x03\0\x12\x03\x03\x07\x1d\n\t\n\x02\x03\x01\
    \x12\x03\x04\x07/\n\x1d\n\x02\x03\x02\x12\x03\x07\x07%\x1a\x12\x20for\
    \x20grpc-gateway\n\n\x08\n\x01\x08\x12\x03\t\0(\n\x0b\n\x04\x08\xe7\x07\
    \0\x12\x03\t\0(\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\t\x07\x20\n\r\n\
    \x06\x08\xe7\x07\0\x02\0\x12\x03\t\x07\x20\n\x0e\n\x07\x08\xe7\x07\0\x02\
    \0\x01\x12\x03\t\x08\x1f\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\t#'\n\x08\
    \n\x01\x08\x12\x03\n\0*\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\n\0*\n\x0c\n\
    \x05\x08\xe7\x07\x01\x02\x12\x03\n\x07\"\n\r\n\x06\x08\xe7\x07\x01\x02\0\
    \x12\x03\n\x07\"\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\n\x08!\n\
    \x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\n%)\nZ\n\x02\x06\0\x12\x04\r\0$\
    \x01\x1aN\x20The\x20lock\x20service\x20exposes\x20client-side\x20locking\
    \x20facilities\x20as\x20a\x20gRPC\x20interface.\n\n\n\n\x03\x06\0\x01\
    \x12\x03\r\x08\x0c\n\x95\x03\n\x04\x06\0\x02\0\x12\x04\x14\x02\x19\x03\
    \x1a\x86\x03\x20Lock\x20acquires\x20a\x20distributed\x20shared\x20lock\
    \x20on\x20a\x20given\x20named\x20lock.\n\x20On\x20success,\x20it\x20will\
    \x20return\x20a\x20unique\x20key\x20that\x20exists\x20so\x20long\x20as\
    \x20the\n\x20lock\x20is\x20held\x20by\x20the\x20caller.\x20This\x20key\
    \x20can\x20be\x20used\x20in\x20conjunction\x20with\n\x20transactions\x20\
    to\x20safely\x20ensure\x20updates\x20to\x20etcd\x20only\x20occur\x20whil\
    e\x20holding\n\x20lock\x20ownership.\x20The\x20lock\x20is\x20held\x20unt\
    il\x20Unlock\x20is\x20called\x20on\x20the\x20key\x20or\x20the\n\x20lease\
    \x20associate\x20with\x20the\x20owner\x20expires.\n\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03\x14\x06\n\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x14\x0b\
    \x16\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x14!-\n\r\n\x05\x06\0\x02\0\x04\
    \x12\x04\x15\x06\x18\x06\n\x10\n\x08\x06\0\x02\0\x04\xe7\x07\0\x12\x04\
    \x15\x06\x18\x06\n\x10\n\t\x06\0\x02\0\x04\xe7\x07\0\x02\x12\x03\x15\r\
    \x1e\n\x11\n\n\x06\0\x02\0\x04\xe7\x07\0\x02\0\x12\x03\x15\r\x1e\n\x12\n\
    \x0b\x06\0\x02\0\x04\xe7\x07\0\x02\0\x01\x12\x03\x15\x0e\x1d\n\x11\n\t\
    \x06\0\x02\0\x04\xe7\x07\0\x08\x12\x04\x15!\x18\x05\n\xb6\x01\n\x04\x06\
    \0\x02\x01\x12\x04\x1e\x02#\x03\x1a\xa7\x01\x20Unlock\x20takes\x20a\x20k\
    ey\x20returned\x20by\x20Lock\x20and\x20releases\x20the\x20hold\x20on\x20\
    lock.\x20The\n\x20next\x20Lock\x20caller\x20waiting\x20for\x20the\x20loc\
    k\x20will\x20then\x20be\x20woken\x20up\x20and\x20given\n\x20ownership\
    \x20of\x20the\x20lock.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x1e\x06\
    \x0c\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x1e\r\x1a\n\x0c\n\x05\x06\0\
    \x02\x01\x03\x12\x03\x1e%3\n\r\n\x05\x06\0\x02\x01\x04\x12\x04\x1f\x06\"\
    \x06\n\x10\n\x08\x06\0\x02\x01\x04\xe7\x07\0\x12\x04\x1f\x06\"\x06\n\x10\
    \n\t\x06\0\x02\x01\x04\xe7\x07\0\x02\x12\x03\x1f\r\x1e\n\x11\n\n\x06\0\
    \x02\x01\x04\xe7\x07\0\x02\0\x12\x03\x1f\r\x1e\n\x12\n\x0b\x06\0\x02\x01\
    \x04\xe7\x07\0\x02\0\x01\x12\x03\x1f\x0e\x1d\n\x11\n\t\x06\0\x02\x01\x04\
    \xe7\x07\0\x08\x12\x04\x1f!\"\x05\n\n\n\x02\x04\0\x12\x04&\0/\x01\n\n\n\
    \x03\x04\0\x01\x12\x03&\x08\x13\nU\n\x04\x04\0\x02\0\x12\x03(\x02\x11\
    \x1aH\x20name\x20is\x20the\x20identifier\x20for\x20the\x20distributed\
    \x20shared\x20lock\x20to\x20be\x20acquired.\n\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x04(\x02&\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03(\x02\x07\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03(\x08\x0c\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03(\x0f\x10\n\xbd\x02\n\x04\x04\0\x02\x01\x12\x03.\x02\x12\x1a\xaf\x02\
    \x20lease\x20is\x20the\x20ID\x20of\x20the\x20lease\x20that\x20will\x20be\
    \x20attached\x20to\x20ownership\x20of\x20the\n\x20lock.\x20If\x20the\x20\
    lease\x20expires\x20or\x20is\x20revoked\x20and\x20currently\x20holds\x20\
    the\x20lock,\n\x20the\x20lock\x20is\x20automatically\x20released.\x20Cal\
    ls\x20to\x20Lock\x20with\x20the\x20same\x20lease\x20will\n\x20be\x20trea\
    ted\x20as\x20a\x20single\x20acquistion;\x20locking\x20twice\x20with\x20t\
    he\x20same\x20lease\x20is\x20a\n\x20no-op.\n\n\r\n\x05\x04\0\x02\x01\x04\
    \x12\x04.\x02(\x11\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03.\x02\x07\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03.\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03.\x10\x11\n\n\n\x02\x04\x01\x12\x041\07\x01\n\n\n\x03\x04\x01\
    \x01\x12\x031\x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x032\x02)\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x042\x021\x16\n\x0c\n\x05\x04\x01\x02\0\x06\x12\
    \x032\x02\x1d\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x032\x1e$\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x032'(\n\xb8\x01\n\x04\x04\x01\x02\x01\x12\x036\
    \x02\x10\x1a\xaa\x01\x20key\x20is\x20a\x20key\x20that\x20will\x20exist\
    \x20on\x20etcd\x20for\x20the\x20duration\x20that\x20the\x20Lock\x20calle\
    r\n\x20owns\x20the\x20lock.\x20Users\x20should\x20not\x20modify\x20this\
    \x20key\x20or\x20the\x20lock\x20may\x20exhibit\n\x20undefined\x20behavio\
    r.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x046\x022)\n\x0c\n\x05\x04\x01\
    \x02\x01\x05\x12\x036\x02\x07\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x036\
    \x08\x0b\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x036\x0e\x0f\n\n\n\x02\x04\
    \x02\x12\x049\0<\x01\n\n\n\x03\x04\x02\x01\x12\x039\x08\x15\n=\n\x04\x04\
    \x02\x02\0\x12\x03;\x02\x10\x1a0\x20key\x20is\x20the\x20lock\x20ownershi\
    p\x20key\x20granted\x20by\x20Lock.\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\
    ;\x029\x17\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03;\x02\x07\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03;\x08\x0b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03;\x0e\x0f\n\n\n\x02\x04\x03\x12\x04>\0@\x01\n\n\n\x03\x04\x03\x01\
    \x12\x03>\x08\x16\n\x0b\n\x04\x04\x03\x02\0\x12\x03?\x02)\n\r\n\x05\x04\
    \x03\x02\0\x04\x12\x04?\x02>\x18\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03?\
    \x02\x1d\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03?\x1e$\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03?'(b\x06proto3\
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
