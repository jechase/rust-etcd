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
pub struct CampaignRequest {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub lease: i64,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CampaignRequest {}

impl CampaignRequest {
    pub fn new() -> CampaignRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CampaignRequest {
        static mut instance: ::protobuf::lazy::Lazy<CampaignRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CampaignRequest,
        };
        unsafe {
            instance.get(CampaignRequest::new)
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

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for CampaignRequest {
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
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
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
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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

impl ::protobuf::MessageStatic for CampaignRequest {
    fn new() -> CampaignRequest {
        CampaignRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CampaignRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    CampaignRequest::get_name_for_reflect,
                    CampaignRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lease",
                    CampaignRequest::get_lease_for_reflect,
                    CampaignRequest::mut_lease_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CampaignRequest::get_value_for_reflect,
                    CampaignRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CampaignRequest>(
                    "CampaignRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CampaignRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_lease();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CampaignRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CampaignRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CampaignResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::rpc::ResponseHeader>,
    pub leader: ::protobuf::SingularPtrField<LeaderKey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CampaignResponse {}

impl CampaignResponse {
    pub fn new() -> CampaignResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CampaignResponse {
        static mut instance: ::protobuf::lazy::Lazy<CampaignResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CampaignResponse,
        };
        unsafe {
            instance.get(CampaignResponse::new)
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

    // .v3electionpb.LeaderKey leader = 2;

    pub fn clear_leader(&mut self) {
        self.leader.clear();
    }

    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: LeaderKey) {
        self.leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader(&mut self) -> &mut LeaderKey {
        if self.leader.is_none() {
            self.leader.set_default();
        }
        self.leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_leader(&mut self) -> LeaderKey {
        self.leader.take().unwrap_or_else(|| LeaderKey::new())
    }

    pub fn get_leader(&self) -> &LeaderKey {
        self.leader.as_ref().unwrap_or_else(|| LeaderKey::default_instance())
    }

    fn get_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<LeaderKey> {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LeaderKey> {
        &mut self.leader
    }
}

impl ::protobuf::Message for CampaignResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.leader {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.leader)?;
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
        if let Some(ref v) = self.leader.as_ref() {
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
        if let Some(ref v) = self.leader.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CampaignResponse {
    fn new() -> CampaignResponse {
        CampaignResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CampaignResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::ResponseHeader>>(
                    "header",
                    CampaignResponse::get_header_for_reflect,
                    CampaignResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LeaderKey>>(
                    "leader",
                    CampaignResponse::get_leader_for_reflect,
                    CampaignResponse::mut_leader_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CampaignResponse>(
                    "CampaignResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CampaignResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CampaignResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CampaignResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaderKey {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub key: ::std::vec::Vec<u8>,
    pub rev: i64,
    pub lease: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaderKey {}

impl LeaderKey {
    pub fn new() -> LeaderKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaderKey {
        static mut instance: ::protobuf::lazy::Lazy<LeaderKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaderKey,
        };
        unsafe {
            instance.get(LeaderKey::new)
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

    // int64 rev = 3;

    pub fn clear_rev(&mut self) {
        self.rev = 0;
    }

    // Param is passed by value, moved
    pub fn set_rev(&mut self, v: i64) {
        self.rev = v;
    }

    pub fn get_rev(&self) -> i64 {
        self.rev
    }

    fn get_rev_for_reflect(&self) -> &i64 {
        &self.rev
    }

    fn mut_rev_for_reflect(&mut self) -> &mut i64 {
        &mut self.rev
    }

    // int64 lease = 4;

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

impl ::protobuf::Message for LeaderKey {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.rev = tmp;
                },
                4 => {
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if self.rev != 0 {
            my_size += ::protobuf::rt::value_size(3, self.rev, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lease != 0 {
            my_size += ::protobuf::rt::value_size(4, self.lease, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if self.rev != 0 {
            os.write_int64(3, self.rev)?;
        }
        if self.lease != 0 {
            os.write_int64(4, self.lease)?;
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

impl ::protobuf::MessageStatic for LeaderKey {
    fn new() -> LeaderKey {
        LeaderKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaderKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    LeaderKey::get_name_for_reflect,
                    LeaderKey::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    LeaderKey::get_key_for_reflect,
                    LeaderKey::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "rev",
                    LeaderKey::get_rev_for_reflect,
                    LeaderKey::mut_rev_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lease",
                    LeaderKey::get_lease_for_reflect,
                    LeaderKey::mut_lease_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaderKey>(
                    "LeaderKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaderKey {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_key();
        self.clear_rev();
        self.clear_lease();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaderKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaderKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaderRequest {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaderRequest {}

impl LeaderRequest {
    pub fn new() -> LeaderRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaderRequest {
        static mut instance: ::protobuf::lazy::Lazy<LeaderRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaderRequest,
        };
        unsafe {
            instance.get(LeaderRequest::new)
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
}

impl ::protobuf::Message for LeaderRequest {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
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

impl ::protobuf::MessageStatic for LeaderRequest {
    fn new() -> LeaderRequest {
        LeaderRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaderRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    LeaderRequest::get_name_for_reflect,
                    LeaderRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaderRequest>(
                    "LeaderRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaderRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaderRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaderRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaderResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::rpc::ResponseHeader>,
    pub kv: ::protobuf::SingularPtrField<super::kv::KeyValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaderResponse {}

impl LeaderResponse {
    pub fn new() -> LeaderResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaderResponse {
        static mut instance: ::protobuf::lazy::Lazy<LeaderResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaderResponse,
        };
        unsafe {
            instance.get(LeaderResponse::new)
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

    // .mvccpb.KeyValue kv = 2;

    pub fn clear_kv(&mut self) {
        self.kv.clear();
    }

    pub fn has_kv(&self) -> bool {
        self.kv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kv(&mut self, v: super::kv::KeyValue) {
        self.kv = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kv(&mut self) -> &mut super::kv::KeyValue {
        if self.kv.is_none() {
            self.kv.set_default();
        }
        self.kv.as_mut().unwrap()
    }

    // Take field
    pub fn take_kv(&mut self) -> super::kv::KeyValue {
        self.kv.take().unwrap_or_else(|| super::kv::KeyValue::new())
    }

    pub fn get_kv(&self) -> &super::kv::KeyValue {
        self.kv.as_ref().unwrap_or_else(|| super::kv::KeyValue::default_instance())
    }

    fn get_kv_for_reflect(&self) -> &::protobuf::SingularPtrField<super::kv::KeyValue> {
        &self.kv
    }

    fn mut_kv_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::kv::KeyValue> {
        &mut self.kv
    }
}

impl ::protobuf::Message for LeaderResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.kv {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.kv)?;
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
        if let Some(ref v) = self.kv.as_ref() {
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
        if let Some(ref v) = self.kv.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LeaderResponse {
    fn new() -> LeaderResponse {
        LeaderResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaderResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::ResponseHeader>>(
                    "header",
                    LeaderResponse::get_header_for_reflect,
                    LeaderResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kv::KeyValue>>(
                    "kv",
                    LeaderResponse::get_kv_for_reflect,
                    LeaderResponse::mut_kv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaderResponse>(
                    "LeaderResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaderResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_kv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaderResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaderResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResignRequest {
    // message fields
    pub leader: ::protobuf::SingularPtrField<LeaderKey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResignRequest {}

impl ResignRequest {
    pub fn new() -> ResignRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResignRequest {
        static mut instance: ::protobuf::lazy::Lazy<ResignRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResignRequest,
        };
        unsafe {
            instance.get(ResignRequest::new)
        }
    }

    // .v3electionpb.LeaderKey leader = 1;

    pub fn clear_leader(&mut self) {
        self.leader.clear();
    }

    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: LeaderKey) {
        self.leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader(&mut self) -> &mut LeaderKey {
        if self.leader.is_none() {
            self.leader.set_default();
        }
        self.leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_leader(&mut self) -> LeaderKey {
        self.leader.take().unwrap_or_else(|| LeaderKey::new())
    }

    pub fn get_leader(&self) -> &LeaderKey {
        self.leader.as_ref().unwrap_or_else(|| LeaderKey::default_instance())
    }

    fn get_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<LeaderKey> {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LeaderKey> {
        &mut self.leader
    }
}

impl ::protobuf::Message for ResignRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.leader {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.leader)?;
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
        if let Some(ref v) = self.leader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.leader.as_ref() {
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

impl ::protobuf::MessageStatic for ResignRequest {
    fn new() -> ResignRequest {
        ResignRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResignRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LeaderKey>>(
                    "leader",
                    ResignRequest::get_leader_for_reflect,
                    ResignRequest::mut_leader_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResignRequest>(
                    "ResignRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResignRequest {
    fn clear(&mut self) {
        self.clear_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResignRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResignRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResignResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::rpc::ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResignResponse {}

impl ResignResponse {
    pub fn new() -> ResignResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResignResponse {
        static mut instance: ::protobuf::lazy::Lazy<ResignResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResignResponse,
        };
        unsafe {
            instance.get(ResignResponse::new)
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

impl ::protobuf::Message for ResignResponse {
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

impl ::protobuf::MessageStatic for ResignResponse {
    fn new() -> ResignResponse {
        ResignResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResignResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::ResponseHeader>>(
                    "header",
                    ResignResponse::get_header_for_reflect,
                    ResignResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResignResponse>(
                    "ResignResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResignResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResignResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResignResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProclaimRequest {
    // message fields
    pub leader: ::protobuf::SingularPtrField<LeaderKey>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProclaimRequest {}

impl ProclaimRequest {
    pub fn new() -> ProclaimRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProclaimRequest {
        static mut instance: ::protobuf::lazy::Lazy<ProclaimRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProclaimRequest,
        };
        unsafe {
            instance.get(ProclaimRequest::new)
        }
    }

    // .v3electionpb.LeaderKey leader = 1;

    pub fn clear_leader(&mut self) {
        self.leader.clear();
    }

    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: LeaderKey) {
        self.leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader(&mut self) -> &mut LeaderKey {
        if self.leader.is_none() {
            self.leader.set_default();
        }
        self.leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_leader(&mut self) -> LeaderKey {
        self.leader.take().unwrap_or_else(|| LeaderKey::new())
    }

    pub fn get_leader(&self) -> &LeaderKey {
        self.leader.as_ref().unwrap_or_else(|| LeaderKey::default_instance())
    }

    fn get_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<LeaderKey> {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LeaderKey> {
        &mut self.leader
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for ProclaimRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.leader {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.leader)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.leader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.leader.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
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

impl ::protobuf::MessageStatic for ProclaimRequest {
    fn new() -> ProclaimRequest {
        ProclaimRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProclaimRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LeaderKey>>(
                    "leader",
                    ProclaimRequest::get_leader_for_reflect,
                    ProclaimRequest::mut_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    ProclaimRequest::get_value_for_reflect,
                    ProclaimRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProclaimRequest>(
                    "ProclaimRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProclaimRequest {
    fn clear(&mut self) {
        self.clear_leader();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProclaimRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProclaimRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProclaimResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::rpc::ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProclaimResponse {}

impl ProclaimResponse {
    pub fn new() -> ProclaimResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProclaimResponse {
        static mut instance: ::protobuf::lazy::Lazy<ProclaimResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProclaimResponse,
        };
        unsafe {
            instance.get(ProclaimResponse::new)
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

impl ::protobuf::Message for ProclaimResponse {
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

impl ::protobuf::MessageStatic for ProclaimResponse {
    fn new() -> ProclaimResponse {
        ProclaimResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProclaimResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::ResponseHeader>>(
                    "header",
                    ProclaimResponse::get_header_for_reflect,
                    ProclaimResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProclaimResponse>(
                    "ProclaimResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProclaimResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProclaimResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProclaimResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10v3election.proto\x12\x0cv3electionpb\x1a\x14gogoproto/gogo.proto\
    \x1a&etcd/etcdserver/etcdserverpb/rpc.proto\x1a\x19etcd/mvcc/mvccpb/kv.p\
    roto\x1a\x1cgoogle/api/annotations.proto\"Q\n\x0fCampaignRequest\x12\x12\
    \n\x04name\x18\x01\x20\x01(\x0cR\x04name\x12\x14\n\x05lease\x18\x02\x20\
    \x01(\x03R\x05lease\x12\x14\n\x05value\x18\x03\x20\x01(\x0cR\x05value\"y\
    \n\x10CampaignResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdse\
    rverpb.ResponseHeaderR\x06header\x12/\n\x06leader\x18\x02\x20\x01(\x0b2\
    \x17.v3electionpb.LeaderKeyR\x06leader\"Y\n\tLeaderKey\x12\x12\n\x04name\
    \x18\x01\x20\x01(\x0cR\x04name\x12\x10\n\x03key\x18\x02\x20\x01(\x0cR\
    \x03key\x12\x10\n\x03rev\x18\x03\x20\x01(\x03R\x03rev\x12\x14\n\x05lease\
    \x18\x04\x20\x01(\x03R\x05lease\"#\n\rLeaderRequest\x12\x12\n\x04name\
    \x18\x01\x20\x01(\x0cR\x04name\"h\n\x0eLeaderResponse\x124\n\x06header\
    \x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12\
    \x20\n\x02kv\x18\x02\x20\x01(\x0b2\x10.mvccpb.KeyValueR\x02kv\"@\n\rResi\
    gnRequest\x12/\n\x06leader\x18\x01\x20\x01(\x0b2\x17.v3electionpb.Leader\
    KeyR\x06leader\"F\n\x0eResignResponse\x124\n\x06header\x18\x01\x20\x01(\
    \x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\"X\n\x0fProclaimRequest\
    \x12/\n\x06leader\x18\x01\x20\x01(\x0b2\x17.v3electionpb.LeaderKeyR\x06l\
    eader\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value\"H\n\x10Proclaim\
    Response\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.Respons\
    eHeaderR\x06header2\xab\x04\n\x08Election\x12o\n\x08Campaign\x12\x1d.v3e\
    lectionpb.CampaignRequest\x1a\x1e.v3electionpb.CampaignResponse\"$\x82\
    \xd3\xe4\x93\x02\x1e\"\x19/v3beta/election/campaign:\x01*\x12o\n\x08Proc\
    laim\x12\x1d.v3electionpb.ProclaimRequest\x1a\x1e.v3electionpb.ProclaimR\
    esponse\"$\x82\xd3\xe4\x93\x02\x1e\"\x19/v3beta/election/proclaim:\x01*\
    \x12g\n\x06Leader\x12\x1b.v3electionpb.LeaderRequest\x1a\x1c.v3electionp\
    b.LeaderResponse\"\"\x82\xd3\xe4\x93\x02\x1c\"\x17/v3beta/election/leade\
    r:\x01*\x12k\n\x07Observe\x12\x1b.v3electionpb.LeaderRequest\x1a\x1c.v3e\
    lectionpb.LeaderResponse\"#\x82\xd3\xe4\x93\x02\x1d\"\x18/v3beta/electio\
    n/observe:\x01*0\x01\x12g\n\x06Resign\x12\x1b.v3electionpb.ResignRequest\
    \x1a\x1c.v3electionpb.ResignResponse\"\"\x82\xd3\xe4\x93\x02\x1c\"\x17/v\
    3beta/election/resign:\x01*B\x08\xd0\xe2\x1e\x01\xc8\xe2\x1e\x01J\x95\"\
    \n\x06\x12\x04\0\0v\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x01\x08\x14\n\t\n\x02\x03\0\x12\x03\x03\x07\x1d\n\t\n\x02\x03\
    \x01\x12\x03\x04\x07/\n\t\n\x02\x03\x02\x12\x03\x05\x07\"\n\x1d\n\x02\
    \x03\x03\x12\x03\x08\x07%\x1a\x12\x20for\x20grpc-gateway\n\n\x08\n\x01\
    \x08\x12\x03\n\0(\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\n\0(\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\n\x07\x20\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\n\
    \x07\x20\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\n\x08\x1f\n\x0c\n\
    \x05\x08\xe7\x07\0\x03\x12\x03\n#'\n\x08\n\x01\x08\x12\x03\x0b\0*\n\x0b\
    \n\x04\x08\xe7\x07\x01\x12\x03\x0b\0*\n\x0c\n\x05\x08\xe7\x07\x01\x02\
    \x12\x03\x0b\x07\"\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x0b\x07\"\n\
    \x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x0b\x08!\n\x0c\n\x05\x08\
    \xe7\x07\x01\x03\x12\x03\x0b%)\n_\n\x02\x06\0\x12\x04\x0e\07\x01\x1aS\
    \x20The\x20election\x20service\x20exposes\x20client-side\x20election\x20\
    facilities\x20as\x20a\x20gRPC\x20interface.\n\n\n\n\x03\x06\0\x01\x12\
    \x03\x0e\x08\x10\n\xae\x02\n\x04\x06\0\x02\0\x12\x04\x13\x02\x18\x03\x1a\
    \x9f\x02\x20Campaign\x20waits\x20to\x20acquire\x20leadership\x20in\x20an\
    \x20election,\x20returning\x20a\x20LeaderKey\n\x20representing\x20the\
    \x20leadership\x20if\x20successful.\x20The\x20LeaderKey\x20can\x20then\
    \x20be\x20used\n\x20to\x20issue\x20new\x20values\x20on\x20the\x20electio\
    n,\x20transactionally\x20guard\x20API\x20requests\x20on\n\x20leadership\
    \x20still\x20being\x20held,\x20and\x20resign\x20from\x20the\x20election.\
    \n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x13\x06\x0e\n\x0c\n\x05\x06\0\x02\
    \0\x02\x12\x03\x13\x0f\x1e\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x13)9\n\r\
    \n\x05\x06\0\x02\0\x04\x12\x04\x14\x06\x17\x06\n\x10\n\x08\x06\0\x02\0\
    \x04\xe7\x07\0\x12\x04\x14\x06\x17\x06\n\x10\n\t\x06\0\x02\0\x04\xe7\x07\
    \0\x02\x12\x03\x14\r\x1e\n\x11\n\n\x06\0\x02\0\x04\xe7\x07\0\x02\0\x12\
    \x03\x14\r\x1e\n\x12\n\x0b\x06\0\x02\0\x04\xe7\x07\0\x02\0\x01\x12\x03\
    \x14\x0e\x1d\n\x11\n\t\x06\0\x02\0\x04\xe7\x07\0\x08\x12\x04\x14!\x17\
    \x05\nL\n\x04\x06\0\x02\x01\x12\x04\x1a\x02\x1f\x03\x1a>\x20Proclaim\x20\
    updates\x20the\x20leader's\x20posted\x20value\x20with\x20a\x20new\x20val\
    ue.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x1a\x06\x0e\n\x0c\n\x05\x06\
    \0\x02\x01\x02\x12\x03\x1a\x0f\x1e\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\
    \x1a)9\n\r\n\x05\x06\0\x02\x01\x04\x12\x04\x1b\x06\x1e\x06\n\x10\n\x08\
    \x06\0\x02\x01\x04\xe7\x07\0\x12\x04\x1b\x06\x1e\x06\n\x10\n\t\x06\0\x02\
    \x01\x04\xe7\x07\0\x02\x12\x03\x1b\r\x1e\n\x11\n\n\x06\0\x02\x01\x04\xe7\
    \x07\0\x02\0\x12\x03\x1b\r\x1e\n\x12\n\x0b\x06\0\x02\x01\x04\xe7\x07\0\
    \x02\0\x01\x12\x03\x1b\x0e\x1d\n\x11\n\t\x06\0\x02\x01\x04\xe7\x07\0\x08\
    \x12\x04\x1b!\x1e\x05\nI\n\x04\x06\0\x02\x02\x12\x04!\x02&\x03\x1a;\x20L\
    eader\x20returns\x20the\x20current\x20election\x20proclamation,\x20if\
    \x20any.\n\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03!\x06\x0c\n\x0c\n\x05\
    \x06\0\x02\x02\x02\x12\x03!\r\x1a\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03!\
    %3\n\r\n\x05\x06\0\x02\x02\x04\x12\x04\"\x06%\x06\n\x10\n\x08\x06\0\x02\
    \x02\x04\xe7\x07\0\x12\x04\"\x06%\x06\n\x10\n\t\x06\0\x02\x02\x04\xe7\
    \x07\0\x02\x12\x03\"\r\x1e\n\x11\n\n\x06\0\x02\x02\x04\xe7\x07\0\x02\0\
    \x12\x03\"\r\x1e\n\x12\n\x0b\x06\0\x02\x02\x04\xe7\x07\0\x02\0\x01\x12\
    \x03\"\x0e\x1d\n\x11\n\t\x06\0\x02\x02\x04\xe7\x07\0\x08\x12\x04\"!%\x05\
    \nk\n\x04\x06\0\x02\x03\x12\x04)\x02.\x03\x1a]\x20Observe\x20streams\x20\
    election\x20proclamations\x20in-order\x20as\x20made\x20by\x20the\x20elec\
    tion's\n\x20elected\x20leaders.\n\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03)\
    \x06\r\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03)\x0e\x1b\n\x0c\n\x05\x06\0\
    \x02\x03\x06\x12\x03)&,\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03)-;\n\r\n\
    \x05\x06\0\x02\x03\x04\x12\x04*\x06-\x06\n\x10\n\x08\x06\0\x02\x03\x04\
    \xe7\x07\0\x12\x04*\x06-\x06\n\x10\n\t\x06\0\x02\x03\x04\xe7\x07\0\x02\
    \x12\x03*\r\x1e\n\x11\n\n\x06\0\x02\x03\x04\xe7\x07\0\x02\0\x12\x03*\r\
    \x1e\n\x12\n\x0b\x06\0\x02\x03\x04\xe7\x07\0\x02\0\x01\x12\x03*\x0e\x1d\
    \n\x11\n\t\x06\0\x02\x03\x04\xe7\x07\0\x08\x12\x04*!-\x05\nq\n\x04\x06\0\
    \x02\x04\x12\x041\x026\x03\x1ac\x20Resign\x20releases\x20election\x20lea\
    dership\x20so\x20other\x20campaigners\x20may\x20acquire\n\x20leadership\
    \x20on\x20the\x20election.\n\n\x0c\n\x05\x06\0\x02\x04\x01\x12\x031\x06\
    \x0c\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x031\r\x1a\n\x0c\n\x05\x06\0\x02\
    \x04\x03\x12\x031%3\n\r\n\x05\x06\0\x02\x04\x04\x12\x042\x065\x06\n\x10\
    \n\x08\x06\0\x02\x04\x04\xe7\x07\0\x12\x042\x065\x06\n\x10\n\t\x06\0\x02\
    \x04\x04\xe7\x07\0\x02\x12\x032\r\x1e\n\x11\n\n\x06\0\x02\x04\x04\xe7\
    \x07\0\x02\0\x12\x032\r\x1e\n\x12\n\x0b\x06\0\x02\x04\x04\xe7\x07\0\x02\
    \0\x01\x12\x032\x0e\x1d\n\x11\n\t\x06\0\x02\x04\x04\xe7\x07\0\x08\x12\
    \x042!5\x05\n\n\n\x02\x04\0\x12\x049\0C\x01\n\n\n\x03\x04\0\x01\x12\x039\
    \x08\x17\nB\n\x04\x04\0\x02\0\x12\x03;\x02\x11\x1a5\x20name\x20is\x20the\
    \x20election's\x20identifier\x20for\x20the\x20campaign.\n\n\r\n\x05\x04\
    \0\x02\0\x04\x12\x04;\x029\x19\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03;\x02\
    \x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03;\x08\x0c\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03;\x0f\x10\n\xd9\x01\n\x04\x04\0\x02\x01\x12\x03?\x02\x12\
    \x1a\xcb\x01\x20lease\x20is\x20the\x20ID\x20of\x20the\x20lease\x20attach\
    ed\x20to\x20leadership\x20of\x20the\x20election.\x20If\x20the\n\x20lease\
    \x20expires\x20or\x20is\x20revoked\x20before\x20resigning\x20leadership,\
    \x20then\x20the\n\x20leadership\x20is\x20transferred\x20to\x20the\x20nex\
    t\x20campaigner,\x20if\x20any.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04?\
    \x02;\x11\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03?\x02\x07\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03?\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03?\x10\
    \x11\n`\n\x04\x04\0\x02\x02\x12\x03B\x02\x12\x1aS\x20value\x20is\x20the\
    \x20initial\x20proclaimed\x20value\x20set\x20when\x20the\x20campaigner\
    \x20wins\x20the\n\x20election.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04B\
    \x02?\x12\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03B\x02\x07\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x03B\x08\r\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03B\x10\
    \x11\n\n\n\x02\x04\x01\x12\x04E\0I\x01\n\n\n\x03\x04\x01\x01\x12\x03E\
    \x08\x18\n\x0b\n\x04\x04\x01\x02\0\x12\x03F\x02)\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04F\x02E\x1a\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03F\x02\x1d\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03F\x1e$\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03F'(\n[\n\x04\x04\x01\x02\x01\x12\x03H\x02\x17\x1aN\x20leader\x20\
    describes\x20the\x20resources\x20used\x20for\x20holding\x20leadereship\
    \x20of\x20the\x20election.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04H\x02F\
    )\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03H\x02\x0b\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03H\x0c\x12\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03H\
    \x15\x16\n\n\n\x02\x04\x02\x12\x04K\0W\x01\n\n\n\x03\x04\x02\x01\x12\x03\
    K\x08\x11\nU\n\x04\x04\x02\x02\0\x12\x03M\x02\x11\x1aH\x20name\x20is\x20\
    the\x20election\x20identifier\x20that\x20correponds\x20to\x20the\x20lead\
    ership\x20key.\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04M\x02K\x13\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03M\x02\x07\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03M\x08\x0c\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03M\x0f\x10\n\x80\
    \x01\n\x04\x04\x02\x02\x01\x12\x03P\x02\x10\x1as\x20key\x20is\x20an\x20o\
    paque\x20key\x20representing\x20the\x20ownership\x20of\x20the\x20electio\
    n.\x20If\x20the\x20key\n\x20is\x20deleted,\x20then\x20leadership\x20is\
    \x20lost.\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04P\x02M\x11\n\x0c\n\x05\
    \x04\x02\x02\x01\x05\x12\x03P\x02\x07\n\x0c\n\x05\x04\x02\x02\x01\x01\
    \x12\x03P\x08\x0b\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03P\x0e\x0f\n\xb6\
    \x01\n\x04\x04\x02\x02\x02\x12\x03T\x02\x10\x1a\xa8\x01\x20rev\x20is\x20\
    the\x20creation\x20revision\x20of\x20the\x20key.\x20It\x20can\x20be\x20u\
    sed\x20to\x20test\x20for\x20ownership\n\x20of\x20an\x20election\x20durin\
    g\x20transactions\x20by\x20testing\x20the\x20key's\x20creation\x20revisi\
    on\n\x20matches\x20rev.\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04T\x02P\
    \x10\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03T\x02\x07\n\x0c\n\x05\x04\
    \x02\x02\x02\x01\x12\x03T\x08\x0b\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\
    \x03T\x0e\x0f\n<\n\x04\x04\x02\x02\x03\x12\x03V\x02\x12\x1a/\x20lease\
    \x20is\x20the\x20lease\x20ID\x20of\x20the\x20election\x20leader.\n\n\r\n\
    \x05\x04\x02\x02\x03\x04\x12\x04V\x02T\x10\n\x0c\n\x05\x04\x02\x02\x03\
    \x05\x12\x03V\x02\x07\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03V\x08\r\n\
    \x0c\n\x05\x04\x02\x02\x03\x03\x12\x03V\x10\x11\n\n\n\x02\x04\x03\x12\
    \x04Y\0\\\x01\n\n\n\x03\x04\x03\x01\x12\x03Y\x08\x15\nN\n\x04\x04\x03\
    \x02\0\x12\x03[\x02\x11\x1aA\x20name\x20is\x20the\x20election\x20identif\
    ier\x20for\x20the\x20leadership\x20information.\n\n\r\n\x05\x04\x03\x02\
    \0\x04\x12\x04[\x02Y\x17\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03[\x02\x07\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03[\x08\x0c\n\x0c\n\x05\x04\x03\x02\
    \0\x03\x12\x03[\x0f\x10\n\n\n\x02\x04\x04\x12\x04^\0b\x01\n\n\n\x03\x04\
    \x04\x01\x12\x03^\x08\x16\n\x0b\n\x04\x04\x04\x02\0\x12\x03_\x02)\n\r\n\
    \x05\x04\x04\x02\0\x04\x12\x04_\x02^\x18\n\x0c\n\x05\x04\x04\x02\0\x06\
    \x12\x03_\x02\x1d\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03_\x1e$\n\x0c\n\
    \x05\x04\x04\x02\0\x03\x12\x03_'(\nN\n\x04\x04\x04\x02\x01\x12\x03a\x02\
    \x19\x1aA\x20kv\x20is\x20the\x20key-value\x20pair\x20representing\x20the\
    \x20latest\x20leader\x20update.\n\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04a\
    \x02_)\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03a\x02\x11\n\x0c\n\x05\x04\
    \x04\x02\x01\x01\x12\x03a\x12\x14\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\
    \x03a\x17\x18\n\n\n\x02\x04\x05\x12\x04d\0g\x01\n\n\n\x03\x04\x05\x01\
    \x12\x03d\x08\x15\nE\n\x04\x04\x05\x02\0\x12\x03f\x02\x17\x1a8\x20leader\
    \x20is\x20the\x20leadership\x20to\x20relinquish\x20by\x20resignation.\n\
    \n\r\n\x05\x04\x05\x02\0\x04\x12\x04f\x02d\x17\n\x0c\n\x05\x04\x05\x02\0\
    \x06\x12\x03f\x02\x0b\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03f\x0c\x12\n\
    \x0c\n\x05\x04\x05\x02\0\x03\x12\x03f\x15\x16\n\n\n\x02\x04\x06\x12\x04i\
    \0k\x01\n\n\n\x03\x04\x06\x01\x12\x03i\x08\x16\n\x0b\n\x04\x04\x06\x02\0\
    \x12\x03j\x02)\n\r\n\x05\x04\x06\x02\0\x04\x12\x04j\x02i\x18\n\x0c\n\x05\
    \x04\x06\x02\0\x06\x12\x03j\x02\x1d\n\x0c\n\x05\x04\x06\x02\0\x01\x12\
    \x03j\x1e$\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03j'(\n\n\n\x02\x04\x07\
    \x12\x04m\0r\x01\n\n\n\x03\x04\x07\x01\x12\x03m\x08\x17\n=\n\x04\x04\x07\
    \x02\0\x12\x03o\x02\x17\x1a0\x20leader\x20is\x20the\x20leadership\x20hol\
    d\x20on\x20the\x20election.\n\n\r\n\x05\x04\x07\x02\0\x04\x12\x04o\x02m\
    \x19\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03o\x02\x0b\n\x0c\n\x05\x04\x07\
    \x02\0\x01\x12\x03o\x0c\x12\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03o\x15\
    \x16\nP\n\x04\x04\x07\x02\x01\x12\x03q\x02\x12\x1aC\x20value\x20is\x20an\
    \x20update\x20meant\x20to\x20overwrite\x20the\x20leader's\x20current\x20\
    value.\n\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04q\x02o\x17\n\x0c\n\x05\x04\
    \x07\x02\x01\x05\x12\x03q\x02\x07\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\
    \x03q\x08\r\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03q\x10\x11\n\n\n\x02\
    \x04\x08\x12\x04t\0v\x01\n\n\n\x03\x04\x08\x01\x12\x03t\x08\x18\n\x0b\n\
    \x04\x04\x08\x02\0\x12\x03u\x02)\n\r\n\x05\x04\x08\x02\0\x04\x12\x04u\
    \x02t\x1a\n\x0c\n\x05\x04\x08\x02\0\x06\x12\x03u\x02\x1d\n\x0c\n\x05\x04\
    \x08\x02\0\x01\x12\x03u\x1e$\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03u'(b\
    \x06proto3\
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
