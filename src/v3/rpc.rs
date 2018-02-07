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
pub struct ResponseHeader {
    // message fields
    pub cluster_id: u64,
    pub member_id: u64,
    pub revision: i64,
    pub raft_term: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseHeader {}

impl ResponseHeader {
    pub fn new() -> ResponseHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseHeader {
        static mut instance: ::protobuf::lazy::Lazy<ResponseHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseHeader,
        };
        unsafe {
            instance.get(ResponseHeader::new)
        }
    }

    // uint64 cluster_id = 1;

    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = v;
    }

    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id
    }

    fn get_cluster_id_for_reflect(&self) -> &u64 {
        &self.cluster_id
    }

    fn mut_cluster_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.cluster_id
    }

    // uint64 member_id = 2;

    pub fn clear_member_id(&mut self) {
        self.member_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_member_id(&mut self, v: u64) {
        self.member_id = v;
    }

    pub fn get_member_id(&self) -> u64 {
        self.member_id
    }

    fn get_member_id_for_reflect(&self) -> &u64 {
        &self.member_id
    }

    fn mut_member_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.member_id
    }

    // int64 revision = 3;

    pub fn clear_revision(&mut self) {
        self.revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: i64) {
        self.revision = v;
    }

    pub fn get_revision(&self) -> i64 {
        self.revision
    }

    fn get_revision_for_reflect(&self) -> &i64 {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.revision
    }

    // uint64 raft_term = 4;

    pub fn clear_raft_term(&mut self) {
        self.raft_term = 0;
    }

    // Param is passed by value, moved
    pub fn set_raft_term(&mut self, v: u64) {
        self.raft_term = v;
    }

    pub fn get_raft_term(&self) -> u64 {
        self.raft_term
    }

    fn get_raft_term_for_reflect(&self) -> &u64 {
        &self.raft_term
    }

    fn mut_raft_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.raft_term
    }
}

impl ::protobuf::Message for ResponseHeader {
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
                    self.cluster_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.member_id = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.revision = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.raft_term = tmp;
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
        if self.cluster_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.cluster_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.member_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.member_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.revision != 0 {
            my_size += ::protobuf::rt::value_size(3, self.revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.raft_term != 0 {
            my_size += ::protobuf::rt::value_size(4, self.raft_term, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cluster_id != 0 {
            os.write_uint64(1, self.cluster_id)?;
        }
        if self.member_id != 0 {
            os.write_uint64(2, self.member_id)?;
        }
        if self.revision != 0 {
            os.write_int64(3, self.revision)?;
        }
        if self.raft_term != 0 {
            os.write_uint64(4, self.raft_term)?;
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

impl ::protobuf::MessageStatic for ResponseHeader {
    fn new() -> ResponseHeader {
        ResponseHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cluster_id",
                    ResponseHeader::get_cluster_id_for_reflect,
                    ResponseHeader::mut_cluster_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "member_id",
                    ResponseHeader::get_member_id_for_reflect,
                    ResponseHeader::mut_member_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "revision",
                    ResponseHeader::get_revision_for_reflect,
                    ResponseHeader::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "raft_term",
                    ResponseHeader::get_raft_term_for_reflect,
                    ResponseHeader::mut_raft_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseHeader>(
                    "ResponseHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseHeader {
    fn clear(&mut self) {
        self.clear_cluster_id();
        self.clear_member_id();
        self.clear_revision();
        self.clear_raft_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RangeRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub range_end: ::std::vec::Vec<u8>,
    pub limit: i64,
    pub revision: i64,
    pub sort_order: RangeRequest_SortOrder,
    pub sort_target: RangeRequest_SortTarget,
    pub serializable: bool,
    pub keys_only: bool,
    pub count_only: bool,
    pub min_mod_revision: i64,
    pub max_mod_revision: i64,
    pub min_create_revision: i64,
    pub max_create_revision: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RangeRequest {}

impl RangeRequest {
    pub fn new() -> RangeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RangeRequest {
        static mut instance: ::protobuf::lazy::Lazy<RangeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RangeRequest,
        };
        unsafe {
            instance.get(RangeRequest::new)
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

    // bytes range_end = 2;

    pub fn clear_range_end(&mut self) {
        self.range_end.clear();
    }

    // Param is passed by value, moved
    pub fn set_range_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // Take field
    pub fn take_range_end(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.range_end, ::std::vec::Vec::new())
    }

    pub fn get_range_end(&self) -> &[u8] {
        &self.range_end
    }

    fn get_range_end_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.range_end
    }

    fn mut_range_end_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // int64 limit = 3;

    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: i64) {
        self.limit = v;
    }

    pub fn get_limit(&self) -> i64 {
        self.limit
    }

    fn get_limit_for_reflect(&self) -> &i64 {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut i64 {
        &mut self.limit
    }

    // int64 revision = 4;

    pub fn clear_revision(&mut self) {
        self.revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: i64) {
        self.revision = v;
    }

    pub fn get_revision(&self) -> i64 {
        self.revision
    }

    fn get_revision_for_reflect(&self) -> &i64 {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.revision
    }

    // .etcdserverpb.RangeRequest.SortOrder sort_order = 5;

    pub fn clear_sort_order(&mut self) {
        self.sort_order = RangeRequest_SortOrder::NONE;
    }

    // Param is passed by value, moved
    pub fn set_sort_order(&mut self, v: RangeRequest_SortOrder) {
        self.sort_order = v;
    }

    pub fn get_sort_order(&self) -> RangeRequest_SortOrder {
        self.sort_order
    }

    fn get_sort_order_for_reflect(&self) -> &RangeRequest_SortOrder {
        &self.sort_order
    }

    fn mut_sort_order_for_reflect(&mut self) -> &mut RangeRequest_SortOrder {
        &mut self.sort_order
    }

    // .etcdserverpb.RangeRequest.SortTarget sort_target = 6;

    pub fn clear_sort_target(&mut self) {
        self.sort_target = RangeRequest_SortTarget::KEY;
    }

    // Param is passed by value, moved
    pub fn set_sort_target(&mut self, v: RangeRequest_SortTarget) {
        self.sort_target = v;
    }

    pub fn get_sort_target(&self) -> RangeRequest_SortTarget {
        self.sort_target
    }

    fn get_sort_target_for_reflect(&self) -> &RangeRequest_SortTarget {
        &self.sort_target
    }

    fn mut_sort_target_for_reflect(&mut self) -> &mut RangeRequest_SortTarget {
        &mut self.sort_target
    }

    // bool serializable = 7;

    pub fn clear_serializable(&mut self) {
        self.serializable = false;
    }

    // Param is passed by value, moved
    pub fn set_serializable(&mut self, v: bool) {
        self.serializable = v;
    }

    pub fn get_serializable(&self) -> bool {
        self.serializable
    }

    fn get_serializable_for_reflect(&self) -> &bool {
        &self.serializable
    }

    fn mut_serializable_for_reflect(&mut self) -> &mut bool {
        &mut self.serializable
    }

    // bool keys_only = 8;

    pub fn clear_keys_only(&mut self) {
        self.keys_only = false;
    }

    // Param is passed by value, moved
    pub fn set_keys_only(&mut self, v: bool) {
        self.keys_only = v;
    }

    pub fn get_keys_only(&self) -> bool {
        self.keys_only
    }

    fn get_keys_only_for_reflect(&self) -> &bool {
        &self.keys_only
    }

    fn mut_keys_only_for_reflect(&mut self) -> &mut bool {
        &mut self.keys_only
    }

    // bool count_only = 9;

    pub fn clear_count_only(&mut self) {
        self.count_only = false;
    }

    // Param is passed by value, moved
    pub fn set_count_only(&mut self, v: bool) {
        self.count_only = v;
    }

    pub fn get_count_only(&self) -> bool {
        self.count_only
    }

    fn get_count_only_for_reflect(&self) -> &bool {
        &self.count_only
    }

    fn mut_count_only_for_reflect(&mut self) -> &mut bool {
        &mut self.count_only
    }

    // int64 min_mod_revision = 10;

    pub fn clear_min_mod_revision(&mut self) {
        self.min_mod_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_mod_revision(&mut self, v: i64) {
        self.min_mod_revision = v;
    }

    pub fn get_min_mod_revision(&self) -> i64 {
        self.min_mod_revision
    }

    fn get_min_mod_revision_for_reflect(&self) -> &i64 {
        &self.min_mod_revision
    }

    fn mut_min_mod_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.min_mod_revision
    }

    // int64 max_mod_revision = 11;

    pub fn clear_max_mod_revision(&mut self) {
        self.max_mod_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_mod_revision(&mut self, v: i64) {
        self.max_mod_revision = v;
    }

    pub fn get_max_mod_revision(&self) -> i64 {
        self.max_mod_revision
    }

    fn get_max_mod_revision_for_reflect(&self) -> &i64 {
        &self.max_mod_revision
    }

    fn mut_max_mod_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.max_mod_revision
    }

    // int64 min_create_revision = 12;

    pub fn clear_min_create_revision(&mut self) {
        self.min_create_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_create_revision(&mut self, v: i64) {
        self.min_create_revision = v;
    }

    pub fn get_min_create_revision(&self) -> i64 {
        self.min_create_revision
    }

    fn get_min_create_revision_for_reflect(&self) -> &i64 {
        &self.min_create_revision
    }

    fn mut_min_create_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.min_create_revision
    }

    // int64 max_create_revision = 13;

    pub fn clear_max_create_revision(&mut self) {
        self.max_create_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_create_revision(&mut self, v: i64) {
        self.max_create_revision = v;
    }

    pub fn get_max_create_revision(&self) -> i64 {
        self.max_create_revision
    }

    fn get_max_create_revision_for_reflect(&self) -> &i64 {
        &self.max_create_revision
    }

    fn mut_max_create_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.max_create_revision
    }
}

impl ::protobuf::Message for RangeRequest {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.range_end)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.limit = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.revision = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.sort_order = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.sort_target = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.serializable = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.keys_only = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.count_only = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.min_mod_revision = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.max_mod_revision = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.min_create_revision = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.max_create_revision = tmp;
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
        if !self.range_end.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.range_end);
        }
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(3, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.revision != 0 {
            my_size += ::protobuf::rt::value_size(4, self.revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sort_order != RangeRequest_SortOrder::NONE {
            my_size += ::protobuf::rt::enum_size(5, self.sort_order);
        }
        if self.sort_target != RangeRequest_SortTarget::KEY {
            my_size += ::protobuf::rt::enum_size(6, self.sort_target);
        }
        if self.serializable != false {
            my_size += 2;
        }
        if self.keys_only != false {
            my_size += 2;
        }
        if self.count_only != false {
            my_size += 2;
        }
        if self.min_mod_revision != 0 {
            my_size += ::protobuf::rt::value_size(10, self.min_mod_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_mod_revision != 0 {
            my_size += ::protobuf::rt::value_size(11, self.max_mod_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.min_create_revision != 0 {
            my_size += ::protobuf::rt::value_size(12, self.min_create_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_create_revision != 0 {
            my_size += ::protobuf::rt::value_size(13, self.max_create_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.range_end.is_empty() {
            os.write_bytes(2, &self.range_end)?;
        }
        if self.limit != 0 {
            os.write_int64(3, self.limit)?;
        }
        if self.revision != 0 {
            os.write_int64(4, self.revision)?;
        }
        if self.sort_order != RangeRequest_SortOrder::NONE {
            os.write_enum(5, self.sort_order.value())?;
        }
        if self.sort_target != RangeRequest_SortTarget::KEY {
            os.write_enum(6, self.sort_target.value())?;
        }
        if self.serializable != false {
            os.write_bool(7, self.serializable)?;
        }
        if self.keys_only != false {
            os.write_bool(8, self.keys_only)?;
        }
        if self.count_only != false {
            os.write_bool(9, self.count_only)?;
        }
        if self.min_mod_revision != 0 {
            os.write_int64(10, self.min_mod_revision)?;
        }
        if self.max_mod_revision != 0 {
            os.write_int64(11, self.max_mod_revision)?;
        }
        if self.min_create_revision != 0 {
            os.write_int64(12, self.min_create_revision)?;
        }
        if self.max_create_revision != 0 {
            os.write_int64(13, self.max_create_revision)?;
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

impl ::protobuf::MessageStatic for RangeRequest {
    fn new() -> RangeRequest {
        RangeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RangeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RangeRequest::get_key_for_reflect,
                    RangeRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_end",
                    RangeRequest::get_range_end_for_reflect,
                    RangeRequest::mut_range_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "limit",
                    RangeRequest::get_limit_for_reflect,
                    RangeRequest::mut_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "revision",
                    RangeRequest::get_revision_for_reflect,
                    RangeRequest::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RangeRequest_SortOrder>>(
                    "sort_order",
                    RangeRequest::get_sort_order_for_reflect,
                    RangeRequest::mut_sort_order_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RangeRequest_SortTarget>>(
                    "sort_target",
                    RangeRequest::get_sort_target_for_reflect,
                    RangeRequest::mut_sort_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "serializable",
                    RangeRequest::get_serializable_for_reflect,
                    RangeRequest::mut_serializable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "keys_only",
                    RangeRequest::get_keys_only_for_reflect,
                    RangeRequest::mut_keys_only_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "count_only",
                    RangeRequest::get_count_only_for_reflect,
                    RangeRequest::mut_count_only_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "min_mod_revision",
                    RangeRequest::get_min_mod_revision_for_reflect,
                    RangeRequest::mut_min_mod_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "max_mod_revision",
                    RangeRequest::get_max_mod_revision_for_reflect,
                    RangeRequest::mut_max_mod_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "min_create_revision",
                    RangeRequest::get_min_create_revision_for_reflect,
                    RangeRequest::mut_min_create_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "max_create_revision",
                    RangeRequest::get_max_create_revision_for_reflect,
                    RangeRequest::mut_max_create_revision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RangeRequest>(
                    "RangeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RangeRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_range_end();
        self.clear_limit();
        self.clear_revision();
        self.clear_sort_order();
        self.clear_sort_target();
        self.clear_serializable();
        self.clear_keys_only();
        self.clear_count_only();
        self.clear_min_mod_revision();
        self.clear_max_mod_revision();
        self.clear_min_create_revision();
        self.clear_max_create_revision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RangeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RangeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RangeRequest_SortOrder {
    NONE = 0,
    ASCEND = 1,
    DESCEND = 2,
}

impl ::protobuf::ProtobufEnum for RangeRequest_SortOrder {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RangeRequest_SortOrder> {
        match value {
            0 => ::std::option::Option::Some(RangeRequest_SortOrder::NONE),
            1 => ::std::option::Option::Some(RangeRequest_SortOrder::ASCEND),
            2 => ::std::option::Option::Some(RangeRequest_SortOrder::DESCEND),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RangeRequest_SortOrder] = &[
            RangeRequest_SortOrder::NONE,
            RangeRequest_SortOrder::ASCEND,
            RangeRequest_SortOrder::DESCEND,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RangeRequest_SortOrder>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RangeRequest_SortOrder", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RangeRequest_SortOrder {
}

impl ::std::default::Default for RangeRequest_SortOrder {
    fn default() -> Self {
        RangeRequest_SortOrder::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for RangeRequest_SortOrder {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RangeRequest_SortTarget {
    KEY = 0,
    VERSION = 1,
    CREATE = 2,
    MOD = 3,
    VALUE = 4,
}

impl ::protobuf::ProtobufEnum for RangeRequest_SortTarget {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RangeRequest_SortTarget> {
        match value {
            0 => ::std::option::Option::Some(RangeRequest_SortTarget::KEY),
            1 => ::std::option::Option::Some(RangeRequest_SortTarget::VERSION),
            2 => ::std::option::Option::Some(RangeRequest_SortTarget::CREATE),
            3 => ::std::option::Option::Some(RangeRequest_SortTarget::MOD),
            4 => ::std::option::Option::Some(RangeRequest_SortTarget::VALUE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RangeRequest_SortTarget] = &[
            RangeRequest_SortTarget::KEY,
            RangeRequest_SortTarget::VERSION,
            RangeRequest_SortTarget::CREATE,
            RangeRequest_SortTarget::MOD,
            RangeRequest_SortTarget::VALUE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RangeRequest_SortTarget>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RangeRequest_SortTarget", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RangeRequest_SortTarget {
}

impl ::std::default::Default for RangeRequest_SortTarget {
    fn default() -> Self {
        RangeRequest_SortTarget::KEY
    }
}

impl ::protobuf::reflect::ProtobufValue for RangeRequest_SortTarget {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RangeResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub kvs: ::protobuf::RepeatedField<super::kv::KeyValue>,
    pub more: bool,
    pub count: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RangeResponse {}

impl RangeResponse {
    pub fn new() -> RangeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RangeResponse {
        static mut instance: ::protobuf::lazy::Lazy<RangeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RangeResponse,
        };
        unsafe {
            instance.get(RangeResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated .mvccpb.KeyValue kvs = 2;

    pub fn clear_kvs(&mut self) {
        self.kvs.clear();
    }

    // Param is passed by value, moved
    pub fn set_kvs(&mut self, v: ::protobuf::RepeatedField<super::kv::KeyValue>) {
        self.kvs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_kvs(&mut self) -> &mut ::protobuf::RepeatedField<super::kv::KeyValue> {
        &mut self.kvs
    }

    // Take field
    pub fn take_kvs(&mut self) -> ::protobuf::RepeatedField<super::kv::KeyValue> {
        ::std::mem::replace(&mut self.kvs, ::protobuf::RepeatedField::new())
    }

    pub fn get_kvs(&self) -> &[super::kv::KeyValue] {
        &self.kvs
    }

    fn get_kvs_for_reflect(&self) -> &::protobuf::RepeatedField<super::kv::KeyValue> {
        &self.kvs
    }

    fn mut_kvs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::kv::KeyValue> {
        &mut self.kvs
    }

    // bool more = 3;

    pub fn clear_more(&mut self) {
        self.more = false;
    }

    // Param is passed by value, moved
    pub fn set_more(&mut self, v: bool) {
        self.more = v;
    }

    pub fn get_more(&self) -> bool {
        self.more
    }

    fn get_more_for_reflect(&self) -> &bool {
        &self.more
    }

    fn mut_more_for_reflect(&mut self) -> &mut bool {
        &mut self.more
    }

    // int64 count = 4;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i64) {
        self.count = v;
    }

    pub fn get_count(&self) -> i64 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &i64 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut i64 {
        &mut self.count
    }
}

impl ::protobuf::Message for RangeResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.kvs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.kvs)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.more = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.count = tmp;
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
        for value in &self.kvs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.more != false {
            my_size += 2;
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(4, self.count, ::protobuf::wire_format::WireTypeVarint);
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
        for v in &self.kvs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.more != false {
            os.write_bool(3, self.more)?;
        }
        if self.count != 0 {
            os.write_int64(4, self.count)?;
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

impl ::protobuf::MessageStatic for RangeResponse {
    fn new() -> RangeResponse {
        RangeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RangeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    RangeResponse::get_header_for_reflect,
                    RangeResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kv::KeyValue>>(
                    "kvs",
                    RangeResponse::get_kvs_for_reflect,
                    RangeResponse::mut_kvs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "more",
                    RangeResponse::get_more_for_reflect,
                    RangeResponse::mut_more_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "count",
                    RangeResponse::get_count_for_reflect,
                    RangeResponse::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RangeResponse>(
                    "RangeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RangeResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_kvs();
        self.clear_more();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RangeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RangeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PutRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    pub lease: i64,
    pub prev_kv: bool,
    pub ignore_value: bool,
    pub ignore_lease: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutRequest {}

impl PutRequest {
    pub fn new() -> PutRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutRequest {
        static mut instance: ::protobuf::lazy::Lazy<PutRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutRequest,
        };
        unsafe {
            instance.get(PutRequest::new)
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

    // int64 lease = 3;

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

    // bool prev_kv = 4;

    pub fn clear_prev_kv(&mut self) {
        self.prev_kv = false;
    }

    // Param is passed by value, moved
    pub fn set_prev_kv(&mut self, v: bool) {
        self.prev_kv = v;
    }

    pub fn get_prev_kv(&self) -> bool {
        self.prev_kv
    }

    fn get_prev_kv_for_reflect(&self) -> &bool {
        &self.prev_kv
    }

    fn mut_prev_kv_for_reflect(&mut self) -> &mut bool {
        &mut self.prev_kv
    }

    // bool ignore_value = 5;

    pub fn clear_ignore_value(&mut self) {
        self.ignore_value = false;
    }

    // Param is passed by value, moved
    pub fn set_ignore_value(&mut self, v: bool) {
        self.ignore_value = v;
    }

    pub fn get_ignore_value(&self) -> bool {
        self.ignore_value
    }

    fn get_ignore_value_for_reflect(&self) -> &bool {
        &self.ignore_value
    }

    fn mut_ignore_value_for_reflect(&mut self) -> &mut bool {
        &mut self.ignore_value
    }

    // bool ignore_lease = 6;

    pub fn clear_ignore_lease(&mut self) {
        self.ignore_lease = false;
    }

    // Param is passed by value, moved
    pub fn set_ignore_lease(&mut self, v: bool) {
        self.ignore_lease = v;
    }

    pub fn get_ignore_lease(&self) -> bool {
        self.ignore_lease
    }

    fn get_ignore_lease_for_reflect(&self) -> &bool {
        &self.ignore_lease
    }

    fn mut_ignore_lease_for_reflect(&mut self) -> &mut bool {
        &mut self.ignore_lease
    }
}

impl ::protobuf::Message for PutRequest {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lease = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.prev_kv = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignore_value = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignore_lease = tmp;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        if self.lease != 0 {
            my_size += ::protobuf::rt::value_size(3, self.lease, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.prev_kv != false {
            my_size += 2;
        }
        if self.ignore_value != false {
            my_size += 2;
        }
        if self.ignore_lease != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        if self.lease != 0 {
            os.write_int64(3, self.lease)?;
        }
        if self.prev_kv != false {
            os.write_bool(4, self.prev_kv)?;
        }
        if self.ignore_value != false {
            os.write_bool(5, self.ignore_value)?;
        }
        if self.ignore_lease != false {
            os.write_bool(6, self.ignore_lease)?;
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

impl ::protobuf::MessageStatic for PutRequest {
    fn new() -> PutRequest {
        PutRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    PutRequest::get_key_for_reflect,
                    PutRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    PutRequest::get_value_for_reflect,
                    PutRequest::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lease",
                    PutRequest::get_lease_for_reflect,
                    PutRequest::mut_lease_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "prev_kv",
                    PutRequest::get_prev_kv_for_reflect,
                    PutRequest::mut_prev_kv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore_value",
                    PutRequest::get_ignore_value_for_reflect,
                    PutRequest::mut_ignore_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore_lease",
                    PutRequest::get_ignore_lease_for_reflect,
                    PutRequest::mut_ignore_lease_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutRequest>(
                    "PutRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.clear_lease();
        self.clear_prev_kv();
        self.clear_ignore_value();
        self.clear_ignore_lease();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PutRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PutRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PutResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub prev_kv: ::protobuf::SingularPtrField<super::kv::KeyValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutResponse {}

impl PutResponse {
    pub fn new() -> PutResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutResponse {
        static mut instance: ::protobuf::lazy::Lazy<PutResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutResponse,
        };
        unsafe {
            instance.get(PutResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // .mvccpb.KeyValue prev_kv = 2;

    pub fn clear_prev_kv(&mut self) {
        self.prev_kv.clear();
    }

    pub fn has_prev_kv(&self) -> bool {
        self.prev_kv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prev_kv(&mut self, v: super::kv::KeyValue) {
        self.prev_kv = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prev_kv(&mut self) -> &mut super::kv::KeyValue {
        if self.prev_kv.is_none() {
            self.prev_kv.set_default();
        }
        self.prev_kv.as_mut().unwrap()
    }

    // Take field
    pub fn take_prev_kv(&mut self) -> super::kv::KeyValue {
        self.prev_kv.take().unwrap_or_else(|| super::kv::KeyValue::new())
    }

    pub fn get_prev_kv(&self) -> &super::kv::KeyValue {
        self.prev_kv.as_ref().unwrap_or_else(|| super::kv::KeyValue::default_instance())
    }

    fn get_prev_kv_for_reflect(&self) -> &::protobuf::SingularPtrField<super::kv::KeyValue> {
        &self.prev_kv
    }

    fn mut_prev_kv_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::kv::KeyValue> {
        &mut self.prev_kv
    }
}

impl ::protobuf::Message for PutResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.prev_kv {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.prev_kv)?;
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
        if let Some(ref v) = self.prev_kv.as_ref() {
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
        if let Some(ref v) = self.prev_kv.as_ref() {
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

impl ::protobuf::MessageStatic for PutResponse {
    fn new() -> PutResponse {
        PutResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    PutResponse::get_header_for_reflect,
                    PutResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kv::KeyValue>>(
                    "prev_kv",
                    PutResponse::get_prev_kv_for_reflect,
                    PutResponse::mut_prev_kv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutResponse>(
                    "PutResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_prev_kv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PutResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PutResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteRangeRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub range_end: ::std::vec::Vec<u8>,
    pub prev_kv: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteRangeRequest {}

impl DeleteRangeRequest {
    pub fn new() -> DeleteRangeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteRangeRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteRangeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteRangeRequest,
        };
        unsafe {
            instance.get(DeleteRangeRequest::new)
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

    // bytes range_end = 2;

    pub fn clear_range_end(&mut self) {
        self.range_end.clear();
    }

    // Param is passed by value, moved
    pub fn set_range_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // Take field
    pub fn take_range_end(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.range_end, ::std::vec::Vec::new())
    }

    pub fn get_range_end(&self) -> &[u8] {
        &self.range_end
    }

    fn get_range_end_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.range_end
    }

    fn mut_range_end_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // bool prev_kv = 3;

    pub fn clear_prev_kv(&mut self) {
        self.prev_kv = false;
    }

    // Param is passed by value, moved
    pub fn set_prev_kv(&mut self, v: bool) {
        self.prev_kv = v;
    }

    pub fn get_prev_kv(&self) -> bool {
        self.prev_kv
    }

    fn get_prev_kv_for_reflect(&self) -> &bool {
        &self.prev_kv
    }

    fn mut_prev_kv_for_reflect(&mut self) -> &mut bool {
        &mut self.prev_kv
    }
}

impl ::protobuf::Message for DeleteRangeRequest {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.range_end)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.prev_kv = tmp;
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
        if !self.range_end.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.range_end);
        }
        if self.prev_kv != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.range_end.is_empty() {
            os.write_bytes(2, &self.range_end)?;
        }
        if self.prev_kv != false {
            os.write_bool(3, self.prev_kv)?;
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

impl ::protobuf::MessageStatic for DeleteRangeRequest {
    fn new() -> DeleteRangeRequest {
        DeleteRangeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteRangeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    DeleteRangeRequest::get_key_for_reflect,
                    DeleteRangeRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_end",
                    DeleteRangeRequest::get_range_end_for_reflect,
                    DeleteRangeRequest::mut_range_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "prev_kv",
                    DeleteRangeRequest::get_prev_kv_for_reflect,
                    DeleteRangeRequest::mut_prev_kv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteRangeRequest>(
                    "DeleteRangeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteRangeRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_range_end();
        self.clear_prev_kv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteRangeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteRangeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteRangeResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub deleted: i64,
    pub prev_kvs: ::protobuf::RepeatedField<super::kv::KeyValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteRangeResponse {}

impl DeleteRangeResponse {
    pub fn new() -> DeleteRangeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteRangeResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteRangeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteRangeResponse,
        };
        unsafe {
            instance.get(DeleteRangeResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // int64 deleted = 2;

    pub fn clear_deleted(&mut self) {
        self.deleted = 0;
    }

    // Param is passed by value, moved
    pub fn set_deleted(&mut self, v: i64) {
        self.deleted = v;
    }

    pub fn get_deleted(&self) -> i64 {
        self.deleted
    }

    fn get_deleted_for_reflect(&self) -> &i64 {
        &self.deleted
    }

    fn mut_deleted_for_reflect(&mut self) -> &mut i64 {
        &mut self.deleted
    }

    // repeated .mvccpb.KeyValue prev_kvs = 3;

    pub fn clear_prev_kvs(&mut self) {
        self.prev_kvs.clear();
    }

    // Param is passed by value, moved
    pub fn set_prev_kvs(&mut self, v: ::protobuf::RepeatedField<super::kv::KeyValue>) {
        self.prev_kvs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_prev_kvs(&mut self) -> &mut ::protobuf::RepeatedField<super::kv::KeyValue> {
        &mut self.prev_kvs
    }

    // Take field
    pub fn take_prev_kvs(&mut self) -> ::protobuf::RepeatedField<super::kv::KeyValue> {
        ::std::mem::replace(&mut self.prev_kvs, ::protobuf::RepeatedField::new())
    }

    pub fn get_prev_kvs(&self) -> &[super::kv::KeyValue] {
        &self.prev_kvs
    }

    fn get_prev_kvs_for_reflect(&self) -> &::protobuf::RepeatedField<super::kv::KeyValue> {
        &self.prev_kvs
    }

    fn mut_prev_kvs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::kv::KeyValue> {
        &mut self.prev_kvs
    }
}

impl ::protobuf::Message for DeleteRangeResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.prev_kvs {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.deleted = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.prev_kvs)?;
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
        if self.deleted != 0 {
            my_size += ::protobuf::rt::value_size(2, self.deleted, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.prev_kvs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        if self.deleted != 0 {
            os.write_int64(2, self.deleted)?;
        }
        for v in &self.prev_kvs {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for DeleteRangeResponse {
    fn new() -> DeleteRangeResponse {
        DeleteRangeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteRangeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    DeleteRangeResponse::get_header_for_reflect,
                    DeleteRangeResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "deleted",
                    DeleteRangeResponse::get_deleted_for_reflect,
                    DeleteRangeResponse::mut_deleted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kv::KeyValue>>(
                    "prev_kvs",
                    DeleteRangeResponse::get_prev_kvs_for_reflect,
                    DeleteRangeResponse::mut_prev_kvs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteRangeResponse>(
                    "DeleteRangeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteRangeResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_deleted();
        self.clear_prev_kvs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteRangeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteRangeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestOp {
    // message oneof groups
    request: ::std::option::Option<RequestOp_oneof_request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestOp {}

#[derive(Clone,PartialEq)]
pub enum RequestOp_oneof_request {
    request_range(RangeRequest),
    request_put(PutRequest),
    request_delete_range(DeleteRangeRequest),
    request_txn(TxnRequest),
}

impl RequestOp {
    pub fn new() -> RequestOp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestOp {
        static mut instance: ::protobuf::lazy::Lazy<RequestOp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestOp,
        };
        unsafe {
            instance.get(RequestOp::new)
        }
    }

    // .etcdserverpb.RangeRequest request_range = 1;

    pub fn clear_request_range(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_request_range(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_range(&mut self, v: RangeRequest) {
        self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_range(v))
    }

    // Mutable pointer to the field.
    pub fn mut_request_range(&mut self) -> &mut RangeRequest {
        if let ::std::option::Option::Some(RequestOp_oneof_request::request_range(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_range(RangeRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_range(&mut self) -> RangeRequest {
        if self.has_request_range() {
            match self.request.take() {
                ::std::option::Option::Some(RequestOp_oneof_request::request_range(v)) => v,
                _ => panic!(),
            }
        } else {
            RangeRequest::new()
        }
    }

    pub fn get_request_range(&self) -> &RangeRequest {
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_range(ref v)) => v,
            _ => RangeRequest::default_instance(),
        }
    }

    // .etcdserverpb.PutRequest request_put = 2;

    pub fn clear_request_put(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_request_put(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_put(&mut self, v: PutRequest) {
        self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_put(v))
    }

    // Mutable pointer to the field.
    pub fn mut_request_put(&mut self) -> &mut PutRequest {
        if let ::std::option::Option::Some(RequestOp_oneof_request::request_put(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_put(PutRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_put(&mut self) -> PutRequest {
        if self.has_request_put() {
            match self.request.take() {
                ::std::option::Option::Some(RequestOp_oneof_request::request_put(v)) => v,
                _ => panic!(),
            }
        } else {
            PutRequest::new()
        }
    }

    pub fn get_request_put(&self) -> &PutRequest {
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_put(ref v)) => v,
            _ => PutRequest::default_instance(),
        }
    }

    // .etcdserverpb.DeleteRangeRequest request_delete_range = 3;

    pub fn clear_request_delete_range(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_request_delete_range(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_delete_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_delete_range(&mut self, v: DeleteRangeRequest) {
        self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_delete_range(v))
    }

    // Mutable pointer to the field.
    pub fn mut_request_delete_range(&mut self) -> &mut DeleteRangeRequest {
        if let ::std::option::Option::Some(RequestOp_oneof_request::request_delete_range(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_delete_range(DeleteRangeRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_delete_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_delete_range(&mut self) -> DeleteRangeRequest {
        if self.has_request_delete_range() {
            match self.request.take() {
                ::std::option::Option::Some(RequestOp_oneof_request::request_delete_range(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRangeRequest::new()
        }
    }

    pub fn get_request_delete_range(&self) -> &DeleteRangeRequest {
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_delete_range(ref v)) => v,
            _ => DeleteRangeRequest::default_instance(),
        }
    }

    // .etcdserverpb.TxnRequest request_txn = 4;

    pub fn clear_request_txn(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_request_txn(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_txn(&mut self, v: TxnRequest) {
        self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_txn(v))
    }

    // Mutable pointer to the field.
    pub fn mut_request_txn(&mut self) -> &mut TxnRequest {
        if let ::std::option::Option::Some(RequestOp_oneof_request::request_txn(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_txn(TxnRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_txn(&mut self) -> TxnRequest {
        if self.has_request_txn() {
            match self.request.take() {
                ::std::option::Option::Some(RequestOp_oneof_request::request_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            TxnRequest::new()
        }
    }

    pub fn get_request_txn(&self) -> &TxnRequest {
        match self.request {
            ::std::option::Option::Some(RequestOp_oneof_request::request_txn(ref v)) => v,
            _ => TxnRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for RequestOp {
    fn is_initialized(&self) -> bool {
        if let Some(RequestOp_oneof_request::request_range(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RequestOp_oneof_request::request_put(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RequestOp_oneof_request::request_delete_range(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RequestOp_oneof_request::request_txn(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_range(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_put(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_delete_range(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(RequestOp_oneof_request::request_txn(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &RequestOp_oneof_request::request_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestOp_oneof_request::request_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestOp_oneof_request::request_delete_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestOp_oneof_request::request_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &RequestOp_oneof_request::request_range(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RequestOp_oneof_request::request_put(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RequestOp_oneof_request::request_delete_range(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RequestOp_oneof_request::request_txn(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for RequestOp {
    fn new() -> RequestOp {
        RequestOp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestOp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RangeRequest>(
                    "request_range",
                    RequestOp::has_request_range,
                    RequestOp::get_request_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PutRequest>(
                    "request_put",
                    RequestOp::has_request_put,
                    RequestOp::get_request_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteRangeRequest>(
                    "request_delete_range",
                    RequestOp::has_request_delete_range,
                    RequestOp::get_request_delete_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TxnRequest>(
                    "request_txn",
                    RequestOp::has_request_txn,
                    RequestOp::get_request_txn,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestOp>(
                    "RequestOp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestOp {
    fn clear(&mut self) {
        self.clear_request_range();
        self.clear_request_put();
        self.clear_request_delete_range();
        self.clear_request_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseOp {
    // message oneof groups
    response: ::std::option::Option<ResponseOp_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseOp {}

#[derive(Clone,PartialEq)]
pub enum ResponseOp_oneof_response {
    response_range(RangeResponse),
    response_put(PutResponse),
    response_delete_range(DeleteRangeResponse),
    response_txn(TxnResponse),
}

impl ResponseOp {
    pub fn new() -> ResponseOp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseOp {
        static mut instance: ::protobuf::lazy::Lazy<ResponseOp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseOp,
        };
        unsafe {
            instance.get(ResponseOp::new)
        }
    }

    // .etcdserverpb.RangeResponse response_range = 1;

    pub fn clear_response_range(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response_range(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_response_range(&mut self, v: RangeResponse) {
        self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_range(v))
    }

    // Mutable pointer to the field.
    pub fn mut_response_range(&mut self) -> &mut RangeResponse {
        if let ::std::option::Option::Some(ResponseOp_oneof_response::response_range(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_range(RangeResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_response_range(&mut self) -> RangeResponse {
        if self.has_response_range() {
            match self.response.take() {
                ::std::option::Option::Some(ResponseOp_oneof_response::response_range(v)) => v,
                _ => panic!(),
            }
        } else {
            RangeResponse::new()
        }
    }

    pub fn get_response_range(&self) -> &RangeResponse {
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_range(ref v)) => v,
            _ => RangeResponse::default_instance(),
        }
    }

    // .etcdserverpb.PutResponse response_put = 2;

    pub fn clear_response_put(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response_put(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_response_put(&mut self, v: PutResponse) {
        self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_put(v))
    }

    // Mutable pointer to the field.
    pub fn mut_response_put(&mut self) -> &mut PutResponse {
        if let ::std::option::Option::Some(ResponseOp_oneof_response::response_put(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_put(PutResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_response_put(&mut self) -> PutResponse {
        if self.has_response_put() {
            match self.response.take() {
                ::std::option::Option::Some(ResponseOp_oneof_response::response_put(v)) => v,
                _ => panic!(),
            }
        } else {
            PutResponse::new()
        }
    }

    pub fn get_response_put(&self) -> &PutResponse {
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_put(ref v)) => v,
            _ => PutResponse::default_instance(),
        }
    }

    // .etcdserverpb.DeleteRangeResponse response_delete_range = 3;

    pub fn clear_response_delete_range(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response_delete_range(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_delete_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_response_delete_range(&mut self, v: DeleteRangeResponse) {
        self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_delete_range(v))
    }

    // Mutable pointer to the field.
    pub fn mut_response_delete_range(&mut self) -> &mut DeleteRangeResponse {
        if let ::std::option::Option::Some(ResponseOp_oneof_response::response_delete_range(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_delete_range(DeleteRangeResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_delete_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_response_delete_range(&mut self) -> DeleteRangeResponse {
        if self.has_response_delete_range() {
            match self.response.take() {
                ::std::option::Option::Some(ResponseOp_oneof_response::response_delete_range(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRangeResponse::new()
        }
    }

    pub fn get_response_delete_range(&self) -> &DeleteRangeResponse {
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_delete_range(ref v)) => v,
            _ => DeleteRangeResponse::default_instance(),
        }
    }

    // .etcdserverpb.TxnResponse response_txn = 4;

    pub fn clear_response_txn(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response_txn(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_response_txn(&mut self, v: TxnResponse) {
        self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_txn(v))
    }

    // Mutable pointer to the field.
    pub fn mut_response_txn(&mut self) -> &mut TxnResponse {
        if let ::std::option::Option::Some(ResponseOp_oneof_response::response_txn(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_txn(TxnResponse::new()));
        }
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_response_txn(&mut self) -> TxnResponse {
        if self.has_response_txn() {
            match self.response.take() {
                ::std::option::Option::Some(ResponseOp_oneof_response::response_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            TxnResponse::new()
        }
    }

    pub fn get_response_txn(&self) -> &TxnResponse {
        match self.response {
            ::std::option::Option::Some(ResponseOp_oneof_response::response_txn(ref v)) => v,
            _ => TxnResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for ResponseOp {
    fn is_initialized(&self) -> bool {
        if let Some(ResponseOp_oneof_response::response_range(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ResponseOp_oneof_response::response_put(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ResponseOp_oneof_response::response_delete_range(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ResponseOp_oneof_response::response_txn(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_range(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_put(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_delete_range(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(ResponseOp_oneof_response::response_txn(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &ResponseOp_oneof_response::response_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseOp_oneof_response::response_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseOp_oneof_response::response_delete_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseOp_oneof_response::response_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &ResponseOp_oneof_response::response_range(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ResponseOp_oneof_response::response_put(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ResponseOp_oneof_response::response_delete_range(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ResponseOp_oneof_response::response_txn(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for ResponseOp {
    fn new() -> ResponseOp {
        ResponseOp::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseOp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RangeResponse>(
                    "response_range",
                    ResponseOp::has_response_range,
                    ResponseOp::get_response_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PutResponse>(
                    "response_put",
                    ResponseOp::has_response_put,
                    ResponseOp::get_response_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteRangeResponse>(
                    "response_delete_range",
                    ResponseOp::has_response_delete_range,
                    ResponseOp::get_response_delete_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TxnResponse>(
                    "response_txn",
                    ResponseOp::has_response_txn,
                    ResponseOp::get_response_txn,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseOp>(
                    "ResponseOp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseOp {
    fn clear(&mut self) {
        self.clear_response_range();
        self.clear_response_put();
        self.clear_response_delete_range();
        self.clear_response_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Compare {
    // message fields
    pub result: Compare_CompareResult,
    pub target: Compare_CompareTarget,
    pub key: ::std::vec::Vec<u8>,
    pub range_end: ::std::vec::Vec<u8>,
    // message oneof groups
    target_union: ::std::option::Option<Compare_oneof_target_union>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Compare {}

#[derive(Clone,PartialEq)]
pub enum Compare_oneof_target_union {
    version(i64),
    create_revision(i64),
    mod_revision(i64),
    value(::std::vec::Vec<u8>),
    lease(i64),
}

impl Compare {
    pub fn new() -> Compare {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Compare {
        static mut instance: ::protobuf::lazy::Lazy<Compare> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Compare,
        };
        unsafe {
            instance.get(Compare::new)
        }
    }

    // .etcdserverpb.Compare.CompareResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = Compare_CompareResult::EQUAL;
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: Compare_CompareResult) {
        self.result = v;
    }

    pub fn get_result(&self) -> Compare_CompareResult {
        self.result
    }

    fn get_result_for_reflect(&self) -> &Compare_CompareResult {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut Compare_CompareResult {
        &mut self.result
    }

    // .etcdserverpb.Compare.CompareTarget target = 2;

    pub fn clear_target(&mut self) {
        self.target = Compare_CompareTarget::VERSION;
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: Compare_CompareTarget) {
        self.target = v;
    }

    pub fn get_target(&self) -> Compare_CompareTarget {
        self.target
    }

    fn get_target_for_reflect(&self) -> &Compare_CompareTarget {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut Compare_CompareTarget {
        &mut self.target
    }

    // bytes key = 3;

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

    // int64 version = 4;

    pub fn clear_version(&mut self) {
        self.target_union = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::version(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i64) {
        self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::version(v))
    }

    pub fn get_version(&self) -> i64 {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::version(v)) => v,
            _ => 0,
        }
    }

    // int64 create_revision = 5;

    pub fn clear_create_revision(&mut self) {
        self.target_union = ::std::option::Option::None;
    }

    pub fn has_create_revision(&self) -> bool {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::create_revision(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_revision(&mut self, v: i64) {
        self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::create_revision(v))
    }

    pub fn get_create_revision(&self) -> i64 {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::create_revision(v)) => v,
            _ => 0,
        }
    }

    // int64 mod_revision = 6;

    pub fn clear_mod_revision(&mut self) {
        self.target_union = ::std::option::Option::None;
    }

    pub fn has_mod_revision(&self) -> bool {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::mod_revision(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_mod_revision(&mut self, v: i64) {
        self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::mod_revision(v))
    }

    pub fn get_mod_revision(&self) -> i64 {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::mod_revision(v)) => v,
            _ => 0,
        }
    }

    // bytes value = 7;

    pub fn clear_value(&mut self) {
        self.target_union = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Compare_oneof_target_union::value(_)) = self.target_union {
        } else {
            self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::value(::std::vec::Vec::new()));
        }
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_value() {
            match self.target_union.take() {
                ::std::option::Option::Some(Compare_oneof_target_union::value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_value(&self) -> &[u8] {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::value(ref v)) => v,
            _ => &[],
        }
    }

    // int64 lease = 8;

    pub fn clear_lease(&mut self) {
        self.target_union = ::std::option::Option::None;
    }

    pub fn has_lease(&self) -> bool {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::lease(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_lease(&mut self, v: i64) {
        self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::lease(v))
    }

    pub fn get_lease(&self) -> i64 {
        match self.target_union {
            ::std::option::Option::Some(Compare_oneof_target_union::lease(v)) => v,
            _ => 0,
        }
    }

    // bytes range_end = 64;

    pub fn clear_range_end(&mut self) {
        self.range_end.clear();
    }

    // Param is passed by value, moved
    pub fn set_range_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // Take field
    pub fn take_range_end(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.range_end, ::std::vec::Vec::new())
    }

    pub fn get_range_end(&self) -> &[u8] {
        &self.range_end
    }

    fn get_range_end_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.range_end
    }

    fn mut_range_end_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }
}

impl ::protobuf::Message for Compare {
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
                    let tmp = is.read_enum()?;
                    self.result = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.target = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::version(is.read_int64()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::create_revision(is.read_int64()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::mod_revision(is.read_int64()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::value(is.read_bytes()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target_union = ::std::option::Option::Some(Compare_oneof_target_union::lease(is.read_int64()?));
                },
                64 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.range_end)?;
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
        if self.result != Compare_CompareResult::EQUAL {
            my_size += ::protobuf::rt::enum_size(1, self.result);
        }
        if self.target != Compare_CompareTarget::VERSION {
            my_size += ::protobuf::rt::enum_size(2, self.target);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.key);
        }
        if !self.range_end.is_empty() {
            my_size += ::protobuf::rt::bytes_size(64, &self.range_end);
        }
        if let ::std::option::Option::Some(ref v) = self.target_union {
            match v {
                &Compare_oneof_target_union::version(v) => {
                    my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Compare_oneof_target_union::create_revision(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Compare_oneof_target_union::mod_revision(v) => {
                    my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Compare_oneof_target_union::value(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(7, &v);
                },
                &Compare_oneof_target_union::lease(v) => {
                    my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.result != Compare_CompareResult::EQUAL {
            os.write_enum(1, self.result.value())?;
        }
        if self.target != Compare_CompareTarget::VERSION {
            os.write_enum(2, self.target.value())?;
        }
        if !self.key.is_empty() {
            os.write_bytes(3, &self.key)?;
        }
        if !self.range_end.is_empty() {
            os.write_bytes(64, &self.range_end)?;
        }
        if let ::std::option::Option::Some(ref v) = self.target_union {
            match v {
                &Compare_oneof_target_union::version(v) => {
                    os.write_int64(4, v)?;
                },
                &Compare_oneof_target_union::create_revision(v) => {
                    os.write_int64(5, v)?;
                },
                &Compare_oneof_target_union::mod_revision(v) => {
                    os.write_int64(6, v)?;
                },
                &Compare_oneof_target_union::value(ref v) => {
                    os.write_bytes(7, v)?;
                },
                &Compare_oneof_target_union::lease(v) => {
                    os.write_int64(8, v)?;
                },
            };
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

impl ::protobuf::MessageStatic for Compare {
    fn new() -> Compare {
        Compare::new()
    }

    fn descriptor_static(_: ::std::option::Option<Compare>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Compare_CompareResult>>(
                    "result",
                    Compare::get_result_for_reflect,
                    Compare::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Compare_CompareTarget>>(
                    "target",
                    Compare::get_target_for_reflect,
                    Compare::mut_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    Compare::get_key_for_reflect,
                    Compare::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "version",
                    Compare::has_version,
                    Compare::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "create_revision",
                    Compare::has_create_revision,
                    Compare::get_create_revision,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "mod_revision",
                    Compare::has_mod_revision,
                    Compare::get_mod_revision,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "value",
                    Compare::has_value,
                    Compare::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "lease",
                    Compare::has_lease,
                    Compare::get_lease,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_end",
                    Compare::get_range_end_for_reflect,
                    Compare::mut_range_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Compare>(
                    "Compare",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Compare {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_target();
        self.clear_key();
        self.clear_version();
        self.clear_create_revision();
        self.clear_mod_revision();
        self.clear_value();
        self.clear_lease();
        self.clear_range_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Compare {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Compare {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Compare_CompareResult {
    EQUAL = 0,
    GREATER = 1,
    LESS = 2,
    NOT_EQUAL = 3,
}

impl ::protobuf::ProtobufEnum for Compare_CompareResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Compare_CompareResult> {
        match value {
            0 => ::std::option::Option::Some(Compare_CompareResult::EQUAL),
            1 => ::std::option::Option::Some(Compare_CompareResult::GREATER),
            2 => ::std::option::Option::Some(Compare_CompareResult::LESS),
            3 => ::std::option::Option::Some(Compare_CompareResult::NOT_EQUAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Compare_CompareResult] = &[
            Compare_CompareResult::EQUAL,
            Compare_CompareResult::GREATER,
            Compare_CompareResult::LESS,
            Compare_CompareResult::NOT_EQUAL,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Compare_CompareResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Compare_CompareResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Compare_CompareResult {
}

impl ::std::default::Default for Compare_CompareResult {
    fn default() -> Self {
        Compare_CompareResult::EQUAL
    }
}

impl ::protobuf::reflect::ProtobufValue for Compare_CompareResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Compare_CompareTarget {
    VERSION = 0,
    CREATE = 1,
    MOD = 2,
    VALUE = 3,
    LEASE = 4,
}

impl ::protobuf::ProtobufEnum for Compare_CompareTarget {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Compare_CompareTarget> {
        match value {
            0 => ::std::option::Option::Some(Compare_CompareTarget::VERSION),
            1 => ::std::option::Option::Some(Compare_CompareTarget::CREATE),
            2 => ::std::option::Option::Some(Compare_CompareTarget::MOD),
            3 => ::std::option::Option::Some(Compare_CompareTarget::VALUE),
            4 => ::std::option::Option::Some(Compare_CompareTarget::LEASE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Compare_CompareTarget] = &[
            Compare_CompareTarget::VERSION,
            Compare_CompareTarget::CREATE,
            Compare_CompareTarget::MOD,
            Compare_CompareTarget::VALUE,
            Compare_CompareTarget::LEASE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Compare_CompareTarget>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Compare_CompareTarget", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Compare_CompareTarget {
}

impl ::std::default::Default for Compare_CompareTarget {
    fn default() -> Self {
        Compare_CompareTarget::VERSION
    }
}

impl ::protobuf::reflect::ProtobufValue for Compare_CompareTarget {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TxnRequest {
    // message fields
    pub compare: ::protobuf::RepeatedField<Compare>,
    pub success: ::protobuf::RepeatedField<RequestOp>,
    pub failure: ::protobuf::RepeatedField<RequestOp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TxnRequest {}

impl TxnRequest {
    pub fn new() -> TxnRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TxnRequest {
        static mut instance: ::protobuf::lazy::Lazy<TxnRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TxnRequest,
        };
        unsafe {
            instance.get(TxnRequest::new)
        }
    }

    // repeated .etcdserverpb.Compare compare = 1;

    pub fn clear_compare(&mut self) {
        self.compare.clear();
    }

    // Param is passed by value, moved
    pub fn set_compare(&mut self, v: ::protobuf::RepeatedField<Compare>) {
        self.compare = v;
    }

    // Mutable pointer to the field.
    pub fn mut_compare(&mut self) -> &mut ::protobuf::RepeatedField<Compare> {
        &mut self.compare
    }

    // Take field
    pub fn take_compare(&mut self) -> ::protobuf::RepeatedField<Compare> {
        ::std::mem::replace(&mut self.compare, ::protobuf::RepeatedField::new())
    }

    pub fn get_compare(&self) -> &[Compare] {
        &self.compare
    }

    fn get_compare_for_reflect(&self) -> &::protobuf::RepeatedField<Compare> {
        &self.compare
    }

    fn mut_compare_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Compare> {
        &mut self.compare
    }

    // repeated .etcdserverpb.RequestOp success = 2;

    pub fn clear_success(&mut self) {
        self.success.clear();
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: ::protobuf::RepeatedField<RequestOp>) {
        self.success = v;
    }

    // Mutable pointer to the field.
    pub fn mut_success(&mut self) -> &mut ::protobuf::RepeatedField<RequestOp> {
        &mut self.success
    }

    // Take field
    pub fn take_success(&mut self) -> ::protobuf::RepeatedField<RequestOp> {
        ::std::mem::replace(&mut self.success, ::protobuf::RepeatedField::new())
    }

    pub fn get_success(&self) -> &[RequestOp] {
        &self.success
    }

    fn get_success_for_reflect(&self) -> &::protobuf::RepeatedField<RequestOp> {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RequestOp> {
        &mut self.success
    }

    // repeated .etcdserverpb.RequestOp failure = 3;

    pub fn clear_failure(&mut self) {
        self.failure.clear();
    }

    // Param is passed by value, moved
    pub fn set_failure(&mut self, v: ::protobuf::RepeatedField<RequestOp>) {
        self.failure = v;
    }

    // Mutable pointer to the field.
    pub fn mut_failure(&mut self) -> &mut ::protobuf::RepeatedField<RequestOp> {
        &mut self.failure
    }

    // Take field
    pub fn take_failure(&mut self) -> ::protobuf::RepeatedField<RequestOp> {
        ::std::mem::replace(&mut self.failure, ::protobuf::RepeatedField::new())
    }

    pub fn get_failure(&self) -> &[RequestOp] {
        &self.failure
    }

    fn get_failure_for_reflect(&self) -> &::protobuf::RepeatedField<RequestOp> {
        &self.failure
    }

    fn mut_failure_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RequestOp> {
        &mut self.failure
    }
}

impl ::protobuf::Message for TxnRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.compare {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.success {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.failure {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.compare)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.success)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.failure)?;
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
        for value in &self.compare {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.success {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.failure {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.compare {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.success {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.failure {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for TxnRequest {
    fn new() -> TxnRequest {
        TxnRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<TxnRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Compare>>(
                    "compare",
                    TxnRequest::get_compare_for_reflect,
                    TxnRequest::mut_compare_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestOp>>(
                    "success",
                    TxnRequest::get_success_for_reflect,
                    TxnRequest::mut_success_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestOp>>(
                    "failure",
                    TxnRequest::get_failure_for_reflect,
                    TxnRequest::mut_failure_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TxnRequest>(
                    "TxnRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TxnRequest {
    fn clear(&mut self) {
        self.clear_compare();
        self.clear_success();
        self.clear_failure();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TxnRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TxnRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TxnResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub succeeded: bool,
    pub responses: ::protobuf::RepeatedField<ResponseOp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TxnResponse {}

impl TxnResponse {
    pub fn new() -> TxnResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TxnResponse {
        static mut instance: ::protobuf::lazy::Lazy<TxnResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TxnResponse,
        };
        unsafe {
            instance.get(TxnResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // bool succeeded = 2;

    pub fn clear_succeeded(&mut self) {
        self.succeeded = false;
    }

    // Param is passed by value, moved
    pub fn set_succeeded(&mut self, v: bool) {
        self.succeeded = v;
    }

    pub fn get_succeeded(&self) -> bool {
        self.succeeded
    }

    fn get_succeeded_for_reflect(&self) -> &bool {
        &self.succeeded
    }

    fn mut_succeeded_for_reflect(&mut self) -> &mut bool {
        &mut self.succeeded
    }

    // repeated .etcdserverpb.ResponseOp responses = 3;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<ResponseOp>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses(&mut self) -> &mut ::protobuf::RepeatedField<ResponseOp> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<ResponseOp> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses(&self) -> &[ResponseOp] {
        &self.responses
    }

    fn get_responses_for_reflect(&self) -> &::protobuf::RepeatedField<ResponseOp> {
        &self.responses
    }

    fn mut_responses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ResponseOp> {
        &mut self.responses
    }
}

impl ::protobuf::Message for TxnResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.responses {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.succeeded = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.responses)?;
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
        if self.succeeded != false {
            my_size += 2;
        }
        for value in &self.responses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        if self.succeeded != false {
            os.write_bool(2, self.succeeded)?;
        }
        for v in &self.responses {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for TxnResponse {
    fn new() -> TxnResponse {
        TxnResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<TxnResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    TxnResponse::get_header_for_reflect,
                    TxnResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "succeeded",
                    TxnResponse::get_succeeded_for_reflect,
                    TxnResponse::mut_succeeded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseOp>>(
                    "responses",
                    TxnResponse::get_responses_for_reflect,
                    TxnResponse::mut_responses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TxnResponse>(
                    "TxnResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TxnResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_succeeded();
        self.clear_responses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TxnResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TxnResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompactionRequest {
    // message fields
    pub revision: i64,
    pub physical: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CompactionRequest {}

impl CompactionRequest {
    pub fn new() -> CompactionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CompactionRequest {
        static mut instance: ::protobuf::lazy::Lazy<CompactionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CompactionRequest,
        };
        unsafe {
            instance.get(CompactionRequest::new)
        }
    }

    // int64 revision = 1;

    pub fn clear_revision(&mut self) {
        self.revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: i64) {
        self.revision = v;
    }

    pub fn get_revision(&self) -> i64 {
        self.revision
    }

    fn get_revision_for_reflect(&self) -> &i64 {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.revision
    }

    // bool physical = 2;

    pub fn clear_physical(&mut self) {
        self.physical = false;
    }

    // Param is passed by value, moved
    pub fn set_physical(&mut self, v: bool) {
        self.physical = v;
    }

    pub fn get_physical(&self) -> bool {
        self.physical
    }

    fn get_physical_for_reflect(&self) -> &bool {
        &self.physical
    }

    fn mut_physical_for_reflect(&mut self) -> &mut bool {
        &mut self.physical
    }
}

impl ::protobuf::Message for CompactionRequest {
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
                    self.revision = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.physical = tmp;
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
        if self.revision != 0 {
            my_size += ::protobuf::rt::value_size(1, self.revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.physical != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.revision != 0 {
            os.write_int64(1, self.revision)?;
        }
        if self.physical != false {
            os.write_bool(2, self.physical)?;
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

impl ::protobuf::MessageStatic for CompactionRequest {
    fn new() -> CompactionRequest {
        CompactionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CompactionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "revision",
                    CompactionRequest::get_revision_for_reflect,
                    CompactionRequest::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "physical",
                    CompactionRequest::get_physical_for_reflect,
                    CompactionRequest::mut_physical_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CompactionRequest>(
                    "CompactionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CompactionRequest {
    fn clear(&mut self) {
        self.clear_revision();
        self.clear_physical();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompactionResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CompactionResponse {}

impl CompactionResponse {
    pub fn new() -> CompactionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CompactionResponse {
        static mut instance: ::protobuf::lazy::Lazy<CompactionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CompactionResponse,
        };
        unsafe {
            instance.get(CompactionResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for CompactionResponse {
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

impl ::protobuf::MessageStatic for CompactionResponse {
    fn new() -> CompactionResponse {
        CompactionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CompactionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    CompactionResponse::get_header_for_reflect,
                    CompactionResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CompactionResponse>(
                    "CompactionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CompactionResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HashRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HashRequest {}

impl HashRequest {
    pub fn new() -> HashRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HashRequest {
        static mut instance: ::protobuf::lazy::Lazy<HashRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HashRequest,
        };
        unsafe {
            instance.get(HashRequest::new)
        }
    }
}

impl ::protobuf::Message for HashRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for HashRequest {
    fn new() -> HashRequest {
        HashRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<HashRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<HashRequest>(
                    "HashRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HashRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HashRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HashRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HashKVRequest {
    // message fields
    pub revision: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HashKVRequest {}

impl HashKVRequest {
    pub fn new() -> HashKVRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HashKVRequest {
        static mut instance: ::protobuf::lazy::Lazy<HashKVRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HashKVRequest,
        };
        unsafe {
            instance.get(HashKVRequest::new)
        }
    }

    // int64 revision = 1;

    pub fn clear_revision(&mut self) {
        self.revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: i64) {
        self.revision = v;
    }

    pub fn get_revision(&self) -> i64 {
        self.revision
    }

    fn get_revision_for_reflect(&self) -> &i64 {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.revision
    }
}

impl ::protobuf::Message for HashKVRequest {
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
                    self.revision = tmp;
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
        if self.revision != 0 {
            my_size += ::protobuf::rt::value_size(1, self.revision, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.revision != 0 {
            os.write_int64(1, self.revision)?;
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

impl ::protobuf::MessageStatic for HashKVRequest {
    fn new() -> HashKVRequest {
        HashKVRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<HashKVRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "revision",
                    HashKVRequest::get_revision_for_reflect,
                    HashKVRequest::mut_revision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HashKVRequest>(
                    "HashKVRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HashKVRequest {
    fn clear(&mut self) {
        self.clear_revision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HashKVRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HashKVRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HashKVResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub hash: u32,
    pub compact_revision: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HashKVResponse {}

impl HashKVResponse {
    pub fn new() -> HashKVResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HashKVResponse {
        static mut instance: ::protobuf::lazy::Lazy<HashKVResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HashKVResponse,
        };
        unsafe {
            instance.get(HashKVResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // uint32 hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash = 0;
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u32) {
        self.hash = v;
    }

    pub fn get_hash(&self) -> u32 {
        self.hash
    }

    fn get_hash_for_reflect(&self) -> &u32 {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut u32 {
        &mut self.hash
    }

    // int64 compact_revision = 3;

    pub fn clear_compact_revision(&mut self) {
        self.compact_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_compact_revision(&mut self, v: i64) {
        self.compact_revision = v;
    }

    pub fn get_compact_revision(&self) -> i64 {
        self.compact_revision
    }

    fn get_compact_revision_for_reflect(&self) -> &i64 {
        &self.compact_revision
    }

    fn mut_compact_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.compact_revision
    }
}

impl ::protobuf::Message for HashKVResponse {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.hash = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.compact_revision = tmp;
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
        if self.hash != 0 {
            my_size += ::protobuf::rt::value_size(2, self.hash, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.compact_revision != 0 {
            my_size += ::protobuf::rt::value_size(3, self.compact_revision, ::protobuf::wire_format::WireTypeVarint);
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
        if self.hash != 0 {
            os.write_uint32(2, self.hash)?;
        }
        if self.compact_revision != 0 {
            os.write_int64(3, self.compact_revision)?;
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

impl ::protobuf::MessageStatic for HashKVResponse {
    fn new() -> HashKVResponse {
        HashKVResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<HashKVResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    HashKVResponse::get_header_for_reflect,
                    HashKVResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hash",
                    HashKVResponse::get_hash_for_reflect,
                    HashKVResponse::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "compact_revision",
                    HashKVResponse::get_compact_revision_for_reflect,
                    HashKVResponse::mut_compact_revision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HashKVResponse>(
                    "HashKVResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HashKVResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_hash();
        self.clear_compact_revision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HashKVResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HashKVResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HashResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub hash: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HashResponse {}

impl HashResponse {
    pub fn new() -> HashResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HashResponse {
        static mut instance: ::protobuf::lazy::Lazy<HashResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HashResponse,
        };
        unsafe {
            instance.get(HashResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // uint32 hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash = 0;
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u32) {
        self.hash = v;
    }

    pub fn get_hash(&self) -> u32 {
        self.hash
    }

    fn get_hash_for_reflect(&self) -> &u32 {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut u32 {
        &mut self.hash
    }
}

impl ::protobuf::Message for HashResponse {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.hash = tmp;
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
        if self.hash != 0 {
            my_size += ::protobuf::rt::value_size(2, self.hash, ::protobuf::wire_format::WireTypeVarint);
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
        if self.hash != 0 {
            os.write_uint32(2, self.hash)?;
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

impl ::protobuf::MessageStatic for HashResponse {
    fn new() -> HashResponse {
        HashResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<HashResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    HashResponse::get_header_for_reflect,
                    HashResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hash",
                    HashResponse::get_hash_for_reflect,
                    HashResponse::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HashResponse>(
                    "HashResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HashResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HashResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HashResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotRequest {}

impl SnapshotRequest {
    pub fn new() -> SnapshotRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotRequest {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotRequest,
        };
        unsafe {
            instance.get(SnapshotRequest::new)
        }
    }
}

impl ::protobuf::Message for SnapshotRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for SnapshotRequest {
    fn new() -> SnapshotRequest {
        SnapshotRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotRequest>(
                    "SnapshotRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub remaining_bytes: u64,
    pub blob: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotResponse {}

impl SnapshotResponse {
    pub fn new() -> SnapshotResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotResponse {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotResponse,
        };
        unsafe {
            instance.get(SnapshotResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // uint64 remaining_bytes = 2;

    pub fn clear_remaining_bytes(&mut self) {
        self.remaining_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_remaining_bytes(&mut self, v: u64) {
        self.remaining_bytes = v;
    }

    pub fn get_remaining_bytes(&self) -> u64 {
        self.remaining_bytes
    }

    fn get_remaining_bytes_for_reflect(&self) -> &u64 {
        &self.remaining_bytes
    }

    fn mut_remaining_bytes_for_reflect(&mut self) -> &mut u64 {
        &mut self.remaining_bytes
    }

    // bytes blob = 3;

    pub fn clear_blob(&mut self) {
        self.blob.clear();
    }

    // Param is passed by value, moved
    pub fn set_blob(&mut self, v: ::std::vec::Vec<u8>) {
        self.blob = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blob(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.blob
    }

    // Take field
    pub fn take_blob(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.blob, ::std::vec::Vec::new())
    }

    pub fn get_blob(&self) -> &[u8] {
        &self.blob
    }

    fn get_blob_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.blob
    }

    fn mut_blob_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.blob
    }
}

impl ::protobuf::Message for SnapshotResponse {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.remaining_bytes = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.blob)?;
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
        if self.remaining_bytes != 0 {
            my_size += ::protobuf::rt::value_size(2, self.remaining_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.blob.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.blob);
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
        if self.remaining_bytes != 0 {
            os.write_uint64(2, self.remaining_bytes)?;
        }
        if !self.blob.is_empty() {
            os.write_bytes(3, &self.blob)?;
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

impl ::protobuf::MessageStatic for SnapshotResponse {
    fn new() -> SnapshotResponse {
        SnapshotResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    SnapshotResponse::get_header_for_reflect,
                    SnapshotResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "remaining_bytes",
                    SnapshotResponse::get_remaining_bytes_for_reflect,
                    SnapshotResponse::mut_remaining_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "blob",
                    SnapshotResponse::get_blob_for_reflect,
                    SnapshotResponse::mut_blob_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotResponse>(
                    "SnapshotResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_remaining_bytes();
        self.clear_blob();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WatchRequest {
    // message oneof groups
    request_union: ::std::option::Option<WatchRequest_oneof_request_union>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WatchRequest {}

#[derive(Clone,PartialEq)]
pub enum WatchRequest_oneof_request_union {
    create_request(WatchCreateRequest),
    cancel_request(WatchCancelRequest),
}

impl WatchRequest {
    pub fn new() -> WatchRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WatchRequest {
        static mut instance: ::protobuf::lazy::Lazy<WatchRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WatchRequest,
        };
        unsafe {
            instance.get(WatchRequest::new)
        }
    }

    // .etcdserverpb.WatchCreateRequest create_request = 1;

    pub fn clear_create_request(&mut self) {
        self.request_union = ::std::option::Option::None;
    }

    pub fn has_create_request(&self) -> bool {
        match self.request_union {
            ::std::option::Option::Some(WatchRequest_oneof_request_union::create_request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_request(&mut self, v: WatchCreateRequest) {
        self.request_union = ::std::option::Option::Some(WatchRequest_oneof_request_union::create_request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_request(&mut self) -> &mut WatchCreateRequest {
        if let ::std::option::Option::Some(WatchRequest_oneof_request_union::create_request(_)) = self.request_union {
        } else {
            self.request_union = ::std::option::Option::Some(WatchRequest_oneof_request_union::create_request(WatchCreateRequest::new()));
        }
        match self.request_union {
            ::std::option::Option::Some(WatchRequest_oneof_request_union::create_request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_request(&mut self) -> WatchCreateRequest {
        if self.has_create_request() {
            match self.request_union.take() {
                ::std::option::Option::Some(WatchRequest_oneof_request_union::create_request(v)) => v,
                _ => panic!(),
            }
        } else {
            WatchCreateRequest::new()
        }
    }

    pub fn get_create_request(&self) -> &WatchCreateRequest {
        match self.request_union {
            ::std::option::Option::Some(WatchRequest_oneof_request_union::create_request(ref v)) => v,
            _ => WatchCreateRequest::default_instance(),
        }
    }

    // .etcdserverpb.WatchCancelRequest cancel_request = 2;

    pub fn clear_cancel_request(&mut self) {
        self.request_union = ::std::option::Option::None;
    }

    pub fn has_cancel_request(&self) -> bool {
        match self.request_union {
            ::std::option::Option::Some(WatchRequest_oneof_request_union::cancel_request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cancel_request(&mut self, v: WatchCancelRequest) {
        self.request_union = ::std::option::Option::Some(WatchRequest_oneof_request_union::cancel_request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cancel_request(&mut self) -> &mut WatchCancelRequest {
        if let ::std::option::Option::Some(WatchRequest_oneof_request_union::cancel_request(_)) = self.request_union {
        } else {
            self.request_union = ::std::option::Option::Some(WatchRequest_oneof_request_union::cancel_request(WatchCancelRequest::new()));
        }
        match self.request_union {
            ::std::option::Option::Some(WatchRequest_oneof_request_union::cancel_request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cancel_request(&mut self) -> WatchCancelRequest {
        if self.has_cancel_request() {
            match self.request_union.take() {
                ::std::option::Option::Some(WatchRequest_oneof_request_union::cancel_request(v)) => v,
                _ => panic!(),
            }
        } else {
            WatchCancelRequest::new()
        }
    }

    pub fn get_cancel_request(&self) -> &WatchCancelRequest {
        match self.request_union {
            ::std::option::Option::Some(WatchRequest_oneof_request_union::cancel_request(ref v)) => v,
            _ => WatchCancelRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for WatchRequest {
    fn is_initialized(&self) -> bool {
        if let Some(WatchRequest_oneof_request_union::create_request(ref v)) = self.request_union {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(WatchRequest_oneof_request_union::cancel_request(ref v)) = self.request_union {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request_union = ::std::option::Option::Some(WatchRequest_oneof_request_union::create_request(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request_union = ::std::option::Option::Some(WatchRequest_oneof_request_union::cancel_request(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.request_union {
            match v {
                &WatchRequest_oneof_request_union::create_request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &WatchRequest_oneof_request_union::cancel_request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.request_union {
            match v {
                &WatchRequest_oneof_request_union::create_request(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &WatchRequest_oneof_request_union::cancel_request(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for WatchRequest {
    fn new() -> WatchRequest {
        WatchRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WatchRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, WatchCreateRequest>(
                    "create_request",
                    WatchRequest::has_create_request,
                    WatchRequest::get_create_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, WatchCancelRequest>(
                    "cancel_request",
                    WatchRequest::has_cancel_request,
                    WatchRequest::get_cancel_request,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WatchRequest>(
                    "WatchRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WatchRequest {
    fn clear(&mut self) {
        self.clear_create_request();
        self.clear_cancel_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WatchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WatchRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WatchCreateRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub range_end: ::std::vec::Vec<u8>,
    pub start_revision: i64,
    pub progress_notify: bool,
    pub filters: ::std::vec::Vec<WatchCreateRequest_FilterType>,
    pub prev_kv: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WatchCreateRequest {}

impl WatchCreateRequest {
    pub fn new() -> WatchCreateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WatchCreateRequest {
        static mut instance: ::protobuf::lazy::Lazy<WatchCreateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WatchCreateRequest,
        };
        unsafe {
            instance.get(WatchCreateRequest::new)
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

    // bytes range_end = 2;

    pub fn clear_range_end(&mut self) {
        self.range_end.clear();
    }

    // Param is passed by value, moved
    pub fn set_range_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // Take field
    pub fn take_range_end(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.range_end, ::std::vec::Vec::new())
    }

    pub fn get_range_end(&self) -> &[u8] {
        &self.range_end
    }

    fn get_range_end_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.range_end
    }

    fn mut_range_end_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // int64 start_revision = 3;

    pub fn clear_start_revision(&mut self) {
        self.start_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_revision(&mut self, v: i64) {
        self.start_revision = v;
    }

    pub fn get_start_revision(&self) -> i64 {
        self.start_revision
    }

    fn get_start_revision_for_reflect(&self) -> &i64 {
        &self.start_revision
    }

    fn mut_start_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.start_revision
    }

    // bool progress_notify = 4;

    pub fn clear_progress_notify(&mut self) {
        self.progress_notify = false;
    }

    // Param is passed by value, moved
    pub fn set_progress_notify(&mut self, v: bool) {
        self.progress_notify = v;
    }

    pub fn get_progress_notify(&self) -> bool {
        self.progress_notify
    }

    fn get_progress_notify_for_reflect(&self) -> &bool {
        &self.progress_notify
    }

    fn mut_progress_notify_for_reflect(&mut self) -> &mut bool {
        &mut self.progress_notify
    }

    // repeated .etcdserverpb.WatchCreateRequest.FilterType filters = 5;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: ::std::vec::Vec<WatchCreateRequest_FilterType>) {
        self.filters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filters(&mut self) -> &mut ::std::vec::Vec<WatchCreateRequest_FilterType> {
        &mut self.filters
    }

    // Take field
    pub fn take_filters(&mut self) -> ::std::vec::Vec<WatchCreateRequest_FilterType> {
        ::std::mem::replace(&mut self.filters, ::std::vec::Vec::new())
    }

    pub fn get_filters(&self) -> &[WatchCreateRequest_FilterType] {
        &self.filters
    }

    fn get_filters_for_reflect(&self) -> &::std::vec::Vec<WatchCreateRequest_FilterType> {
        &self.filters
    }

    fn mut_filters_for_reflect(&mut self) -> &mut ::std::vec::Vec<WatchCreateRequest_FilterType> {
        &mut self.filters
    }

    // bool prev_kv = 6;

    pub fn clear_prev_kv(&mut self) {
        self.prev_kv = false;
    }

    // Param is passed by value, moved
    pub fn set_prev_kv(&mut self, v: bool) {
        self.prev_kv = v;
    }

    pub fn get_prev_kv(&self) -> bool {
        self.prev_kv
    }

    fn get_prev_kv_for_reflect(&self) -> &bool {
        &self.prev_kv
    }

    fn mut_prev_kv_for_reflect(&mut self) -> &mut bool {
        &mut self.prev_kv
    }
}

impl ::protobuf::Message for WatchCreateRequest {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.range_end)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.start_revision = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.progress_notify = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.filters)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.prev_kv = tmp;
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
        if !self.range_end.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.range_end);
        }
        if self.start_revision != 0 {
            my_size += ::protobuf::rt::value_size(3, self.start_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.progress_notify != false {
            my_size += 2;
        }
        for value in &self.filters {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        if self.prev_kv != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.range_end.is_empty() {
            os.write_bytes(2, &self.range_end)?;
        }
        if self.start_revision != 0 {
            os.write_int64(3, self.start_revision)?;
        }
        if self.progress_notify != false {
            os.write_bool(4, self.progress_notify)?;
        }
        for v in &self.filters {
            os.write_enum(5, v.value())?;
        };
        if self.prev_kv != false {
            os.write_bool(6, self.prev_kv)?;
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

impl ::protobuf::MessageStatic for WatchCreateRequest {
    fn new() -> WatchCreateRequest {
        WatchCreateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WatchCreateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    WatchCreateRequest::get_key_for_reflect,
                    WatchCreateRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_end",
                    WatchCreateRequest::get_range_end_for_reflect,
                    WatchCreateRequest::mut_range_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "start_revision",
                    WatchCreateRequest::get_start_revision_for_reflect,
                    WatchCreateRequest::mut_start_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "progress_notify",
                    WatchCreateRequest::get_progress_notify_for_reflect,
                    WatchCreateRequest::mut_progress_notify_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<WatchCreateRequest_FilterType>>(
                    "filters",
                    WatchCreateRequest::get_filters_for_reflect,
                    WatchCreateRequest::mut_filters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "prev_kv",
                    WatchCreateRequest::get_prev_kv_for_reflect,
                    WatchCreateRequest::mut_prev_kv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WatchCreateRequest>(
                    "WatchCreateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WatchCreateRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_range_end();
        self.clear_start_revision();
        self.clear_progress_notify();
        self.clear_filters();
        self.clear_prev_kv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WatchCreateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WatchCreateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum WatchCreateRequest_FilterType {
    NOPUT = 0,
    NODELETE = 1,
}

impl ::protobuf::ProtobufEnum for WatchCreateRequest_FilterType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<WatchCreateRequest_FilterType> {
        match value {
            0 => ::std::option::Option::Some(WatchCreateRequest_FilterType::NOPUT),
            1 => ::std::option::Option::Some(WatchCreateRequest_FilterType::NODELETE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [WatchCreateRequest_FilterType] = &[
            WatchCreateRequest_FilterType::NOPUT,
            WatchCreateRequest_FilterType::NODELETE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<WatchCreateRequest_FilterType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("WatchCreateRequest_FilterType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for WatchCreateRequest_FilterType {
}

impl ::std::default::Default for WatchCreateRequest_FilterType {
    fn default() -> Self {
        WatchCreateRequest_FilterType::NOPUT
    }
}

impl ::protobuf::reflect::ProtobufValue for WatchCreateRequest_FilterType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WatchCancelRequest {
    // message fields
    pub watch_id: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WatchCancelRequest {}

impl WatchCancelRequest {
    pub fn new() -> WatchCancelRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WatchCancelRequest {
        static mut instance: ::protobuf::lazy::Lazy<WatchCancelRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WatchCancelRequest,
        };
        unsafe {
            instance.get(WatchCancelRequest::new)
        }
    }

    // int64 watch_id = 1;

    pub fn clear_watch_id(&mut self) {
        self.watch_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_watch_id(&mut self, v: i64) {
        self.watch_id = v;
    }

    pub fn get_watch_id(&self) -> i64 {
        self.watch_id
    }

    fn get_watch_id_for_reflect(&self) -> &i64 {
        &self.watch_id
    }

    fn mut_watch_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.watch_id
    }
}

impl ::protobuf::Message for WatchCancelRequest {
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
                    self.watch_id = tmp;
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
        if self.watch_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.watch_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.watch_id != 0 {
            os.write_int64(1, self.watch_id)?;
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

impl ::protobuf::MessageStatic for WatchCancelRequest {
    fn new() -> WatchCancelRequest {
        WatchCancelRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WatchCancelRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "watch_id",
                    WatchCancelRequest::get_watch_id_for_reflect,
                    WatchCancelRequest::mut_watch_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WatchCancelRequest>(
                    "WatchCancelRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WatchCancelRequest {
    fn clear(&mut self) {
        self.clear_watch_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WatchCancelRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WatchCancelRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WatchResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub watch_id: i64,
    pub created: bool,
    pub canceled: bool,
    pub compact_revision: i64,
    pub cancel_reason: ::std::string::String,
    pub events: ::protobuf::RepeatedField<super::kv::Event>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WatchResponse {}

impl WatchResponse {
    pub fn new() -> WatchResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WatchResponse {
        static mut instance: ::protobuf::lazy::Lazy<WatchResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WatchResponse,
        };
        unsafe {
            instance.get(WatchResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // int64 watch_id = 2;

    pub fn clear_watch_id(&mut self) {
        self.watch_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_watch_id(&mut self, v: i64) {
        self.watch_id = v;
    }

    pub fn get_watch_id(&self) -> i64 {
        self.watch_id
    }

    fn get_watch_id_for_reflect(&self) -> &i64 {
        &self.watch_id
    }

    fn mut_watch_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.watch_id
    }

    // bool created = 3;

    pub fn clear_created(&mut self) {
        self.created = false;
    }

    // Param is passed by value, moved
    pub fn set_created(&mut self, v: bool) {
        self.created = v;
    }

    pub fn get_created(&self) -> bool {
        self.created
    }

    fn get_created_for_reflect(&self) -> &bool {
        &self.created
    }

    fn mut_created_for_reflect(&mut self) -> &mut bool {
        &mut self.created
    }

    // bool canceled = 4;

    pub fn clear_canceled(&mut self) {
        self.canceled = false;
    }

    // Param is passed by value, moved
    pub fn set_canceled(&mut self, v: bool) {
        self.canceled = v;
    }

    pub fn get_canceled(&self) -> bool {
        self.canceled
    }

    fn get_canceled_for_reflect(&self) -> &bool {
        &self.canceled
    }

    fn mut_canceled_for_reflect(&mut self) -> &mut bool {
        &mut self.canceled
    }

    // int64 compact_revision = 5;

    pub fn clear_compact_revision(&mut self) {
        self.compact_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_compact_revision(&mut self, v: i64) {
        self.compact_revision = v;
    }

    pub fn get_compact_revision(&self) -> i64 {
        self.compact_revision
    }

    fn get_compact_revision_for_reflect(&self) -> &i64 {
        &self.compact_revision
    }

    fn mut_compact_revision_for_reflect(&mut self) -> &mut i64 {
        &mut self.compact_revision
    }

    // string cancel_reason = 6;

    pub fn clear_cancel_reason(&mut self) {
        self.cancel_reason.clear();
    }

    // Param is passed by value, moved
    pub fn set_cancel_reason(&mut self, v: ::std::string::String) {
        self.cancel_reason = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cancel_reason(&mut self) -> &mut ::std::string::String {
        &mut self.cancel_reason
    }

    // Take field
    pub fn take_cancel_reason(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cancel_reason, ::std::string::String::new())
    }

    pub fn get_cancel_reason(&self) -> &str {
        &self.cancel_reason
    }

    fn get_cancel_reason_for_reflect(&self) -> &::std::string::String {
        &self.cancel_reason
    }

    fn mut_cancel_reason_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cancel_reason
    }

    // repeated .mvccpb.Event events = 11;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<super::kv::Event>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<super::kv::Event> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<super::kv::Event> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[super::kv::Event] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<super::kv::Event> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::kv::Event> {
        &mut self.events
    }
}

impl ::protobuf::Message for WatchResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.events {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.watch_id = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.created = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.canceled = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.compact_revision = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cancel_reason)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
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
        if self.watch_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.watch_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.created != false {
            my_size += 2;
        }
        if self.canceled != false {
            my_size += 2;
        }
        if self.compact_revision != 0 {
            my_size += ::protobuf::rt::value_size(5, self.compact_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.cancel_reason.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.cancel_reason);
        }
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        if self.watch_id != 0 {
            os.write_int64(2, self.watch_id)?;
        }
        if self.created != false {
            os.write_bool(3, self.created)?;
        }
        if self.canceled != false {
            os.write_bool(4, self.canceled)?;
        }
        if self.compact_revision != 0 {
            os.write_int64(5, self.compact_revision)?;
        }
        if !self.cancel_reason.is_empty() {
            os.write_string(6, &self.cancel_reason)?;
        }
        for v in &self.events {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for WatchResponse {
    fn new() -> WatchResponse {
        WatchResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<WatchResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    WatchResponse::get_header_for_reflect,
                    WatchResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "watch_id",
                    WatchResponse::get_watch_id_for_reflect,
                    WatchResponse::mut_watch_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "created",
                    WatchResponse::get_created_for_reflect,
                    WatchResponse::mut_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "canceled",
                    WatchResponse::get_canceled_for_reflect,
                    WatchResponse::mut_canceled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "compact_revision",
                    WatchResponse::get_compact_revision_for_reflect,
                    WatchResponse::mut_compact_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cancel_reason",
                    WatchResponse::get_cancel_reason_for_reflect,
                    WatchResponse::mut_cancel_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kv::Event>>(
                    "events",
                    WatchResponse::get_events_for_reflect,
                    WatchResponse::mut_events_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WatchResponse>(
                    "WatchResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WatchResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_watch_id();
        self.clear_created();
        self.clear_canceled();
        self.clear_compact_revision();
        self.clear_cancel_reason();
        self.clear_events();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WatchResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WatchResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseGrantRequest {
    // message fields
    pub TTL: i64,
    pub ID: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseGrantRequest {}

impl LeaseGrantRequest {
    pub fn new() -> LeaseGrantRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseGrantRequest {
        static mut instance: ::protobuf::lazy::Lazy<LeaseGrantRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseGrantRequest,
        };
        unsafe {
            instance.get(LeaseGrantRequest::new)
        }
    }

    // int64 TTL = 1;

    pub fn clear_TTL(&mut self) {
        self.TTL = 0;
    }

    // Param is passed by value, moved
    pub fn set_TTL(&mut self, v: i64) {
        self.TTL = v;
    }

    pub fn get_TTL(&self) -> i64 {
        self.TTL
    }

    fn get_TTL_for_reflect(&self) -> &i64 {
        &self.TTL
    }

    fn mut_TTL_for_reflect(&mut self) -> &mut i64 {
        &mut self.TTL
    }

    // int64 ID = 2;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: i64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> i64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &i64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut i64 {
        &mut self.ID
    }
}

impl ::protobuf::Message for LeaseGrantRequest {
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
                    self.TTL = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ID = tmp;
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
        if self.TTL != 0 {
            my_size += ::protobuf::rt::value_size(1, self.TTL, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(2, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.TTL != 0 {
            os.write_int64(1, self.TTL)?;
        }
        if self.ID != 0 {
            os.write_int64(2, self.ID)?;
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

impl ::protobuf::MessageStatic for LeaseGrantRequest {
    fn new() -> LeaseGrantRequest {
        LeaseGrantRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseGrantRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "TTL",
                    LeaseGrantRequest::get_TTL_for_reflect,
                    LeaseGrantRequest::mut_TTL_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    LeaseGrantRequest::get_ID_for_reflect,
                    LeaseGrantRequest::mut_ID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseGrantRequest>(
                    "LeaseGrantRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseGrantRequest {
    fn clear(&mut self) {
        self.clear_TTL();
        self.clear_ID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseGrantRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseGrantRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseGrantResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub ID: i64,
    pub TTL: i64,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseGrantResponse {}

impl LeaseGrantResponse {
    pub fn new() -> LeaseGrantResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseGrantResponse {
        static mut instance: ::protobuf::lazy::Lazy<LeaseGrantResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseGrantResponse,
        };
        unsafe {
            instance.get(LeaseGrantResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // int64 ID = 2;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: i64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> i64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &i64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut i64 {
        &mut self.ID
    }

    // int64 TTL = 3;

    pub fn clear_TTL(&mut self) {
        self.TTL = 0;
    }

    // Param is passed by value, moved
    pub fn set_TTL(&mut self, v: i64) {
        self.TTL = v;
    }

    pub fn get_TTL(&self) -> i64 {
        self.TTL
    }

    fn get_TTL_for_reflect(&self) -> &i64 {
        &self.TTL
    }

    fn mut_TTL_for_reflect(&mut self) -> &mut i64 {
        &mut self.TTL
    }

    // string error = 4;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for LeaseGrantResponse {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ID = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.TTL = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(2, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.TTL != 0 {
            my_size += ::protobuf::rt::value_size(3, self.TTL, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.error);
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
        if self.ID != 0 {
            os.write_int64(2, self.ID)?;
        }
        if self.TTL != 0 {
            os.write_int64(3, self.TTL)?;
        }
        if !self.error.is_empty() {
            os.write_string(4, &self.error)?;
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

impl ::protobuf::MessageStatic for LeaseGrantResponse {
    fn new() -> LeaseGrantResponse {
        LeaseGrantResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseGrantResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    LeaseGrantResponse::get_header_for_reflect,
                    LeaseGrantResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    LeaseGrantResponse::get_ID_for_reflect,
                    LeaseGrantResponse::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "TTL",
                    LeaseGrantResponse::get_TTL_for_reflect,
                    LeaseGrantResponse::mut_TTL_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    LeaseGrantResponse::get_error_for_reflect,
                    LeaseGrantResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseGrantResponse>(
                    "LeaseGrantResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseGrantResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_ID();
        self.clear_TTL();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseGrantResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseGrantResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseRevokeRequest {
    // message fields
    pub ID: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseRevokeRequest {}

impl LeaseRevokeRequest {
    pub fn new() -> LeaseRevokeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseRevokeRequest {
        static mut instance: ::protobuf::lazy::Lazy<LeaseRevokeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseRevokeRequest,
        };
        unsafe {
            instance.get(LeaseRevokeRequest::new)
        }
    }

    // int64 ID = 1;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: i64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> i64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &i64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut i64 {
        &mut self.ID
    }
}

impl ::protobuf::Message for LeaseRevokeRequest {
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
                    self.ID = tmp;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_int64(1, self.ID)?;
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

impl ::protobuf::MessageStatic for LeaseRevokeRequest {
    fn new() -> LeaseRevokeRequest {
        LeaseRevokeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseRevokeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    LeaseRevokeRequest::get_ID_for_reflect,
                    LeaseRevokeRequest::mut_ID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseRevokeRequest>(
                    "LeaseRevokeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseRevokeRequest {
    fn clear(&mut self) {
        self.clear_ID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseRevokeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseRevokeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseRevokeResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseRevokeResponse {}

impl LeaseRevokeResponse {
    pub fn new() -> LeaseRevokeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseRevokeResponse {
        static mut instance: ::protobuf::lazy::Lazy<LeaseRevokeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseRevokeResponse,
        };
        unsafe {
            instance.get(LeaseRevokeResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for LeaseRevokeResponse {
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

impl ::protobuf::MessageStatic for LeaseRevokeResponse {
    fn new() -> LeaseRevokeResponse {
        LeaseRevokeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseRevokeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    LeaseRevokeResponse::get_header_for_reflect,
                    LeaseRevokeResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseRevokeResponse>(
                    "LeaseRevokeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseRevokeResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseRevokeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseRevokeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseKeepAliveRequest {
    // message fields
    pub ID: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseKeepAliveRequest {}

impl LeaseKeepAliveRequest {
    pub fn new() -> LeaseKeepAliveRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseKeepAliveRequest {
        static mut instance: ::protobuf::lazy::Lazy<LeaseKeepAliveRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseKeepAliveRequest,
        };
        unsafe {
            instance.get(LeaseKeepAliveRequest::new)
        }
    }

    // int64 ID = 1;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: i64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> i64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &i64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut i64 {
        &mut self.ID
    }
}

impl ::protobuf::Message for LeaseKeepAliveRequest {
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
                    self.ID = tmp;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_int64(1, self.ID)?;
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

impl ::protobuf::MessageStatic for LeaseKeepAliveRequest {
    fn new() -> LeaseKeepAliveRequest {
        LeaseKeepAliveRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseKeepAliveRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    LeaseKeepAliveRequest::get_ID_for_reflect,
                    LeaseKeepAliveRequest::mut_ID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseKeepAliveRequest>(
                    "LeaseKeepAliveRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseKeepAliveRequest {
    fn clear(&mut self) {
        self.clear_ID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseKeepAliveRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseKeepAliveRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseKeepAliveResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub ID: i64,
    pub TTL: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseKeepAliveResponse {}

impl LeaseKeepAliveResponse {
    pub fn new() -> LeaseKeepAliveResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseKeepAliveResponse {
        static mut instance: ::protobuf::lazy::Lazy<LeaseKeepAliveResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseKeepAliveResponse,
        };
        unsafe {
            instance.get(LeaseKeepAliveResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // int64 ID = 2;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: i64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> i64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &i64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut i64 {
        &mut self.ID
    }

    // int64 TTL = 3;

    pub fn clear_TTL(&mut self) {
        self.TTL = 0;
    }

    // Param is passed by value, moved
    pub fn set_TTL(&mut self, v: i64) {
        self.TTL = v;
    }

    pub fn get_TTL(&self) -> i64 {
        self.TTL
    }

    fn get_TTL_for_reflect(&self) -> &i64 {
        &self.TTL
    }

    fn mut_TTL_for_reflect(&mut self) -> &mut i64 {
        &mut self.TTL
    }
}

impl ::protobuf::Message for LeaseKeepAliveResponse {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ID = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.TTL = tmp;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(2, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.TTL != 0 {
            my_size += ::protobuf::rt::value_size(3, self.TTL, ::protobuf::wire_format::WireTypeVarint);
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
        if self.ID != 0 {
            os.write_int64(2, self.ID)?;
        }
        if self.TTL != 0 {
            os.write_int64(3, self.TTL)?;
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

impl ::protobuf::MessageStatic for LeaseKeepAliveResponse {
    fn new() -> LeaseKeepAliveResponse {
        LeaseKeepAliveResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseKeepAliveResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    LeaseKeepAliveResponse::get_header_for_reflect,
                    LeaseKeepAliveResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    LeaseKeepAliveResponse::get_ID_for_reflect,
                    LeaseKeepAliveResponse::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "TTL",
                    LeaseKeepAliveResponse::get_TTL_for_reflect,
                    LeaseKeepAliveResponse::mut_TTL_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseKeepAliveResponse>(
                    "LeaseKeepAliveResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseKeepAliveResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_ID();
        self.clear_TTL();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseKeepAliveResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseKeepAliveResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseTimeToLiveRequest {
    // message fields
    pub ID: i64,
    pub keys: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseTimeToLiveRequest {}

impl LeaseTimeToLiveRequest {
    pub fn new() -> LeaseTimeToLiveRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseTimeToLiveRequest {
        static mut instance: ::protobuf::lazy::Lazy<LeaseTimeToLiveRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseTimeToLiveRequest,
        };
        unsafe {
            instance.get(LeaseTimeToLiveRequest::new)
        }
    }

    // int64 ID = 1;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: i64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> i64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &i64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut i64 {
        &mut self.ID
    }

    // bool keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys = false;
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: bool) {
        self.keys = v;
    }

    pub fn get_keys(&self) -> bool {
        self.keys
    }

    fn get_keys_for_reflect(&self) -> &bool {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut bool {
        &mut self.keys
    }
}

impl ::protobuf::Message for LeaseTimeToLiveRequest {
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
                    self.ID = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.keys = tmp;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.keys != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_int64(1, self.ID)?;
        }
        if self.keys != false {
            os.write_bool(2, self.keys)?;
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

impl ::protobuf::MessageStatic for LeaseTimeToLiveRequest {
    fn new() -> LeaseTimeToLiveRequest {
        LeaseTimeToLiveRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseTimeToLiveRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    LeaseTimeToLiveRequest::get_ID_for_reflect,
                    LeaseTimeToLiveRequest::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "keys",
                    LeaseTimeToLiveRequest::get_keys_for_reflect,
                    LeaseTimeToLiveRequest::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseTimeToLiveRequest>(
                    "LeaseTimeToLiveRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseTimeToLiveRequest {
    fn clear(&mut self) {
        self.clear_ID();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseTimeToLiveRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseTimeToLiveRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseTimeToLiveResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub ID: i64,
    pub TTL: i64,
    pub grantedTTL: i64,
    pub keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseTimeToLiveResponse {}

impl LeaseTimeToLiveResponse {
    pub fn new() -> LeaseTimeToLiveResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseTimeToLiveResponse {
        static mut instance: ::protobuf::lazy::Lazy<LeaseTimeToLiveResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseTimeToLiveResponse,
        };
        unsafe {
            instance.get(LeaseTimeToLiveResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // int64 ID = 2;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: i64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> i64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &i64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut i64 {
        &mut self.ID
    }

    // int64 TTL = 3;

    pub fn clear_TTL(&mut self) {
        self.TTL = 0;
    }

    // Param is passed by value, moved
    pub fn set_TTL(&mut self, v: i64) {
        self.TTL = v;
    }

    pub fn get_TTL(&self) -> i64 {
        self.TTL
    }

    fn get_TTL_for_reflect(&self) -> &i64 {
        &self.TTL
    }

    fn mut_TTL_for_reflect(&mut self) -> &mut i64 {
        &mut self.TTL
    }

    // int64 grantedTTL = 4;

    pub fn clear_grantedTTL(&mut self) {
        self.grantedTTL = 0;
    }

    // Param is passed by value, moved
    pub fn set_grantedTTL(&mut self, v: i64) {
        self.grantedTTL = v;
    }

    pub fn get_grantedTTL(&self) -> i64 {
        self.grantedTTL
    }

    fn get_grantedTTL_for_reflect(&self) -> &i64 {
        &self.grantedTTL
    }

    fn mut_grantedTTL_for_reflect(&mut self) -> &mut i64 {
        &mut self.grantedTTL
    }

    // repeated bytes keys = 5;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }
}

impl ::protobuf::Message for LeaseTimeToLiveResponse {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ID = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.TTL = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.grantedTTL = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys)?;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(2, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.TTL != 0 {
            my_size += ::protobuf::rt::value_size(3, self.TTL, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.grantedTTL != 0 {
            my_size += ::protobuf::rt::value_size(4, self.grantedTTL, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
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
        if self.ID != 0 {
            os.write_int64(2, self.ID)?;
        }
        if self.TTL != 0 {
            os.write_int64(3, self.TTL)?;
        }
        if self.grantedTTL != 0 {
            os.write_int64(4, self.grantedTTL)?;
        }
        for v in &self.keys {
            os.write_bytes(5, &v)?;
        };
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

impl ::protobuf::MessageStatic for LeaseTimeToLiveResponse {
    fn new() -> LeaseTimeToLiveResponse {
        LeaseTimeToLiveResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseTimeToLiveResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    LeaseTimeToLiveResponse::get_header_for_reflect,
                    LeaseTimeToLiveResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    LeaseTimeToLiveResponse::get_ID_for_reflect,
                    LeaseTimeToLiveResponse::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "TTL",
                    LeaseTimeToLiveResponse::get_TTL_for_reflect,
                    LeaseTimeToLiveResponse::mut_TTL_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "grantedTTL",
                    LeaseTimeToLiveResponse::get_grantedTTL_for_reflect,
                    LeaseTimeToLiveResponse::mut_grantedTTL_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keys",
                    LeaseTimeToLiveResponse::get_keys_for_reflect,
                    LeaseTimeToLiveResponse::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseTimeToLiveResponse>(
                    "LeaseTimeToLiveResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseTimeToLiveResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_ID();
        self.clear_TTL();
        self.clear_grantedTTL();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseTimeToLiveResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseTimeToLiveResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseLeasesRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseLeasesRequest {}

impl LeaseLeasesRequest {
    pub fn new() -> LeaseLeasesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseLeasesRequest {
        static mut instance: ::protobuf::lazy::Lazy<LeaseLeasesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseLeasesRequest,
        };
        unsafe {
            instance.get(LeaseLeasesRequest::new)
        }
    }
}

impl ::protobuf::Message for LeaseLeasesRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for LeaseLeasesRequest {
    fn new() -> LeaseLeasesRequest {
        LeaseLeasesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseLeasesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<LeaseLeasesRequest>(
                    "LeaseLeasesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseLeasesRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseLeasesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseLeasesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseStatus {
    // message fields
    pub ID: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseStatus {}

impl LeaseStatus {
    pub fn new() -> LeaseStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseStatus {
        static mut instance: ::protobuf::lazy::Lazy<LeaseStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseStatus,
        };
        unsafe {
            instance.get(LeaseStatus::new)
        }
    }

    // int64 ID = 1;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: i64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> i64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &i64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut i64 {
        &mut self.ID
    }
}

impl ::protobuf::Message for LeaseStatus {
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
                    self.ID = tmp;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_int64(1, self.ID)?;
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

impl ::protobuf::MessageStatic for LeaseStatus {
    fn new() -> LeaseStatus {
        LeaseStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    LeaseStatus::get_ID_for_reflect,
                    LeaseStatus::mut_ID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseStatus>(
                    "LeaseStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseStatus {
    fn clear(&mut self) {
        self.clear_ID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseLeasesResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub leases: ::protobuf::RepeatedField<LeaseStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseLeasesResponse {}

impl LeaseLeasesResponse {
    pub fn new() -> LeaseLeasesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseLeasesResponse {
        static mut instance: ::protobuf::lazy::Lazy<LeaseLeasesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseLeasesResponse,
        };
        unsafe {
            instance.get(LeaseLeasesResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated .etcdserverpb.LeaseStatus leases = 2;

    pub fn clear_leases(&mut self) {
        self.leases.clear();
    }

    // Param is passed by value, moved
    pub fn set_leases(&mut self, v: ::protobuf::RepeatedField<LeaseStatus>) {
        self.leases = v;
    }

    // Mutable pointer to the field.
    pub fn mut_leases(&mut self) -> &mut ::protobuf::RepeatedField<LeaseStatus> {
        &mut self.leases
    }

    // Take field
    pub fn take_leases(&mut self) -> ::protobuf::RepeatedField<LeaseStatus> {
        ::std::mem::replace(&mut self.leases, ::protobuf::RepeatedField::new())
    }

    pub fn get_leases(&self) -> &[LeaseStatus] {
        &self.leases
    }

    fn get_leases_for_reflect(&self) -> &::protobuf::RepeatedField<LeaseStatus> {
        &self.leases
    }

    fn mut_leases_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LeaseStatus> {
        &mut self.leases
    }
}

impl ::protobuf::Message for LeaseLeasesResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.leases {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.leases)?;
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
        for value in &self.leases {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.leases {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for LeaseLeasesResponse {
    fn new() -> LeaseLeasesResponse {
        LeaseLeasesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseLeasesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    LeaseLeasesResponse::get_header_for_reflect,
                    LeaseLeasesResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LeaseStatus>>(
                    "leases",
                    LeaseLeasesResponse::get_leases_for_reflect,
                    LeaseLeasesResponse::mut_leases_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseLeasesResponse>(
                    "LeaseLeasesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseLeasesResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_leases();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseLeasesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseLeasesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Member {
    // message fields
    pub ID: u64,
    pub name: ::std::string::String,
    pub peerURLs: ::protobuf::RepeatedField<::std::string::String>,
    pub clientURLs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Member {}

impl Member {
    pub fn new() -> Member {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Member {
        static mut instance: ::protobuf::lazy::Lazy<Member> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Member,
        };
        unsafe {
            instance.get(Member::new)
        }
    }

    // uint64 ID = 1;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: u64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> u64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &u64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut u64 {
        &mut self.ID
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated string peerURLs = 3;

    pub fn clear_peerURLs(&mut self) {
        self.peerURLs.clear();
    }

    // Param is passed by value, moved
    pub fn set_peerURLs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.peerURLs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peerURLs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.peerURLs
    }

    // Take field
    pub fn take_peerURLs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.peerURLs, ::protobuf::RepeatedField::new())
    }

    pub fn get_peerURLs(&self) -> &[::std::string::String] {
        &self.peerURLs
    }

    fn get_peerURLs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.peerURLs
    }

    fn mut_peerURLs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.peerURLs
    }

    // repeated string clientURLs = 4;

    pub fn clear_clientURLs(&mut self) {
        self.clientURLs.clear();
    }

    // Param is passed by value, moved
    pub fn set_clientURLs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.clientURLs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_clientURLs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.clientURLs
    }

    // Take field
    pub fn take_clientURLs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.clientURLs, ::protobuf::RepeatedField::new())
    }

    pub fn get_clientURLs(&self) -> &[::std::string::String] {
        &self.clientURLs
    }

    fn get_clientURLs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.clientURLs
    }

    fn mut_clientURLs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.clientURLs
    }
}

impl ::protobuf::Message for Member {
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
                    self.ID = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.peerURLs)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.clientURLs)?;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        for value in &self.peerURLs {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.clientURLs {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_uint64(1, self.ID)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        for v in &self.peerURLs {
            os.write_string(3, &v)?;
        };
        for v in &self.clientURLs {
            os.write_string(4, &v)?;
        };
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

impl ::protobuf::MessageStatic for Member {
    fn new() -> Member {
        Member::new()
    }

    fn descriptor_static(_: ::std::option::Option<Member>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ID",
                    Member::get_ID_for_reflect,
                    Member::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Member::get_name_for_reflect,
                    Member::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "peerURLs",
                    Member::get_peerURLs_for_reflect,
                    Member::mut_peerURLs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientURLs",
                    Member::get_clientURLs_for_reflect,
                    Member::mut_clientURLs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Member>(
                    "Member",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Member {
    fn clear(&mut self) {
        self.clear_ID();
        self.clear_name();
        self.clear_peerURLs();
        self.clear_clientURLs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Member {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Member {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemberAddRequest {
    // message fields
    pub peerURLs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemberAddRequest {}

impl MemberAddRequest {
    pub fn new() -> MemberAddRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemberAddRequest {
        static mut instance: ::protobuf::lazy::Lazy<MemberAddRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemberAddRequest,
        };
        unsafe {
            instance.get(MemberAddRequest::new)
        }
    }

    // repeated string peerURLs = 1;

    pub fn clear_peerURLs(&mut self) {
        self.peerURLs.clear();
    }

    // Param is passed by value, moved
    pub fn set_peerURLs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.peerURLs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peerURLs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.peerURLs
    }

    // Take field
    pub fn take_peerURLs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.peerURLs, ::protobuf::RepeatedField::new())
    }

    pub fn get_peerURLs(&self) -> &[::std::string::String] {
        &self.peerURLs
    }

    fn get_peerURLs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.peerURLs
    }

    fn mut_peerURLs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.peerURLs
    }
}

impl ::protobuf::Message for MemberAddRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.peerURLs)?;
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
        for value in &self.peerURLs {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.peerURLs {
            os.write_string(1, &v)?;
        };
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

impl ::protobuf::MessageStatic for MemberAddRequest {
    fn new() -> MemberAddRequest {
        MemberAddRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemberAddRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "peerURLs",
                    MemberAddRequest::get_peerURLs_for_reflect,
                    MemberAddRequest::mut_peerURLs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemberAddRequest>(
                    "MemberAddRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemberAddRequest {
    fn clear(&mut self) {
        self.clear_peerURLs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemberAddRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemberAddRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemberAddResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub member: ::protobuf::SingularPtrField<Member>,
    pub members: ::protobuf::RepeatedField<Member>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemberAddResponse {}

impl MemberAddResponse {
    pub fn new() -> MemberAddResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemberAddResponse {
        static mut instance: ::protobuf::lazy::Lazy<MemberAddResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemberAddResponse,
        };
        unsafe {
            instance.get(MemberAddResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // .etcdserverpb.Member member = 2;

    pub fn clear_member(&mut self) {
        self.member.clear();
    }

    pub fn has_member(&self) -> bool {
        self.member.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member(&mut self, v: Member) {
        self.member = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_member(&mut self) -> &mut Member {
        if self.member.is_none() {
            self.member.set_default();
        }
        self.member.as_mut().unwrap()
    }

    // Take field
    pub fn take_member(&mut self) -> Member {
        self.member.take().unwrap_or_else(|| Member::new())
    }

    pub fn get_member(&self) -> &Member {
        self.member.as_ref().unwrap_or_else(|| Member::default_instance())
    }

    fn get_member_for_reflect(&self) -> &::protobuf::SingularPtrField<Member> {
        &self.member
    }

    fn mut_member_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Member> {
        &mut self.member
    }

    // repeated .etcdserverpb.Member members = 3;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<Member>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<Member> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<Member> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[Member] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<Member> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Member> {
        &mut self.members
    }
}

impl ::protobuf::Message for MemberAddResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.member {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.members {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.member)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
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
        if let Some(ref v) = self.member.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        if let Some(ref v) = self.member.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.members {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for MemberAddResponse {
    fn new() -> MemberAddResponse {
        MemberAddResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemberAddResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    MemberAddResponse::get_header_for_reflect,
                    MemberAddResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Member>>(
                    "member",
                    MemberAddResponse::get_member_for_reflect,
                    MemberAddResponse::mut_member_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Member>>(
                    "members",
                    MemberAddResponse::get_members_for_reflect,
                    MemberAddResponse::mut_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemberAddResponse>(
                    "MemberAddResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemberAddResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_member();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemberAddResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemberAddResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemberRemoveRequest {
    // message fields
    pub ID: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemberRemoveRequest {}

impl MemberRemoveRequest {
    pub fn new() -> MemberRemoveRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemberRemoveRequest {
        static mut instance: ::protobuf::lazy::Lazy<MemberRemoveRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemberRemoveRequest,
        };
        unsafe {
            instance.get(MemberRemoveRequest::new)
        }
    }

    // uint64 ID = 1;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: u64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> u64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &u64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut u64 {
        &mut self.ID
    }
}

impl ::protobuf::Message for MemberRemoveRequest {
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
                    self.ID = tmp;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_uint64(1, self.ID)?;
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

impl ::protobuf::MessageStatic for MemberRemoveRequest {
    fn new() -> MemberRemoveRequest {
        MemberRemoveRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemberRemoveRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ID",
                    MemberRemoveRequest::get_ID_for_reflect,
                    MemberRemoveRequest::mut_ID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemberRemoveRequest>(
                    "MemberRemoveRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemberRemoveRequest {
    fn clear(&mut self) {
        self.clear_ID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemberRemoveRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemberRemoveRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemberRemoveResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub members: ::protobuf::RepeatedField<Member>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemberRemoveResponse {}

impl MemberRemoveResponse {
    pub fn new() -> MemberRemoveResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemberRemoveResponse {
        static mut instance: ::protobuf::lazy::Lazy<MemberRemoveResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemberRemoveResponse,
        };
        unsafe {
            instance.get(MemberRemoveResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated .etcdserverpb.Member members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<Member>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<Member> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<Member> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[Member] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<Member> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Member> {
        &mut self.members
    }
}

impl ::protobuf::Message for MemberRemoveResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.members {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
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
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.members {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for MemberRemoveResponse {
    fn new() -> MemberRemoveResponse {
        MemberRemoveResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemberRemoveResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    MemberRemoveResponse::get_header_for_reflect,
                    MemberRemoveResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Member>>(
                    "members",
                    MemberRemoveResponse::get_members_for_reflect,
                    MemberRemoveResponse::mut_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemberRemoveResponse>(
                    "MemberRemoveResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemberRemoveResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemberRemoveResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemberRemoveResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemberUpdateRequest {
    // message fields
    pub ID: u64,
    pub peerURLs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemberUpdateRequest {}

impl MemberUpdateRequest {
    pub fn new() -> MemberUpdateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemberUpdateRequest {
        static mut instance: ::protobuf::lazy::Lazy<MemberUpdateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemberUpdateRequest,
        };
        unsafe {
            instance.get(MemberUpdateRequest::new)
        }
    }

    // uint64 ID = 1;

    pub fn clear_ID(&mut self) {
        self.ID = 0;
    }

    // Param is passed by value, moved
    pub fn set_ID(&mut self, v: u64) {
        self.ID = v;
    }

    pub fn get_ID(&self) -> u64 {
        self.ID
    }

    fn get_ID_for_reflect(&self) -> &u64 {
        &self.ID
    }

    fn mut_ID_for_reflect(&mut self) -> &mut u64 {
        &mut self.ID
    }

    // repeated string peerURLs = 2;

    pub fn clear_peerURLs(&mut self) {
        self.peerURLs.clear();
    }

    // Param is passed by value, moved
    pub fn set_peerURLs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.peerURLs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peerURLs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.peerURLs
    }

    // Take field
    pub fn take_peerURLs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.peerURLs, ::protobuf::RepeatedField::new())
    }

    pub fn get_peerURLs(&self) -> &[::std::string::String] {
        &self.peerURLs
    }

    fn get_peerURLs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.peerURLs
    }

    fn mut_peerURLs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.peerURLs
    }
}

impl ::protobuf::Message for MemberUpdateRequest {
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
                    self.ID = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.peerURLs)?;
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.peerURLs {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_uint64(1, self.ID)?;
        }
        for v in &self.peerURLs {
            os.write_string(2, &v)?;
        };
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

impl ::protobuf::MessageStatic for MemberUpdateRequest {
    fn new() -> MemberUpdateRequest {
        MemberUpdateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemberUpdateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ID",
                    MemberUpdateRequest::get_ID_for_reflect,
                    MemberUpdateRequest::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "peerURLs",
                    MemberUpdateRequest::get_peerURLs_for_reflect,
                    MemberUpdateRequest::mut_peerURLs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemberUpdateRequest>(
                    "MemberUpdateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemberUpdateRequest {
    fn clear(&mut self) {
        self.clear_ID();
        self.clear_peerURLs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemberUpdateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemberUpdateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemberUpdateResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub members: ::protobuf::RepeatedField<Member>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemberUpdateResponse {}

impl MemberUpdateResponse {
    pub fn new() -> MemberUpdateResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemberUpdateResponse {
        static mut instance: ::protobuf::lazy::Lazy<MemberUpdateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemberUpdateResponse,
        };
        unsafe {
            instance.get(MemberUpdateResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated .etcdserverpb.Member members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<Member>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<Member> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<Member> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[Member] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<Member> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Member> {
        &mut self.members
    }
}

impl ::protobuf::Message for MemberUpdateResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.members {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
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
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.members {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for MemberUpdateResponse {
    fn new() -> MemberUpdateResponse {
        MemberUpdateResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemberUpdateResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    MemberUpdateResponse::get_header_for_reflect,
                    MemberUpdateResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Member>>(
                    "members",
                    MemberUpdateResponse::get_members_for_reflect,
                    MemberUpdateResponse::mut_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemberUpdateResponse>(
                    "MemberUpdateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemberUpdateResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemberUpdateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemberUpdateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemberListRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemberListRequest {}

impl MemberListRequest {
    pub fn new() -> MemberListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemberListRequest {
        static mut instance: ::protobuf::lazy::Lazy<MemberListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemberListRequest,
        };
        unsafe {
            instance.get(MemberListRequest::new)
        }
    }
}

impl ::protobuf::Message for MemberListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for MemberListRequest {
    fn new() -> MemberListRequest {
        MemberListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemberListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<MemberListRequest>(
                    "MemberListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemberListRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemberListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemberListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemberListResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub members: ::protobuf::RepeatedField<Member>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemberListResponse {}

impl MemberListResponse {
    pub fn new() -> MemberListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemberListResponse {
        static mut instance: ::protobuf::lazy::Lazy<MemberListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemberListResponse,
        };
        unsafe {
            instance.get(MemberListResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated .etcdserverpb.Member members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<Member>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<Member> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<Member> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[Member] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<Member> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Member> {
        &mut self.members
    }
}

impl ::protobuf::Message for MemberListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.members {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
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
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.members {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for MemberListResponse {
    fn new() -> MemberListResponse {
        MemberListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemberListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    MemberListResponse::get_header_for_reflect,
                    MemberListResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Member>>(
                    "members",
                    MemberListResponse::get_members_for_reflect,
                    MemberListResponse::mut_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemberListResponse>(
                    "MemberListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemberListResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemberListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemberListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DefragmentRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DefragmentRequest {}

impl DefragmentRequest {
    pub fn new() -> DefragmentRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DefragmentRequest {
        static mut instance: ::protobuf::lazy::Lazy<DefragmentRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DefragmentRequest,
        };
        unsafe {
            instance.get(DefragmentRequest::new)
        }
    }
}

impl ::protobuf::Message for DefragmentRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for DefragmentRequest {
    fn new() -> DefragmentRequest {
        DefragmentRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DefragmentRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DefragmentRequest>(
                    "DefragmentRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DefragmentRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DefragmentRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DefragmentRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DefragmentResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DefragmentResponse {}

impl DefragmentResponse {
    pub fn new() -> DefragmentResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DefragmentResponse {
        static mut instance: ::protobuf::lazy::Lazy<DefragmentResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DefragmentResponse,
        };
        unsafe {
            instance.get(DefragmentResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for DefragmentResponse {
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

impl ::protobuf::MessageStatic for DefragmentResponse {
    fn new() -> DefragmentResponse {
        DefragmentResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DefragmentResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    DefragmentResponse::get_header_for_reflect,
                    DefragmentResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DefragmentResponse>(
                    "DefragmentResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DefragmentResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DefragmentResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DefragmentResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MoveLeaderRequest {
    // message fields
    pub targetID: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MoveLeaderRequest {}

impl MoveLeaderRequest {
    pub fn new() -> MoveLeaderRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MoveLeaderRequest {
        static mut instance: ::protobuf::lazy::Lazy<MoveLeaderRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MoveLeaderRequest,
        };
        unsafe {
            instance.get(MoveLeaderRequest::new)
        }
    }

    // uint64 targetID = 1;

    pub fn clear_targetID(&mut self) {
        self.targetID = 0;
    }

    // Param is passed by value, moved
    pub fn set_targetID(&mut self, v: u64) {
        self.targetID = v;
    }

    pub fn get_targetID(&self) -> u64 {
        self.targetID
    }

    fn get_targetID_for_reflect(&self) -> &u64 {
        &self.targetID
    }

    fn mut_targetID_for_reflect(&mut self) -> &mut u64 {
        &mut self.targetID
    }
}

impl ::protobuf::Message for MoveLeaderRequest {
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
                    self.targetID = tmp;
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
        if self.targetID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.targetID, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.targetID != 0 {
            os.write_uint64(1, self.targetID)?;
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

impl ::protobuf::MessageStatic for MoveLeaderRequest {
    fn new() -> MoveLeaderRequest {
        MoveLeaderRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MoveLeaderRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "targetID",
                    MoveLeaderRequest::get_targetID_for_reflect,
                    MoveLeaderRequest::mut_targetID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MoveLeaderRequest>(
                    "MoveLeaderRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MoveLeaderRequest {
    fn clear(&mut self) {
        self.clear_targetID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MoveLeaderRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MoveLeaderRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MoveLeaderResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MoveLeaderResponse {}

impl MoveLeaderResponse {
    pub fn new() -> MoveLeaderResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MoveLeaderResponse {
        static mut instance: ::protobuf::lazy::Lazy<MoveLeaderResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MoveLeaderResponse,
        };
        unsafe {
            instance.get(MoveLeaderResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for MoveLeaderResponse {
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

impl ::protobuf::MessageStatic for MoveLeaderResponse {
    fn new() -> MoveLeaderResponse {
        MoveLeaderResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<MoveLeaderResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    MoveLeaderResponse::get_header_for_reflect,
                    MoveLeaderResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MoveLeaderResponse>(
                    "MoveLeaderResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MoveLeaderResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MoveLeaderResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MoveLeaderResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlarmRequest {
    // message fields
    pub action: AlarmRequest_AlarmAction,
    pub memberID: u64,
    pub alarm: AlarmType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlarmRequest {}

impl AlarmRequest {
    pub fn new() -> AlarmRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlarmRequest {
        static mut instance: ::protobuf::lazy::Lazy<AlarmRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlarmRequest,
        };
        unsafe {
            instance.get(AlarmRequest::new)
        }
    }

    // .etcdserverpb.AlarmRequest.AlarmAction action = 1;

    pub fn clear_action(&mut self) {
        self.action = AlarmRequest_AlarmAction::GET;
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: AlarmRequest_AlarmAction) {
        self.action = v;
    }

    pub fn get_action(&self) -> AlarmRequest_AlarmAction {
        self.action
    }

    fn get_action_for_reflect(&self) -> &AlarmRequest_AlarmAction {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut AlarmRequest_AlarmAction {
        &mut self.action
    }

    // uint64 memberID = 2;

    pub fn clear_memberID(&mut self) {
        self.memberID = 0;
    }

    // Param is passed by value, moved
    pub fn set_memberID(&mut self, v: u64) {
        self.memberID = v;
    }

    pub fn get_memberID(&self) -> u64 {
        self.memberID
    }

    fn get_memberID_for_reflect(&self) -> &u64 {
        &self.memberID
    }

    fn mut_memberID_for_reflect(&mut self) -> &mut u64 {
        &mut self.memberID
    }

    // .etcdserverpb.AlarmType alarm = 3;

    pub fn clear_alarm(&mut self) {
        self.alarm = AlarmType::NONE;
    }

    // Param is passed by value, moved
    pub fn set_alarm(&mut self, v: AlarmType) {
        self.alarm = v;
    }

    pub fn get_alarm(&self) -> AlarmType {
        self.alarm
    }

    fn get_alarm_for_reflect(&self) -> &AlarmType {
        &self.alarm
    }

    fn mut_alarm_for_reflect(&mut self) -> &mut AlarmType {
        &mut self.alarm
    }
}

impl ::protobuf::Message for AlarmRequest {
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
                    let tmp = is.read_enum()?;
                    self.action = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.memberID = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.alarm = tmp;
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
        if self.action != AlarmRequest_AlarmAction::GET {
            my_size += ::protobuf::rt::enum_size(1, self.action);
        }
        if self.memberID != 0 {
            my_size += ::protobuf::rt::value_size(2, self.memberID, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.alarm != AlarmType::NONE {
            my_size += ::protobuf::rt::enum_size(3, self.alarm);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.action != AlarmRequest_AlarmAction::GET {
            os.write_enum(1, self.action.value())?;
        }
        if self.memberID != 0 {
            os.write_uint64(2, self.memberID)?;
        }
        if self.alarm != AlarmType::NONE {
            os.write_enum(3, self.alarm.value())?;
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

impl ::protobuf::MessageStatic for AlarmRequest {
    fn new() -> AlarmRequest {
        AlarmRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlarmRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AlarmRequest_AlarmAction>>(
                    "action",
                    AlarmRequest::get_action_for_reflect,
                    AlarmRequest::mut_action_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "memberID",
                    AlarmRequest::get_memberID_for_reflect,
                    AlarmRequest::mut_memberID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AlarmType>>(
                    "alarm",
                    AlarmRequest::get_alarm_for_reflect,
                    AlarmRequest::mut_alarm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlarmRequest>(
                    "AlarmRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlarmRequest {
    fn clear(&mut self) {
        self.clear_action();
        self.clear_memberID();
        self.clear_alarm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlarmRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlarmRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AlarmRequest_AlarmAction {
    GET = 0,
    ACTIVATE = 1,
    DEACTIVATE = 2,
}

impl ::protobuf::ProtobufEnum for AlarmRequest_AlarmAction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AlarmRequest_AlarmAction> {
        match value {
            0 => ::std::option::Option::Some(AlarmRequest_AlarmAction::GET),
            1 => ::std::option::Option::Some(AlarmRequest_AlarmAction::ACTIVATE),
            2 => ::std::option::Option::Some(AlarmRequest_AlarmAction::DEACTIVATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AlarmRequest_AlarmAction] = &[
            AlarmRequest_AlarmAction::GET,
            AlarmRequest_AlarmAction::ACTIVATE,
            AlarmRequest_AlarmAction::DEACTIVATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AlarmRequest_AlarmAction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AlarmRequest_AlarmAction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AlarmRequest_AlarmAction {
}

impl ::std::default::Default for AlarmRequest_AlarmAction {
    fn default() -> Self {
        AlarmRequest_AlarmAction::GET
    }
}

impl ::protobuf::reflect::ProtobufValue for AlarmRequest_AlarmAction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlarmMember {
    // message fields
    pub memberID: u64,
    pub alarm: AlarmType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlarmMember {}

impl AlarmMember {
    pub fn new() -> AlarmMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlarmMember {
        static mut instance: ::protobuf::lazy::Lazy<AlarmMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlarmMember,
        };
        unsafe {
            instance.get(AlarmMember::new)
        }
    }

    // uint64 memberID = 1;

    pub fn clear_memberID(&mut self) {
        self.memberID = 0;
    }

    // Param is passed by value, moved
    pub fn set_memberID(&mut self, v: u64) {
        self.memberID = v;
    }

    pub fn get_memberID(&self) -> u64 {
        self.memberID
    }

    fn get_memberID_for_reflect(&self) -> &u64 {
        &self.memberID
    }

    fn mut_memberID_for_reflect(&mut self) -> &mut u64 {
        &mut self.memberID
    }

    // .etcdserverpb.AlarmType alarm = 2;

    pub fn clear_alarm(&mut self) {
        self.alarm = AlarmType::NONE;
    }

    // Param is passed by value, moved
    pub fn set_alarm(&mut self, v: AlarmType) {
        self.alarm = v;
    }

    pub fn get_alarm(&self) -> AlarmType {
        self.alarm
    }

    fn get_alarm_for_reflect(&self) -> &AlarmType {
        &self.alarm
    }

    fn mut_alarm_for_reflect(&mut self) -> &mut AlarmType {
        &mut self.alarm
    }
}

impl ::protobuf::Message for AlarmMember {
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
                    self.memberID = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.alarm = tmp;
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
        if self.memberID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.memberID, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.alarm != AlarmType::NONE {
            my_size += ::protobuf::rt::enum_size(2, self.alarm);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.memberID != 0 {
            os.write_uint64(1, self.memberID)?;
        }
        if self.alarm != AlarmType::NONE {
            os.write_enum(2, self.alarm.value())?;
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

impl ::protobuf::MessageStatic for AlarmMember {
    fn new() -> AlarmMember {
        AlarmMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlarmMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "memberID",
                    AlarmMember::get_memberID_for_reflect,
                    AlarmMember::mut_memberID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AlarmType>>(
                    "alarm",
                    AlarmMember::get_alarm_for_reflect,
                    AlarmMember::mut_alarm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlarmMember>(
                    "AlarmMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlarmMember {
    fn clear(&mut self) {
        self.clear_memberID();
        self.clear_alarm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlarmMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlarmMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlarmResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub alarms: ::protobuf::RepeatedField<AlarmMember>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlarmResponse {}

impl AlarmResponse {
    pub fn new() -> AlarmResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlarmResponse {
        static mut instance: ::protobuf::lazy::Lazy<AlarmResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlarmResponse,
        };
        unsafe {
            instance.get(AlarmResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated .etcdserverpb.AlarmMember alarms = 2;

    pub fn clear_alarms(&mut self) {
        self.alarms.clear();
    }

    // Param is passed by value, moved
    pub fn set_alarms(&mut self, v: ::protobuf::RepeatedField<AlarmMember>) {
        self.alarms = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alarms(&mut self) -> &mut ::protobuf::RepeatedField<AlarmMember> {
        &mut self.alarms
    }

    // Take field
    pub fn take_alarms(&mut self) -> ::protobuf::RepeatedField<AlarmMember> {
        ::std::mem::replace(&mut self.alarms, ::protobuf::RepeatedField::new())
    }

    pub fn get_alarms(&self) -> &[AlarmMember] {
        &self.alarms
    }

    fn get_alarms_for_reflect(&self) -> &::protobuf::RepeatedField<AlarmMember> {
        &self.alarms
    }

    fn mut_alarms_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AlarmMember> {
        &mut self.alarms
    }
}

impl ::protobuf::Message for AlarmResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.alarms {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.alarms)?;
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
        for value in &self.alarms {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.alarms {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for AlarmResponse {
    fn new() -> AlarmResponse {
        AlarmResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlarmResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AlarmResponse::get_header_for_reflect,
                    AlarmResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlarmMember>>(
                    "alarms",
                    AlarmResponse::get_alarms_for_reflect,
                    AlarmResponse::mut_alarms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlarmResponse>(
                    "AlarmResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlarmResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_alarms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlarmResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlarmResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatusRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatusRequest {}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusRequest {
        static mut instance: ::protobuf::lazy::Lazy<StatusRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusRequest,
        };
        unsafe {
            instance.get(StatusRequest::new)
        }
    }
}

impl ::protobuf::Message for StatusRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for StatusRequest {
    fn new() -> StatusRequest {
        StatusRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StatusRequest>(
                    "StatusRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatusRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatusRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatusResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub version: ::std::string::String,
    pub dbSize: i64,
    pub leader: u64,
    pub raftIndex: u64,
    pub raftTerm: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatusResponse {}

impl StatusResponse {
    pub fn new() -> StatusResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusResponse {
        static mut instance: ::protobuf::lazy::Lazy<StatusResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusResponse,
        };
        unsafe {
            instance.get(StatusResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // string version = 2;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // int64 dbSize = 3;

    pub fn clear_dbSize(&mut self) {
        self.dbSize = 0;
    }

    // Param is passed by value, moved
    pub fn set_dbSize(&mut self, v: i64) {
        self.dbSize = v;
    }

    pub fn get_dbSize(&self) -> i64 {
        self.dbSize
    }

    fn get_dbSize_for_reflect(&self) -> &i64 {
        &self.dbSize
    }

    fn mut_dbSize_for_reflect(&mut self) -> &mut i64 {
        &mut self.dbSize
    }

    // uint64 leader = 4;

    pub fn clear_leader(&mut self) {
        self.leader = 0;
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: u64) {
        self.leader = v;
    }

    pub fn get_leader(&self) -> u64 {
        self.leader
    }

    fn get_leader_for_reflect(&self) -> &u64 {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut u64 {
        &mut self.leader
    }

    // uint64 raftIndex = 5;

    pub fn clear_raftIndex(&mut self) {
        self.raftIndex = 0;
    }

    // Param is passed by value, moved
    pub fn set_raftIndex(&mut self, v: u64) {
        self.raftIndex = v;
    }

    pub fn get_raftIndex(&self) -> u64 {
        self.raftIndex
    }

    fn get_raftIndex_for_reflect(&self) -> &u64 {
        &self.raftIndex
    }

    fn mut_raftIndex_for_reflect(&mut self) -> &mut u64 {
        &mut self.raftIndex
    }

    // uint64 raftTerm = 6;

    pub fn clear_raftTerm(&mut self) {
        self.raftTerm = 0;
    }

    // Param is passed by value, moved
    pub fn set_raftTerm(&mut self, v: u64) {
        self.raftTerm = v;
    }

    pub fn get_raftTerm(&self) -> u64 {
        self.raftTerm
    }

    fn get_raftTerm_for_reflect(&self) -> &u64 {
        &self.raftTerm
    }

    fn mut_raftTerm_for_reflect(&mut self) -> &mut u64 {
        &mut self.raftTerm
    }
}

impl ::protobuf::Message for StatusResponse {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.dbSize = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.leader = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.raftIndex = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.raftTerm = tmp;
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
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.version);
        }
        if self.dbSize != 0 {
            my_size += ::protobuf::rt::value_size(3, self.dbSize, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.leader != 0 {
            my_size += ::protobuf::rt::value_size(4, self.leader, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.raftIndex != 0 {
            my_size += ::protobuf::rt::value_size(5, self.raftIndex, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.raftTerm != 0 {
            my_size += ::protobuf::rt::value_size(6, self.raftTerm, ::protobuf::wire_format::WireTypeVarint);
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
        if !self.version.is_empty() {
            os.write_string(2, &self.version)?;
        }
        if self.dbSize != 0 {
            os.write_int64(3, self.dbSize)?;
        }
        if self.leader != 0 {
            os.write_uint64(4, self.leader)?;
        }
        if self.raftIndex != 0 {
            os.write_uint64(5, self.raftIndex)?;
        }
        if self.raftTerm != 0 {
            os.write_uint64(6, self.raftTerm)?;
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

impl ::protobuf::MessageStatic for StatusResponse {
    fn new() -> StatusResponse {
        StatusResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    StatusResponse::get_header_for_reflect,
                    StatusResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    StatusResponse::get_version_for_reflect,
                    StatusResponse::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "dbSize",
                    StatusResponse::get_dbSize_for_reflect,
                    StatusResponse::mut_dbSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "leader",
                    StatusResponse::get_leader_for_reflect,
                    StatusResponse::mut_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "raftIndex",
                    StatusResponse::get_raftIndex_for_reflect,
                    StatusResponse::mut_raftIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "raftTerm",
                    StatusResponse::get_raftTerm_for_reflect,
                    StatusResponse::mut_raftTerm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusResponse>(
                    "StatusResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_version();
        self.clear_dbSize();
        self.clear_leader();
        self.clear_raftIndex();
        self.clear_raftTerm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatusResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatusResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthEnableRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthEnableRequest {}

impl AuthEnableRequest {
    pub fn new() -> AuthEnableRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthEnableRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthEnableRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthEnableRequest,
        };
        unsafe {
            instance.get(AuthEnableRequest::new)
        }
    }
}

impl ::protobuf::Message for AuthEnableRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for AuthEnableRequest {
    fn new() -> AuthEnableRequest {
        AuthEnableRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthEnableRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AuthEnableRequest>(
                    "AuthEnableRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthEnableRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthEnableRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthEnableRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthDisableRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthDisableRequest {}

impl AuthDisableRequest {
    pub fn new() -> AuthDisableRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthDisableRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthDisableRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthDisableRequest,
        };
        unsafe {
            instance.get(AuthDisableRequest::new)
        }
    }
}

impl ::protobuf::Message for AuthDisableRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for AuthDisableRequest {
    fn new() -> AuthDisableRequest {
        AuthDisableRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthDisableRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AuthDisableRequest>(
                    "AuthDisableRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthDisableRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthDisableRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthDisableRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthenticateRequest {
    // message fields
    pub name: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthenticateRequest {}

impl AuthenticateRequest {
    pub fn new() -> AuthenticateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthenticateRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthenticateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthenticateRequest,
        };
        unsafe {
            instance.get(AuthenticateRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::string::String {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }
}

impl ::protobuf::Message for AuthenticateRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
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

impl ::protobuf::MessageStatic for AuthenticateRequest {
    fn new() -> AuthenticateRequest {
        AuthenticateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthenticateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AuthenticateRequest::get_name_for_reflect,
                    AuthenticateRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "password",
                    AuthenticateRequest::get_password_for_reflect,
                    AuthenticateRequest::mut_password_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthenticateRequest>(
                    "AuthenticateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthenticateRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_password();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthenticateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthenticateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserAddRequest {
    // message fields
    pub name: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserAddRequest {}

impl AuthUserAddRequest {
    pub fn new() -> AuthUserAddRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserAddRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserAddRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserAddRequest,
        };
        unsafe {
            instance.get(AuthUserAddRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::string::String {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }
}

impl ::protobuf::Message for AuthUserAddRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
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

impl ::protobuf::MessageStatic for AuthUserAddRequest {
    fn new() -> AuthUserAddRequest {
        AuthUserAddRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserAddRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AuthUserAddRequest::get_name_for_reflect,
                    AuthUserAddRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "password",
                    AuthUserAddRequest::get_password_for_reflect,
                    AuthUserAddRequest::mut_password_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserAddRequest>(
                    "AuthUserAddRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserAddRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_password();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserAddRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserAddRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserGetRequest {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserGetRequest {}

impl AuthUserGetRequest {
    pub fn new() -> AuthUserGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserGetRequest,
        };
        unsafe {
            instance.get(AuthUserGetRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for AuthUserGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for AuthUserGetRequest {
    fn new() -> AuthUserGetRequest {
        AuthUserGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AuthUserGetRequest::get_name_for_reflect,
                    AuthUserGetRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserGetRequest>(
                    "AuthUserGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserGetRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserDeleteRequest {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserDeleteRequest {}

impl AuthUserDeleteRequest {
    pub fn new() -> AuthUserDeleteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserDeleteRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserDeleteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserDeleteRequest,
        };
        unsafe {
            instance.get(AuthUserDeleteRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for AuthUserDeleteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for AuthUserDeleteRequest {
    fn new() -> AuthUserDeleteRequest {
        AuthUserDeleteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserDeleteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AuthUserDeleteRequest::get_name_for_reflect,
                    AuthUserDeleteRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserDeleteRequest>(
                    "AuthUserDeleteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserDeleteRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserDeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserDeleteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserChangePasswordRequest {
    // message fields
    pub name: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserChangePasswordRequest {}

impl AuthUserChangePasswordRequest {
    pub fn new() -> AuthUserChangePasswordRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserChangePasswordRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserChangePasswordRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserChangePasswordRequest,
        };
        unsafe {
            instance.get(AuthUserChangePasswordRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::string::String {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }
}

impl ::protobuf::Message for AuthUserChangePasswordRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
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

impl ::protobuf::MessageStatic for AuthUserChangePasswordRequest {
    fn new() -> AuthUserChangePasswordRequest {
        AuthUserChangePasswordRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserChangePasswordRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AuthUserChangePasswordRequest::get_name_for_reflect,
                    AuthUserChangePasswordRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "password",
                    AuthUserChangePasswordRequest::get_password_for_reflect,
                    AuthUserChangePasswordRequest::mut_password_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserChangePasswordRequest>(
                    "AuthUserChangePasswordRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserChangePasswordRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_password();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserChangePasswordRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserChangePasswordRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserGrantRoleRequest {
    // message fields
    pub user: ::std::string::String,
    pub role: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserGrantRoleRequest {}

impl AuthUserGrantRoleRequest {
    pub fn new() -> AuthUserGrantRoleRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserGrantRoleRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserGrantRoleRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserGrantRoleRequest,
        };
        unsafe {
            instance.get(AuthUserGrantRoleRequest::new)
        }
    }

    // string user = 1;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: ::std::string::String) {
        self.user = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user(&mut self) -> &mut ::std::string::String {
        &mut self.user
    }

    // Take field
    pub fn take_user(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user, ::std::string::String::new())
    }

    pub fn get_user(&self) -> &str {
        &self.user
    }

    fn get_user_for_reflect(&self) -> &::std::string::String {
        &self.user
    }

    fn mut_user_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.user
    }

    // string role = 2;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // Take field
    pub fn take_role(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.role, ::std::string::String::new())
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    fn get_role_for_reflect(&self) -> &::std::string::String {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }
}

impl ::protobuf::Message for AuthUserGrantRoleRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.role)?;
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
        if !self.user.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user);
        }
        if !self.role.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.role);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.user.is_empty() {
            os.write_string(1, &self.user)?;
        }
        if !self.role.is_empty() {
            os.write_string(2, &self.role)?;
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

impl ::protobuf::MessageStatic for AuthUserGrantRoleRequest {
    fn new() -> AuthUserGrantRoleRequest {
        AuthUserGrantRoleRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserGrantRoleRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "user",
                    AuthUserGrantRoleRequest::get_user_for_reflect,
                    AuthUserGrantRoleRequest::mut_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "role",
                    AuthUserGrantRoleRequest::get_role_for_reflect,
                    AuthUserGrantRoleRequest::mut_role_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserGrantRoleRequest>(
                    "AuthUserGrantRoleRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserGrantRoleRequest {
    fn clear(&mut self) {
        self.clear_user();
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserGrantRoleRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserGrantRoleRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserRevokeRoleRequest {
    // message fields
    pub name: ::std::string::String,
    pub role: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserRevokeRoleRequest {}

impl AuthUserRevokeRoleRequest {
    pub fn new() -> AuthUserRevokeRoleRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserRevokeRoleRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserRevokeRoleRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserRevokeRoleRequest,
        };
        unsafe {
            instance.get(AuthUserRevokeRoleRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string role = 2;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // Take field
    pub fn take_role(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.role, ::std::string::String::new())
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    fn get_role_for_reflect(&self) -> &::std::string::String {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }
}

impl ::protobuf::Message for AuthUserRevokeRoleRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.role)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.role.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.role);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.role.is_empty() {
            os.write_string(2, &self.role)?;
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

impl ::protobuf::MessageStatic for AuthUserRevokeRoleRequest {
    fn new() -> AuthUserRevokeRoleRequest {
        AuthUserRevokeRoleRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserRevokeRoleRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AuthUserRevokeRoleRequest::get_name_for_reflect,
                    AuthUserRevokeRoleRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "role",
                    AuthUserRevokeRoleRequest::get_role_for_reflect,
                    AuthUserRevokeRoleRequest::mut_role_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserRevokeRoleRequest>(
                    "AuthUserRevokeRoleRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserRevokeRoleRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserRevokeRoleRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserRevokeRoleRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleAddRequest {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleAddRequest {}

impl AuthRoleAddRequest {
    pub fn new() -> AuthRoleAddRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleAddRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleAddRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleAddRequest,
        };
        unsafe {
            instance.get(AuthRoleAddRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for AuthRoleAddRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for AuthRoleAddRequest {
    fn new() -> AuthRoleAddRequest {
        AuthRoleAddRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleAddRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AuthRoleAddRequest::get_name_for_reflect,
                    AuthRoleAddRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleAddRequest>(
                    "AuthRoleAddRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleAddRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleAddRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleAddRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleGetRequest {
    // message fields
    pub role: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleGetRequest {}

impl AuthRoleGetRequest {
    pub fn new() -> AuthRoleGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleGetRequest,
        };
        unsafe {
            instance.get(AuthRoleGetRequest::new)
        }
    }

    // string role = 1;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // Take field
    pub fn take_role(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.role, ::std::string::String::new())
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    fn get_role_for_reflect(&self) -> &::std::string::String {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }
}

impl ::protobuf::Message for AuthRoleGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.role)?;
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
        if !self.role.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.role);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.role.is_empty() {
            os.write_string(1, &self.role)?;
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

impl ::protobuf::MessageStatic for AuthRoleGetRequest {
    fn new() -> AuthRoleGetRequest {
        AuthRoleGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "role",
                    AuthRoleGetRequest::get_role_for_reflect,
                    AuthRoleGetRequest::mut_role_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleGetRequest>(
                    "AuthRoleGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleGetRequest {
    fn clear(&mut self) {
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserListRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserListRequest {}

impl AuthUserListRequest {
    pub fn new() -> AuthUserListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserListRequest,
        };
        unsafe {
            instance.get(AuthUserListRequest::new)
        }
    }
}

impl ::protobuf::Message for AuthUserListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for AuthUserListRequest {
    fn new() -> AuthUserListRequest {
        AuthUserListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserListRequest>(
                    "AuthUserListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserListRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleListRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleListRequest {}

impl AuthRoleListRequest {
    pub fn new() -> AuthRoleListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleListRequest,
        };
        unsafe {
            instance.get(AuthRoleListRequest::new)
        }
    }
}

impl ::protobuf::Message for AuthRoleListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for AuthRoleListRequest {
    fn new() -> AuthRoleListRequest {
        AuthRoleListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleListRequest>(
                    "AuthRoleListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleListRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleDeleteRequest {
    // message fields
    pub role: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleDeleteRequest {}

impl AuthRoleDeleteRequest {
    pub fn new() -> AuthRoleDeleteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleDeleteRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleDeleteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleDeleteRequest,
        };
        unsafe {
            instance.get(AuthRoleDeleteRequest::new)
        }
    }

    // string role = 1;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // Take field
    pub fn take_role(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.role, ::std::string::String::new())
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    fn get_role_for_reflect(&self) -> &::std::string::String {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }
}

impl ::protobuf::Message for AuthRoleDeleteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.role)?;
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
        if !self.role.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.role);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.role.is_empty() {
            os.write_string(1, &self.role)?;
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

impl ::protobuf::MessageStatic for AuthRoleDeleteRequest {
    fn new() -> AuthRoleDeleteRequest {
        AuthRoleDeleteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleDeleteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "role",
                    AuthRoleDeleteRequest::get_role_for_reflect,
                    AuthRoleDeleteRequest::mut_role_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleDeleteRequest>(
                    "AuthRoleDeleteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleDeleteRequest {
    fn clear(&mut self) {
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleDeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleDeleteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleGrantPermissionRequest {
    // message fields
    pub name: ::std::string::String,
    pub perm: ::protobuf::SingularPtrField<super::auth::Permission>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleGrantPermissionRequest {}

impl AuthRoleGrantPermissionRequest {
    pub fn new() -> AuthRoleGrantPermissionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleGrantPermissionRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleGrantPermissionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleGrantPermissionRequest,
        };
        unsafe {
            instance.get(AuthRoleGrantPermissionRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .authpb.Permission perm = 2;

    pub fn clear_perm(&mut self) {
        self.perm.clear();
    }

    pub fn has_perm(&self) -> bool {
        self.perm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perm(&mut self, v: super::auth::Permission) {
        self.perm = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_perm(&mut self) -> &mut super::auth::Permission {
        if self.perm.is_none() {
            self.perm.set_default();
        }
        self.perm.as_mut().unwrap()
    }

    // Take field
    pub fn take_perm(&mut self) -> super::auth::Permission {
        self.perm.take().unwrap_or_else(|| super::auth::Permission::new())
    }

    pub fn get_perm(&self) -> &super::auth::Permission {
        self.perm.as_ref().unwrap_or_else(|| super::auth::Permission::default_instance())
    }

    fn get_perm_for_reflect(&self) -> &::protobuf::SingularPtrField<super::auth::Permission> {
        &self.perm
    }

    fn mut_perm_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::auth::Permission> {
        &mut self.perm
    }
}

impl ::protobuf::Message for AuthRoleGrantPermissionRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.perm {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.perm)?;
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
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.perm.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.perm.as_ref() {
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

impl ::protobuf::MessageStatic for AuthRoleGrantPermissionRequest {
    fn new() -> AuthRoleGrantPermissionRequest {
        AuthRoleGrantPermissionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleGrantPermissionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AuthRoleGrantPermissionRequest::get_name_for_reflect,
                    AuthRoleGrantPermissionRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::auth::Permission>>(
                    "perm",
                    AuthRoleGrantPermissionRequest::get_perm_for_reflect,
                    AuthRoleGrantPermissionRequest::mut_perm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleGrantPermissionRequest>(
                    "AuthRoleGrantPermissionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleGrantPermissionRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_perm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleGrantPermissionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleGrantPermissionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleRevokePermissionRequest {
    // message fields
    pub role: ::std::string::String,
    pub key: ::std::string::String,
    pub range_end: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleRevokePermissionRequest {}

impl AuthRoleRevokePermissionRequest {
    pub fn new() -> AuthRoleRevokePermissionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleRevokePermissionRequest {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleRevokePermissionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleRevokePermissionRequest,
        };
        unsafe {
            instance.get(AuthRoleRevokePermissionRequest::new)
        }
    }

    // string role = 1;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // Take field
    pub fn take_role(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.role, ::std::string::String::new())
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    fn get_role_for_reflect(&self) -> &::std::string::String {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.role
    }

    // string key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // string range_end = 3;

    pub fn clear_range_end(&mut self) {
        self.range_end.clear();
    }

    // Param is passed by value, moved
    pub fn set_range_end(&mut self, v: ::std::string::String) {
        self.range_end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_end(&mut self) -> &mut ::std::string::String {
        &mut self.range_end
    }

    // Take field
    pub fn take_range_end(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.range_end, ::std::string::String::new())
    }

    pub fn get_range_end(&self) -> &str {
        &self.range_end
    }

    fn get_range_end_for_reflect(&self) -> &::std::string::String {
        &self.range_end
    }

    fn mut_range_end_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.range_end
    }
}

impl ::protobuf::Message for AuthRoleRevokePermissionRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.role)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.range_end)?;
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
        if !self.role.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.role);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.key);
        }
        if !self.range_end.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.range_end);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.role.is_empty() {
            os.write_string(1, &self.role)?;
        }
        if !self.key.is_empty() {
            os.write_string(2, &self.key)?;
        }
        if !self.range_end.is_empty() {
            os.write_string(3, &self.range_end)?;
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

impl ::protobuf::MessageStatic for AuthRoleRevokePermissionRequest {
    fn new() -> AuthRoleRevokePermissionRequest {
        AuthRoleRevokePermissionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleRevokePermissionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "role",
                    AuthRoleRevokePermissionRequest::get_role_for_reflect,
                    AuthRoleRevokePermissionRequest::mut_role_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    AuthRoleRevokePermissionRequest::get_key_for_reflect,
                    AuthRoleRevokePermissionRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "range_end",
                    AuthRoleRevokePermissionRequest::get_range_end_for_reflect,
                    AuthRoleRevokePermissionRequest::mut_range_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleRevokePermissionRequest>(
                    "AuthRoleRevokePermissionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleRevokePermissionRequest {
    fn clear(&mut self) {
        self.clear_role();
        self.clear_key();
        self.clear_range_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleRevokePermissionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleRevokePermissionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthEnableResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthEnableResponse {}

impl AuthEnableResponse {
    pub fn new() -> AuthEnableResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthEnableResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthEnableResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthEnableResponse,
        };
        unsafe {
            instance.get(AuthEnableResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthEnableResponse {
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

impl ::protobuf::MessageStatic for AuthEnableResponse {
    fn new() -> AuthEnableResponse {
        AuthEnableResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthEnableResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthEnableResponse::get_header_for_reflect,
                    AuthEnableResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthEnableResponse>(
                    "AuthEnableResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthEnableResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthEnableResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthEnableResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthDisableResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthDisableResponse {}

impl AuthDisableResponse {
    pub fn new() -> AuthDisableResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthDisableResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthDisableResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthDisableResponse,
        };
        unsafe {
            instance.get(AuthDisableResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthDisableResponse {
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

impl ::protobuf::MessageStatic for AuthDisableResponse {
    fn new() -> AuthDisableResponse {
        AuthDisableResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthDisableResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthDisableResponse::get_header_for_reflect,
                    AuthDisableResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthDisableResponse>(
                    "AuthDisableResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthDisableResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthDisableResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthDisableResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthenticateResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub token: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthenticateResponse {}

impl AuthenticateResponse {
    pub fn new() -> AuthenticateResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthenticateResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthenticateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthenticateResponse,
        };
        unsafe {
            instance.get(AuthenticateResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // string token = 2;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.token, ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    fn get_token_for_reflect(&self) -> &::std::string::String {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.token
    }
}

impl ::protobuf::Message for AuthenticateResponse {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.token)?;
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
        if !self.token.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.token);
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
        if !self.token.is_empty() {
            os.write_string(2, &self.token)?;
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

impl ::protobuf::MessageStatic for AuthenticateResponse {
    fn new() -> AuthenticateResponse {
        AuthenticateResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthenticateResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthenticateResponse::get_header_for_reflect,
                    AuthenticateResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    AuthenticateResponse::get_token_for_reflect,
                    AuthenticateResponse::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthenticateResponse>(
                    "AuthenticateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthenticateResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthenticateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthenticateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserAddResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserAddResponse {}

impl AuthUserAddResponse {
    pub fn new() -> AuthUserAddResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserAddResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserAddResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserAddResponse,
        };
        unsafe {
            instance.get(AuthUserAddResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthUserAddResponse {
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

impl ::protobuf::MessageStatic for AuthUserAddResponse {
    fn new() -> AuthUserAddResponse {
        AuthUserAddResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserAddResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthUserAddResponse::get_header_for_reflect,
                    AuthUserAddResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserAddResponse>(
                    "AuthUserAddResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserAddResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserAddResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserAddResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserGetResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserGetResponse {}

impl AuthUserGetResponse {
    pub fn new() -> AuthUserGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserGetResponse,
        };
        unsafe {
            instance.get(AuthUserGetResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated string roles = 2;

    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }

    // Take field
    pub fn take_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.roles, ::protobuf::RepeatedField::new())
    }

    pub fn get_roles(&self) -> &[::std::string::String] {
        &self.roles
    }

    fn get_roles_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.roles
    }

    fn mut_roles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }
}

impl ::protobuf::Message for AuthUserGetResponse {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.roles)?;
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
        for value in &self.roles {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
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
        for v in &self.roles {
            os.write_string(2, &v)?;
        };
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

impl ::protobuf::MessageStatic for AuthUserGetResponse {
    fn new() -> AuthUserGetResponse {
        AuthUserGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthUserGetResponse::get_header_for_reflect,
                    AuthUserGetResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "roles",
                    AuthUserGetResponse::get_roles_for_reflect,
                    AuthUserGetResponse::mut_roles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserGetResponse>(
                    "AuthUserGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserGetResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserDeleteResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserDeleteResponse {}

impl AuthUserDeleteResponse {
    pub fn new() -> AuthUserDeleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserDeleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserDeleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserDeleteResponse,
        };
        unsafe {
            instance.get(AuthUserDeleteResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthUserDeleteResponse {
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

impl ::protobuf::MessageStatic for AuthUserDeleteResponse {
    fn new() -> AuthUserDeleteResponse {
        AuthUserDeleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserDeleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthUserDeleteResponse::get_header_for_reflect,
                    AuthUserDeleteResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserDeleteResponse>(
                    "AuthUserDeleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserDeleteResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserDeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserDeleteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserChangePasswordResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserChangePasswordResponse {}

impl AuthUserChangePasswordResponse {
    pub fn new() -> AuthUserChangePasswordResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserChangePasswordResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserChangePasswordResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserChangePasswordResponse,
        };
        unsafe {
            instance.get(AuthUserChangePasswordResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthUserChangePasswordResponse {
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

impl ::protobuf::MessageStatic for AuthUserChangePasswordResponse {
    fn new() -> AuthUserChangePasswordResponse {
        AuthUserChangePasswordResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserChangePasswordResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthUserChangePasswordResponse::get_header_for_reflect,
                    AuthUserChangePasswordResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserChangePasswordResponse>(
                    "AuthUserChangePasswordResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserChangePasswordResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserChangePasswordResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserChangePasswordResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserGrantRoleResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserGrantRoleResponse {}

impl AuthUserGrantRoleResponse {
    pub fn new() -> AuthUserGrantRoleResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserGrantRoleResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserGrantRoleResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserGrantRoleResponse,
        };
        unsafe {
            instance.get(AuthUserGrantRoleResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthUserGrantRoleResponse {
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

impl ::protobuf::MessageStatic for AuthUserGrantRoleResponse {
    fn new() -> AuthUserGrantRoleResponse {
        AuthUserGrantRoleResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserGrantRoleResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthUserGrantRoleResponse::get_header_for_reflect,
                    AuthUserGrantRoleResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserGrantRoleResponse>(
                    "AuthUserGrantRoleResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserGrantRoleResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserGrantRoleResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserGrantRoleResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserRevokeRoleResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserRevokeRoleResponse {}

impl AuthUserRevokeRoleResponse {
    pub fn new() -> AuthUserRevokeRoleResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserRevokeRoleResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserRevokeRoleResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserRevokeRoleResponse,
        };
        unsafe {
            instance.get(AuthUserRevokeRoleResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthUserRevokeRoleResponse {
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

impl ::protobuf::MessageStatic for AuthUserRevokeRoleResponse {
    fn new() -> AuthUserRevokeRoleResponse {
        AuthUserRevokeRoleResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserRevokeRoleResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthUserRevokeRoleResponse::get_header_for_reflect,
                    AuthUserRevokeRoleResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserRevokeRoleResponse>(
                    "AuthUserRevokeRoleResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserRevokeRoleResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserRevokeRoleResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserRevokeRoleResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleAddResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleAddResponse {}

impl AuthRoleAddResponse {
    pub fn new() -> AuthRoleAddResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleAddResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleAddResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleAddResponse,
        };
        unsafe {
            instance.get(AuthRoleAddResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthRoleAddResponse {
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

impl ::protobuf::MessageStatic for AuthRoleAddResponse {
    fn new() -> AuthRoleAddResponse {
        AuthRoleAddResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleAddResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthRoleAddResponse::get_header_for_reflect,
                    AuthRoleAddResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleAddResponse>(
                    "AuthRoleAddResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleAddResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleAddResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleAddResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleGetResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub perm: ::protobuf::RepeatedField<super::auth::Permission>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleGetResponse {}

impl AuthRoleGetResponse {
    pub fn new() -> AuthRoleGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleGetResponse,
        };
        unsafe {
            instance.get(AuthRoleGetResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated .authpb.Permission perm = 2;

    pub fn clear_perm(&mut self) {
        self.perm.clear();
    }

    // Param is passed by value, moved
    pub fn set_perm(&mut self, v: ::protobuf::RepeatedField<super::auth::Permission>) {
        self.perm = v;
    }

    // Mutable pointer to the field.
    pub fn mut_perm(&mut self) -> &mut ::protobuf::RepeatedField<super::auth::Permission> {
        &mut self.perm
    }

    // Take field
    pub fn take_perm(&mut self) -> ::protobuf::RepeatedField<super::auth::Permission> {
        ::std::mem::replace(&mut self.perm, ::protobuf::RepeatedField::new())
    }

    pub fn get_perm(&self) -> &[super::auth::Permission] {
        &self.perm
    }

    fn get_perm_for_reflect(&self) -> &::protobuf::RepeatedField<super::auth::Permission> {
        &self.perm
    }

    fn mut_perm_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::auth::Permission> {
        &mut self.perm
    }
}

impl ::protobuf::Message for AuthRoleGetResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.perm {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.perm)?;
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
        for value in &self.perm {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.perm {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for AuthRoleGetResponse {
    fn new() -> AuthRoleGetResponse {
        AuthRoleGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthRoleGetResponse::get_header_for_reflect,
                    AuthRoleGetResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::auth::Permission>>(
                    "perm",
                    AuthRoleGetResponse::get_perm_for_reflect,
                    AuthRoleGetResponse::mut_perm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleGetResponse>(
                    "AuthRoleGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleGetResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_perm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleListResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleListResponse {}

impl AuthRoleListResponse {
    pub fn new() -> AuthRoleListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleListResponse,
        };
        unsafe {
            instance.get(AuthRoleListResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated string roles = 2;

    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }

    // Take field
    pub fn take_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.roles, ::protobuf::RepeatedField::new())
    }

    pub fn get_roles(&self) -> &[::std::string::String] {
        &self.roles
    }

    fn get_roles_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.roles
    }

    fn mut_roles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }
}

impl ::protobuf::Message for AuthRoleListResponse {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.roles)?;
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
        for value in &self.roles {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
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
        for v in &self.roles {
            os.write_string(2, &v)?;
        };
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

impl ::protobuf::MessageStatic for AuthRoleListResponse {
    fn new() -> AuthRoleListResponse {
        AuthRoleListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthRoleListResponse::get_header_for_reflect,
                    AuthRoleListResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "roles",
                    AuthRoleListResponse::get_roles_for_reflect,
                    AuthRoleListResponse::mut_roles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleListResponse>(
                    "AuthRoleListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleListResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthUserListResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    pub users: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthUserListResponse {}

impl AuthUserListResponse {
    pub fn new() -> AuthUserListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthUserListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthUserListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthUserListResponse,
        };
        unsafe {
            instance.get(AuthUserListResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }

    // repeated string users = 2;

    pub fn clear_users(&mut self) {
        self.users.clear();
    }

    // Param is passed by value, moved
    pub fn set_users(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.users = v;
    }

    // Mutable pointer to the field.
    pub fn mut_users(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.users
    }

    // Take field
    pub fn take_users(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.users, ::protobuf::RepeatedField::new())
    }

    pub fn get_users(&self) -> &[::std::string::String] {
        &self.users
    }

    fn get_users_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.users
    }

    fn mut_users_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.users
    }
}

impl ::protobuf::Message for AuthUserListResponse {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.users)?;
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
        for value in &self.users {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
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
        for v in &self.users {
            os.write_string(2, &v)?;
        };
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

impl ::protobuf::MessageStatic for AuthUserListResponse {
    fn new() -> AuthUserListResponse {
        AuthUserListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthUserListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthUserListResponse::get_header_for_reflect,
                    AuthUserListResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "users",
                    AuthUserListResponse::get_users_for_reflect,
                    AuthUserListResponse::mut_users_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthUserListResponse>(
                    "AuthUserListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthUserListResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_users();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthUserListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthUserListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleDeleteResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleDeleteResponse {}

impl AuthRoleDeleteResponse {
    pub fn new() -> AuthRoleDeleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleDeleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleDeleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleDeleteResponse,
        };
        unsafe {
            instance.get(AuthRoleDeleteResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthRoleDeleteResponse {
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

impl ::protobuf::MessageStatic for AuthRoleDeleteResponse {
    fn new() -> AuthRoleDeleteResponse {
        AuthRoleDeleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleDeleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthRoleDeleteResponse::get_header_for_reflect,
                    AuthRoleDeleteResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleDeleteResponse>(
                    "AuthRoleDeleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleDeleteResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleDeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleDeleteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleGrantPermissionResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleGrantPermissionResponse {}

impl AuthRoleGrantPermissionResponse {
    pub fn new() -> AuthRoleGrantPermissionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleGrantPermissionResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleGrantPermissionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleGrantPermissionResponse,
        };
        unsafe {
            instance.get(AuthRoleGrantPermissionResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthRoleGrantPermissionResponse {
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

impl ::protobuf::MessageStatic for AuthRoleGrantPermissionResponse {
    fn new() -> AuthRoleGrantPermissionResponse {
        AuthRoleGrantPermissionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleGrantPermissionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthRoleGrantPermissionResponse::get_header_for_reflect,
                    AuthRoleGrantPermissionResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleGrantPermissionResponse>(
                    "AuthRoleGrantPermissionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleGrantPermissionResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleGrantPermissionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleGrantPermissionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AuthRoleRevokePermissionResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthRoleRevokePermissionResponse {}

impl AuthRoleRevokePermissionResponse {
    pub fn new() -> AuthRoleRevokePermissionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthRoleRevokePermissionResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthRoleRevokePermissionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthRoleRevokePermissionResponse,
        };
        unsafe {
            instance.get(AuthRoleRevokePermissionResponse::new)
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResponseHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for AuthRoleRevokePermissionResponse {
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

impl ::protobuf::MessageStatic for AuthRoleRevokePermissionResponse {
    fn new() -> AuthRoleRevokePermissionResponse {
        AuthRoleRevokePermissionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthRoleRevokePermissionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseHeader>>(
                    "header",
                    AuthRoleRevokePermissionResponse::get_header_for_reflect,
                    AuthRoleRevokePermissionResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthRoleRevokePermissionResponse>(
                    "AuthRoleRevokePermissionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthRoleRevokePermissionResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRoleRevokePermissionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRoleRevokePermissionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AlarmType {
    NONE = 0,
    NOSPACE = 1,
    CORRUPT = 2,
}

impl ::protobuf::ProtobufEnum for AlarmType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AlarmType> {
        match value {
            0 => ::std::option::Option::Some(AlarmType::NONE),
            1 => ::std::option::Option::Some(AlarmType::NOSPACE),
            2 => ::std::option::Option::Some(AlarmType::CORRUPT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AlarmType] = &[
            AlarmType::NONE,
            AlarmType::NOSPACE,
            AlarmType::CORRUPT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AlarmType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AlarmType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AlarmType {
}

impl ::std::default::Default for AlarmType {
    fn default() -> Self {
        AlarmType::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for AlarmType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\trpc.proto\x12\x0cetcdserverpb\x1a\x14gogoproto/gogo.proto\x1a\x19etc\
    d/mvcc/mvccpb/kv.proto\x1a\x1betcd/auth/authpb/auth.proto\x1a\x1cgoogle/\
    api/annotations.proto\"\x85\x01\n\x0eResponseHeader\x12\x1d\n\ncluster_i\
    d\x18\x01\x20\x01(\x04R\tclusterId\x12\x1b\n\tmember_id\x18\x02\x20\x01(\
    \x04R\x08memberId\x12\x1a\n\x08revision\x18\x03\x20\x01(\x03R\x08revisio\
    n\x12\x1b\n\traft_term\x18\x04\x20\x01(\x04R\x08raftTerm\"\x84\x05\n\x0c\
    RangeRequest\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\x12\x1b\n\tra\
    nge_end\x18\x02\x20\x01(\x0cR\x08rangeEnd\x12\x14\n\x05limit\x18\x03\x20\
    \x01(\x03R\x05limit\x12\x1a\n\x08revision\x18\x04\x20\x01(\x03R\x08revis\
    ion\x12C\n\nsort_order\x18\x05\x20\x01(\x0e2$.etcdserverpb.RangeRequest.\
    SortOrderR\tsortOrder\x12F\n\x0bsort_target\x18\x06\x20\x01(\x0e2%.etcds\
    erverpb.RangeRequest.SortTargetR\nsortTarget\x12\"\n\x0cserializable\x18\
    \x07\x20\x01(\x08R\x0cserializable\x12\x1b\n\tkeys_only\x18\x08\x20\x01(\
    \x08R\x08keysOnly\x12\x1d\n\ncount_only\x18\t\x20\x01(\x08R\tcountOnly\
    \x12(\n\x10min_mod_revision\x18\n\x20\x01(\x03R\x0eminModRevision\x12(\n\
    \x10max_mod_revision\x18\x0b\x20\x01(\x03R\x0emaxModRevision\x12.\n\x13m\
    in_create_revision\x18\x0c\x20\x01(\x03R\x11minCreateRevision\x12.\n\x13\
    max_create_revision\x18\r\x20\x01(\x03R\x11maxCreateRevision\".\n\tSortO\
    rder\x12\x08\n\x04NONE\x10\0\x12\n\n\x06ASCEND\x10\x01\x12\x0b\n\x07DESC\
    END\x10\x02\"B\n\nSortTarget\x12\x07\n\x03KEY\x10\0\x12\x0b\n\x07VERSION\
    \x10\x01\x12\n\n\x06CREATE\x10\x02\x12\x07\n\x03MOD\x10\x03\x12\t\n\x05V\
    ALUE\x10\x04\"\x93\x01\n\rRangeResponse\x124\n\x06header\x18\x01\x20\x01\
    (\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12\"\n\x03kvs\x18\x02\
    \x20\x03(\x0b2\x10.mvccpb.KeyValueR\x03kvs\x12\x12\n\x04more\x18\x03\x20\
    \x01(\x08R\x04more\x12\x14\n\x05count\x18\x04\x20\x01(\x03R\x05count\"\
    \xa9\x01\n\nPutRequest\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value\x12\x14\n\x05lease\x18\
    \x03\x20\x01(\x03R\x05lease\x12\x17\n\x07prev_kv\x18\x04\x20\x01(\x08R\
    \x06prevKv\x12!\n\x0cignore_value\x18\x05\x20\x01(\x08R\x0bignoreValue\
    \x12!\n\x0cignore_lease\x18\x06\x20\x01(\x08R\x0bignoreLease\"n\n\x0bPut\
    Response\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.Respons\
    eHeaderR\x06header\x12)\n\x07prev_kv\x18\x02\x20\x01(\x0b2\x10.mvccpb.Ke\
    yValueR\x06prevKv\"\\\n\x12DeleteRangeRequest\x12\x10\n\x03key\x18\x01\
    \x20\x01(\x0cR\x03key\x12\x1b\n\trange_end\x18\x02\x20\x01(\x0cR\x08rang\
    eEnd\x12\x17\n\x07prev_kv\x18\x03\x20\x01(\x08R\x06prevKv\"\x92\x01\n\
    \x13DeleteRangeResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcds\
    erverpb.ResponseHeaderR\x06header\x12\x18\n\x07deleted\x18\x02\x20\x01(\
    \x03R\x07deleted\x12+\n\x08prev_kvs\x18\x03\x20\x03(\x0b2\x10.mvccpb.Key\
    ValueR\x07prevKvs\"\xa9\x02\n\tRequestOp\x12A\n\rrequest_range\x18\x01\
    \x20\x01(\x0b2\x1a.etcdserverpb.RangeRequestH\0R\x0crequestRange\x12;\n\
    \x0brequest_put\x18\x02\x20\x01(\x0b2\x18.etcdserverpb.PutRequestH\0R\nr\
    equestPut\x12T\n\x14request_delete_range\x18\x03\x20\x01(\x0b2\x20.etcds\
    erverpb.DeleteRangeRequestH\0R\x12requestDeleteRange\x12;\n\x0brequest_t\
    xn\x18\x04\x20\x01(\x0b2\x18.etcdserverpb.TxnRequestH\0R\nrequestTxnB\t\
    \n\x07request\"\xb7\x02\n\nResponseOp\x12D\n\x0eresponse_range\x18\x01\
    \x20\x01(\x0b2\x1b.etcdserverpb.RangeResponseH\0R\rresponseRange\x12>\n\
    \x0cresponse_put\x18\x02\x20\x01(\x0b2\x19.etcdserverpb.PutResponseH\0R\
    \x0bresponsePut\x12W\n\x15response_delete_range\x18\x03\x20\x01(\x0b2!.e\
    tcdserverpb.DeleteRangeResponseH\0R\x13responseDeleteRange\x12>\n\x0cres\
    ponse_txn\x18\x04\x20\x01(\x0b2\x19.etcdserverpb.TxnResponseH\0R\x0bresp\
    onseTxnB\n\n\x08response\"\xe9\x03\n\x07Compare\x12;\n\x06result\x18\x01\
    \x20\x01(\x0e2#.etcdserverpb.Compare.CompareResultR\x06result\x12;\n\x06\
    target\x18\x02\x20\x01(\x0e2#.etcdserverpb.Compare.CompareTargetR\x06tar\
    get\x12\x10\n\x03key\x18\x03\x20\x01(\x0cR\x03key\x12\x1a\n\x07version\
    \x18\x04\x20\x01(\x03H\0R\x07version\x12)\n\x0fcreate_revision\x18\x05\
    \x20\x01(\x03H\0R\x0ecreateRevision\x12#\n\x0cmod_revision\x18\x06\x20\
    \x01(\x03H\0R\x0bmodRevision\x12\x16\n\x05value\x18\x07\x20\x01(\x0cH\0R\
    \x05value\x12\x16\n\x05lease\x18\x08\x20\x01(\x03H\0R\x05lease\x12\x1b\n\
    \trange_end\x18@\x20\x01(\x0cR\x08rangeEnd\"@\n\rCompareResult\x12\t\n\
    \x05EQUAL\x10\0\x12\x0b\n\x07GREATER\x10\x01\x12\x08\n\x04LESS\x10\x02\
    \x12\r\n\tNOT_EQUAL\x10\x03\"G\n\rCompareTarget\x12\x0b\n\x07VERSION\x10\
    \0\x12\n\n\x06CREATE\x10\x01\x12\x07\n\x03MOD\x10\x02\x12\t\n\x05VALUE\
    \x10\x03\x12\t\n\x05LEASE\x10\x04B\x0e\n\x0ctarget_union\"\xa3\x01\n\nTx\
    nRequest\x12/\n\x07compare\x18\x01\x20\x03(\x0b2\x15.etcdserverpb.Compar\
    eR\x07compare\x121\n\x07success\x18\x02\x20\x03(\x0b2\x17.etcdserverpb.R\
    equestOpR\x07success\x121\n\x07failure\x18\x03\x20\x03(\x0b2\x17.etcdser\
    verpb.RequestOpR\x07failure\"\x99\x01\n\x0bTxnResponse\x124\n\x06header\
    \x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12\
    \x1c\n\tsucceeded\x18\x02\x20\x01(\x08R\tsucceeded\x126\n\tresponses\x18\
    \x03\x20\x03(\x0b2\x18.etcdserverpb.ResponseOpR\tresponses\"K\n\x11Compa\
    ctionRequest\x12\x1a\n\x08revision\x18\x01\x20\x01(\x03R\x08revision\x12\
    \x1a\n\x08physical\x18\x02\x20\x01(\x08R\x08physical\"J\n\x12CompactionR\
    esponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.Response\
    HeaderR\x06header\"\r\n\x0bHashRequest\"+\n\rHashKVRequest\x12\x1a\n\x08\
    revision\x18\x01\x20\x01(\x03R\x08revision\"\x85\x01\n\x0eHashKVResponse\
    \x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\
    \x06header\x12\x12\n\x04hash\x18\x02\x20\x01(\rR\x04hash\x12)\n\x10compa\
    ct_revision\x18\x03\x20\x01(\x03R\x0fcompactRevision\"X\n\x0cHashRespons\
    e\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeader\
    R\x06header\x12\x12\n\x04hash\x18\x02\x20\x01(\rR\x04hash\"\x11\n\x0fSna\
    pshotRequest\"\x85\x01\n\x10SnapshotResponse\x124\n\x06header\x18\x01\
    \x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12'\n\x0frema\
    ining_bytes\x18\x02\x20\x01(\x04R\x0eremainingBytes\x12\x12\n\x04blob\
    \x18\x03\x20\x01(\x0cR\x04blob\"\xb5\x01\n\x0cWatchRequest\x12I\n\x0ecre\
    ate_request\x18\x01\x20\x01(\x0b2\x20.etcdserverpb.WatchCreateRequestH\0\
    R\rcreateRequest\x12I\n\x0ecancel_request\x18\x02\x20\x01(\x0b2\x20.etcd\
    serverpb.WatchCancelRequestH\0R\rcancelRequestB\x0f\n\rrequest_union\"\
    \x9a\x02\n\x12WatchCreateRequest\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\
    \x03key\x12\x1b\n\trange_end\x18\x02\x20\x01(\x0cR\x08rangeEnd\x12%\n\
    \x0estart_revision\x18\x03\x20\x01(\x03R\rstartRevision\x12'\n\x0fprogre\
    ss_notify\x18\x04\x20\x01(\x08R\x0eprogressNotify\x12E\n\x07filters\x18\
    \x05\x20\x03(\x0e2+.etcdserverpb.WatchCreateRequest.FilterTypeR\x07filte\
    rs\x12\x17\n\x07prev_kv\x18\x06\x20\x01(\x08R\x06prevKv\"%\n\nFilterType\
    \x12\t\n\x05NOPUT\x10\0\x12\x0c\n\x08NODELETE\x10\x01\"/\n\x12WatchCance\
    lRequest\x12\x19\n\x08watch_id\x18\x01\x20\x01(\x03R\x07watchId\"\x8d\
    \x02\n\rWatchResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdser\
    verpb.ResponseHeaderR\x06header\x12\x19\n\x08watch_id\x18\x02\x20\x01(\
    \x03R\x07watchId\x12\x18\n\x07created\x18\x03\x20\x01(\x08R\x07created\
    \x12\x1a\n\x08canceled\x18\x04\x20\x01(\x08R\x08canceled\x12)\n\x10compa\
    ct_revision\x18\x05\x20\x01(\x03R\x0fcompactRevision\x12#\n\rcancel_reas\
    on\x18\x06\x20\x01(\tR\x0ccancelReason\x12%\n\x06events\x18\x0b\x20\x03(\
    \x0b2\r.mvccpb.EventR\x06events\"5\n\x11LeaseGrantRequest\x12\x10\n\x03T\
    TL\x18\x01\x20\x01(\x03R\x03TTL\x12\x0e\n\x02ID\x18\x02\x20\x01(\x03R\
    \x02ID\"\x82\x01\n\x12LeaseGrantResponse\x124\n\x06header\x18\x01\x20\
    \x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12\x0e\n\x02ID\
    \x18\x02\x20\x01(\x03R\x02ID\x12\x10\n\x03TTL\x18\x03\x20\x01(\x03R\x03T\
    TL\x12\x14\n\x05error\x18\x04\x20\x01(\tR\x05error\"$\n\x12LeaseRevokeRe\
    quest\x12\x0e\n\x02ID\x18\x01\x20\x01(\x03R\x02ID\"K\n\x13LeaseRevokeRes\
    ponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHe\
    aderR\x06header\"'\n\x15LeaseKeepAliveRequest\x12\x0e\n\x02ID\x18\x01\
    \x20\x01(\x03R\x02ID\"p\n\x16LeaseKeepAliveResponse\x124\n\x06header\x18\
    \x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12\x0e\n\
    \x02ID\x18\x02\x20\x01(\x03R\x02ID\x12\x10\n\x03TTL\x18\x03\x20\x01(\x03\
    R\x03TTL\"<\n\x16LeaseTimeToLiveRequest\x12\x0e\n\x02ID\x18\x01\x20\x01(\
    \x03R\x02ID\x12\x12\n\x04keys\x18\x02\x20\x01(\x08R\x04keys\"\xa5\x01\n\
    \x17LeaseTimeToLiveResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.e\
    tcdserverpb.ResponseHeaderR\x06header\x12\x0e\n\x02ID\x18\x02\x20\x01(\
    \x03R\x02ID\x12\x10\n\x03TTL\x18\x03\x20\x01(\x03R\x03TTL\x12\x1e\n\ngra\
    ntedTTL\x18\x04\x20\x01(\x03R\ngrantedTTL\x12\x12\n\x04keys\x18\x05\x20\
    \x03(\x0cR\x04keys\"\x14\n\x12LeaseLeasesRequest\"\x1d\n\x0bLeaseStatus\
    \x12\x0e\n\x02ID\x18\x01\x20\x01(\x03R\x02ID\"~\n\x13LeaseLeasesResponse\
    \x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\
    \x06header\x121\n\x06leases\x18\x02\x20\x03(\x0b2\x19.etcdserverpb.Lease\
    StatusR\x06leases\"h\n\x06Member\x12\x0e\n\x02ID\x18\x01\x20\x01(\x04R\
    \x02ID\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x1a\n\x08peerUR\
    Ls\x18\x03\x20\x03(\tR\x08peerURLs\x12\x1e\n\nclientURLs\x18\x04\x20\x03\
    (\tR\nclientURLs\".\n\x10MemberAddRequest\x12\x1a\n\x08peerURLs\x18\x01\
    \x20\x03(\tR\x08peerURLs\"\xa7\x01\n\x11MemberAddResponse\x124\n\x06head\
    er\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12,\
    \n\x06member\x18\x02\x20\x01(\x0b2\x14.etcdserverpb.MemberR\x06member\
    \x12.\n\x07members\x18\x03\x20\x03(\x0b2\x14.etcdserverpb.MemberR\x07mem\
    bers\"%\n\x13MemberRemoveRequest\x12\x0e\n\x02ID\x18\x01\x20\x01(\x04R\
    \x02ID\"|\n\x14MemberRemoveResponse\x124\n\x06header\x18\x01\x20\x01(\
    \x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12.\n\x07members\x18\
    \x02\x20\x03(\x0b2\x14.etcdserverpb.MemberR\x07members\"A\n\x13MemberUpd\
    ateRequest\x12\x0e\n\x02ID\x18\x01\x20\x01(\x04R\x02ID\x12\x1a\n\x08peer\
    URLs\x18\x02\x20\x03(\tR\x08peerURLs\"|\n\x14MemberUpdateResponse\x124\n\
    \x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06hea\
    der\x12.\n\x07members\x18\x02\x20\x03(\x0b2\x14.etcdserverpb.MemberR\x07\
    members\"\x13\n\x11MemberListRequest\"z\n\x12MemberListResponse\x124\n\
    \x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06hea\
    der\x12.\n\x07members\x18\x02\x20\x03(\x0b2\x14.etcdserverpb.MemberR\x07\
    members\"\x13\n\x11DefragmentRequest\"J\n\x12DefragmentResponse\x124\n\
    \x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06hea\
    der\"/\n\x11MoveLeaderRequest\x12\x1a\n\x08targetID\x18\x01\x20\x01(\x04\
    R\x08targetID\"J\n\x12MoveLeaderResponse\x124\n\x06header\x18\x01\x20\
    \x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\"\xcf\x01\n\x0cAla\
    rmRequest\x12>\n\x06action\x18\x01\x20\x01(\x0e2&.etcdserverpb.AlarmRequ\
    est.AlarmActionR\x06action\x12\x1a\n\x08memberID\x18\x02\x20\x01(\x04R\
    \x08memberID\x12-\n\x05alarm\x18\x03\x20\x01(\x0e2\x17.etcdserverpb.Alar\
    mTypeR\x05alarm\"4\n\x0bAlarmAction\x12\x07\n\x03GET\x10\0\x12\x0c\n\x08\
    ACTIVATE\x10\x01\x12\x0e\n\nDEACTIVATE\x10\x02\"X\n\x0bAlarmMember\x12\
    \x1a\n\x08memberID\x18\x01\x20\x01(\x04R\x08memberID\x12-\n\x05alarm\x18\
    \x02\x20\x01(\x0e2\x17.etcdserverpb.AlarmTypeR\x05alarm\"x\n\rAlarmRespo\
    nse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHead\
    erR\x06header\x121\n\x06alarms\x18\x02\x20\x03(\x0b2\x19.etcdserverpb.Al\
    armMemberR\x06alarms\"\x0f\n\rStatusRequest\"\xca\x01\n\x0eStatusRespons\
    e\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeader\
    R\x06header\x12\x18\n\x07version\x18\x02\x20\x01(\tR\x07version\x12\x16\
    \n\x06dbSize\x18\x03\x20\x01(\x03R\x06dbSize\x12\x16\n\x06leader\x18\x04\
    \x20\x01(\x04R\x06leader\x12\x1c\n\traftIndex\x18\x05\x20\x01(\x04R\traf\
    tIndex\x12\x1a\n\x08raftTerm\x18\x06\x20\x01(\x04R\x08raftTerm\"\x13\n\
    \x11AuthEnableRequest\"\x14\n\x12AuthDisableRequest\"E\n\x13Authenticate\
    Request\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x1a\n\x08passw\
    ord\x18\x02\x20\x01(\tR\x08password\"D\n\x12AuthUserAddRequest\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12\x1a\n\x08password\x18\x02\x20\
    \x01(\tR\x08password\"(\n\x12AuthUserGetRequest\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\"+\n\x15AuthUserDeleteRequest\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\"O\n\x1dAuthUserChangePasswordRequest\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x1a\n\x08password\x18\x02\
    \x20\x01(\tR\x08password\"B\n\x18AuthUserGrantRoleRequest\x12\x12\n\x04u\
    ser\x18\x01\x20\x01(\tR\x04user\x12\x12\n\x04role\x18\x02\x20\x01(\tR\
    \x04role\"C\n\x19AuthUserRevokeRoleRequest\x12\x12\n\x04name\x18\x01\x20\
    \x01(\tR\x04name\x12\x12\n\x04role\x18\x02\x20\x01(\tR\x04role\"(\n\x12A\
    uthRoleAddRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"(\n\x12\
    AuthRoleGetRequest\x12\x12\n\x04role\x18\x01\x20\x01(\tR\x04role\"\x15\n\
    \x13AuthUserListRequest\"\x15\n\x13AuthRoleListRequest\"+\n\x15AuthRoleD\
    eleteRequest\x12\x12\n\x04role\x18\x01\x20\x01(\tR\x04role\"\\\n\x1eAuth\
    RoleGrantPermissionRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12&\n\x04perm\x18\x02\x20\x01(\x0b2\x12.authpb.PermissionR\x04perm\"d\
    \n\x1fAuthRoleRevokePermissionRequest\x12\x12\n\x04role\x18\x01\x20\x01(\
    \tR\x04role\x12\x10\n\x03key\x18\x02\x20\x01(\tR\x03key\x12\x1b\n\trange\
    _end\x18\x03\x20\x01(\tR\x08rangeEnd\"J\n\x12AuthEnableResponse\x124\n\
    \x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06hea\
    der\"K\n\x13AuthDisableResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\
    \x1c.etcdserverpb.ResponseHeaderR\x06header\"b\n\x14AuthenticateResponse\
    \x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\
    \x06header\x12\x14\n\x05token\x18\x02\x20\x01(\tR\x05token\"K\n\x13AuthU\
    serAddResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.R\
    esponseHeaderR\x06header\"a\n\x13AuthUserGetResponse\x124\n\x06header\
    \x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12\
    \x14\n\x05roles\x18\x02\x20\x03(\tR\x05roles\"N\n\x16AuthUserDeleteRespo\
    nse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHead\
    erR\x06header\"V\n\x1eAuthUserChangePasswordResponse\x124\n\x06header\
    \x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\"Q\n\
    \x19AuthUserGrantRoleResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c\
    .etcdserverpb.ResponseHeaderR\x06header\"R\n\x1aAuthUserRevokeRoleRespon\
    se\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeade\
    rR\x06header\"K\n\x13AuthRoleAddResponse\x124\n\x06header\x18\x01\x20\
    \x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\"s\n\x13AuthRoleGe\
    tResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.Respon\
    seHeaderR\x06header\x12&\n\x04perm\x18\x02\x20\x03(\x0b2\x12.authpb.Perm\
    issionR\x04perm\"b\n\x14AuthRoleListResponse\x124\n\x06header\x18\x01\
    \x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12\x14\n\x05r\
    oles\x18\x02\x20\x03(\tR\x05roles\"b\n\x14AuthUserListResponse\x124\n\
    \x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06hea\
    der\x12\x14\n\x05users\x18\x02\x20\x03(\tR\x05users\"N\n\x16AuthRoleDele\
    teResponse\x124\n\x06header\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.Respo\
    nseHeaderR\x06header\"W\n\x1fAuthRoleGrantPermissionResponse\x124\n\x06h\
    eader\x18\x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\"\
    X\n\x20AuthRoleRevokePermissionResponse\x124\n\x06header\x18\x01\x20\x01\
    (\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header*/\n\tAlarmType\x12\x08\
    \n\x04NONE\x10\0\x12\x0b\n\x07NOSPACE\x10\x01\x12\x0b\n\x07CORRUPT\x10\
    \x022\xf8\x03\n\x02KV\x12]\n\x05Range\x12\x1a.etcdserverpb.RangeRequest\
    \x1a\x1b.etcdserverpb.RangeResponse\"\x1b\x82\xd3\xe4\x93\x02\x15\"\x10/\
    v3beta/kv/range:\x01*\x12U\n\x03Put\x12\x18.etcdserverpb.PutRequest\x1a\
    \x19.etcdserverpb.PutResponse\"\x19\x82\xd3\xe4\x93\x02\x13\"\x0e/v3beta\
    /kv/put:\x01*\x12u\n\x0bDeleteRange\x12\x20.etcdserverpb.DeleteRangeRequ\
    est\x1a!.etcdserverpb.DeleteRangeResponse\"!\x82\xd3\xe4\x93\x02\x1b\"\
    \x16/v3beta/kv/deleterange:\x01*\x12U\n\x03Txn\x12\x18.etcdserverpb.TxnR\
    equest\x1a\x19.etcdserverpb.TxnResponse\"\x19\x82\xd3\xe4\x93\x02\x13\"\
    \x0e/v3beta/kv/txn:\x01*\x12n\n\x07Compact\x12\x1f.etcdserverpb.Compacti\
    onRequest\x1a\x20.etcdserverpb.CompactionResponse\"\x20\x82\xd3\xe4\x93\
    \x02\x1a\"\x15/v3beta/kv/compaction:\x01*2g\n\x05Watch\x12^\n\x05Watch\
    \x12\x1a.etcdserverpb.WatchRequest\x1a\x1b.etcdserverpb.WatchResponse\"\
    \x18\x82\xd3\xe4\x93\x02\x12\"\r/v3beta/watch:\x01*(\x010\x012\xf7\x04\n\
    \x05Lease\x12o\n\nLeaseGrant\x12\x1f.etcdserverpb.LeaseGrantRequest\x1a\
    \x20.etcdserverpb.LeaseGrantResponse\"\x1e\x82\xd3\xe4\x93\x02\x18\"\x13\
    /v3beta/lease/grant:\x01*\x12v\n\x0bLeaseRevoke\x12\x20.etcdserverpb.Lea\
    seRevokeRequest\x1a!.etcdserverpb.LeaseRevokeResponse\"\"\x82\xd3\xe4\
    \x93\x02\x1c\"\x17/v3beta/kv/lease/revoke:\x01*\x12\x83\x01\n\x0eLeaseKe\
    epAlive\x12#.etcdserverpb.LeaseKeepAliveRequest\x1a$.etcdserverpb.LeaseK\
    eepAliveResponse\"\"\x82\xd3\xe4\x93\x02\x1c\"\x17/v3beta/lease/keepaliv\
    e:\x01*(\x010\x01\x12\x86\x01\n\x0fLeaseTimeToLive\x12$.etcdserverpb.Lea\
    seTimeToLiveRequest\x1a%.etcdserverpb.LeaseTimeToLiveResponse\"&\x82\xd3\
    \xe4\x93\x02\x20\"\x1b/v3beta/kv/lease/timetolive:\x01*\x12v\n\x0bLeaseL\
    eases\x12\x20.etcdserverpb.LeaseLeasesRequest\x1a!.etcdserverpb.LeaseLea\
    sesResponse\"\"\x82\xd3\xe4\x93\x02\x1c\"\x17/v3beta/kv/lease/leases:\
    \x01*2\xf9\x03\n\x07Cluster\x12s\n\tMemberAdd\x12\x1e.etcdserverpb.Membe\
    rAddRequest\x1a\x1f.etcdserverpb.MemberAddResponse\"%\x82\xd3\xe4\x93\
    \x02\x1f\"\x1a/v3beta/cluster/member/add:\x01*\x12\x7f\n\x0cMemberRemove\
    \x12!.etcdserverpb.MemberRemoveRequest\x1a\".etcdserverpb.MemberRemoveRe\
    sponse\"(\x82\xd3\xe4\x93\x02\"\"\x1d/v3beta/cluster/member/remove:\x01*\
    \x12\x7f\n\x0cMemberUpdate\x12!.etcdserverpb.MemberUpdateRequest\x1a\".e\
    tcdserverpb.MemberUpdateResponse\"(\x82\xd3\xe4\x93\x02\"\"\x1d/v3beta/c\
    luster/member/update:\x01*\x12w\n\nMemberList\x12\x1f.etcdserverpb.Membe\
    rListRequest\x1a\x20.etcdserverpb.MemberListResponse\"&\x82\xd3\xe4\x93\
    \x02\x20\"\x1b/v3beta/cluster/member/list:\x01*2\xa7\x06\n\x0bMaintenanc\
    e\x12f\n\x05Alarm\x12\x1a.etcdserverpb.AlarmRequest\x1a\x1b.etcdserverpb\
    .AlarmResponse\"$\x82\xd3\xe4\x93\x02\x1e\"\x19/v3beta/maintenance/alarm\
    :\x01*\x12j\n\x06Status\x12\x1b.etcdserverpb.StatusRequest\x1a\x1c.etcds\
    erverpb.StatusResponse\"%\x82\xd3\xe4\x93\x02\x1f\"\x1a/v3beta/maintenan\
    ce/status:\x01*\x12z\n\nDefragment\x12\x1f.etcdserverpb.DefragmentReques\
    t\x1a\x20.etcdserverpb.DefragmentResponse\")\x82\xd3\xe4\x93\x02#\"\x1e/\
    v3beta/maintenance/defragment:\x01*\x12b\n\x04Hash\x12\x19.etcdserverpb.\
    HashRequest\x1a\x1a.etcdserverpb.HashResponse\"#\x82\xd3\xe4\x93\x02\x1d\
    \"\x18/v3beta/maintenance/hash:\x01*\x12h\n\x06HashKV\x12\x1b.etcdserver\
    pb.HashKVRequest\x1a\x1c.etcdserverpb.HashKVResponse\"#\x82\xd3\xe4\x93\
    \x02\x1d\"\x18/v3beta/maintenance/hash:\x01*\x12t\n\x08Snapshot\x12\x1d.\
    etcdserverpb.SnapshotRequest\x1a\x1e.etcdserverpb.SnapshotResponse\"'\
    \x82\xd3\xe4\x93\x02!\"\x1c/v3beta/maintenance/snapshot:\x01*0\x01\x12\
    \x83\x01\n\nMoveLeader\x12\x1f.etcdserverpb.MoveLeaderRequest\x1a\x20.et\
    cdserverpb.MoveLeaderResponse\"2\x82\xd3\xe4\x93\x02,\"'/v3beta/maintena\
    nce/transfer-leadership:\x01*2\xfa\x0f\n\x04Auth\x12o\n\nAuthEnable\x12\
    \x1f.etcdserverpb.AuthEnableRequest\x1a\x20.etcdserverpb.AuthEnableRespo\
    nse\"\x1e\x82\xd3\xe4\x93\x02\x18\"\x13/v3beta/auth/enable:\x01*\x12s\n\
    \x0bAuthDisable\x12\x20.etcdserverpb.AuthDisableRequest\x1a!.etcdserverp\
    b.AuthDisableResponse\"\x1f\x82\xd3\xe4\x93\x02\x19\"\x14/v3beta/auth/di\
    sable:\x01*\x12{\n\x0cAuthenticate\x12!.etcdserverpb.AuthenticateRequest\
    \x1a\".etcdserverpb.AuthenticateResponse\"$\x82\xd3\xe4\x93\x02\x1e\"\
    \x19/v3beta/auth/authenticate:\x01*\x12p\n\x07UserAdd\x12\x20.etcdserver\
    pb.AuthUserAddRequest\x1a!.etcdserverpb.AuthUserAddResponse\"\x20\x82\
    \xd3\xe4\x93\x02\x1a\"\x15/v3beta/auth/user/add:\x01*\x12p\n\x07UserGet\
    \x12\x20.etcdserverpb.AuthUserGetRequest\x1a!.etcdserverpb.AuthUserGetRe\
    sponse\"\x20\x82\xd3\xe4\x93\x02\x1a\"\x15/v3beta/auth/user/get:\x01*\
    \x12t\n\x08UserList\x12!.etcdserverpb.AuthUserListRequest\x1a\".etcdserv\
    erpb.AuthUserListResponse\"!\x82\xd3\xe4\x93\x02\x1b\"\x16/v3beta/auth/u\
    ser/list:\x01*\x12|\n\nUserDelete\x12#.etcdserverpb.AuthUserDeleteReques\
    t\x1a$.etcdserverpb.AuthUserDeleteResponse\"#\x82\xd3\xe4\x93\x02\x1d\"\
    \x18/v3beta/auth/user/delete:\x01*\x12\x96\x01\n\x12UserChangePassword\
    \x12+.etcdserverpb.AuthUserChangePasswordRequest\x1a,.etcdserverpb.AuthU\
    serChangePasswordResponse\"%\x82\xd3\xe4\x93\x02\x1f\"\x1a/v3beta/auth/u\
    ser/changepw:\x01*\x12\x84\x01\n\rUserGrantRole\x12&.etcdserverpb.AuthUs\
    erGrantRoleRequest\x1a'.etcdserverpb.AuthUserGrantRoleResponse\"\"\x82\
    \xd3\xe4\x93\x02\x1c\"\x17/v3beta/auth/user/grant:\x01*\x12\x88\x01\n\
    \x0eUserRevokeRole\x12'.etcdserverpb.AuthUserRevokeRoleRequest\x1a(.etcd\
    serverpb.AuthUserRevokeRoleResponse\"#\x82\xd3\xe4\x93\x02\x1d\"\x18/v3b\
    eta/auth/user/revoke:\x01*\x12p\n\x07RoleAdd\x12\x20.etcdserverpb.AuthRo\
    leAddRequest\x1a!.etcdserverpb.AuthRoleAddResponse\"\x20\x82\xd3\xe4\x93\
    \x02\x1a\"\x15/v3beta/auth/role/add:\x01*\x12p\n\x07RoleGet\x12\x20.etcd\
    serverpb.AuthRoleGetRequest\x1a!.etcdserverpb.AuthRoleGetResponse\"\x20\
    \x82\xd3\xe4\x93\x02\x1a\"\x15/v3beta/auth/role/get:\x01*\x12t\n\x08Role\
    List\x12!.etcdserverpb.AuthRoleListRequest\x1a\".etcdserverpb.AuthRoleLi\
    stResponse\"!\x82\xd3\xe4\x93\x02\x1b\"\x16/v3beta/auth/role/list:\x01*\
    \x12|\n\nRoleDelete\x12#.etcdserverpb.AuthRoleDeleteRequest\x1a$.etcdser\
    verpb.AuthRoleDeleteResponse\"#\x82\xd3\xe4\x93\x02\x1d\"\x18/v3beta/aut\
    h/role/delete:\x01*\x12\x96\x01\n\x13RoleGrantPermission\x12,.etcdserver\
    pb.AuthRoleGrantPermissionRequest\x1a-.etcdserverpb.AuthRoleGrantPermiss\
    ionResponse\"\"\x82\xd3\xe4\x93\x02\x1c\"\x17/v3beta/auth/role/grant:\
    \x01*\x12\x9a\x01\n\x14RoleRevokePermission\x12-.etcdserverpb.AuthRoleRe\
    vokePermissionRequest\x1a..etcdserverpb.AuthRoleRevokePermissionResponse\
    \"#\x82\xd3\xe4\x93\x02\x1d\"\x18/v3beta/auth/role/revoke:\x01*B\x08\xc8\
    \xe2\x1e\x01\xd0\xe2\x1e\x01J\xae\xc4\x02\n\x07\x12\x05\0\0\x9c\x08\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x14\n\t\
    \n\x02\x03\0\x12\x03\x03\x07\x1d\n\t\n\x02\x03\x01\x12\x03\x04\x07\"\n\t\
    \n\x02\x03\x02\x12\x03\x05\x07$\n\x1d\n\x02\x03\x03\x12\x03\x08\x07%\x1a\
    \x12\x20for\x20grpc-gateway\n\n\x08\n\x01\x08\x12\x03\n\0(\n\x0b\n\x04\
    \x08\xe7\x07\0\x12\x03\n\0(\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\n\x07\
    \x20\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\n\x07\x20\n\x0e\n\x07\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\n\x08\x1f\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\
    \n#'\n\x08\n\x01\x08\x12\x03\x0b\0*\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\
    \x0b\0*\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x0b\x07\"\n\r\n\x06\x08\
    \xe7\x07\x01\x02\0\x12\x03\x0b\x07\"\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\
    \x01\x12\x03\x0b\x08!\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\x0b%)\n\n\
    \n\x02\x06\0\x12\x04\r\0>\x01\n\n\n\x03\x06\0\x01\x12\x03\r\x08\n\nJ\n\
    \x04\x06\0\x02\0\x12\x04\x0f\x02\x14\x03\x1a<\x20Range\x20gets\x20the\
    \x20keys\x20in\x20the\x20range\x20from\x20the\x20key-value\x20store.\n\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x03\x0f\x06\x0b\n\x0c\n\x05\x06\0\x02\0\
    \x02\x12\x03\x0f\x0c\x18\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x0f#0\n\r\n\
    \x05\x06\0\x02\0\x04\x12\x04\x10\x06\x13\x06\n\x10\n\x08\x06\0\x02\0\x04\
    \xe7\x07\0\x12\x04\x10\x06\x13\x06\n\x10\n\t\x06\0\x02\0\x04\xe7\x07\0\
    \x02\x12\x03\x10\r\x1e\n\x11\n\n\x06\0\x02\0\x04\xe7\x07\0\x02\0\x12\x03\
    \x10\r\x1e\n\x12\n\x0b\x06\0\x02\0\x04\xe7\x07\0\x02\0\x01\x12\x03\x10\
    \x0e\x1d\n\x11\n\t\x06\0\x02\0\x04\xe7\x07\0\x08\x12\x04\x10!\x13\x05\n\
    \xae\x01\n\x04\x06\0\x02\x01\x12\x04\x19\x02\x1e\x03\x1a\x9f\x01\x20Put\
    \x20puts\x20the\x20given\x20key\x20into\x20the\x20key-value\x20store.\n\
    \x20A\x20put\x20request\x20increments\x20the\x20revision\x20of\x20the\
    \x20key-value\x20store\n\x20and\x20generates\x20one\x20event\x20in\x20th\
    e\x20event\x20history.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x19\x06\t\
    \n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x19\n\x14\n\x0c\n\x05\x06\0\x02\
    \x01\x03\x12\x03\x19\x1f*\n\r\n\x05\x06\0\x02\x01\x04\x12\x04\x1a\x06\
    \x1d\x06\n\x10\n\x08\x06\0\x02\x01\x04\xe7\x07\0\x12\x04\x1a\x06\x1d\x06\
    \n\x10\n\t\x06\0\x02\x01\x04\xe7\x07\0\x02\x12\x03\x1a\r\x1e\n\x11\n\n\
    \x06\0\x02\x01\x04\xe7\x07\0\x02\0\x12\x03\x1a\r\x1e\n\x12\n\x0b\x06\0\
    \x02\x01\x04\xe7\x07\0\x02\0\x01\x12\x03\x1a\x0e\x1d\n\x11\n\t\x06\0\x02\
    \x01\x04\xe7\x07\0\x08\x12\x04\x1a!\x1d\x05\n\xd9\x01\n\x04\x06\0\x02\
    \x02\x12\x04#\x02(\x03\x1a\xca\x01\x20DeleteRange\x20deletes\x20the\x20g\
    iven\x20range\x20from\x20the\x20key-value\x20store.\n\x20A\x20delete\x20\
    request\x20increments\x20the\x20revision\x20of\x20the\x20key-value\x20st\
    ore\n\x20and\x20generates\x20a\x20delete\x20event\x20in\x20the\x20event\
    \x20history\x20for\x20every\x20deleted\x20key.\n\n\x0c\n\x05\x06\0\x02\
    \x02\x01\x12\x03#\x06\x11\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03#\x12$\n\
    \x0c\n\x05\x06\0\x02\x02\x03\x12\x03#/B\n\r\n\x05\x06\0\x02\x02\x04\x12\
    \x04$\x06'\x06\n\x10\n\x08\x06\0\x02\x02\x04\xe7\x07\0\x12\x04$\x06'\x06\
    \n\x10\n\t\x06\0\x02\x02\x04\xe7\x07\0\x02\x12\x03$\r\x1e\n\x11\n\n\x06\
    \0\x02\x02\x04\xe7\x07\0\x02\0\x12\x03$\r\x1e\n\x12\n\x0b\x06\0\x02\x02\
    \x04\xe7\x07\0\x02\0\x01\x12\x03$\x0e\x1d\n\x11\n\t\x06\0\x02\x02\x04\
    \xe7\x07\0\x08\x12\x04$!'\x05\n\x99\x02\n\x04\x06\0\x02\x03\x12\x04.\x02\
    3\x03\x1a\x8a\x02\x20Txn\x20processes\x20multiple\x20requests\x20in\x20a\
    \x20single\x20transaction.\n\x20A\x20txn\x20request\x20increments\x20the\
    \x20revision\x20of\x20the\x20key-value\x20store\n\x20and\x20generates\
    \x20events\x20with\x20the\x20same\x20revision\x20for\x20every\x20complet\
    ed\x20request.\n\x20It\x20is\x20not\x20allowed\x20to\x20modify\x20the\
    \x20same\x20key\x20several\x20times\x20within\x20one\x20txn.\n\n\x0c\n\
    \x05\x06\0\x02\x03\x01\x12\x03.\x06\t\n\x0c\n\x05\x06\0\x02\x03\x02\x12\
    \x03.\n\x14\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03.\x1f*\n\r\n\x05\x06\0\
    \x02\x03\x04\x12\x04/\x062\x06\n\x10\n\x08\x06\0\x02\x03\x04\xe7\x07\0\
    \x12\x04/\x062\x06\n\x10\n\t\x06\0\x02\x03\x04\xe7\x07\0\x02\x12\x03/\r\
    \x1e\n\x11\n\n\x06\0\x02\x03\x04\xe7\x07\0\x02\0\x12\x03/\r\x1e\n\x12\n\
    \x0b\x06\0\x02\x03\x04\xe7\x07\0\x02\0\x01\x12\x03/\x0e\x1d\n\x11\n\t\
    \x06\0\x02\x03\x04\xe7\x07\0\x08\x12\x04/!2\x05\n\xc0\x01\n\x04\x06\0\
    \x02\x04\x12\x048\x02=\x03\x1a\xb1\x01\x20Compact\x20compacts\x20the\x20\
    event\x20history\x20in\x20the\x20etcd\x20key-value\x20store.\x20The\x20k\
    ey-value\n\x20store\x20should\x20be\x20periodically\x20compacted\x20or\
    \x20the\x20event\x20history\x20will\x20continue\x20to\x20grow\n\x20indef\
    initely.\n\n\x0c\n\x05\x06\0\x02\x04\x01\x12\x038\x06\r\n\x0c\n\x05\x06\
    \0\x02\x04\x02\x12\x038\x0e\x1f\n\x0c\n\x05\x06\0\x02\x04\x03\x12\x038*<\
    \n\r\n\x05\x06\0\x02\x04\x04\x12\x049\x06<\x06\n\x10\n\x08\x06\0\x02\x04\
    \x04\xe7\x07\0\x12\x049\x06<\x06\n\x10\n\t\x06\0\x02\x04\x04\xe7\x07\0\
    \x02\x12\x039\r\x1e\n\x11\n\n\x06\0\x02\x04\x04\xe7\x07\0\x02\0\x12\x039\
    \r\x1e\n\x12\n\x0b\x06\0\x02\x04\x04\xe7\x07\0\x02\0\x01\x12\x039\x0e\
    \x1d\n\x11\n\t\x06\0\x02\x04\x04\xe7\x07\0\x08\x12\x049!<\x05\n\n\n\x02\
    \x06\x01\x12\x04@\0L\x01\n\n\n\x03\x06\x01\x01\x12\x03@\x08\r\n\xff\x02\
    \n\x04\x06\x01\x02\0\x12\x04F\x02K\x03\x1a\xf0\x02\x20Watch\x20watches\
    \x20for\x20events\x20happening\x20or\x20that\x20have\x20happened.\x20Bot\
    h\x20input\x20and\x20output\n\x20are\x20streams;\x20the\x20input\x20stre\
    am\x20is\x20for\x20creating\x20and\x20canceling\x20watchers\x20and\x20th\
    e\x20output\n\x20stream\x20sends\x20events.\x20One\x20watch\x20RPC\x20ca\
    n\x20watch\x20on\x20multiple\x20key\x20ranges,\x20streaming\x20events\n\
    \x20for\x20several\x20watches\x20at\x20once.\x20The\x20entire\x20event\
    \x20history\x20can\x20be\x20watched\x20starting\x20from\x20the\n\x20last\
    \x20compaction\x20revision.\n\n\x0c\n\x05\x06\x01\x02\0\x01\x12\x03F\x06\
    \x0b\n\x0c\n\x05\x06\x01\x02\0\x05\x12\x03F\x0c\x12\n\x0c\n\x05\x06\x01\
    \x02\0\x02\x12\x03F\x13\x1f\n\x0c\n\x05\x06\x01\x02\0\x06\x12\x03F*0\n\
    \x0c\n\x05\x06\x01\x02\0\x03\x12\x03F1>\n\r\n\x05\x06\x01\x02\0\x04\x12\
    \x04G\x06J\x06\n\x10\n\x08\x06\x01\x02\0\x04\xe7\x07\0\x12\x04G\x06J\x06\
    \n\x10\n\t\x06\x01\x02\0\x04\xe7\x07\0\x02\x12\x03G\r\x1e\n\x11\n\n\x06\
    \x01\x02\0\x04\xe7\x07\0\x02\0\x12\x03G\r\x1e\n\x12\n\x0b\x06\x01\x02\0\
    \x04\xe7\x07\0\x02\0\x01\x12\x03G\x0e\x1d\n\x11\n\t\x06\x01\x02\0\x04\
    \xe7\x07\0\x08\x12\x04G!J\x05\n\n\n\x02\x06\x02\x12\x04N\0y\x01\n\n\n\
    \x03\x06\x02\x01\x12\x03N\x08\r\n\x9b\x02\n\x04\x06\x02\x02\0\x12\x04R\
    \x02W\x03\x1a\x8c\x02\x20LeaseGrant\x20creates\x20a\x20lease\x20which\
    \x20expires\x20if\x20the\x20server\x20does\x20not\x20receive\x20a\x20kee\
    pAlive\n\x20within\x20a\x20given\x20time\x20to\x20live\x20period.\x20All\
    \x20keys\x20attached\x20to\x20the\x20lease\x20will\x20be\x20expired\x20a\
    nd\n\x20deleted\x20if\x20the\x20lease\x20expires.\x20Each\x20expired\x20\
    key\x20generates\x20a\x20delete\x20event\x20in\x20the\x20event\x20histor\
    y.\n\n\x0c\n\x05\x06\x02\x02\0\x01\x12\x03R\x06\x10\n\x0c\n\x05\x06\x02\
    \x02\0\x02\x12\x03R\x11\"\n\x0c\n\x05\x06\x02\x02\0\x03\x12\x03R-?\n\r\n\
    \x05\x06\x02\x02\0\x04\x12\x04S\x06V\x06\n\x10\n\x08\x06\x02\x02\0\x04\
    \xe7\x07\0\x12\x04S\x06V\x06\n\x10\n\t\x06\x02\x02\0\x04\xe7\x07\0\x02\
    \x12\x03S\r\x1e\n\x11\n\n\x06\x02\x02\0\x04\xe7\x07\0\x02\0\x12\x03S\r\
    \x1e\n\x12\n\x0b\x06\x02\x02\0\x04\xe7\x07\0\x02\0\x01\x12\x03S\x0e\x1d\
    \n\x11\n\t\x06\x02\x02\0\x04\xe7\x07\0\x08\x12\x04S!V\x05\ng\n\x04\x06\
    \x02\x02\x01\x12\x04Z\x02_\x03\x1aY\x20LeaseRevoke\x20revokes\x20a\x20le\
    ase.\x20All\x20keys\x20attached\x20to\x20the\x20lease\x20will\x20expire\
    \x20and\x20be\x20deleted.\n\n\x0c\n\x05\x06\x02\x02\x01\x01\x12\x03Z\x06\
    \x11\n\x0c\n\x05\x06\x02\x02\x01\x02\x12\x03Z\x12$\n\x0c\n\x05\x06\x02\
    \x02\x01\x03\x12\x03Z/B\n\r\n\x05\x06\x02\x02\x01\x04\x12\x04[\x06^\x06\
    \n\x10\n\x08\x06\x02\x02\x01\x04\xe7\x07\0\x12\x04[\x06^\x06\n\x10\n\t\
    \x06\x02\x02\x01\x04\xe7\x07\0\x02\x12\x03[\r\x1e\n\x11\n\n\x06\x02\x02\
    \x01\x04\xe7\x07\0\x02\0\x12\x03[\r\x1e\n\x12\n\x0b\x06\x02\x02\x01\x04\
    \xe7\x07\0\x02\0\x01\x12\x03[\x0e\x1d\n\x11\n\t\x06\x02\x02\x01\x04\xe7\
    \x07\0\x08\x12\x04[!^\x05\n\xb7\x01\n\x04\x06\x02\x02\x02\x12\x04c\x02h\
    \x03\x1a\xa8\x01\x20LeaseKeepAlive\x20keeps\x20the\x20lease\x20alive\x20\
    by\x20streaming\x20keep\x20alive\x20requests\x20from\x20the\x20client\n\
    \x20to\x20the\x20server\x20and\x20streaming\x20keep\x20alive\x20response\
    s\x20from\x20the\x20server\x20to\x20the\x20client.\n\n\x0c\n\x05\x06\x02\
    \x02\x02\x01\x12\x03c\x06\x14\n\x0c\n\x05\x06\x02\x02\x02\x05\x12\x03c\
    \x15\x1b\n\x0c\n\x05\x06\x02\x02\x02\x02\x12\x03c\x1c1\n\x0c\n\x05\x06\
    \x02\x02\x02\x06\x12\x03c<B\n\x0c\n\x05\x06\x02\x02\x02\x03\x12\x03cCY\n\
    \r\n\x05\x06\x02\x02\x02\x04\x12\x04d\x06g\x06\n\x10\n\x08\x06\x02\x02\
    \x02\x04\xe7\x07\0\x12\x04d\x06g\x06\n\x10\n\t\x06\x02\x02\x02\x04\xe7\
    \x07\0\x02\x12\x03d\r\x1e\n\x11\n\n\x06\x02\x02\x02\x04\xe7\x07\0\x02\0\
    \x12\x03d\r\x1e\n\x12\n\x0b\x06\x02\x02\x02\x04\xe7\x07\0\x02\0\x01\x12\
    \x03d\x0e\x1d\n\x11\n\t\x06\x02\x02\x02\x04\xe7\x07\0\x08\x12\x04d!g\x05\
    \n<\n\x04\x06\x02\x02\x03\x12\x04k\x02p\x03\x1a.\x20LeaseTimeToLive\x20r\
    etrieves\x20lease\x20information.\n\n\x0c\n\x05\x06\x02\x02\x03\x01\x12\
    \x03k\x06\x15\n\x0c\n\x05\x06\x02\x02\x03\x02\x12\x03k\x16,\n\x0c\n\x05\
    \x06\x02\x02\x03\x03\x12\x03k7N\n\r\n\x05\x06\x02\x02\x03\x04\x12\x04l\
    \x06o\x06\n\x10\n\x08\x06\x02\x02\x03\x04\xe7\x07\0\x12\x04l\x06o\x06\n\
    \x10\n\t\x06\x02\x02\x03\x04\xe7\x07\0\x02\x12\x03l\r\x1e\n\x11\n\n\x06\
    \x02\x02\x03\x04\xe7\x07\0\x02\0\x12\x03l\r\x1e\n\x12\n\x0b\x06\x02\x02\
    \x03\x04\xe7\x07\0\x02\0\x01\x12\x03l\x0e\x1d\n\x11\n\t\x06\x02\x02\x03\
    \x04\xe7\x07\0\x08\x12\x04l!o\x05\n6\n\x04\x06\x02\x02\x04\x12\x04s\x02x\
    \x03\x1a(\x20LeaseLeases\x20lists\x20all\x20existing\x20leases.\n\n\x0c\
    \n\x05\x06\x02\x02\x04\x01\x12\x03s\x06\x11\n\x0c\n\x05\x06\x02\x02\x04\
    \x02\x12\x03s\x12$\n\x0c\n\x05\x06\x02\x02\x04\x03\x12\x03s/B\n\r\n\x05\
    \x06\x02\x02\x04\x04\x12\x04t\x06w\x06\n\x10\n\x08\x06\x02\x02\x04\x04\
    \xe7\x07\0\x12\x04t\x06w\x06\n\x10\n\t\x06\x02\x02\x04\x04\xe7\x07\0\x02\
    \x12\x03t\r\x1e\n\x11\n\n\x06\x02\x02\x04\x04\xe7\x07\0\x02\0\x12\x03t\r\
    \x1e\n\x12\n\x0b\x06\x02\x02\x04\x04\xe7\x07\0\x02\0\x01\x12\x03t\x0e\
    \x1d\n\x11\n\t\x06\x02\x02\x04\x04\xe7\x07\0\x08\x12\x04t!w\x05\n\x0b\n\
    \x02\x06\x03\x12\x05{\0\x9b\x01\x01\n\n\n\x03\x06\x03\x01\x12\x03{\x08\
    \x0f\n:\n\x04\x06\x03\x02\0\x12\x05}\x02\x82\x01\x03\x1a+\x20MemberAdd\
    \x20adds\x20a\x20member\x20into\x20the\x20cluster.\n\n\x0c\n\x05\x06\x03\
    \x02\0\x01\x12\x03}\x06\x0f\n\x0c\n\x05\x06\x03\x02\0\x02\x12\x03}\x10\
    \x20\n\x0c\n\x05\x06\x03\x02\0\x03\x12\x03}+<\n\x0e\n\x05\x06\x03\x02\0\
    \x04\x12\x05~\x06\x81\x01\x06\n\x11\n\x08\x06\x03\x02\0\x04\xe7\x07\0\
    \x12\x05~\x06\x81\x01\x06\n\x10\n\t\x06\x03\x02\0\x04\xe7\x07\0\x02\x12\
    \x03~\r\x1e\n\x11\n\n\x06\x03\x02\0\x04\xe7\x07\0\x02\0\x12\x03~\r\x1e\n\
    \x12\n\x0b\x06\x03\x02\0\x04\xe7\x07\0\x02\0\x01\x12\x03~\x0e\x1d\n\x12\
    \n\t\x06\x03\x02\0\x04\xe7\x07\0\x08\x12\x05~!\x81\x01\x05\nK\n\x04\x06\
    \x03\x02\x01\x12\x06\x85\x01\x02\x8a\x01\x03\x1a;\x20MemberRemove\x20rem\
    oves\x20an\x20existing\x20member\x20from\x20the\x20cluster.\n\n\r\n\x05\
    \x06\x03\x02\x01\x01\x12\x04\x85\x01\x06\x12\n\r\n\x05\x06\x03\x02\x01\
    \x02\x12\x04\x85\x01\x13&\n\r\n\x05\x06\x03\x02\x01\x03\x12\x04\x85\x011\
    E\n\x0f\n\x05\x06\x03\x02\x01\x04\x12\x06\x86\x01\x06\x89\x01\x06\n\x12\
    \n\x08\x06\x03\x02\x01\x04\xe7\x07\0\x12\x06\x86\x01\x06\x89\x01\x06\n\
    \x11\n\t\x06\x03\x02\x01\x04\xe7\x07\0\x02\x12\x04\x86\x01\r\x1e\n\x12\n\
    \n\x06\x03\x02\x01\x04\xe7\x07\0\x02\0\x12\x04\x86\x01\r\x1e\n\x13\n\x0b\
    \x06\x03\x02\x01\x04\xe7\x07\0\x02\0\x01\x12\x04\x86\x01\x0e\x1d\n\x13\n\
    \t\x06\x03\x02\x01\x04\xe7\x07\0\x08\x12\x06\x86\x01!\x89\x01\x05\n@\n\
    \x04\x06\x03\x02\x02\x12\x06\x8d\x01\x02\x92\x01\x03\x1a0\x20MemberUpdat\
    e\x20updates\x20the\x20member\x20configuration.\n\n\r\n\x05\x06\x03\x02\
    \x02\x01\x12\x04\x8d\x01\x06\x12\n\r\n\x05\x06\x03\x02\x02\x02\x12\x04\
    \x8d\x01\x13&\n\r\n\x05\x06\x03\x02\x02\x03\x12\x04\x8d\x011E\n\x0f\n\
    \x05\x06\x03\x02\x02\x04\x12\x06\x8e\x01\x06\x91\x01\x06\n\x12\n\x08\x06\
    \x03\x02\x02\x04\xe7\x07\0\x12\x06\x8e\x01\x06\x91\x01\x06\n\x11\n\t\x06\
    \x03\x02\x02\x04\xe7\x07\0\x02\x12\x04\x8e\x01\r\x1e\n\x12\n\n\x06\x03\
    \x02\x02\x04\xe7\x07\0\x02\0\x12\x04\x8e\x01\r\x1e\n\x13\n\x0b\x06\x03\
    \x02\x02\x04\xe7\x07\0\x02\0\x01\x12\x04\x8e\x01\x0e\x1d\n\x13\n\t\x06\
    \x03\x02\x02\x04\xe7\x07\0\x08\x12\x06\x8e\x01!\x91\x01\x05\nB\n\x04\x06\
    \x03\x02\x03\x12\x06\x95\x01\x02\x9a\x01\x03\x1a2\x20MemberList\x20lists\
    \x20all\x20the\x20members\x20in\x20the\x20cluster.\n\n\r\n\x05\x06\x03\
    \x02\x03\x01\x12\x04\x95\x01\x06\x10\n\r\n\x05\x06\x03\x02\x03\x02\x12\
    \x04\x95\x01\x11\"\n\r\n\x05\x06\x03\x02\x03\x03\x12\x04\x95\x01-?\n\x0f\
    \n\x05\x06\x03\x02\x03\x04\x12\x06\x96\x01\x06\x99\x01\x06\n\x12\n\x08\
    \x06\x03\x02\x03\x04\xe7\x07\0\x12\x06\x96\x01\x06\x99\x01\x06\n\x11\n\t\
    \x06\x03\x02\x03\x04\xe7\x07\0\x02\x12\x04\x96\x01\r\x1e\n\x12\n\n\x06\
    \x03\x02\x03\x04\xe7\x07\0\x02\0\x12\x04\x96\x01\r\x1e\n\x13\n\x0b\x06\
    \x03\x02\x03\x04\xe7\x07\0\x02\0\x01\x12\x04\x96\x01\x0e\x1d\n\x13\n\t\
    \x06\x03\x02\x03\x04\xe7\x07\0\x08\x12\x06\x96\x01!\x99\x01\x05\n\x0c\n\
    \x02\x06\x04\x12\x06\x9d\x01\0\xd7\x01\x01\n\x0b\n\x03\x06\x04\x01\x12\
    \x04\x9d\x01\x08\x13\n\\\n\x04\x06\x04\x02\0\x12\x06\x9f\x01\x02\xa4\x01\
    \x03\x1aL\x20Alarm\x20activates,\x20deactivates,\x20and\x20queries\x20al\
    arms\x20regarding\x20cluster\x20health.\n\n\r\n\x05\x06\x04\x02\0\x01\
    \x12\x04\x9f\x01\x06\x0b\n\r\n\x05\x06\x04\x02\0\x02\x12\x04\x9f\x01\x0c\
    \x18\n\r\n\x05\x06\x04\x02\0\x03\x12\x04\x9f\x01#0\n\x0f\n\x05\x06\x04\
    \x02\0\x04\x12\x06\xa0\x01\x06\xa3\x01\x06\n\x12\n\x08\x06\x04\x02\0\x04\
    \xe7\x07\0\x12\x06\xa0\x01\x06\xa3\x01\x06\n\x11\n\t\x06\x04\x02\0\x04\
    \xe7\x07\0\x02\x12\x04\xa0\x01\r\x1e\n\x12\n\n\x06\x04\x02\0\x04\xe7\x07\
    \0\x02\0\x12\x04\xa0\x01\r\x1e\n\x13\n\x0b\x06\x04\x02\0\x04\xe7\x07\0\
    \x02\0\x01\x12\x04\xa0\x01\x0e\x1d\n\x13\n\t\x06\x04\x02\0\x04\xe7\x07\0\
    \x08\x12\x06\xa0\x01!\xa3\x01\x05\n7\n\x04\x06\x04\x02\x01\x12\x06\xa7\
    \x01\x02\xac\x01\x03\x1a'\x20Status\x20gets\x20the\x20status\x20of\x20th\
    e\x20member.\n\n\r\n\x05\x06\x04\x02\x01\x01\x12\x04\xa7\x01\x06\x0c\n\r\
    \n\x05\x06\x04\x02\x01\x02\x12\x04\xa7\x01\r\x1a\n\r\n\x05\x06\x04\x02\
    \x01\x03\x12\x04\xa7\x01%3\n\x0f\n\x05\x06\x04\x02\x01\x04\x12\x06\xa8\
    \x01\x06\xab\x01\x06\n\x12\n\x08\x06\x04\x02\x01\x04\xe7\x07\0\x12\x06\
    \xa8\x01\x06\xab\x01\x06\n\x11\n\t\x06\x04\x02\x01\x04\xe7\x07\0\x02\x12\
    \x04\xa8\x01\r\x1e\n\x12\n\n\x06\x04\x02\x01\x04\xe7\x07\0\x02\0\x12\x04\
    \xa8\x01\r\x1e\n\x13\n\x0b\x06\x04\x02\x01\x04\xe7\x07\0\x02\0\x01\x12\
    \x04\xa8\x01\x0e\x1d\n\x13\n\t\x06\x04\x02\x01\x04\xe7\x07\0\x08\x12\x06\
    \xa8\x01!\xab\x01\x05\n^\n\x04\x06\x04\x02\x02\x12\x06\xaf\x01\x02\xb4\
    \x01\x03\x1aN\x20Defragment\x20defragments\x20a\x20member's\x20backend\
    \x20database\x20to\x20recover\x20storage\x20space.\n\n\r\n\x05\x06\x04\
    \x02\x02\x01\x12\x04\xaf\x01\x06\x10\n\r\n\x05\x06\x04\x02\x02\x02\x12\
    \x04\xaf\x01\x11\"\n\r\n\x05\x06\x04\x02\x02\x03\x12\x04\xaf\x01-?\n\x0f\
    \n\x05\x06\x04\x02\x02\x04\x12\x06\xb0\x01\x06\xb3\x01\x06\n\x12\n\x08\
    \x06\x04\x02\x02\x04\xe7\x07\0\x12\x06\xb0\x01\x06\xb3\x01\x06\n\x11\n\t\
    \x06\x04\x02\x02\x04\xe7\x07\0\x02\x12\x04\xb0\x01\r\x1e\n\x12\n\n\x06\
    \x04\x02\x02\x04\xe7\x07\0\x02\0\x12\x04\xb0\x01\r\x1e\n\x13\n\x0b\x06\
    \x04\x02\x02\x04\xe7\x07\0\x02\0\x01\x12\x04\xb0\x01\x0e\x1d\n\x13\n\t\
    \x06\x04\x02\x02\x04\xe7\x07\0\x08\x12\x06\xb0\x01!\xb3\x01\x05\n\xa1\
    \x01\n\x04\x06\x04\x02\x03\x12\x06\xb9\x01\x02\xbe\x01\x03\x1a\x90\x01\
    \x20Hash\x20computes\x20the\x20hash\x20of\x20the\x20KV's\x20backend.\n\
    \x20This\x20is\x20designed\x20for\x20testing;\x20do\x20not\x20use\x20thi\
    s\x20in\x20production\x20when\x20there\n\x20are\x20ongoing\x20transactio\
    ns.\n\n\r\n\x05\x06\x04\x02\x03\x01\x12\x04\xb9\x01\x06\n\n\r\n\x05\x06\
    \x04\x02\x03\x02\x12\x04\xb9\x01\x0b\x16\n\r\n\x05\x06\x04\x02\x03\x03\
    \x12\x04\xb9\x01!-\n\x0f\n\x05\x06\x04\x02\x03\x04\x12\x06\xba\x01\x06\
    \xbd\x01\x06\n\x12\n\x08\x06\x04\x02\x03\x04\xe7\x07\0\x12\x06\xba\x01\
    \x06\xbd\x01\x06\n\x11\n\t\x06\x04\x02\x03\x04\xe7\x07\0\x02\x12\x04\xba\
    \x01\r\x1e\n\x12\n\n\x06\x04\x02\x03\x04\xe7\x07\0\x02\0\x12\x04\xba\x01\
    \r\x1e\n\x13\n\x0b\x06\x04\x02\x03\x04\xe7\x07\0\x02\0\x01\x12\x04\xba\
    \x01\x0e\x1d\n\x13\n\t\x06\x04\x02\x03\x04\xe7\x07\0\x08\x12\x06\xba\x01\
    !\xbd\x01\x05\nS\n\x04\x06\x04\x02\x04\x12\x06\xc1\x01\x02\xc6\x01\x03\
    \x1aC\x20HashKV\x20computes\x20the\x20hash\x20of\x20all\x20MVCC\x20keys\
    \x20up\x20to\x20a\x20given\x20revision.\n\n\r\n\x05\x06\x04\x02\x04\x01\
    \x12\x04\xc1\x01\x06\x0c\n\r\n\x05\x06\x04\x02\x04\x02\x12\x04\xc1\x01\r\
    \x1a\n\r\n\x05\x06\x04\x02\x04\x03\x12\x04\xc1\x01%3\n\x0f\n\x05\x06\x04\
    \x02\x04\x04\x12\x06\xc2\x01\x06\xc5\x01\x06\n\x12\n\x08\x06\x04\x02\x04\
    \x04\xe7\x07\0\x12\x06\xc2\x01\x06\xc5\x01\x06\n\x11\n\t\x06\x04\x02\x04\
    \x04\xe7\x07\0\x02\x12\x04\xc2\x01\r\x1e\n\x12\n\n\x06\x04\x02\x04\x04\
    \xe7\x07\0\x02\0\x12\x04\xc2\x01\r\x1e\n\x13\n\x0b\x06\x04\x02\x04\x04\
    \xe7\x07\0\x02\0\x01\x12\x04\xc2\x01\x0e\x1d\n\x13\n\t\x06\x04\x02\x04\
    \x04\xe7\x07\0\x08\x12\x06\xc2\x01!\xc5\x01\x05\nj\n\x04\x06\x04\x02\x05\
    \x12\x06\xc9\x01\x02\xce\x01\x03\x1aZ\x20Snapshot\x20sends\x20a\x20snaps\
    hot\x20of\x20the\x20entire\x20backend\x20from\x20a\x20member\x20over\x20\
    a\x20stream\x20to\x20a\x20client.\n\n\r\n\x05\x06\x04\x02\x05\x01\x12\
    \x04\xc9\x01\x06\x0e\n\r\n\x05\x06\x04\x02\x05\x02\x12\x04\xc9\x01\x0f\
    \x1e\n\r\n\x05\x06\x04\x02\x05\x06\x12\x04\xc9\x01)/\n\r\n\x05\x06\x04\
    \x02\x05\x03\x12\x04\xc9\x010@\n\x0f\n\x05\x06\x04\x02\x05\x04\x12\x06\
    \xca\x01\x06\xcd\x01\x06\n\x12\n\x08\x06\x04\x02\x05\x04\xe7\x07\0\x12\
    \x06\xca\x01\x06\xcd\x01\x06\n\x11\n\t\x06\x04\x02\x05\x04\xe7\x07\0\x02\
    \x12\x04\xca\x01\r\x1e\n\x12\n\n\x06\x04\x02\x05\x04\xe7\x07\0\x02\0\x12\
    \x04\xca\x01\r\x1e\n\x13\n\x0b\x06\x04\x02\x05\x04\xe7\x07\0\x02\0\x01\
    \x12\x04\xca\x01\x0e\x1d\n\x13\n\t\x06\x04\x02\x05\x04\xe7\x07\0\x08\x12\
    \x06\xca\x01!\xcd\x01\x05\nc\n\x04\x06\x04\x02\x06\x12\x06\xd1\x01\x02\
    \xd6\x01\x03\x1aS\x20MoveLeader\x20requests\x20current\x20leader\x20node\
    \x20to\x20transfer\x20its\x20leadership\x20to\x20transferee.\n\n\r\n\x05\
    \x06\x04\x02\x06\x01\x12\x04\xd1\x01\x06\x10\n\r\n\x05\x06\x04\x02\x06\
    \x02\x12\x04\xd1\x01\x11\"\n\r\n\x05\x06\x04\x02\x06\x03\x12\x04\xd1\x01\
    -?\n\x0f\n\x05\x06\x04\x02\x06\x04\x12\x06\xd2\x01\x06\xd5\x01\x06\n\x12\
    \n\x08\x06\x04\x02\x06\x04\xe7\x07\0\x12\x06\xd2\x01\x06\xd5\x01\x06\n\
    \x11\n\t\x06\x04\x02\x06\x04\xe7\x07\0\x02\x12\x04\xd2\x01\r\x1e\n\x12\n\
    \n\x06\x04\x02\x06\x04\xe7\x07\0\x02\0\x12\x04\xd2\x01\r\x1e\n\x13\n\x0b\
    \x06\x04\x02\x06\x04\xe7\x07\0\x02\0\x01\x12\x04\xd2\x01\x0e\x1d\n\x13\n\
    \t\x06\x04\x02\x06\x04\xe7\x07\0\x08\x12\x06\xd2\x01!\xd5\x01\x05\n\x0c\
    \n\x02\x06\x05\x12\x06\xd9\x01\0\xd9\x02\x01\n\x0b\n\x03\x06\x05\x01\x12\
    \x04\xd9\x01\x08\x0c\n4\n\x04\x06\x05\x02\0\x12\x06\xdb\x01\x02\xe0\x01\
    \x03\x1a$\x20AuthEnable\x20enables\x20authentication.\n\n\r\n\x05\x06\
    \x05\x02\0\x01\x12\x04\xdb\x01\x06\x10\n\r\n\x05\x06\x05\x02\0\x02\x12\
    \x04\xdb\x01\x11\"\n\r\n\x05\x06\x05\x02\0\x03\x12\x04\xdb\x01-?\n\x0f\n\
    \x05\x06\x05\x02\0\x04\x12\x06\xdc\x01\x06\xdf\x01\x06\n\x12\n\x08\x06\
    \x05\x02\0\x04\xe7\x07\0\x12\x06\xdc\x01\x06\xdf\x01\x06\n\x11\n\t\x06\
    \x05\x02\0\x04\xe7\x07\0\x02\x12\x04\xdc\x01\r\x1e\n\x12\n\n\x06\x05\x02\
    \0\x04\xe7\x07\0\x02\0\x12\x04\xdc\x01\r\x1e\n\x13\n\x0b\x06\x05\x02\0\
    \x04\xe7\x07\0\x02\0\x01\x12\x04\xdc\x01\x0e\x1d\n\x13\n\t\x06\x05\x02\0\
    \x04\xe7\x07\0\x08\x12\x06\xdc\x01!\xdf\x01\x05\n6\n\x04\x06\x05\x02\x01\
    \x12\x06\xe3\x01\x02\xe8\x01\x03\x1a&\x20AuthDisable\x20disables\x20auth\
    entication.\n\n\r\n\x05\x06\x05\x02\x01\x01\x12\x04\xe3\x01\x06\x11\n\r\
    \n\x05\x06\x05\x02\x01\x02\x12\x04\xe3\x01\x12$\n\r\n\x05\x06\x05\x02\
    \x01\x03\x12\x04\xe3\x01/B\n\x0f\n\x05\x06\x05\x02\x01\x04\x12\x06\xe4\
    \x01\x06\xe7\x01\x06\n\x12\n\x08\x06\x05\x02\x01\x04\xe7\x07\0\x12\x06\
    \xe4\x01\x06\xe7\x01\x06\n\x11\n\t\x06\x05\x02\x01\x04\xe7\x07\0\x02\x12\
    \x04\xe4\x01\r\x1e\n\x12\n\n\x06\x05\x02\x01\x04\xe7\x07\0\x02\0\x12\x04\
    \xe4\x01\r\x1e\n\x13\n\x0b\x06\x05\x02\x01\x04\xe7\x07\0\x02\0\x01\x12\
    \x04\xe4\x01\x0e\x1d\n\x13\n\t\x06\x05\x02\x01\x04\xe7\x07\0\x08\x12\x06\
    \xe4\x01!\xe7\x01\x05\nA\n\x04\x06\x05\x02\x02\x12\x06\xeb\x01\x02\xf0\
    \x01\x03\x1a1\x20Authenticate\x20processes\x20an\x20authenticate\x20requ\
    est.\n\n\r\n\x05\x06\x05\x02\x02\x01\x12\x04\xeb\x01\x06\x12\n\r\n\x05\
    \x06\x05\x02\x02\x02\x12\x04\xeb\x01\x13&\n\r\n\x05\x06\x05\x02\x02\x03\
    \x12\x04\xeb\x011E\n\x0f\n\x05\x06\x05\x02\x02\x04\x12\x06\xec\x01\x06\
    \xef\x01\x06\n\x12\n\x08\x06\x05\x02\x02\x04\xe7\x07\0\x12\x06\xec\x01\
    \x06\xef\x01\x06\n\x11\n\t\x06\x05\x02\x02\x04\xe7\x07\0\x02\x12\x04\xec\
    \x01\r\x1e\n\x12\n\n\x06\x05\x02\x02\x04\xe7\x07\0\x02\0\x12\x04\xec\x01\
    \r\x1e\n\x13\n\x0b\x06\x05\x02\x02\x04\xe7\x07\0\x02\0\x01\x12\x04\xec\
    \x01\x0e\x1d\n\x13\n\t\x06\x05\x02\x02\x04\xe7\x07\0\x08\x12\x06\xec\x01\
    !\xef\x01\x05\n*\n\x04\x06\x05\x02\x03\x12\x06\xf3\x01\x02\xf8\x01\x03\
    \x1a\x1a\x20UserAdd\x20adds\x20a\x20new\x20user.\n\n\r\n\x05\x06\x05\x02\
    \x03\x01\x12\x04\xf3\x01\x06\r\n\r\n\x05\x06\x05\x02\x03\x02\x12\x04\xf3\
    \x01\x0e\x20\n\r\n\x05\x06\x05\x02\x03\x03\x12\x04\xf3\x01+>\n\x0f\n\x05\
    \x06\x05\x02\x03\x04\x12\x06\xf4\x01\x06\xf7\x01\x06\n\x12\n\x08\x06\x05\
    \x02\x03\x04\xe7\x07\0\x12\x06\xf4\x01\x06\xf7\x01\x06\n\x11\n\t\x06\x05\
    \x02\x03\x04\xe7\x07\0\x02\x12\x04\xf4\x01\r\x1e\n\x12\n\n\x06\x05\x02\
    \x03\x04\xe7\x07\0\x02\0\x12\x04\xf4\x01\r\x1e\n\x13\n\x0b\x06\x05\x02\
    \x03\x04\xe7\x07\0\x02\0\x01\x12\x04\xf4\x01\x0e\x1d\n\x13\n\t\x06\x05\
    \x02\x03\x04\xe7\x07\0\x08\x12\x06\xf4\x01!\xf7\x01\x05\n9\n\x04\x06\x05\
    \x02\x04\x12\x06\xfb\x01\x02\x80\x02\x03\x1a)\x20UserGet\x20gets\x20deta\
    iled\x20user\x20information.\n\n\r\n\x05\x06\x05\x02\x04\x01\x12\x04\xfb\
    \x01\x06\r\n\r\n\x05\x06\x05\x02\x04\x02\x12\x04\xfb\x01\x0e\x20\n\r\n\
    \x05\x06\x05\x02\x04\x03\x12\x04\xfb\x01+>\n\x0f\n\x05\x06\x05\x02\x04\
    \x04\x12\x06\xfc\x01\x06\xff\x01\x06\n\x12\n\x08\x06\x05\x02\x04\x04\xe7\
    \x07\0\x12\x06\xfc\x01\x06\xff\x01\x06\n\x11\n\t\x06\x05\x02\x04\x04\xe7\
    \x07\0\x02\x12\x04\xfc\x01\r\x1e\n\x12\n\n\x06\x05\x02\x04\x04\xe7\x07\0\
    \x02\0\x12\x04\xfc\x01\r\x1e\n\x13\n\x0b\x06\x05\x02\x04\x04\xe7\x07\0\
    \x02\0\x01\x12\x04\xfc\x01\x0e\x1d\n\x13\n\t\x06\x05\x02\x04\x04\xe7\x07\
    \0\x08\x12\x06\xfc\x01!\xff\x01\x05\n4\n\x04\x06\x05\x02\x05\x12\x06\x83\
    \x02\x02\x88\x02\x03\x1a$\x20UserList\x20gets\x20a\x20list\x20of\x20all\
    \x20users.\n\n\r\n\x05\x06\x05\x02\x05\x01\x12\x04\x83\x02\x06\x0e\n\r\n\
    \x05\x06\x05\x02\x05\x02\x12\x04\x83\x02\x0f\"\n\r\n\x05\x06\x05\x02\x05\
    \x03\x12\x04\x83\x02-A\n\x0f\n\x05\x06\x05\x02\x05\x04\x12\x06\x84\x02\
    \x06\x87\x02\x06\n\x12\n\x08\x06\x05\x02\x05\x04\xe7\x07\0\x12\x06\x84\
    \x02\x06\x87\x02\x06\n\x11\n\t\x06\x05\x02\x05\x04\xe7\x07\0\x02\x12\x04\
    \x84\x02\r\x1e\n\x12\n\n\x06\x05\x02\x05\x04\xe7\x07\0\x02\0\x12\x04\x84\
    \x02\r\x1e\n\x13\n\x0b\x06\x05\x02\x05\x04\xe7\x07\0\x02\0\x01\x12\x04\
    \x84\x02\x0e\x1d\n\x13\n\t\x06\x05\x02\x05\x04\xe7\x07\0\x08\x12\x06\x84\
    \x02!\x87\x02\x05\n6\n\x04\x06\x05\x02\x06\x12\x06\x8b\x02\x02\x90\x02\
    \x03\x1a&\x20UserDelete\x20deletes\x20a\x20specified\x20user.\n\n\r\n\
    \x05\x06\x05\x02\x06\x01\x12\x04\x8b\x02\x06\x10\n\r\n\x05\x06\x05\x02\
    \x06\x02\x12\x04\x8b\x02\x11&\n\r\n\x05\x06\x05\x02\x06\x03\x12\x04\x8b\
    \x021G\n\x0f\n\x05\x06\x05\x02\x06\x04\x12\x06\x8c\x02\x06\x8f\x02\x06\n\
    \x12\n\x08\x06\x05\x02\x06\x04\xe7\x07\0\x12\x06\x8c\x02\x06\x8f\x02\x06\
    \n\x11\n\t\x06\x05\x02\x06\x04\xe7\x07\0\x02\x12\x04\x8c\x02\r\x1e\n\x12\
    \n\n\x06\x05\x02\x06\x04\xe7\x07\0\x02\0\x12\x04\x8c\x02\r\x1e\n\x13\n\
    \x0b\x06\x05\x02\x06\x04\xe7\x07\0\x02\0\x01\x12\x04\x8c\x02\x0e\x1d\n\
    \x13\n\t\x06\x05\x02\x06\x04\xe7\x07\0\x08\x12\x06\x8c\x02!\x8f\x02\x05\
    \nN\n\x04\x06\x05\x02\x07\x12\x06\x93\x02\x02\x98\x02\x03\x1a>\x20UserCh\
    angePassword\x20changes\x20the\x20password\x20of\x20a\x20specified\x20us\
    er.\n\n\r\n\x05\x06\x05\x02\x07\x01\x12\x04\x93\x02\x06\x18\n\r\n\x05\
    \x06\x05\x02\x07\x02\x12\x04\x93\x02\x196\n\r\n\x05\x06\x05\x02\x07\x03\
    \x12\x04\x93\x02A_\n\x0f\n\x05\x06\x05\x02\x07\x04\x12\x06\x94\x02\x06\
    \x97\x02\x06\n\x12\n\x08\x06\x05\x02\x07\x04\xe7\x07\0\x12\x06\x94\x02\
    \x06\x97\x02\x06\n\x11\n\t\x06\x05\x02\x07\x04\xe7\x07\0\x02\x12\x04\x94\
    \x02\r\x1e\n\x12\n\n\x06\x05\x02\x07\x04\xe7\x07\0\x02\0\x12\x04\x94\x02\
    \r\x1e\n\x13\n\x0b\x06\x05\x02\x07\x04\xe7\x07\0\x02\0\x01\x12\x04\x94\
    \x02\x0e\x1d\n\x13\n\t\x06\x05\x02\x07\x04\xe7\x07\0\x08\x12\x06\x94\x02\
    !\x97\x02\x05\n>\n\x04\x06\x05\x02\x08\x12\x06\x9b\x02\x02\xa0\x02\x03\
    \x1a.\x20UserGrant\x20grants\x20a\x20role\x20to\x20a\x20specified\x20use\
    r.\n\n\r\n\x05\x06\x05\x02\x08\x01\x12\x04\x9b\x02\x06\x13\n\r\n\x05\x06\
    \x05\x02\x08\x02\x12\x04\x9b\x02\x14,\n\r\n\x05\x06\x05\x02\x08\x03\x12\
    \x04\x9b\x027P\n\x0f\n\x05\x06\x05\x02\x08\x04\x12\x06\x9c\x02\x06\x9f\
    \x02\x06\n\x12\n\x08\x06\x05\x02\x08\x04\xe7\x07\0\x12\x06\x9c\x02\x06\
    \x9f\x02\x06\n\x11\n\t\x06\x05\x02\x08\x04\xe7\x07\0\x02\x12\x04\x9c\x02\
    \r\x1e\n\x12\n\n\x06\x05\x02\x08\x04\xe7\x07\0\x02\0\x12\x04\x9c\x02\r\
    \x1e\n\x13\n\x0b\x06\x05\x02\x08\x04\xe7\x07\0\x02\0\x01\x12\x04\x9c\x02\
    \x0e\x1d\n\x13\n\t\x06\x05\x02\x08\x04\xe7\x07\0\x08\x12\x06\x9c\x02!\
    \x9f\x02\x05\nB\n\x04\x06\x05\x02\t\x12\x06\xa3\x02\x02\xa8\x02\x03\x1a2\
    \x20UserRevokeRole\x20revokes\x20a\x20role\x20of\x20specified\x20user.\n\
    \n\r\n\x05\x06\x05\x02\t\x01\x12\x04\xa3\x02\x06\x14\n\r\n\x05\x06\x05\
    \x02\t\x02\x12\x04\xa3\x02\x15.\n\r\n\x05\x06\x05\x02\t\x03\x12\x04\xa3\
    \x029S\n\x0f\n\x05\x06\x05\x02\t\x04\x12\x06\xa4\x02\x06\xa7\x02\x06\n\
    \x12\n\x08\x06\x05\x02\t\x04\xe7\x07\0\x12\x06\xa4\x02\x06\xa7\x02\x06\n\
    \x11\n\t\x06\x05\x02\t\x04\xe7\x07\0\x02\x12\x04\xa4\x02\r\x1e\n\x12\n\n\
    \x06\x05\x02\t\x04\xe7\x07\0\x02\0\x12\x04\xa4\x02\r\x1e\n\x13\n\x0b\x06\
    \x05\x02\t\x04\xe7\x07\0\x02\0\x01\x12\x04\xa4\x02\x0e\x1d\n\x13\n\t\x06\
    \x05\x02\t\x04\xe7\x07\0\x08\x12\x06\xa4\x02!\xa7\x02\x05\n*\n\x04\x06\
    \x05\x02\n\x12\x06\xab\x02\x02\xb0\x02\x03\x1a\x1a\x20RoleAdd\x20adds\
    \x20a\x20new\x20role.\n\n\r\n\x05\x06\x05\x02\n\x01\x12\x04\xab\x02\x06\
    \r\n\r\n\x05\x06\x05\x02\n\x02\x12\x04\xab\x02\x0e\x20\n\r\n\x05\x06\x05\
    \x02\n\x03\x12\x04\xab\x02+>\n\x0f\n\x05\x06\x05\x02\n\x04\x12\x06\xac\
    \x02\x06\xaf\x02\x06\n\x12\n\x08\x06\x05\x02\n\x04\xe7\x07\0\x12\x06\xac\
    \x02\x06\xaf\x02\x06\n\x11\n\t\x06\x05\x02\n\x04\xe7\x07\0\x02\x12\x04\
    \xac\x02\r\x1e\n\x12\n\n\x06\x05\x02\n\x04\xe7\x07\0\x02\0\x12\x04\xac\
    \x02\r\x1e\n\x13\n\x0b\x06\x05\x02\n\x04\xe7\x07\0\x02\0\x01\x12\x04\xac\
    \x02\x0e\x1d\n\x13\n\t\x06\x05\x02\n\x04\xe7\x07\0\x08\x12\x06\xac\x02!\
    \xaf\x02\x05\n9\n\x04\x06\x05\x02\x0b\x12\x06\xb3\x02\x02\xb8\x02\x03\
    \x1a)\x20RoleGet\x20gets\x20detailed\x20role\x20information.\n\n\r\n\x05\
    \x06\x05\x02\x0b\x01\x12\x04\xb3\x02\x06\r\n\r\n\x05\x06\x05\x02\x0b\x02\
    \x12\x04\xb3\x02\x0e\x20\n\r\n\x05\x06\x05\x02\x0b\x03\x12\x04\xb3\x02+>\
    \n\x0f\n\x05\x06\x05\x02\x0b\x04\x12\x06\xb4\x02\x06\xb7\x02\x06\n\x12\n\
    \x08\x06\x05\x02\x0b\x04\xe7\x07\0\x12\x06\xb4\x02\x06\xb7\x02\x06\n\x11\
    \n\t\x06\x05\x02\x0b\x04\xe7\x07\0\x02\x12\x04\xb4\x02\r\x1e\n\x12\n\n\
    \x06\x05\x02\x0b\x04\xe7\x07\0\x02\0\x12\x04\xb4\x02\r\x1e\n\x13\n\x0b\
    \x06\x05\x02\x0b\x04\xe7\x07\0\x02\0\x01\x12\x04\xb4\x02\x0e\x1d\n\x13\n\
    \t\x06\x05\x02\x0b\x04\xe7\x07\0\x08\x12\x06\xb4\x02!\xb7\x02\x05\n3\n\
    \x04\x06\x05\x02\x0c\x12\x06\xbb\x02\x02\xc0\x02\x03\x1a#\x20RoleList\
    \x20gets\x20lists\x20of\x20all\x20roles.\n\n\r\n\x05\x06\x05\x02\x0c\x01\
    \x12\x04\xbb\x02\x06\x0e\n\r\n\x05\x06\x05\x02\x0c\x02\x12\x04\xbb\x02\
    \x0f\"\n\r\n\x05\x06\x05\x02\x0c\x03\x12\x04\xbb\x02-A\n\x0f\n\x05\x06\
    \x05\x02\x0c\x04\x12\x06\xbc\x02\x06\xbf\x02\x06\n\x12\n\x08\x06\x05\x02\
    \x0c\x04\xe7\x07\0\x12\x06\xbc\x02\x06\xbf\x02\x06\n\x11\n\t\x06\x05\x02\
    \x0c\x04\xe7\x07\0\x02\x12\x04\xbc\x02\r\x1e\n\x12\n\n\x06\x05\x02\x0c\
    \x04\xe7\x07\0\x02\0\x12\x04\xbc\x02\r\x1e\n\x13\n\x0b\x06\x05\x02\x0c\
    \x04\xe7\x07\0\x02\0\x01\x12\x04\xbc\x02\x0e\x1d\n\x13\n\t\x06\x05\x02\
    \x0c\x04\xe7\x07\0\x08\x12\x06\xbc\x02!\xbf\x02\x05\n6\n\x04\x06\x05\x02\
    \r\x12\x06\xc3\x02\x02\xc8\x02\x03\x1a&\x20RoleDelete\x20deletes\x20a\
    \x20specified\x20role.\n\n\r\n\x05\x06\x05\x02\r\x01\x12\x04\xc3\x02\x06\
    \x10\n\r\n\x05\x06\x05\x02\r\x02\x12\x04\xc3\x02\x11&\n\r\n\x05\x06\x05\
    \x02\r\x03\x12\x04\xc3\x021G\n\x0f\n\x05\x06\x05\x02\r\x04\x12\x06\xc4\
    \x02\x06\xc7\x02\x06\n\x12\n\x08\x06\x05\x02\r\x04\xe7\x07\0\x12\x06\xc4\
    \x02\x06\xc7\x02\x06\n\x11\n\t\x06\x05\x02\r\x04\xe7\x07\0\x02\x12\x04\
    \xc4\x02\r\x1e\n\x12\n\n\x06\x05\x02\r\x04\xe7\x07\0\x02\0\x12\x04\xc4\
    \x02\r\x1e\n\x13\n\x0b\x06\x05\x02\r\x04\xe7\x07\0\x02\0\x01\x12\x04\xc4\
    \x02\x0e\x1d\n\x13\n\t\x06\x05\x02\r\x04\xe7\x07\0\x08\x12\x06\xc4\x02!\
    \xc7\x02\x05\nj\n\x04\x06\x05\x02\x0e\x12\x06\xcb\x02\x02\xd0\x02\x03\
    \x1aZ\x20RoleGrantPermission\x20grants\x20a\x20permission\x20of\x20a\x20\
    specified\x20key\x20or\x20range\x20to\x20a\x20specified\x20role.\n\n\r\n\
    \x05\x06\x05\x02\x0e\x01\x12\x04\xcb\x02\x06\x19\n\r\n\x05\x06\x05\x02\
    \x0e\x02\x12\x04\xcb\x02\x1a8\n\r\n\x05\x06\x05\x02\x0e\x03\x12\x04\xcb\
    \x02Cb\n\x0f\n\x05\x06\x05\x02\x0e\x04\x12\x06\xcc\x02\x06\xcf\x02\x06\n\
    \x12\n\x08\x06\x05\x02\x0e\x04\xe7\x07\0\x12\x06\xcc\x02\x06\xcf\x02\x06\
    \n\x11\n\t\x06\x05\x02\x0e\x04\xe7\x07\0\x02\x12\x04\xcc\x02\r\x1e\n\x12\
    \n\n\x06\x05\x02\x0e\x04\xe7\x07\0\x02\0\x12\x04\xcc\x02\r\x1e\n\x13\n\
    \x0b\x06\x05\x02\x0e\x04\xe7\x07\0\x02\0\x01\x12\x04\xcc\x02\x0e\x1d\n\
    \x13\n\t\x06\x05\x02\x0e\x04\xe7\x07\0\x08\x12\x06\xcc\x02!\xcf\x02\x05\
    \n]\n\x04\x06\x05\x02\x0f\x12\x06\xd3\x02\x02\xd8\x02\x03\x1aM\x20RoleRe\
    vokePermission\x20revokes\x20a\x20key\x20or\x20range\x20permission\x20of\
    \x20a\x20specified\x20role.\n\n\r\n\x05\x06\x05\x02\x0f\x01\x12\x04\xd3\
    \x02\x06\x1a\n\r\n\x05\x06\x05\x02\x0f\x02\x12\x04\xd3\x02\x1b:\n\r\n\
    \x05\x06\x05\x02\x0f\x03\x12\x04\xd3\x02Ee\n\x0f\n\x05\x06\x05\x02\x0f\
    \x04\x12\x06\xd4\x02\x06\xd7\x02\x06\n\x12\n\x08\x06\x05\x02\x0f\x04\xe7\
    \x07\0\x12\x06\xd4\x02\x06\xd7\x02\x06\n\x11\n\t\x06\x05\x02\x0f\x04\xe7\
    \x07\0\x02\x12\x04\xd4\x02\r\x1e\n\x12\n\n\x06\x05\x02\x0f\x04\xe7\x07\0\
    \x02\0\x12\x04\xd4\x02\r\x1e\n\x13\n\x0b\x06\x05\x02\x0f\x04\xe7\x07\0\
    \x02\0\x01\x12\x04\xd4\x02\x0e\x1d\n\x13\n\t\x06\x05\x02\x0f\x04\xe7\x07\
    \0\x08\x12\x06\xd4\x02!\xd7\x02\x05\n\x0c\n\x02\x04\0\x12\x06\xdb\x02\0\
    \xe4\x02\x01\n\x0b\n\x03\x04\0\x01\x12\x04\xdb\x02\x08\x16\nL\n\x04\x04\
    \0\x02\0\x12\x04\xdd\x02\x02\x18\x1a>\x20cluster_id\x20is\x20the\x20ID\
    \x20of\x20the\x20cluster\x20which\x20sent\x20the\x20response.\n\n\x0f\n\
    \x05\x04\0\x02\0\x04\x12\x06\xdd\x02\x02\xdb\x02\x18\n\r\n\x05\x04\0\x02\
    \0\x05\x12\x04\xdd\x02\x02\x08\n\r\n\x05\x04\0\x02\0\x01\x12\x04\xdd\x02\
    \t\x13\n\r\n\x05\x04\0\x02\0\x03\x12\x04\xdd\x02\x16\x17\nJ\n\x04\x04\0\
    \x02\x01\x12\x04\xdf\x02\x02\x17\x1a<\x20member_id\x20is\x20the\x20ID\
    \x20of\x20the\x20member\x20which\x20sent\x20the\x20response.\n\n\x0f\n\
    \x05\x04\0\x02\x01\x04\x12\x06\xdf\x02\x02\xdd\x02\x18\n\r\n\x05\x04\0\
    \x02\x01\x05\x12\x04\xdf\x02\x02\x08\n\r\n\x05\x04\0\x02\x01\x01\x12\x04\
    \xdf\x02\t\x12\n\r\n\x05\x04\0\x02\x01\x03\x12\x04\xdf\x02\x15\x16\nV\n\
    \x04\x04\0\x02\x02\x12\x04\xe1\x02\x02\x15\x1aH\x20revision\x20is\x20the\
    \x20key-value\x20store\x20revision\x20when\x20the\x20request\x20was\x20a\
    pplied.\n\n\x0f\n\x05\x04\0\x02\x02\x04\x12\x06\xe1\x02\x02\xdf\x02\x17\
    \n\r\n\x05\x04\0\x02\x02\x05\x12\x04\xe1\x02\x02\x07\n\r\n\x05\x04\0\x02\
    \x02\x01\x12\x04\xe1\x02\x08\x10\n\r\n\x05\x04\0\x02\x02\x03\x12\x04\xe1\
    \x02\x13\x14\nH\n\x04\x04\0\x02\x03\x12\x04\xe3\x02\x02\x17\x1a:\x20raft\
    _term\x20is\x20the\x20raft\x20term\x20when\x20the\x20request\x20was\x20a\
    pplied.\n\n\x0f\n\x05\x04\0\x02\x03\x04\x12\x06\xe3\x02\x02\xe1\x02\x15\
    \n\r\n\x05\x04\0\x02\x03\x05\x12\x04\xe3\x02\x02\x08\n\r\n\x05\x04\0\x02\
    \x03\x01\x12\x04\xe3\x02\t\x12\n\r\n\x05\x04\0\x02\x03\x03\x12\x04\xe3\
    \x02\x15\x16\n\x0c\n\x02\x04\x01\x12\x06\xe6\x02\0\xa7\x03\x01\n\x0b\n\
    \x03\x04\x01\x01\x12\x04\xe6\x02\x08\x14\n\x0e\n\x04\x04\x01\x04\0\x12\
    \x06\xe7\x02\x02\xeb\x02\x03\n\r\n\x05\x04\x01\x04\0\x01\x12\x04\xe7\x02\
    \x07\x10\n%\n\x06\x04\x01\x04\0\x02\0\x12\x04\xe8\x02\x08\x11\"\x15\x20d\
    efault,\x20no\x20sorting\n\n\x0f\n\x07\x04\x01\x04\0\x02\0\x01\x12\x04\
    \xe8\x02\x08\x0c\n\x0f\n\x07\x04\x01\x04\0\x02\0\x02\x12\x04\xe8\x02\x0f\
    \x10\n+\n\x06\x04\x01\x04\0\x02\x01\x12\x04\xe9\x02\x08\x13\"\x1b\x20low\
    est\x20target\x20value\x20first\n\n\x0f\n\x07\x04\x01\x04\0\x02\x01\x01\
    \x12\x04\xe9\x02\x08\x0e\n\x0f\n\x07\x04\x01\x04\0\x02\x01\x02\x12\x04\
    \xe9\x02\x11\x12\n,\n\x06\x04\x01\x04\0\x02\x02\x12\x04\xea\x02\x08\x14\
    \"\x1c\x20highest\x20target\x20value\x20first\n\n\x0f\n\x07\x04\x01\x04\
    \0\x02\x02\x01\x12\x04\xea\x02\x08\x0f\n\x0f\n\x07\x04\x01\x04\0\x02\x02\
    \x02\x12\x04\xea\x02\x12\x13\n\x0e\n\x04\x04\x01\x04\x01\x12\x06\xec\x02\
    \x02\xf2\x02\x03\n\r\n\x05\x04\x01\x04\x01\x01\x12\x04\xec\x02\x07\x11\n\
    \x0e\n\x06\x04\x01\x04\x01\x02\0\x12\x04\xed\x02\x08\x10\n\x0f\n\x07\x04\
    \x01\x04\x01\x02\0\x01\x12\x04\xed\x02\x08\x0b\n\x0f\n\x07\x04\x01\x04\
    \x01\x02\0\x02\x12\x04\xed\x02\x0e\x0f\n\x0e\n\x06\x04\x01\x04\x01\x02\
    \x01\x12\x04\xee\x02\x08\x14\n\x0f\n\x07\x04\x01\x04\x01\x02\x01\x01\x12\
    \x04\xee\x02\x08\x0f\n\x0f\n\x07\x04\x01\x04\x01\x02\x01\x02\x12\x04\xee\
    \x02\x12\x13\n\x0e\n\x06\x04\x01\x04\x01\x02\x02\x12\x04\xef\x02\x08\x13\
    \n\x0f\n\x07\x04\x01\x04\x01\x02\x02\x01\x12\x04\xef\x02\x08\x0e\n\x0f\n\
    \x07\x04\x01\x04\x01\x02\x02\x02\x12\x04\xef\x02\x11\x12\n\x0e\n\x06\x04\
    \x01\x04\x01\x02\x03\x12\x04\xf0\x02\x08\x10\n\x0f\n\x07\x04\x01\x04\x01\
    \x02\x03\x01\x12\x04\xf0\x02\x08\x0b\n\x0f\n\x07\x04\x01\x04\x01\x02\x03\
    \x02\x12\x04\xf0\x02\x0e\x0f\n\x0e\n\x06\x04\x01\x04\x01\x02\x04\x12\x04\
    \xf1\x02\x08\x12\n\x0f\n\x07\x04\x01\x04\x01\x02\x04\x01\x12\x04\xf1\x02\
    \x08\r\n\x0f\n\x07\x04\x01\x04\x01\x02\x04\x02\x12\x04\xf1\x02\x10\x11\n\
    m\n\x04\x04\x01\x02\0\x12\x04\xf5\x02\x02\x10\x1a_\x20key\x20is\x20the\
    \x20first\x20key\x20for\x20the\x20range.\x20If\x20range_end\x20is\x20not\
    \x20given,\x20the\x20request\x20only\x20looks\x20up\x20key.\n\n\x0f\n\
    \x05\x04\x01\x02\0\x04\x12\x06\xf5\x02\x02\xf2\x02\x03\n\r\n\x05\x04\x01\
    \x02\0\x05\x12\x04\xf5\x02\x02\x07\n\r\n\x05\x04\x01\x02\0\x01\x12\x04\
    \xf5\x02\x08\x0b\n\r\n\x05\x04\x01\x02\0\x03\x12\x04\xf5\x02\x0e\x0f\n\
    \xda\x02\n\x04\x04\x01\x02\x01\x12\x04\xfb\x02\x02\x16\x1a\xcb\x02\x20ra\
    nge_end\x20is\x20the\x20upper\x20bound\x20on\x20the\x20requested\x20rang\
    e\x20[key,\x20range_end).\n\x20If\x20range_end\x20is\x20'\\0',\x20the\
    \x20range\x20is\x20all\x20keys\x20>=\x20key.\n\x20If\x20range_end\x20is\
    \x20key\x20plus\x20one\x20(e.g.,\x20\"aa\"+1\x20==\x20\"ab\",\x20\"a\\xf\
    f\"+1\x20==\x20\"b\"),\n\x20then\x20the\x20range\x20request\x20gets\x20a\
    ll\x20keys\x20prefixed\x20with\x20key.\n\x20If\x20both\x20key\x20and\x20\
    range_end\x20are\x20'\\0',\x20then\x20the\x20range\x20request\x20returns\
    \x20all\x20keys.\n\n\x0f\n\x05\x04\x01\x02\x01\x04\x12\x06\xfb\x02\x02\
    \xf5\x02\x10\n\r\n\x05\x04\x01\x02\x01\x05\x12\x04\xfb\x02\x02\x07\n\r\n\
    \x05\x04\x01\x02\x01\x01\x12\x04\xfb\x02\x08\x11\n\r\n\x05\x04\x01\x02\
    \x01\x03\x12\x04\xfb\x02\x14\x15\n\x84\x01\n\x04\x04\x01\x02\x02\x12\x04\
    \xfe\x02\x02\x12\x1av\x20limit\x20is\x20a\x20limit\x20on\x20the\x20numbe\
    r\x20of\x20keys\x20returned\x20for\x20the\x20request.\x20When\x20limit\
    \x20is\x20set\x20to\x200,\n\x20it\x20is\x20treated\x20as\x20no\x20limit.\
    \n\n\x0f\n\x05\x04\x01\x02\x02\x04\x12\x06\xfe\x02\x02\xfb\x02\x16\n\r\n\
    \x05\x04\x01\x02\x02\x05\x12\x04\xfe\x02\x02\x07\n\r\n\x05\x04\x01\x02\
    \x02\x01\x12\x04\xfe\x02\x08\r\n\r\n\x05\x04\x01\x02\x02\x03\x12\x04\xfe\
    \x02\x10\x11\n\xfd\x01\n\x04\x04\x01\x02\x03\x12\x04\x82\x03\x02\x15\x1a\
    \xee\x01\x20revision\x20is\x20the\x20point-in-time\x20of\x20the\x20key-v\
    alue\x20store\x20to\x20use\x20for\x20the\x20range.\n\x20If\x20revision\
    \x20is\x20less\x20or\x20equal\x20to\x20zero,\x20the\x20range\x20is\x20ov\
    er\x20the\x20newest\x20key-value\x20store.\n\x20If\x20the\x20revision\
    \x20has\x20been\x20compacted,\x20ErrCompacted\x20is\x20returned\x20as\
    \x20a\x20response.\n\n\x0f\n\x05\x04\x01\x02\x03\x04\x12\x06\x82\x03\x02\
    \xfe\x02\x12\n\r\n\x05\x04\x01\x02\x03\x05\x12\x04\x82\x03\x02\x07\n\r\n\
    \x05\x04\x01\x02\x03\x01\x12\x04\x82\x03\x08\x10\n\r\n\x05\x04\x01\x02\
    \x03\x03\x12\x04\x82\x03\x13\x14\nD\n\x04\x04\x01\x02\x04\x12\x04\x85\
    \x03\x02\x1b\x1a6\x20sort_order\x20is\x20the\x20order\x20for\x20returned\
    \x20sorted\x20results.\n\n\x0f\n\x05\x04\x01\x02\x04\x04\x12\x06\x85\x03\
    \x02\x82\x03\x15\n\r\n\x05\x04\x01\x02\x04\x06\x12\x04\x85\x03\x02\x0b\n\
    \r\n\x05\x04\x01\x02\x04\x01\x12\x04\x85\x03\x0c\x16\n\r\n\x05\x04\x01\
    \x02\x04\x03\x12\x04\x85\x03\x19\x1a\nF\n\x04\x04\x01\x02\x05\x12\x04\
    \x88\x03\x02\x1d\x1a8\x20sort_target\x20is\x20the\x20key-value\x20field\
    \x20to\x20use\x20for\x20sorting.\n\n\x0f\n\x05\x04\x01\x02\x05\x04\x12\
    \x06\x88\x03\x02\x85\x03\x1b\n\r\n\x05\x04\x01\x02\x05\x06\x12\x04\x88\
    \x03\x02\x0c\n\r\n\x05\x04\x01\x02\x05\x01\x12\x04\x88\x03\r\x18\n\r\n\
    \x05\x04\x01\x02\x05\x03\x12\x04\x88\x03\x1b\x1c\n\xca\x03\n\x04\x04\x01\
    \x02\x06\x12\x04\x90\x03\x02\x18\x1a\xbb\x03\x20serializable\x20sets\x20\
    the\x20range\x20request\x20to\x20use\x20serializable\x20member-local\x20\
    reads.\n\x20Range\x20requests\x20are\x20linearizable\x20by\x20default;\
    \x20linearizable\x20requests\x20have\x20higher\n\x20latency\x20and\x20lo\
    wer\x20throughput\x20than\x20serializable\x20requests\x20but\x20reflect\
    \x20the\x20current\n\x20consensus\x20of\x20the\x20cluster.\x20For\x20bet\
    ter\x20performance,\x20in\x20exchange\x20for\x20possible\x20stale\x20rea\
    ds,\n\x20a\x20serializable\x20range\x20request\x20is\x20served\x20locall\
    y\x20without\x20needing\x20to\x20reach\x20consensus\n\x20with\x20other\
    \x20nodes\x20in\x20the\x20cluster.\n\n\x0f\n\x05\x04\x01\x02\x06\x04\x12\
    \x06\x90\x03\x02\x88\x03\x1d\n\r\n\x05\x04\x01\x02\x06\x05\x12\x04\x90\
    \x03\x02\x06\n\r\n\x05\x04\x01\x02\x06\x01\x12\x04\x90\x03\x07\x13\n\r\n\
    \x05\x04\x01\x02\x06\x03\x12\x04\x90\x03\x16\x17\nL\n\x04\x04\x01\x02\
    \x07\x12\x04\x93\x03\x02\x15\x1a>\x20keys_only\x20when\x20set\x20returns\
    \x20only\x20the\x20keys\x20and\x20not\x20the\x20values.\n\n\x0f\n\x05\
    \x04\x01\x02\x07\x04\x12\x06\x93\x03\x02\x90\x03\x18\n\r\n\x05\x04\x01\
    \x02\x07\x05\x12\x04\x93\x03\x02\x06\n\r\n\x05\x04\x01\x02\x07\x01\x12\
    \x04\x93\x03\x07\x10\n\r\n\x05\x04\x01\x02\x07\x03\x12\x04\x93\x03\x13\
    \x14\nT\n\x04\x04\x01\x02\x08\x12\x04\x96\x03\x02\x16\x1aF\x20count_only\
    \x20when\x20set\x20returns\x20only\x20the\x20count\x20of\x20the\x20keys\
    \x20in\x20the\x20range.\n\n\x0f\n\x05\x04\x01\x02\x08\x04\x12\x06\x96\
    \x03\x02\x93\x03\x15\n\r\n\x05\x04\x01\x02\x08\x05\x12\x04\x96\x03\x02\
    \x06\n\r\n\x05\x04\x01\x02\x08\x01\x12\x04\x96\x03\x07\x11\n\r\n\x05\x04\
    \x01\x02\x08\x03\x12\x04\x96\x03\x14\x15\n\x8f\x01\n\x04\x04\x01\x02\t\
    \x12\x04\x9a\x03\x02\x1e\x1a\x80\x01\x20min_mod_revision\x20is\x20the\
    \x20lower\x20bound\x20for\x20returned\x20key\x20mod\x20revisions;\x20all\
    \x20keys\x20with\n\x20lesser\x20mod\x20revisions\x20will\x20be\x20filter\
    ed\x20away.\n\n\x0f\n\x05\x04\x01\x02\t\x04\x12\x06\x9a\x03\x02\x96\x03\
    \x16\n\r\n\x05\x04\x01\x02\t\x05\x12\x04\x9a\x03\x02\x07\n\r\n\x05\x04\
    \x01\x02\t\x01\x12\x04\x9a\x03\x08\x18\n\r\n\x05\x04\x01\x02\t\x03\x12\
    \x04\x9a\x03\x1b\x1d\n\x90\x01\n\x04\x04\x01\x02\n\x12\x04\x9e\x03\x02\
    \x1e\x1a\x81\x01\x20max_mod_revision\x20is\x20the\x20upper\x20bound\x20f\
    or\x20returned\x20key\x20mod\x20revisions;\x20all\x20keys\x20with\n\x20g\
    reater\x20mod\x20revisions\x20will\x20be\x20filtered\x20away.\n\n\x0f\n\
    \x05\x04\x01\x02\n\x04\x12\x06\x9e\x03\x02\x9a\x03\x1e\n\r\n\x05\x04\x01\
    \x02\n\x05\x12\x04\x9e\x03\x02\x07\n\r\n\x05\x04\x01\x02\n\x01\x12\x04\
    \x9e\x03\x08\x18\n\r\n\x05\x04\x01\x02\n\x03\x12\x04\x9e\x03\x1b\x1d\n\
    \x99\x01\n\x04\x04\x01\x02\x0b\x12\x04\xa2\x03\x02!\x1a\x8a\x01\x20min_c\
    reate_revision\x20is\x20the\x20lower\x20bound\x20for\x20returned\x20key\
    \x20create\x20revisions;\x20all\x20keys\x20with\n\x20lesser\x20create\
    \x20trevisions\x20will\x20be\x20filtered\x20away.\n\n\x0f\n\x05\x04\x01\
    \x02\x0b\x04\x12\x06\xa2\x03\x02\x9e\x03\x1e\n\r\n\x05\x04\x01\x02\x0b\
    \x05\x12\x04\xa2\x03\x02\x07\n\r\n\x05\x04\x01\x02\x0b\x01\x12\x04\xa2\
    \x03\x08\x1b\n\r\n\x05\x04\x01\x02\x0b\x03\x12\x04\xa2\x03\x1e\x20\n\x99\
    \x01\n\x04\x04\x01\x02\x0c\x12\x04\xa6\x03\x02!\x1a\x8a\x01\x20max_creat\
    e_revision\x20is\x20the\x20upper\x20bound\x20for\x20returned\x20key\x20c\
    reate\x20revisions;\x20all\x20keys\x20with\n\x20greater\x20create\x20rev\
    isions\x20will\x20be\x20filtered\x20away.\n\n\x0f\n\x05\x04\x01\x02\x0c\
    \x04\x12\x06\xa6\x03\x02\xa2\x03!\n\r\n\x05\x04\x01\x02\x0c\x05\x12\x04\
    \xa6\x03\x02\x07\n\r\n\x05\x04\x01\x02\x0c\x01\x12\x04\xa6\x03\x08\x1b\n\
    \r\n\x05\x04\x01\x02\x0c\x03\x12\x04\xa6\x03\x1e\x20\n\x0c\n\x02\x04\x02\
    \x12\x06\xa9\x03\0\xb2\x03\x01\n\x0b\n\x03\x04\x02\x01\x12\x04\xa9\x03\
    \x08\x15\n\x0c\n\x04\x04\x02\x02\0\x12\x04\xaa\x03\x02\x1c\n\x0f\n\x05\
    \x04\x02\x02\0\x04\x12\x06\xaa\x03\x02\xa9\x03\x17\n\r\n\x05\x04\x02\x02\
    \0\x06\x12\x04\xaa\x03\x02\x10\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\xaa\
    \x03\x11\x17\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\xaa\x03\x1a\x1b\nw\n\
    \x04\x04\x02\x02\x01\x12\x04\xad\x03\x02#\x1ai\x20kvs\x20is\x20the\x20li\
    st\x20of\x20key-value\x20pairs\x20matched\x20by\x20the\x20range\x20reque\
    st.\n\x20kvs\x20is\x20empty\x20when\x20count\x20is\x20requested.\n\n\r\n\
    \x05\x04\x02\x02\x01\x04\x12\x04\xad\x03\x02\n\n\r\n\x05\x04\x02\x02\x01\
    \x06\x12\x04\xad\x03\x0b\x1a\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\xad\
    \x03\x1b\x1e\n\r\n\x05\x04\x02\x02\x01\x03\x12\x04\xad\x03!\"\nW\n\x04\
    \x04\x02\x02\x02\x12\x04\xaf\x03\x02\x10\x1aI\x20more\x20indicates\x20if\
    \x20there\x20are\x20more\x20keys\x20to\x20return\x20in\x20the\x20request\
    ed\x20range.\n\n\x0f\n\x05\x04\x02\x02\x02\x04\x12\x06\xaf\x03\x02\xad\
    \x03#\n\r\n\x05\x04\x02\x02\x02\x05\x12\x04\xaf\x03\x02\x06\n\r\n\x05\
    \x04\x02\x02\x02\x01\x12\x04\xaf\x03\x07\x0b\n\r\n\x05\x04\x02\x02\x02\
    \x03\x12\x04\xaf\x03\x0e\x0f\nS\n\x04\x04\x02\x02\x03\x12\x04\xb1\x03\
    \x02\x12\x1aE\x20count\x20is\x20set\x20to\x20the\x20number\x20of\x20keys\
    \x20within\x20the\x20range\x20when\x20requested.\n\n\x0f\n\x05\x04\x02\
    \x02\x03\x04\x12\x06\xb1\x03\x02\xaf\x03\x10\n\r\n\x05\x04\x02\x02\x03\
    \x05\x12\x04\xb1\x03\x02\x07\n\r\n\x05\x04\x02\x02\x03\x01\x12\x04\xb1\
    \x03\x08\r\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\xb1\x03\x10\x11\n\x0c\n\
    \x02\x04\x03\x12\x06\xb4\x03\0\xc8\x03\x01\n\x0b\n\x03\x04\x03\x01\x12\
    \x04\xb4\x03\x08\x12\nJ\n\x04\x04\x03\x02\0\x12\x04\xb6\x03\x02\x10\x1a<\
    \x20key\x20is\x20the\x20key,\x20in\x20bytes,\x20to\x20put\x20into\x20the\
    \x20key-value\x20store.\n\n\x0f\n\x05\x04\x03\x02\0\x04\x12\x06\xb6\x03\
    \x02\xb4\x03\x14\n\r\n\x05\x04\x03\x02\0\x05\x12\x04\xb6\x03\x02\x07\n\r\
    \n\x05\x04\x03\x02\0\x01\x12\x04\xb6\x03\x08\x0b\n\r\n\x05\x04\x03\x02\0\
    \x03\x12\x04\xb6\x03\x0e\x0f\n_\n\x04\x04\x03\x02\x01\x12\x04\xb8\x03\
    \x02\x12\x1aQ\x20value\x20is\x20the\x20value,\x20in\x20bytes,\x20to\x20a\
    ssociate\x20with\x20the\x20key\x20in\x20the\x20key-value\x20store.\n\n\
    \x0f\n\x05\x04\x03\x02\x01\x04\x12\x06\xb8\x03\x02\xb6\x03\x10\n\r\n\x05\
    \x04\x03\x02\x01\x05\x12\x04\xb8\x03\x02\x07\n\r\n\x05\x04\x03\x02\x01\
    \x01\x12\x04\xb8\x03\x08\r\n\r\n\x05\x04\x03\x02\x01\x03\x12\x04\xb8\x03\
    \x10\x11\n\x7f\n\x04\x04\x03\x02\x02\x12\x04\xbb\x03\x02\x12\x1aq\x20lea\
    se\x20is\x20the\x20lease\x20ID\x20to\x20associate\x20with\x20the\x20key\
    \x20in\x20the\x20key-value\x20store.\x20A\x20lease\n\x20value\x20of\x200\
    \x20indicates\x20no\x20lease.\n\n\x0f\n\x05\x04\x03\x02\x02\x04\x12\x06\
    \xbb\x03\x02\xb8\x03\x12\n\r\n\x05\x04\x03\x02\x02\x05\x12\x04\xbb\x03\
    \x02\x07\n\r\n\x05\x04\x03\x02\x02\x01\x12\x04\xbb\x03\x08\r\n\r\n\x05\
    \x04\x03\x02\x02\x03\x12\x04\xbb\x03\x10\x11\n\xa0\x01\n\x04\x04\x03\x02\
    \x03\x12\x04\xbf\x03\x02\x13\x1a\x91\x01\x20If\x20prev_kv\x20is\x20set,\
    \x20etcd\x20gets\x20the\x20previous\x20key-value\x20pair\x20before\x20ch\
    anging\x20it.\n\x20The\x20previous\x20key-value\x20pair\x20will\x20be\
    \x20returned\x20in\x20the\x20put\x20response.\n\n\x0f\n\x05\x04\x03\x02\
    \x03\x04\x12\x06\xbf\x03\x02\xbb\x03\x12\n\r\n\x05\x04\x03\x02\x03\x05\
    \x12\x04\xbf\x03\x02\x06\n\r\n\x05\x04\x03\x02\x03\x01\x12\x04\xbf\x03\
    \x07\x0e\n\r\n\x05\x04\x03\x02\x03\x03\x12\x04\xbf\x03\x11\x12\n\x82\x01\
    \n\x04\x04\x03\x02\x04\x12\x04\xc3\x03\x02\x18\x1at\x20If\x20ignore_valu\
    e\x20is\x20set,\x20etcd\x20updates\x20the\x20key\x20using\x20its\x20curr\
    ent\x20value.\n\x20Returns\x20an\x20error\x20if\x20the\x20key\x20does\
    \x20not\x20exist.\n\n\x0f\n\x05\x04\x03\x02\x04\x04\x12\x06\xc3\x03\x02\
    \xbf\x03\x13\n\r\n\x05\x04\x03\x02\x04\x05\x12\x04\xc3\x03\x02\x06\n\r\n\
    \x05\x04\x03\x02\x04\x01\x12\x04\xc3\x03\x07\x13\n\r\n\x05\x04\x03\x02\
    \x04\x03\x12\x04\xc3\x03\x16\x17\n\x82\x01\n\x04\x04\x03\x02\x05\x12\x04\
    \xc7\x03\x02\x18\x1at\x20If\x20ignore_lease\x20is\x20set,\x20etcd\x20upd\
    ates\x20the\x20key\x20using\x20its\x20current\x20lease.\n\x20Returns\x20\
    an\x20error\x20if\x20the\x20key\x20does\x20not\x20exist.\n\n\x0f\n\x05\
    \x04\x03\x02\x05\x04\x12\x06\xc7\x03\x02\xc3\x03\x18\n\r\n\x05\x04\x03\
    \x02\x05\x05\x12\x04\xc7\x03\x02\x06\n\r\n\x05\x04\x03\x02\x05\x01\x12\
    \x04\xc7\x03\x07\x13\n\r\n\x05\x04\x03\x02\x05\x03\x12\x04\xc7\x03\x16\
    \x17\n\x0c\n\x02\x04\x04\x12\x06\xca\x03\0\xce\x03\x01\n\x0b\n\x03\x04\
    \x04\x01\x12\x04\xca\x03\x08\x13\n\x0c\n\x04\x04\x04\x02\0\x12\x04\xcb\
    \x03\x02\x1c\n\x0f\n\x05\x04\x04\x02\0\x04\x12\x06\xcb\x03\x02\xca\x03\
    \x15\n\r\n\x05\x04\x04\x02\0\x06\x12\x04\xcb\x03\x02\x10\n\r\n\x05\x04\
    \x04\x02\0\x01\x12\x04\xcb\x03\x11\x17\n\r\n\x05\x04\x04\x02\0\x03\x12\
    \x04\xcb\x03\x1a\x1b\n_\n\x04\x04\x04\x02\x01\x12\x04\xcd\x03\x02\x1e\
    \x1aQ\x20if\x20prev_kv\x20is\x20set\x20in\x20the\x20request,\x20the\x20p\
    revious\x20key-value\x20pair\x20will\x20be\x20returned.\n\n\x0f\n\x05\
    \x04\x04\x02\x01\x04\x12\x06\xcd\x03\x02\xcb\x03\x1c\n\r\n\x05\x04\x04\
    \x02\x01\x06\x12\x04\xcd\x03\x02\x11\n\r\n\x05\x04\x04\x02\x01\x01\x12\
    \x04\xcd\x03\x12\x19\n\r\n\x05\x04\x04\x02\x01\x03\x12\x04\xcd\x03\x1c\
    \x1d\n\x0c\n\x02\x04\x05\x12\x06\xd0\x03\0\xdd\x03\x01\n\x0b\n\x03\x04\
    \x05\x01\x12\x04\xd0\x03\x08\x1a\n<\n\x04\x04\x05\x02\0\x12\x04\xd2\x03\
    \x02\x10\x1a.\x20key\x20is\x20the\x20first\x20key\x20to\x20delete\x20in\
    \x20the\x20range.\n\n\x0f\n\x05\x04\x05\x02\0\x04\x12\x06\xd2\x03\x02\
    \xd0\x03\x1c\n\r\n\x05\x04\x05\x02\0\x05\x12\x04\xd2\x03\x02\x07\n\r\n\
    \x05\x04\x05\x02\0\x01\x12\x04\xd2\x03\x08\x0b\n\r\n\x05\x04\x05\x02\0\
    \x03\x12\x04\xd2\x03\x0e\x0f\n\x86\x03\n\x04\x04\x05\x02\x01\x12\x04\xd8\
    \x03\x02\x16\x1a\xf7\x02\x20range_end\x20is\x20the\x20key\x20following\
    \x20the\x20last\x20key\x20to\x20delete\x20for\x20the\x20range\x20[key,\
    \x20range_end).\n\x20If\x20range_end\x20is\x20not\x20given,\x20the\x20ra\
    nge\x20is\x20defined\x20to\x20contain\x20only\x20the\x20key\x20argument.\
    \n\x20If\x20range_end\x20is\x20one\x20bit\x20larger\x20than\x20the\x20gi\
    ven\x20key,\x20then\x20the\x20range\x20is\x20all\x20the\x20keys\n\x20wit\
    h\x20the\x20prefix\x20(the\x20given\x20key).\n\x20If\x20range_end\x20is\
    \x20'\\0',\x20the\x20range\x20is\x20all\x20keys\x20greater\x20than\x20or\
    \x20equal\x20to\x20the\x20key\x20argument.\n\n\x0f\n\x05\x04\x05\x02\x01\
    \x04\x12\x06\xd8\x03\x02\xd2\x03\x10\n\r\n\x05\x04\x05\x02\x01\x05\x12\
    \x04\xd8\x03\x02\x07\n\r\n\x05\x04\x05\x02\x01\x01\x12\x04\xd8\x03\x08\
    \x11\n\r\n\x05\x04\x05\x02\x01\x03\x12\x04\xd8\x03\x14\x15\n\xa5\x01\n\
    \x04\x04\x05\x02\x02\x12\x04\xdc\x03\x02\x13\x1a\x96\x01\x20If\x20prev_k\
    v\x20is\x20set,\x20etcd\x20gets\x20the\x20previous\x20key-value\x20pairs\
    \x20before\x20deleting\x20it.\n\x20The\x20previous\x20key-value\x20pairs\
    \x20will\x20be\x20returned\x20in\x20the\x20delete\x20response.\n\n\x0f\n\
    \x05\x04\x05\x02\x02\x04\x12\x06\xdc\x03\x02\xd8\x03\x16\n\r\n\x05\x04\
    \x05\x02\x02\x05\x12\x04\xdc\x03\x02\x06\n\r\n\x05\x04\x05\x02\x02\x01\
    \x12\x04\xdc\x03\x07\x0e\n\r\n\x05\x04\x05\x02\x02\x03\x12\x04\xdc\x03\
    \x11\x12\n\x0c\n\x02\x04\x06\x12\x06\xdf\x03\0\xe5\x03\x01\n\x0b\n\x03\
    \x04\x06\x01\x12\x04\xdf\x03\x08\x1b\n\x0c\n\x04\x04\x06\x02\0\x12\x04\
    \xe0\x03\x02\x1c\n\x0f\n\x05\x04\x06\x02\0\x04\x12\x06\xe0\x03\x02\xdf\
    \x03\x1d\n\r\n\x05\x04\x06\x02\0\x06\x12\x04\xe0\x03\x02\x10\n\r\n\x05\
    \x04\x06\x02\0\x01\x12\x04\xe0\x03\x11\x17\n\r\n\x05\x04\x06\x02\0\x03\
    \x12\x04\xe0\x03\x1a\x1b\nR\n\x04\x04\x06\x02\x01\x12\x04\xe2\x03\x02\
    \x14\x1aD\x20deleted\x20is\x20the\x20number\x20of\x20keys\x20deleted\x20\
    by\x20the\x20delete\x20range\x20request.\n\n\x0f\n\x05\x04\x06\x02\x01\
    \x04\x12\x06\xe2\x03\x02\xe0\x03\x1c\n\r\n\x05\x04\x06\x02\x01\x05\x12\
    \x04\xe2\x03\x02\x07\n\r\n\x05\x04\x06\x02\x01\x01\x12\x04\xe2\x03\x08\
    \x0f\n\r\n\x05\x04\x06\x02\x01\x03\x12\x04\xe2\x03\x12\x13\n`\n\x04\x04\
    \x06\x02\x02\x12\x04\xe4\x03\x02(\x1aR\x20if\x20prev_kv\x20is\x20set\x20\
    in\x20the\x20request,\x20the\x20previous\x20key-value\x20pairs\x20will\
    \x20be\x20returned.\n\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04\xe4\x03\x02\
    \n\n\r\n\x05\x04\x06\x02\x02\x06\x12\x04\xe4\x03\x0b\x1a\n\r\n\x05\x04\
    \x06\x02\x02\x01\x12\x04\xe4\x03\x1b#\n\r\n\x05\x04\x06\x02\x02\x03\x12\
    \x04\xe4\x03&'\n\x0c\n\x02\x04\x07\x12\x06\xe7\x03\0\xef\x03\x01\n\x0b\n\
    \x03\x04\x07\x01\x12\x04\xe7\x03\x08\x11\nP\n\x04\x04\x07\x08\0\x12\x06\
    \xe9\x03\x02\xee\x03\x03\x1a@\x20request\x20is\x20a\x20union\x20of\x20re\
    quest\x20types\x20accepted\x20by\x20a\x20transaction.\n\n\r\n\x05\x04\
    \x07\x08\0\x01\x12\x04\xe9\x03\x08\x0f\n\x0c\n\x04\x04\x07\x02\0\x12\x04\
    \xea\x03\x04#\n\r\n\x05\x04\x07\x02\0\x06\x12\x04\xea\x03\x04\x10\n\r\n\
    \x05\x04\x07\x02\0\x01\x12\x04\xea\x03\x11\x1e\n\r\n\x05\x04\x07\x02\0\
    \x03\x12\x04\xea\x03!\"\n\x0c\n\x04\x04\x07\x02\x01\x12\x04\xeb\x03\x04\
    \x1f\n\r\n\x05\x04\x07\x02\x01\x06\x12\x04\xeb\x03\x04\x0e\n\r\n\x05\x04\
    \x07\x02\x01\x01\x12\x04\xeb\x03\x0f\x1a\n\r\n\x05\x04\x07\x02\x01\x03\
    \x12\x04\xeb\x03\x1d\x1e\n\x0c\n\x04\x04\x07\x02\x02\x12\x04\xec\x03\x04\
    0\n\r\n\x05\x04\x07\x02\x02\x06\x12\x04\xec\x03\x04\x16\n\r\n\x05\x04\
    \x07\x02\x02\x01\x12\x04\xec\x03\x17+\n\r\n\x05\x04\x07\x02\x02\x03\x12\
    \x04\xec\x03./\n\x0c\n\x04\x04\x07\x02\x03\x12\x04\xed\x03\x04\x1f\n\r\n\
    \x05\x04\x07\x02\x03\x06\x12\x04\xed\x03\x04\x0e\n\r\n\x05\x04\x07\x02\
    \x03\x01\x12\x04\xed\x03\x0f\x1a\n\r\n\x05\x04\x07\x02\x03\x03\x12\x04\
    \xed\x03\x1d\x1e\n\x0c\n\x02\x04\x08\x12\x06\xf1\x03\0\xf9\x03\x01\n\x0b\
    \n\x03\x04\x08\x01\x12\x04\xf1\x03\x08\x12\nR\n\x04\x04\x08\x08\0\x12\
    \x06\xf3\x03\x02\xf8\x03\x03\x1aB\x20response\x20is\x20a\x20union\x20of\
    \x20response\x20types\x20returned\x20by\x20a\x20transaction.\n\n\r\n\x05\
    \x04\x08\x08\0\x01\x12\x04\xf3\x03\x08\x10\n\x0c\n\x04\x04\x08\x02\0\x12\
    \x04\xf4\x03\x04%\n\r\n\x05\x04\x08\x02\0\x06\x12\x04\xf4\x03\x04\x11\n\
    \r\n\x05\x04\x08\x02\0\x01\x12\x04\xf4\x03\x12\x20\n\r\n\x05\x04\x08\x02\
    \0\x03\x12\x04\xf4\x03#$\n\x0c\n\x04\x04\x08\x02\x01\x12\x04\xf5\x03\x04\
    !\n\r\n\x05\x04\x08\x02\x01\x06\x12\x04\xf5\x03\x04\x0f\n\r\n\x05\x04\
    \x08\x02\x01\x01\x12\x04\xf5\x03\x10\x1c\n\r\n\x05\x04\x08\x02\x01\x03\
    \x12\x04\xf5\x03\x1f\x20\n\x0c\n\x04\x04\x08\x02\x02\x12\x04\xf6\x03\x04\
    2\n\r\n\x05\x04\x08\x02\x02\x06\x12\x04\xf6\x03\x04\x17\n\r\n\x05\x04\
    \x08\x02\x02\x01\x12\x04\xf6\x03\x18-\n\r\n\x05\x04\x08\x02\x02\x03\x12\
    \x04\xf6\x0301\n\x0c\n\x04\x04\x08\x02\x03\x12\x04\xf7\x03\x04!\n\r\n\
    \x05\x04\x08\x02\x03\x06\x12\x04\xf7\x03\x04\x0f\n\r\n\x05\x04\x08\x02\
    \x03\x01\x12\x04\xf7\x03\x10\x1c\n\r\n\x05\x04\x08\x02\x03\x03\x12\x04\
    \xf7\x03\x1f\x20\n\x0c\n\x02\x04\t\x12\x06\xfb\x03\0\xa1\x04\x01\n\x0b\n\
    \x03\x04\t\x01\x12\x04\xfb\x03\x08\x0f\n\x0e\n\x04\x04\t\x04\0\x12\x06\
    \xfc\x03\x02\x81\x04\x03\n\r\n\x05\x04\t\x04\0\x01\x12\x04\xfc\x03\x07\
    \x14\n\x0e\n\x06\x04\t\x04\0\x02\0\x12\x04\xfd\x03\x04\x0e\n\x0f\n\x07\
    \x04\t\x04\0\x02\0\x01\x12\x04\xfd\x03\x04\t\n\x0f\n\x07\x04\t\x04\0\x02\
    \0\x02\x12\x04\xfd\x03\x0c\r\n\x0e\n\x06\x04\t\x04\0\x02\x01\x12\x04\xfe\
    \x03\x04\x10\n\x0f\n\x07\x04\t\x04\0\x02\x01\x01\x12\x04\xfe\x03\x04\x0b\
    \n\x0f\n\x07\x04\t\x04\0\x02\x01\x02\x12\x04\xfe\x03\x0e\x0f\n\x0e\n\x06\
    \x04\t\x04\0\x02\x02\x12\x04\xff\x03\x04\r\n\x0f\n\x07\x04\t\x04\0\x02\
    \x02\x01\x12\x04\xff\x03\x04\x08\n\x0f\n\x07\x04\t\x04\0\x02\x02\x02\x12\
    \x04\xff\x03\x0b\x0c\n\x0e\n\x06\x04\t\x04\0\x02\x03\x12\x04\x80\x04\x04\
    \x12\n\x0f\n\x07\x04\t\x04\0\x02\x03\x01\x12\x04\x80\x04\x04\r\n\x0f\n\
    \x07\x04\t\x04\0\x02\x03\x02\x12\x04\x80\x04\x10\x11\n\x0e\n\x04\x04\t\
    \x04\x01\x12\x06\x82\x04\x02\x88\x04\x03\n\r\n\x05\x04\t\x04\x01\x01\x12\
    \x04\x82\x04\x07\x14\n\x0e\n\x06\x04\t\x04\x01\x02\0\x12\x04\x83\x04\x04\
    \x10\n\x0f\n\x07\x04\t\x04\x01\x02\0\x01\x12\x04\x83\x04\x04\x0b\n\x0f\n\
    \x07\x04\t\x04\x01\x02\0\x02\x12\x04\x83\x04\x0e\x0f\n\x0e\n\x06\x04\t\
    \x04\x01\x02\x01\x12\x04\x84\x04\x04\x0f\n\x0f\n\x07\x04\t\x04\x01\x02\
    \x01\x01\x12\x04\x84\x04\x04\n\n\x0f\n\x07\x04\t\x04\x01\x02\x01\x02\x12\
    \x04\x84\x04\r\x0e\n\x0e\n\x06\x04\t\x04\x01\x02\x02\x12\x04\x85\x04\x04\
    \x0c\n\x0f\n\x07\x04\t\x04\x01\x02\x02\x01\x12\x04\x85\x04\x04\x07\n\x0f\
    \n\x07\x04\t\x04\x01\x02\x02\x02\x12\x04\x85\x04\n\x0b\n\x0e\n\x06\x04\t\
    \x04\x01\x02\x03\x12\x04\x86\x04\x04\r\n\x0f\n\x07\x04\t\x04\x01\x02\x03\
    \x01\x12\x04\x86\x04\x04\t\n\x0f\n\x07\x04\t\x04\x01\x02\x03\x02\x12\x04\
    \x86\x04\x0b\x0c\n\x0e\n\x06\x04\t\x04\x01\x02\x04\x12\x04\x87\x04\x04\
    \x0e\n\x0f\n\x07\x04\t\x04\x01\x02\x04\x01\x12\x04\x87\x04\x04\t\n\x0f\n\
    \x07\x04\t\x04\x01\x02\x04\x02\x12\x04\x87\x04\x0c\r\nK\n\x04\x04\t\x02\
    \0\x12\x04\x8a\x04\x02\x1b\x1a=\x20result\x20is\x20logical\x20comparison\
    \x20operation\x20for\x20this\x20comparison.\n\n\x0f\n\x05\x04\t\x02\0\
    \x04\x12\x06\x8a\x04\x02\x88\x04\x03\n\r\n\x05\x04\t\x02\0\x06\x12\x04\
    \x8a\x04\x02\x0f\n\r\n\x05\x04\t\x02\0\x01\x12\x04\x8a\x04\x10\x16\n\r\n\
    \x05\x04\t\x02\0\x03\x12\x04\x8a\x04\x19\x1a\nL\n\x04\x04\t\x02\x01\x12\
    \x04\x8c\x04\x02\x1b\x1a>\x20target\x20is\x20the\x20key-value\x20field\
    \x20to\x20inspect\x20for\x20the\x20comparison.\n\n\x0f\n\x05\x04\t\x02\
    \x01\x04\x12\x06\x8c\x04\x02\x8a\x04\x1b\n\r\n\x05\x04\t\x02\x01\x06\x12\
    \x04\x8c\x04\x02\x0f\n\r\n\x05\x04\t\x02\x01\x01\x12\x04\x8c\x04\x10\x16\
    \n\r\n\x05\x04\t\x02\x01\x03\x12\x04\x8c\x04\x19\x1a\nD\n\x04\x04\t\x02\
    \x02\x12\x04\x8e\x04\x02\x10\x1a6\x20key\x20is\x20the\x20subject\x20key\
    \x20for\x20the\x20comparison\x20operation.\n\n\x0f\n\x05\x04\t\x02\x02\
    \x04\x12\x06\x8e\x04\x02\x8c\x04\x1b\n\r\n\x05\x04\t\x02\x02\x05\x12\x04\
    \x8e\x04\x02\x07\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\x8e\x04\x08\x0b\n\r\
    \n\x05\x04\t\x02\x02\x03\x12\x04\x8e\x04\x0e\x0f\n\x0e\n\x04\x04\t\x08\0\
    \x12\x06\x8f\x04\x02\x9b\x04\x03\n\r\n\x05\x04\t\x08\0\x01\x12\x04\x8f\
    \x04\x08\x14\n7\n\x04\x04\t\x02\x03\x12\x04\x91\x04\x04\x16\x1a)\x20vers\
    ion\x20is\x20the\x20version\x20of\x20the\x20given\x20key\n\n\r\n\x05\x04\
    \t\x02\x03\x05\x12\x04\x91\x04\x04\t\n\r\n\x05\x04\t\x02\x03\x01\x12\x04\
    \x91\x04\n\x11\n\r\n\x05\x04\t\x02\x03\x03\x12\x04\x91\x04\x14\x15\nI\n\
    \x04\x04\t\x02\x04\x12\x04\x93\x04\x04\x1e\x1a;\x20create_revision\x20is\
    \x20the\x20creation\x20revision\x20of\x20the\x20given\x20key\n\n\r\n\x05\
    \x04\t\x02\x04\x05\x12\x04\x93\x04\x04\t\n\r\n\x05\x04\t\x02\x04\x01\x12\
    \x04\x93\x04\n\x19\n\r\n\x05\x04\t\x02\x04\x03\x12\x04\x93\x04\x1c\x1d\n\
    L\n\x04\x04\t\x02\x05\x12\x04\x95\x04\x04\x1b\x1a>\x20mod_revision\x20is\
    \x20the\x20last\x20modified\x20revision\x20of\x20the\x20given\x20key.\n\
    \n\r\n\x05\x04\t\x02\x05\x05\x12\x04\x95\x04\x04\t\n\r\n\x05\x04\t\x02\
    \x05\x01\x12\x04\x95\x04\n\x16\n\r\n\x05\x04\t\x02\x05\x03\x12\x04\x95\
    \x04\x19\x1a\n>\n\x04\x04\t\x02\x06\x12\x04\x97\x04\x04\x14\x1a0\x20valu\
    e\x20is\x20the\x20value\x20of\x20the\x20given\x20key,\x20in\x20bytes.\n\
    \n\r\n\x05\x04\t\x02\x06\x05\x12\x04\x97\x04\x04\t\n\r\n\x05\x04\t\x02\
    \x06\x01\x12\x04\x97\x04\n\x0f\n\r\n\x05\x04\t\x02\x06\x03\x12\x04\x97\
    \x04\x12\x13\nr\n\x04\x04\t\x02\x07\x12\x04\x99\x04\x04\x14\x1a)\x20leas\
    e\x20is\x20the\x20lease\x20id\x20of\x20the\x20given\x20key.\n\"9\x20leav\
    e\x20room\x20for\x20more\x20target_union\x20field\x20tags,\x20jump\x20to\
    \x2064\n\n\r\n\x05\x04\t\x02\x07\x05\x12\x04\x99\x04\x04\t\n\r\n\x05\x04\
    \t\x02\x07\x01\x12\x04\x99\x04\n\x0f\n\r\n\x05\x04\t\x02\x07\x03\x12\x04\
    \x99\x04\x12\x13\n\xdd\x01\n\x04\x04\t\x02\x08\x12\x04\x9f\x04\x02\x17\
    \x1a\x82\x01\x20range_end\x20compares\x20the\x20given\x20target\x20to\
    \x20all\x20keys\x20in\x20the\x20range\x20[key,\x20range_end).\n\x20See\
    \x20RangeRequest\x20for\x20more\x20details\x20on\x20key\x20ranges.\n\"J\
    \x20TODO:\x20fill\x20out\x20with\x20most\x20of\x20the\x20rest\x20of\x20R\
    angeRequest\x20fields\x20when\x20needed.\n\n\x0f\n\x05\x04\t\x02\x08\x04\
    \x12\x06\x9f\x04\x02\x9b\x04\x03\n\r\n\x05\x04\t\x02\x08\x05\x12\x04\x9f\
    \x04\x02\x07\n\r\n\x05\x04\t\x02\x08\x01\x12\x04\x9f\x04\x08\x11\n\r\n\
    \x05\x04\t\x02\x08\x03\x12\x04\x9f\x04\x14\x16\n\x89\t\n\x02\x04\n\x12\
    \x06\xb2\x04\0\xbd\x04\x01\x1a\xfa\x08\x20From\x20google\x20paxosdb\x20p\
    aper:\n\x20Our\x20implementation\x20hinges\x20around\x20a\x20powerful\
    \x20primitive\x20which\x20we\x20call\x20MultiOp.\x20All\x20other\x20data\
    base\n\x20operations\x20except\x20for\x20iteration\x20are\x20implemented\
    \x20as\x20a\x20single\x20call\x20to\x20MultiOp.\x20A\x20MultiOp\x20is\
    \x20applied\x20atomically\n\x20and\x20consists\x20of\x20three\x20compone\
    nts:\n\x201.\x20A\x20list\x20of\x20tests\x20called\x20guard.\x20Each\x20\
    test\x20in\x20guard\x20checks\x20a\x20single\x20entry\x20in\x20the\x20da\
    tabase.\x20It\x20may\x20check\n\x20for\x20the\x20absence\x20or\x20presen\
    ce\x20of\x20a\x20value,\x20or\x20compare\x20with\x20a\x20given\x20value.\
    \x20Two\x20different\x20tests\x20in\x20the\x20guard\n\x20may\x20apply\
    \x20to\x20the\x20same\x20or\x20different\x20entries\x20in\x20the\x20data\
    base.\x20All\x20tests\x20in\x20the\x20guard\x20are\x20applied\x20and\n\
    \x20MultiOp\x20returns\x20the\x20results.\x20If\x20all\x20tests\x20are\
    \x20true,\x20MultiOp\x20executes\x20t\x20op\x20(see\x20item\x202\x20belo\
    w),\x20otherwise\n\x20it\x20executes\x20f\x20op\x20(see\x20item\x203\x20\
    below).\n\x202.\x20A\x20list\x20of\x20database\x20operations\x20called\
    \x20t\x20op.\x20Each\x20operation\x20in\x20the\x20list\x20is\x20either\
    \x20an\x20insert,\x20delete,\x20or\n\x20lookup\x20operation,\x20and\x20a\
    pplies\x20to\x20a\x20single\x20database\x20entry.\x20Two\x20different\
    \x20operations\x20in\x20the\x20list\x20may\x20apply\n\x20to\x20the\x20sa\
    me\x20or\x20different\x20entries\x20in\x20the\x20database.\x20These\x20o\
    perations\x20are\x20executed\n\x20if\x20guard\x20evaluates\x20to\n\x20tr\
    ue.\n\x203.\x20A\x20list\x20of\x20database\x20operations\x20called\x20f\
    \x20op.\x20Like\x20t\x20op,\x20but\x20executed\x20if\x20guard\x20evaluat\
    es\x20to\x20false.\n\n\x0b\n\x03\x04\n\x01\x12\x04\xb2\x04\x08\x12\n\x80\
    \x03\n\x04\x04\n\x02\0\x12\x04\xb8\x04\x02\x1f\x1a\xf1\x02\x20compare\
    \x20is\x20a\x20list\x20of\x20predicates\x20representing\x20a\x20conjunct\
    ion\x20of\x20terms.\n\x20If\x20the\x20comparisons\x20succeed,\x20then\
    \x20the\x20success\x20requests\x20will\x20be\x20processed\x20in\x20order\
    ,\n\x20and\x20the\x20response\x20will\x20contain\x20their\x20respective\
    \x20responses\x20in\x20order.\n\x20If\x20the\x20comparisons\x20fail,\x20\
    then\x20the\x20failure\x20requests\x20will\x20be\x20processed\x20in\x20o\
    rder,\n\x20and\x20the\x20response\x20will\x20contain\x20their\x20respect\
    ive\x20responses\x20in\x20order.\n\n\r\n\x05\x04\n\x02\0\x04\x12\x04\xb8\
    \x04\x02\n\n\r\n\x05\x04\n\x02\0\x06\x12\x04\xb8\x04\x0b\x12\n\r\n\x05\
    \x04\n\x02\0\x01\x12\x04\xb8\x04\x13\x1a\n\r\n\x05\x04\n\x02\0\x03\x12\
    \x04\xb8\x04\x1d\x1e\nc\n\x04\x04\n\x02\x01\x12\x04\xba\x04\x02!\x1aU\
    \x20success\x20is\x20a\x20list\x20of\x20requests\x20which\x20will\x20be\
    \x20applied\x20when\x20compare\x20evaluates\x20to\x20true.\n\n\r\n\x05\
    \x04\n\x02\x01\x04\x12\x04\xba\x04\x02\n\n\r\n\x05\x04\n\x02\x01\x06\x12\
    \x04\xba\x04\x0b\x14\n\r\n\x05\x04\n\x02\x01\x01\x12\x04\xba\x04\x15\x1c\
    \n\r\n\x05\x04\n\x02\x01\x03\x12\x04\xba\x04\x1f\x20\nd\n\x04\x04\n\x02\
    \x02\x12\x04\xbc\x04\x02!\x1aV\x20failure\x20is\x20a\x20list\x20of\x20re\
    quests\x20which\x20will\x20be\x20applied\x20when\x20compare\x20evaluates\
    \x20to\x20false.\n\n\r\n\x05\x04\n\x02\x02\x04\x12\x04\xbc\x04\x02\n\n\r\
    \n\x05\x04\n\x02\x02\x06\x12\x04\xbc\x04\x0b\x14\n\r\n\x05\x04\n\x02\x02\
    \x01\x12\x04\xbc\x04\x15\x1c\n\r\n\x05\x04\n\x02\x02\x03\x12\x04\xbc\x04\
    \x1f\x20\n\x0c\n\x02\x04\x0b\x12\x06\xbf\x04\0\xc6\x04\x01\n\x0b\n\x03\
    \x04\x0b\x01\x12\x04\xbf\x04\x08\x13\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\
    \xc0\x04\x02\x1c\n\x0f\n\x05\x04\x0b\x02\0\x04\x12\x06\xc0\x04\x02\xbf\
    \x04\x15\n\r\n\x05\x04\x0b\x02\0\x06\x12\x04\xc0\x04\x02\x10\n\r\n\x05\
    \x04\x0b\x02\0\x01\x12\x04\xc0\x04\x11\x17\n\r\n\x05\x04\x0b\x02\0\x03\
    \x12\x04\xc0\x04\x1a\x1b\n]\n\x04\x04\x0b\x02\x01\x12\x04\xc2\x04\x02\
    \x15\x1aO\x20succeeded\x20is\x20set\x20to\x20true\x20if\x20the\x20compar\
    e\x20evaluated\x20to\x20true\x20or\x20false\x20otherwise.\n\n\x0f\n\x05\
    \x04\x0b\x02\x01\x04\x12\x06\xc2\x04\x02\xc0\x04\x1c\n\r\n\x05\x04\x0b\
    \x02\x01\x05\x12\x04\xc2\x04\x02\x06\n\r\n\x05\x04\x0b\x02\x01\x01\x12\
    \x04\xc2\x04\x07\x10\n\r\n\x05\x04\x0b\x02\x01\x03\x12\x04\xc2\x04\x13\
    \x14\n\x9c\x01\n\x04\x04\x0b\x02\x02\x12\x04\xc5\x04\x02$\x1a\x8d\x01\
    \x20responses\x20is\x20a\x20list\x20of\x20responses\x20corresponding\x20\
    to\x20the\x20results\x20from\x20applying\n\x20success\x20if\x20succeeded\
    \x20is\x20true\x20or\x20failure\x20if\x20succeeded\x20is\x20false.\n\n\r\
    \n\x05\x04\x0b\x02\x02\x04\x12\x04\xc5\x04\x02\n\n\r\n\x05\x04\x0b\x02\
    \x02\x06\x12\x04\xc5\x04\x0b\x15\n\r\n\x05\x04\x0b\x02\x02\x01\x12\x04\
    \xc5\x04\x16\x1f\n\r\n\x05\x04\x0b\x02\x02\x03\x12\x04\xc5\x04\"#\n\xaf\
    \x01\n\x02\x04\x0c\x12\x06\xca\x04\0\xd1\x04\x01\x1a\xa0\x01\x20Compacti\
    onRequest\x20compacts\x20the\x20key-value\x20store\x20up\x20to\x20a\x20g\
    iven\x20revision.\x20All\x20superseded\x20keys\n\x20with\x20a\x20revisio\
    n\x20less\x20than\x20the\x20compaction\x20revision\x20will\x20be\x20remo\
    ved.\n\n\x0b\n\x03\x04\x0c\x01\x12\x04\xca\x04\x08\x19\nV\n\x04\x04\x0c\
    \x02\0\x12\x04\xcc\x04\x02\x15\x1aH\x20revision\x20is\x20the\x20key-valu\
    e\x20store\x20revision\x20for\x20the\x20compaction\x20operation.\n\n\x0f\
    \n\x05\x04\x0c\x02\0\x04\x12\x06\xcc\x04\x02\xca\x04\x1b\n\r\n\x05\x04\
    \x0c\x02\0\x05\x12\x04\xcc\x04\x02\x07\n\r\n\x05\x04\x0c\x02\0\x01\x12\
    \x04\xcc\x04\x08\x10\n\r\n\x05\x04\x0c\x02\0\x03\x12\x04\xcc\x04\x13\x14\
    \n\xc3\x01\n\x04\x04\x0c\x02\x01\x12\x04\xd0\x04\x02\x14\x1a\xb4\x01\x20\
    physical\x20is\x20set\x20so\x20the\x20RPC\x20will\x20wait\x20until\x20th\
    e\x20compaction\x20is\x20physically\n\x20applied\x20to\x20the\x20local\
    \x20database\x20such\x20that\x20compacted\x20entries\x20are\x20totally\n\
    \x20removed\x20from\x20the\x20backend\x20database.\n\n\x0f\n\x05\x04\x0c\
    \x02\x01\x04\x12\x06\xd0\x04\x02\xcc\x04\x15\n\r\n\x05\x04\x0c\x02\x01\
    \x05\x12\x04\xd0\x04\x02\x06\n\r\n\x05\x04\x0c\x02\x01\x01\x12\x04\xd0\
    \x04\x07\x0f\n\r\n\x05\x04\x0c\x02\x01\x03\x12\x04\xd0\x04\x12\x13\n\x0c\
    \n\x02\x04\r\x12\x06\xd3\x04\0\xd5\x04\x01\n\x0b\n\x03\x04\r\x01\x12\x04\
    \xd3\x04\x08\x1a\n\x0c\n\x04\x04\r\x02\0\x12\x04\xd4\x04\x02\x1c\n\x0f\n\
    \x05\x04\r\x02\0\x04\x12\x06\xd4\x04\x02\xd3\x04\x1c\n\r\n\x05\x04\r\x02\
    \0\x06\x12\x04\xd4\x04\x02\x10\n\r\n\x05\x04\r\x02\0\x01\x12\x04\xd4\x04\
    \x11\x17\n\r\n\x05\x04\r\x02\0\x03\x12\x04\xd4\x04\x1a\x1b\n\x0c\n\x02\
    \x04\x0e\x12\x06\xd7\x04\0\xd8\x04\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\
    \xd7\x04\x08\x13\n\x0c\n\x02\x04\x0f\x12\x06\xda\x04\0\xdd\x04\x01\n\x0b\
    \n\x03\x04\x0f\x01\x12\x04\xda\x04\x08\x15\nP\n\x04\x04\x0f\x02\0\x12\
    \x04\xdc\x04\x02\x15\x1aB\x20revision\x20is\x20the\x20key-value\x20store\
    \x20revision\x20for\x20the\x20hash\x20operation.\n\n\x0f\n\x05\x04\x0f\
    \x02\0\x04\x12\x06\xdc\x04\x02\xda\x04\x17\n\r\n\x05\x04\x0f\x02\0\x05\
    \x12\x04\xdc\x04\x02\x07\n\r\n\x05\x04\x0f\x02\0\x01\x12\x04\xdc\x04\x08\
    \x10\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\xdc\x04\x13\x14\n\x0c\n\x02\x04\
    \x10\x12\x06\xdf\x04\0\xe5\x04\x01\n\x0b\n\x03\x04\x10\x01\x12\x04\xdf\
    \x04\x08\x16\n\x0c\n\x04\x04\x10\x02\0\x12\x04\xe0\x04\x02\x1c\n\x0f\n\
    \x05\x04\x10\x02\0\x04\x12\x06\xe0\x04\x02\xdf\x04\x18\n\r\n\x05\x04\x10\
    \x02\0\x06\x12\x04\xe0\x04\x02\x10\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\
    \xe0\x04\x11\x17\n\r\n\x05\x04\x10\x02\0\x03\x12\x04\xe0\x04\x1a\x1b\nn\
    \n\x04\x04\x10\x02\x01\x12\x04\xe2\x04\x02\x12\x1a`\x20hash\x20is\x20the\
    \x20hash\x20value\x20computed\x20from\x20the\x20responding\x20member's\
    \x20MVCC\x20keys\x20up\x20to\x20a\x20given\x20revision.\n\n\x0f\n\x05\
    \x04\x10\x02\x01\x04\x12\x06\xe2\x04\x02\xe0\x04\x1c\n\r\n\x05\x04\x10\
    \x02\x01\x05\x12\x04\xe2\x04\x02\x08\n\r\n\x05\x04\x10\x02\x01\x01\x12\
    \x04\xe2\x04\t\r\n\r\n\x05\x04\x10\x02\x01\x03\x12\x04\xe2\x04\x10\x11\n\
    _\n\x04\x04\x10\x02\x02\x12\x04\xe4\x04\x02\x1d\x1aQ\x20compact_revision\
    \x20is\x20the\x20compacted\x20revision\x20of\x20key-value\x20store\x20wh\
    en\x20hash\x20begins.\n\n\x0f\n\x05\x04\x10\x02\x02\x04\x12\x06\xe4\x04\
    \x02\xe2\x04\x12\n\r\n\x05\x04\x10\x02\x02\x05\x12\x04\xe4\x04\x02\x07\n\
    \r\n\x05\x04\x10\x02\x02\x01\x12\x04\xe4\x04\x08\x18\n\r\n\x05\x04\x10\
    \x02\x02\x03\x12\x04\xe4\x04\x1b\x1c\n\x0c\n\x02\x04\x11\x12\x06\xe7\x04\
    \0\xeb\x04\x01\n\x0b\n\x03\x04\x11\x01\x12\x04\xe7\x04\x08\x14\n\x0c\n\
    \x04\x04\x11\x02\0\x12\x04\xe8\x04\x02\x1c\n\x0f\n\x05\x04\x11\x02\0\x04\
    \x12\x06\xe8\x04\x02\xe7\x04\x16\n\r\n\x05\x04\x11\x02\0\x06\x12\x04\xe8\
    \x04\x02\x10\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\xe8\x04\x11\x17\n\r\n\
    \x05\x04\x11\x02\0\x03\x12\x04\xe8\x04\x1a\x1b\nZ\n\x04\x04\x11\x02\x01\
    \x12\x04\xea\x04\x02\x12\x1aL\x20hash\x20is\x20the\x20hash\x20value\x20c\
    omputed\x20from\x20the\x20responding\x20member's\x20KV's\x20backend.\n\n\
    \x0f\n\x05\x04\x11\x02\x01\x04\x12\x06\xea\x04\x02\xe8\x04\x1c\n\r\n\x05\
    \x04\x11\x02\x01\x05\x12\x04\xea\x04\x02\x08\n\r\n\x05\x04\x11\x02\x01\
    \x01\x12\x04\xea\x04\t\r\n\r\n\x05\x04\x11\x02\x01\x03\x12\x04\xea\x04\
    \x10\x11\n\x0c\n\x02\x04\x12\x12\x06\xed\x04\0\xee\x04\x01\n\x0b\n\x03\
    \x04\x12\x01\x12\x04\xed\x04\x08\x17\n\x0c\n\x02\x04\x13\x12\x06\xf0\x04\
    \0\xfa\x04\x01\n\x0b\n\x03\x04\x13\x01\x12\x04\xf0\x04\x08\x18\n\x9a\x01\
    \n\x04\x04\x13\x02\0\x12\x04\xf3\x04\x02\x1c\x1a\x8b\x01\x20header\x20ha\
    s\x20the\x20current\x20key-value\x20store\x20information.\x20The\x20firs\
    t\x20header\x20in\x20the\x20snapshot\n\x20stream\x20indicates\x20the\x20\
    point\x20in\x20time\x20of\x20the\x20snapshot.\n\n\x0f\n\x05\x04\x13\x02\
    \0\x04\x12\x06\xf3\x04\x02\xf0\x04\x1a\n\r\n\x05\x04\x13\x02\0\x06\x12\
    \x04\xf3\x04\x02\x10\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\xf3\x04\x11\x17\
    \n\r\n\x05\x04\x13\x02\0\x03\x12\x04\xf3\x04\x1a\x1b\nY\n\x04\x04\x13\
    \x02\x01\x12\x04\xf6\x04\x02\x1d\x1aK\x20remaining_bytes\x20is\x20the\
    \x20number\x20of\x20blob\x20bytes\x20to\x20be\x20sent\x20after\x20this\
    \x20message\n\n\x0f\n\x05\x04\x13\x02\x01\x04\x12\x06\xf6\x04\x02\xf3\
    \x04\x1c\n\r\n\x05\x04\x13\x02\x01\x05\x12\x04\xf6\x04\x02\x08\n\r\n\x05\
    \x04\x13\x02\x01\x01\x12\x04\xf6\x04\t\x18\n\r\n\x05\x04\x13\x02\x01\x03\
    \x12\x04\xf6\x04\x1b\x1c\nT\n\x04\x04\x13\x02\x02\x12\x04\xf9\x04\x02\
    \x11\x1aF\x20blob\x20contains\x20the\x20next\x20chunk\x20of\x20the\x20sn\
    apshot\x20in\x20the\x20snapshot\x20stream.\n\n\x0f\n\x05\x04\x13\x02\x02\
    \x04\x12\x06\xf9\x04\x02\xf6\x04\x1d\n\r\n\x05\x04\x13\x02\x02\x05\x12\
    \x04\xf9\x04\x02\x07\n\r\n\x05\x04\x13\x02\x02\x01\x12\x04\xf9\x04\x08\
    \x0c\n\r\n\x05\x04\x13\x02\x02\x03\x12\x04\xf9\x04\x0f\x10\n\x0c\n\x02\
    \x04\x14\x12\x06\xfc\x04\0\x82\x05\x01\n\x0b\n\x03\x04\x14\x01\x12\x04\
    \xfc\x04\x08\x14\nj\n\x04\x04\x14\x08\0\x12\x06\xfe\x04\x02\x81\x05\x03\
    \x1aZ\x20request_union\x20is\x20a\x20request\x20to\x20either\x20create\
    \x20a\x20new\x20watcher\x20or\x20cancel\x20an\x20existing\x20watcher.\n\
    \n\r\n\x05\x04\x14\x08\0\x01\x12\x04\xfe\x04\x08\x15\n\x0c\n\x04\x04\x14\
    \x02\0\x12\x04\xff\x04\x04*\n\r\n\x05\x04\x14\x02\0\x06\x12\x04\xff\x04\
    \x04\x16\n\r\n\x05\x04\x14\x02\0\x01\x12\x04\xff\x04\x17%\n\r\n\x05\x04\
    \x14\x02\0\x03\x12\x04\xff\x04()\n\x0c\n\x04\x04\x14\x02\x01\x12\x04\x80\
    \x05\x04*\n\r\n\x05\x04\x14\x02\x01\x06\x12\x04\x80\x05\x04\x16\n\r\n\
    \x05\x04\x14\x02\x01\x01\x12\x04\x80\x05\x17%\n\r\n\x05\x04\x14\x02\x01\
    \x03\x12\x04\x80\x05()\n\x0c\n\x02\x04\x15\x12\x06\x84\x05\0\xa1\x05\x01\
    \n\x0b\n\x03\x04\x15\x01\x12\x04\x84\x05\x08\x1a\n8\n\x04\x04\x15\x02\0\
    \x12\x04\x86\x05\x02\x10\x1a*\x20key\x20is\x20the\x20key\x20to\x20regist\
    er\x20for\x20watching.\n\n\x0f\n\x05\x04\x15\x02\0\x04\x12\x06\x86\x05\
    \x02\x84\x05\x1c\n\r\n\x05\x04\x15\x02\0\x05\x12\x04\x86\x05\x02\x07\n\r\
    \n\x05\x04\x15\x02\0\x01\x12\x04\x86\x05\x08\x0b\n\r\n\x05\x04\x15\x02\0\
    \x03\x12\x04\x86\x05\x0e\x0f\n\xe3\x02\n\x04\x04\x15\x02\x01\x12\x04\x8c\
    \x05\x02\x16\x1a\xd4\x02\x20range_end\x20is\x20the\x20end\x20of\x20the\
    \x20range\x20[key,\x20range_end)\x20to\x20watch.\x20If\x20range_end\x20i\
    s\x20not\x20given,\n\x20only\x20the\x20key\x20argument\x20is\x20watched.\
    \x20If\x20range_end\x20is\x20equal\x20to\x20'\\0',\x20all\x20keys\x20gre\
    ater\x20than\n\x20or\x20equal\x20to\x20the\x20key\x20argument\x20are\x20\
    watched.\n\x20If\x20the\x20range_end\x20is\x20one\x20bit\x20larger\x20th\
    an\x20the\x20given\x20key,\n\x20then\x20all\x20keys\x20with\x20the\x20pr\
    efix\x20(the\x20given\x20key)\x20will\x20be\x20watched.\n\n\x0f\n\x05\
    \x04\x15\x02\x01\x04\x12\x06\x8c\x05\x02\x86\x05\x10\n\r\n\x05\x04\x15\
    \x02\x01\x05\x12\x04\x8c\x05\x02\x07\n\r\n\x05\x04\x15\x02\x01\x01\x12\
    \x04\x8c\x05\x08\x11\n\r\n\x05\x04\x15\x02\x01\x03\x12\x04\x8c\x05\x14\
    \x15\nm\n\x04\x04\x15\x02\x02\x12\x04\x8e\x05\x02\x1b\x1a_\x20start_revi\
    sion\x20is\x20an\x20optional\x20revision\x20to\x20watch\x20from\x20(incl\
    usive).\x20No\x20start_revision\x20is\x20\"now\".\n\n\x0f\n\x05\x04\x15\
    \x02\x02\x04\x12\x06\x8e\x05\x02\x8c\x05\x16\n\r\n\x05\x04\x15\x02\x02\
    \x05\x12\x04\x8e\x05\x02\x07\n\r\n\x05\x04\x15\x02\x02\x01\x12\x04\x8e\
    \x05\x08\x16\n\r\n\x05\x04\x15\x02\x02\x03\x12\x04\x8e\x05\x19\x1a\n\xe9\
    \x02\n\x04\x04\x15\x02\x03\x12\x04\x93\x05\x02\x1b\x1a\xda\x02\x20progre\
    ss_notify\x20is\x20set\x20so\x20that\x20the\x20etcd\x20server\x20will\
    \x20periodically\x20send\x20a\x20WatchResponse\x20with\n\x20no\x20events\
    \x20to\x20the\x20new\x20watcher\x20if\x20there\x20are\x20no\x20recent\
    \x20events.\x20It\x20is\x20useful\x20when\x20clients\n\x20wish\x20to\x20\
    recover\x20a\x20disconnected\x20watcher\x20starting\x20from\x20a\x20rece\
    nt\x20known\x20revision.\n\x20The\x20etcd\x20server\x20may\x20decide\x20\
    how\x20often\x20it\x20will\x20send\x20notifications\x20based\x20on\x20cu\
    rrent\x20load.\n\n\x0f\n\x05\x04\x15\x02\x03\x04\x12\x06\x93\x05\x02\x8e\
    \x05\x1b\n\r\n\x05\x04\x15\x02\x03\x05\x12\x04\x93\x05\x02\x06\n\r\n\x05\
    \x04\x15\x02\x03\x01\x12\x04\x93\x05\x07\x16\n\r\n\x05\x04\x15\x02\x03\
    \x03\x12\x04\x93\x05\x19\x1a\n\x0e\n\x04\x04\x15\x04\0\x12\x06\x95\x05\
    \x02\x9a\x05\x03\n\r\n\x05\x04\x15\x04\0\x01\x12\x04\x95\x05\x07\x11\n'\
    \n\x06\x04\x15\x04\0\x02\0\x12\x04\x97\x05\x02\x0c\x1a\x17\x20filter\x20\
    out\x20put\x20event.\n\n\x0f\n\x07\x04\x15\x04\0\x02\0\x01\x12\x04\x97\
    \x05\x02\x07\n\x0f\n\x07\x04\x15\x04\0\x02\0\x02\x12\x04\x97\x05\n\x0b\n\
    *\n\x06\x04\x15\x04\0\x02\x01\x12\x04\x99\x05\x02\x0f\x1a\x1a\x20filter\
    \x20out\x20delete\x20event.\n\n\x0f\n\x07\x04\x15\x04\0\x02\x01\x01\x12\
    \x04\x99\x05\x02\n\n\x0f\n\x07\x04\x15\x04\0\x02\x01\x02\x12\x04\x99\x05\
    \r\x0e\n]\n\x04\x04\x15\x02\x04\x12\x04\x9c\x05\x02\"\x1aO\x20filters\
    \x20filter\x20the\x20events\x20at\x20server\x20side\x20before\x20it\x20s\
    ends\x20back\x20to\x20the\x20watcher.\n\n\r\n\x05\x04\x15\x02\x04\x04\
    \x12\x04\x9c\x05\x02\n\n\r\n\x05\x04\x15\x02\x04\x06\x12\x04\x9c\x05\x0b\
    \x15\n\r\n\x05\x04\x15\x02\x04\x01\x12\x04\x9c\x05\x16\x1d\n\r\n\x05\x04\
    \x15\x02\x04\x03\x12\x04\x9c\x05\x20!\n\xa6\x01\n\x04\x04\x15\x02\x05\
    \x12\x04\xa0\x05\x02\x13\x1a\x97\x01\x20If\x20prev_kv\x20is\x20set,\x20c\
    reated\x20watcher\x20gets\x20the\x20previous\x20KV\x20before\x20the\x20e\
    vent\x20happens.\n\x20If\x20the\x20previous\x20KV\x20is\x20already\x20co\
    mpacted,\x20nothing\x20will\x20be\x20returned.\n\n\x0f\n\x05\x04\x15\x02\
    \x05\x04\x12\x06\xa0\x05\x02\x9c\x05\"\n\r\n\x05\x04\x15\x02\x05\x05\x12\
    \x04\xa0\x05\x02\x06\n\r\n\x05\x04\x15\x02\x05\x01\x12\x04\xa0\x05\x07\
    \x0e\n\r\n\x05\x04\x15\x02\x05\x03\x12\x04\xa0\x05\x11\x12\n\x0c\n\x02\
    \x04\x16\x12\x06\xa3\x05\0\xa6\x05\x01\n\x0b\n\x03\x04\x16\x01\x12\x04\
    \xa3\x05\x08\x1a\n\\\n\x04\x04\x16\x02\0\x12\x04\xa5\x05\x02\x15\x1aN\
    \x20watch_id\x20is\x20the\x20watcher\x20id\x20to\x20cancel\x20so\x20that\
    \x20no\x20more\x20events\x20are\x20transmitted.\n\n\x0f\n\x05\x04\x16\
    \x02\0\x04\x12\x06\xa5\x05\x02\xa3\x05\x1c\n\r\n\x05\x04\x16\x02\0\x05\
    \x12\x04\xa5\x05\x02\x07\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\xa5\x05\x08\
    \x10\n\r\n\x05\x04\x16\x02\0\x03\x12\x04\xa5\x05\x13\x14\n\x0c\n\x02\x04\
    \x17\x12\x06\xa8\x05\0\xc2\x05\x01\n\x0b\n\x03\x04\x17\x01\x12\x04\xa8\
    \x05\x08\x15\n\x0c\n\x04\x04\x17\x02\0\x12\x04\xa9\x05\x02\x1c\n\x0f\n\
    \x05\x04\x17\x02\0\x04\x12\x06\xa9\x05\x02\xa8\x05\x17\n\r\n\x05\x04\x17\
    \x02\0\x06\x12\x04\xa9\x05\x02\x10\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\
    \xa9\x05\x11\x17\n\r\n\x05\x04\x17\x02\0\x03\x12\x04\xa9\x05\x1a\x1b\nS\
    \n\x04\x04\x17\x02\x01\x12\x04\xab\x05\x02\x15\x1aE\x20watch_id\x20is\
    \x20the\x20ID\x20of\x20the\x20watcher\x20that\x20corresponds\x20to\x20th\
    e\x20response.\n\n\x0f\n\x05\x04\x17\x02\x01\x04\x12\x06\xab\x05\x02\xa9\
    \x05\x1c\n\r\n\x05\x04\x17\x02\x01\x05\x12\x04\xab\x05\x02\x07\n\r\n\x05\
    \x04\x17\x02\x01\x01\x12\x04\xab\x05\x08\x10\n\r\n\x05\x04\x17\x02\x01\
    \x03\x12\x04\xab\x05\x13\x14\n\x95\x02\n\x04\x04\x17\x02\x02\x12\x04\xb0\
    \x05\x02\x13\x1a\x86\x02\x20created\x20is\x20set\x20to\x20true\x20if\x20\
    the\x20response\x20is\x20for\x20a\x20create\x20watch\x20request.\n\x20Th\
    e\x20client\x20should\x20record\x20the\x20watch_id\x20and\x20expect\x20t\
    o\x20receive\x20events\x20for\n\x20the\x20created\x20watcher\x20from\x20\
    the\x20same\x20stream.\n\x20All\x20events\x20sent\x20to\x20the\x20create\
    d\x20watcher\x20will\x20attach\x20with\x20the\x20same\x20watch_id.\n\n\
    \x0f\n\x05\x04\x17\x02\x02\x04\x12\x06\xb0\x05\x02\xab\x05\x15\n\r\n\x05\
    \x04\x17\x02\x02\x05\x12\x04\xb0\x05\x02\x06\n\r\n\x05\x04\x17\x02\x02\
    \x01\x12\x04\xb0\x05\x07\x0e\n\r\n\x05\x04\x17\x02\x02\x03\x12\x04\xb0\
    \x05\x11\x12\n\x90\x01\n\x04\x04\x17\x02\x03\x12\x04\xb3\x05\x02\x14\x1a\
    \x81\x01\x20canceled\x20is\x20set\x20to\x20true\x20if\x20the\x20response\
    \x20is\x20for\x20a\x20cancel\x20watch\x20request.\n\x20No\x20further\x20\
    events\x20will\x20be\x20sent\x20to\x20the\x20canceled\x20watcher.\n\n\
    \x0f\n\x05\x04\x17\x02\x03\x04\x12\x06\xb3\x05\x02\xb0\x05\x13\n\r\n\x05\
    \x04\x17\x02\x03\x05\x12\x04\xb3\x05\x02\x06\n\r\n\x05\x04\x17\x02\x03\
    \x01\x12\x04\xb3\x05\x07\x0f\n\r\n\x05\x04\x17\x02\x03\x03\x12\x04\xb3\
    \x05\x12\x13\n\xf9\x02\n\x04\x04\x17\x02\x04\x12\x04\xbc\x05\x02\x1e\x1a\
    \xea\x02\x20compact_revision\x20is\x20set\x20to\x20the\x20minimum\x20ind\
    ex\x20if\x20a\x20watcher\x20tries\x20to\x20watch\n\x20at\x20a\x20compact\
    ed\x20index.\n\n\x20This\x20happens\x20when\x20creating\x20a\x20watcher\
    \x20at\x20a\x20compacted\x20revision\x20or\x20the\x20watcher\x20cannot\n\
    \x20catch\x20up\x20with\x20the\x20progress\x20of\x20the\x20key-value\x20\
    store.\n\n\x20The\x20client\x20should\x20treat\x20the\x20watcher\x20as\
    \x20canceled\x20and\x20should\x20not\x20try\x20to\x20create\x20any\n\x20\
    watcher\x20with\x20the\x20same\x20start_revision\x20again.\n\n\x0f\n\x05\
    \x04\x17\x02\x04\x04\x12\x06\xbc\x05\x02\xb3\x05\x14\n\r\n\x05\x04\x17\
    \x02\x04\x05\x12\x04\xbc\x05\x02\x07\n\r\n\x05\x04\x17\x02\x04\x01\x12\
    \x04\xbc\x05\x08\x18\n\r\n\x05\x04\x17\x02\x04\x03\x12\x04\xbc\x05\x1c\
    \x1d\nM\n\x04\x04\x17\x02\x05\x12\x04\xbf\x05\x02\x1b\x1a?\x20cancel_rea\
    son\x20indicates\x20the\x20reason\x20for\x20canceling\x20the\x20watcher.\
    \n\n\x0f\n\x05\x04\x17\x02\x05\x04\x12\x06\xbf\x05\x02\xbc\x05\x1e\n\r\n\
    \x05\x04\x17\x02\x05\x05\x12\x04\xbf\x05\x02\x08\n\r\n\x05\x04\x17\x02\
    \x05\x01\x12\x04\xbf\x05\t\x16\n\r\n\x05\x04\x17\x02\x05\x03\x12\x04\xbf\
    \x05\x19\x1a\n\x0c\n\x04\x04\x17\x02\x06\x12\x04\xc1\x05\x02$\n\r\n\x05\
    \x04\x17\x02\x06\x04\x12\x04\xc1\x05\x02\n\n\r\n\x05\x04\x17\x02\x06\x06\
    \x12\x04\xc1\x05\x0b\x17\n\r\n\x05\x04\x17\x02\x06\x01\x12\x04\xc1\x05\
    \x18\x1e\n\r\n\x05\x04\x17\x02\x06\x03\x12\x04\xc1\x05!#\n\x0c\n\x02\x04\
    \x18\x12\x06\xc4\x05\0\xc9\x05\x01\n\x0b\n\x03\x04\x18\x01\x12\x04\xc4\
    \x05\x08\x19\nZ\n\x04\x04\x18\x02\0\x12\x04\xc6\x05\x02\x10\x1aL\x20TTL\
    \x20is\x20the\x20advisory\x20time-to-live\x20in\x20seconds.\x20Expired\
    \x20lease\x20will\x20return\x20-1.\n\n\x0f\n\x05\x04\x18\x02\0\x04\x12\
    \x06\xc6\x05\x02\xc4\x05\x1b\n\r\n\x05\x04\x18\x02\0\x05\x12\x04\xc6\x05\
    \x02\x07\n\r\n\x05\x04\x18\x02\0\x01\x12\x04\xc6\x05\x08\x0b\n\r\n\x05\
    \x04\x18\x02\0\x03\x12\x04\xc6\x05\x0e\x0f\nb\n\x04\x04\x18\x02\x01\x12\
    \x04\xc8\x05\x02\x0f\x1aT\x20ID\x20is\x20the\x20requested\x20ID\x20for\
    \x20the\x20lease.\x20If\x20ID\x20is\x20set\x20to\x200,\x20the\x20lessor\
    \x20chooses\x20an\x20ID.\n\n\x0f\n\x05\x04\x18\x02\x01\x04\x12\x06\xc8\
    \x05\x02\xc6\x05\x10\n\r\n\x05\x04\x18\x02\x01\x05\x12\x04\xc8\x05\x02\
    \x07\n\r\n\x05\x04\x18\x02\x01\x01\x12\x04\xc8\x05\x08\n\n\r\n\x05\x04\
    \x18\x02\x01\x03\x12\x04\xc8\x05\r\x0e\n\x0c\n\x02\x04\x19\x12\x06\xcb\
    \x05\0\xd2\x05\x01\n\x0b\n\x03\x04\x19\x01\x12\x04\xcb\x05\x08\x1a\n\x0c\
    \n\x04\x04\x19\x02\0\x12\x04\xcc\x05\x02\x1c\n\x0f\n\x05\x04\x19\x02\0\
    \x04\x12\x06\xcc\x05\x02\xcb\x05\x1c\n\r\n\x05\x04\x19\x02\0\x06\x12\x04\
    \xcc\x05\x02\x10\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\xcc\x05\x11\x17\n\r\
    \n\x05\x04\x19\x02\0\x03\x12\x04\xcc\x05\x1a\x1b\n9\n\x04\x04\x19\x02\
    \x01\x12\x04\xce\x05\x02\x0f\x1a+\x20ID\x20is\x20the\x20lease\x20ID\x20f\
    or\x20the\x20granted\x20lease.\n\n\x0f\n\x05\x04\x19\x02\x01\x04\x12\x06\
    \xce\x05\x02\xcc\x05\x1c\n\r\n\x05\x04\x19\x02\x01\x05\x12\x04\xce\x05\
    \x02\x07\n\r\n\x05\x04\x19\x02\x01\x01\x12\x04\xce\x05\x08\n\n\r\n\x05\
    \x04\x19\x02\x01\x03\x12\x04\xce\x05\r\x0e\nG\n\x04\x04\x19\x02\x02\x12\
    \x04\xd0\x05\x02\x10\x1a9\x20TTL\x20is\x20the\x20server\x20chosen\x20lea\
    se\x20time-to-live\x20in\x20seconds.\n\n\x0f\n\x05\x04\x19\x02\x02\x04\
    \x12\x06\xd0\x05\x02\xce\x05\x0f\n\r\n\x05\x04\x19\x02\x02\x05\x12\x04\
    \xd0\x05\x02\x07\n\r\n\x05\x04\x19\x02\x02\x01\x12\x04\xd0\x05\x08\x0b\n\
    \r\n\x05\x04\x19\x02\x02\x03\x12\x04\xd0\x05\x0e\x0f\n\x0c\n\x04\x04\x19\
    \x02\x03\x12\x04\xd1\x05\x02\x13\n\x0f\n\x05\x04\x19\x02\x03\x04\x12\x06\
    \xd1\x05\x02\xd0\x05\x10\n\r\n\x05\x04\x19\x02\x03\x05\x12\x04\xd1\x05\
    \x02\x08\n\r\n\x05\x04\x19\x02\x03\x01\x12\x04\xd1\x05\t\x0e\n\r\n\x05\
    \x04\x19\x02\x03\x03\x12\x04\xd1\x05\x11\x12\n\x0c\n\x02\x04\x1a\x12\x06\
    \xd4\x05\0\xd7\x05\x01\n\x0b\n\x03\x04\x1a\x01\x12\x04\xd4\x05\x08\x1a\n\
    j\n\x04\x04\x1a\x02\0\x12\x04\xd6\x05\x02\x0f\x1a\\\x20ID\x20is\x20the\
    \x20lease\x20ID\x20to\x20revoke.\x20When\x20the\x20ID\x20is\x20revoked,\
    \x20all\x20associated\x20keys\x20will\x20be\x20deleted.\n\n\x0f\n\x05\
    \x04\x1a\x02\0\x04\x12\x06\xd6\x05\x02\xd4\x05\x1c\n\r\n\x05\x04\x1a\x02\
    \0\x05\x12\x04\xd6\x05\x02\x07\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\xd6\
    \x05\x08\n\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\xd6\x05\r\x0e\n\x0c\n\x02\
    \x04\x1b\x12\x06\xd9\x05\0\xdb\x05\x01\n\x0b\n\x03\x04\x1b\x01\x12\x04\
    \xd9\x05\x08\x1b\n\x0c\n\x04\x04\x1b\x02\0\x12\x04\xda\x05\x02\x1c\n\x0f\
    \n\x05\x04\x1b\x02\0\x04\x12\x06\xda\x05\x02\xd9\x05\x1d\n\r\n\x05\x04\
    \x1b\x02\0\x06\x12\x04\xda\x05\x02\x10\n\r\n\x05\x04\x1b\x02\0\x01\x12\
    \x04\xda\x05\x11\x17\n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\xda\x05\x1a\x1b\
    \n\x0c\n\x02\x04\x1c\x12\x06\xdd\x05\0\xe0\x05\x01\n\x0b\n\x03\x04\x1c\
    \x01\x12\x04\xdd\x05\x08\x1d\n?\n\x04\x04\x1c\x02\0\x12\x04\xdf\x05\x02\
    \x0f\x1a1\x20ID\x20is\x20the\x20lease\x20ID\x20for\x20the\x20lease\x20to\
    \x20keep\x20alive.\n\n\x0f\n\x05\x04\x1c\x02\0\x04\x12\x06\xdf\x05\x02\
    \xdd\x05\x1f\n\r\n\x05\x04\x1c\x02\0\x05\x12\x04\xdf\x05\x02\x07\n\r\n\
    \x05\x04\x1c\x02\0\x01\x12\x04\xdf\x05\x08\n\n\r\n\x05\x04\x1c\x02\0\x03\
    \x12\x04\xdf\x05\r\x0e\n\x0c\n\x02\x04\x1d\x12\x06\xe2\x05\0\xe8\x05\x01\
    \n\x0b\n\x03\x04\x1d\x01\x12\x04\xe2\x05\x08\x1e\n\x0c\n\x04\x04\x1d\x02\
    \0\x12\x04\xe3\x05\x02\x1c\n\x0f\n\x05\x04\x1d\x02\0\x04\x12\x06\xe3\x05\
    \x02\xe2\x05\x20\n\r\n\x05\x04\x1d\x02\0\x06\x12\x04\xe3\x05\x02\x10\n\r\
    \n\x05\x04\x1d\x02\0\x01\x12\x04\xe3\x05\x11\x17\n\r\n\x05\x04\x1d\x02\0\
    \x03\x12\x04\xe3\x05\x1a\x1b\n?\n\x04\x04\x1d\x02\x01\x12\x04\xe5\x05\
    \x02\x0f\x1a1\x20ID\x20is\x20the\x20lease\x20ID\x20from\x20the\x20keep\
    \x20alive\x20request.\n\n\x0f\n\x05\x04\x1d\x02\x01\x04\x12\x06\xe5\x05\
    \x02\xe3\x05\x1c\n\r\n\x05\x04\x1d\x02\x01\x05\x12\x04\xe5\x05\x02\x07\n\
    \r\n\x05\x04\x1d\x02\x01\x01\x12\x04\xe5\x05\x08\n\n\r\n\x05\x04\x1d\x02\
    \x01\x03\x12\x04\xe5\x05\r\x0e\n:\n\x04\x04\x1d\x02\x02\x12\x04\xe7\x05\
    \x02\x10\x1a,\x20TTL\x20is\x20the\x20new\x20time-to-live\x20for\x20the\
    \x20lease.\n\n\x0f\n\x05\x04\x1d\x02\x02\x04\x12\x06\xe7\x05\x02\xe5\x05\
    \x0f\n\r\n\x05\x04\x1d\x02\x02\x05\x12\x04\xe7\x05\x02\x07\n\r\n\x05\x04\
    \x1d\x02\x02\x01\x12\x04\xe7\x05\x08\x0b\n\r\n\x05\x04\x1d\x02\x02\x03\
    \x12\x04\xe7\x05\x0e\x0f\n\x0c\n\x02\x04\x1e\x12\x06\xea\x05\0\xef\x05\
    \x01\n\x0b\n\x03\x04\x1e\x01\x12\x04\xea\x05\x08\x1e\n1\n\x04\x04\x1e\
    \x02\0\x12\x04\xec\x05\x02\x0f\x1a#\x20ID\x20is\x20the\x20lease\x20ID\
    \x20for\x20the\x20lease.\n\n\x0f\n\x05\x04\x1e\x02\0\x04\x12\x06\xec\x05\
    \x02\xea\x05\x20\n\r\n\x05\x04\x1e\x02\0\x05\x12\x04\xec\x05\x02\x07\n\r\
    \n\x05\x04\x1e\x02\0\x01\x12\x04\xec\x05\x08\n\n\r\n\x05\x04\x1e\x02\0\
    \x03\x12\x04\xec\x05\r\x0e\nJ\n\x04\x04\x1e\x02\x01\x12\x04\xee\x05\x02\
    \x10\x1a<\x20keys\x20is\x20true\x20to\x20query\x20all\x20the\x20keys\x20\
    attached\x20to\x20this\x20lease.\n\n\x0f\n\x05\x04\x1e\x02\x01\x04\x12\
    \x06\xee\x05\x02\xec\x05\x0f\n\r\n\x05\x04\x1e\x02\x01\x05\x12\x04\xee\
    \x05\x02\x06\n\r\n\x05\x04\x1e\x02\x01\x01\x12\x04\xee\x05\x07\x0b\n\r\n\
    \x05\x04\x1e\x02\x01\x03\x12\x04\xee\x05\x0e\x0f\n\x0c\n\x02\x04\x1f\x12\
    \x06\xf1\x05\0\xfb\x05\x01\n\x0b\n\x03\x04\x1f\x01\x12\x04\xf1\x05\x08\
    \x1f\n\x0c\n\x04\x04\x1f\x02\0\x12\x04\xf2\x05\x02\x1c\n\x0f\n\x05\x04\
    \x1f\x02\0\x04\x12\x06\xf2\x05\x02\xf1\x05!\n\r\n\x05\x04\x1f\x02\0\x06\
    \x12\x04\xf2\x05\x02\x10\n\r\n\x05\x04\x1f\x02\0\x01\x12\x04\xf2\x05\x11\
    \x17\n\r\n\x05\x04\x1f\x02\0\x03\x12\x04\xf2\x05\x1a\x1b\n?\n\x04\x04\
    \x1f\x02\x01\x12\x04\xf4\x05\x02\x0f\x1a1\x20ID\x20is\x20the\x20lease\
    \x20ID\x20from\x20the\x20keep\x20alive\x20request.\n\n\x0f\n\x05\x04\x1f\
    \x02\x01\x04\x12\x06\xf4\x05\x02\xf2\x05\x1c\n\r\n\x05\x04\x1f\x02\x01\
    \x05\x12\x04\xf4\x05\x02\x07\n\r\n\x05\x04\x1f\x02\x01\x01\x12\x04\xf4\
    \x05\x08\n\n\r\n\x05\x04\x1f\x02\x01\x03\x12\x04\xf4\x05\r\x0e\np\n\x04\
    \x04\x1f\x02\x02\x12\x04\xf6\x05\x02\x10\x1ab\x20TTL\x20is\x20the\x20rem\
    aining\x20TTL\x20in\x20seconds\x20for\x20the\x20lease;\x20the\x20lease\
    \x20will\x20expire\x20in\x20under\x20TTL+1\x20seconds.\n\n\x0f\n\x05\x04\
    \x1f\x02\x02\x04\x12\x06\xf6\x05\x02\xf4\x05\x0f\n\r\n\x05\x04\x1f\x02\
    \x02\x05\x12\x04\xf6\x05\x02\x07\n\r\n\x05\x04\x1f\x02\x02\x01\x12\x04\
    \xf6\x05\x08\x0b\n\r\n\x05\x04\x1f\x02\x02\x03\x12\x04\xf6\x05\x0e\x0f\n\
    ^\n\x04\x04\x1f\x02\x03\x12\x04\xf8\x05\x02\x17\x1aP\x20GrantedTTL\x20is\
    \x20the\x20initial\x20granted\x20time\x20in\x20seconds\x20upon\x20lease\
    \x20creation/renewal.\n\n\x0f\n\x05\x04\x1f\x02\x03\x04\x12\x06\xf8\x05\
    \x02\xf6\x05\x10\n\r\n\x05\x04\x1f\x02\x03\x05\x12\x04\xf8\x05\x02\x07\n\
    \r\n\x05\x04\x1f\x02\x03\x01\x12\x04\xf8\x05\x08\x12\n\r\n\x05\x04\x1f\
    \x02\x03\x03\x12\x04\xf8\x05\x15\x16\n@\n\x04\x04\x1f\x02\x04\x12\x04\
    \xfa\x05\x02\x1a\x1a2\x20Keys\x20is\x20the\x20list\x20of\x20keys\x20atta\
    ched\x20to\x20this\x20lease.\n\n\r\n\x05\x04\x1f\x02\x04\x04\x12\x04\xfa\
    \x05\x02\n\n\r\n\x05\x04\x1f\x02\x04\x05\x12\x04\xfa\x05\x0b\x10\n\r\n\
    \x05\x04\x1f\x02\x04\x01\x12\x04\xfa\x05\x11\x15\n\r\n\x05\x04\x1f\x02\
    \x04\x03\x12\x04\xfa\x05\x18\x19\n\x0c\n\x02\x04\x20\x12\x06\xfd\x05\0\
    \xfe\x05\x01\n\x0b\n\x03\x04\x20\x01\x12\x04\xfd\x05\x08\x1a\n\x0c\n\x02\
    \x04!\x12\x06\x80\x06\0\x83\x06\x01\n\x0b\n\x03\x04!\x01\x12\x04\x80\x06\
    \x08\x13\n$\n\x04\x04!\x02\0\x12\x04\x81\x06\x02\x0f\"\x16\x20TODO:\x20i\
    nt64\x20TTL\x20=\x202;\n\n\x0f\n\x05\x04!\x02\0\x04\x12\x06\x81\x06\x02\
    \x80\x06\x15\n\r\n\x05\x04!\x02\0\x05\x12\x04\x81\x06\x02\x07\n\r\n\x05\
    \x04!\x02\0\x01\x12\x04\x81\x06\x08\n\n\r\n\x05\x04!\x02\0\x03\x12\x04\
    \x81\x06\r\x0e\n\x0c\n\x02\x04\"\x12\x06\x85\x06\0\x88\x06\x01\n\x0b\n\
    \x03\x04\"\x01\x12\x04\x85\x06\x08\x1b\n\x0c\n\x04\x04\"\x02\0\x12\x04\
    \x86\x06\x02\x1c\n\x0f\n\x05\x04\"\x02\0\x04\x12\x06\x86\x06\x02\x85\x06\
    \x1d\n\r\n\x05\x04\"\x02\0\x06\x12\x04\x86\x06\x02\x10\n\r\n\x05\x04\"\
    \x02\0\x01\x12\x04\x86\x06\x11\x17\n\r\n\x05\x04\"\x02\0\x03\x12\x04\x86\
    \x06\x1a\x1b\n\x0c\n\x04\x04\"\x02\x01\x12\x04\x87\x06\x02\"\n\r\n\x05\
    \x04\"\x02\x01\x04\x12\x04\x87\x06\x02\n\n\r\n\x05\x04\"\x02\x01\x06\x12\
    \x04\x87\x06\x0b\x16\n\r\n\x05\x04\"\x02\x01\x01\x12\x04\x87\x06\x17\x1d\
    \n\r\n\x05\x04\"\x02\x01\x03\x12\x04\x87\x06\x20!\n\x0c\n\x02\x04#\x12\
    \x06\x8a\x06\0\x93\x06\x01\n\x0b\n\x03\x04#\x01\x12\x04\x8a\x06\x08\x0e\
    \n4\n\x04\x04#\x02\0\x12\x04\x8c\x06\x02\x10\x1a&\x20ID\x20is\x20the\x20\
    member\x20ID\x20for\x20this\x20member.\n\n\x0f\n\x05\x04#\x02\0\x04\x12\
    \x06\x8c\x06\x02\x8a\x06\x10\n\r\n\x05\x04#\x02\0\x05\x12\x04\x8c\x06\
    \x02\x08\n\r\n\x05\x04#\x02\0\x01\x12\x04\x8c\x06\t\x0b\n\r\n\x05\x04#\
    \x02\0\x03\x12\x04\x8c\x06\x0e\x0f\n~\n\x04\x04#\x02\x01\x12\x04\x8e\x06\
    \x02\x12\x1ap\x20name\x20is\x20the\x20human-readable\x20name\x20of\x20th\
    e\x20member.\x20If\x20the\x20member\x20is\x20not\x20started,\x20the\x20n\
    ame\x20will\x20be\x20an\x20empty\x20string.\n\n\x0f\n\x05\x04#\x02\x01\
    \x04\x12\x06\x8e\x06\x02\x8c\x06\x10\n\r\n\x05\x04#\x02\x01\x05\x12\x04\
    \x8e\x06\x02\x08\n\r\n\x05\x04#\x02\x01\x01\x12\x04\x8e\x06\t\r\n\r\n\
    \x05\x04#\x02\x01\x03\x12\x04\x8e\x06\x10\x11\na\n\x04\x04#\x02\x02\x12\
    \x04\x90\x06\x02\x1f\x1aS\x20peerURLs\x20is\x20the\x20list\x20of\x20URLs\
    \x20the\x20member\x20exposes\x20to\x20the\x20cluster\x20for\x20communica\
    tion.\n\n\r\n\x05\x04#\x02\x02\x04\x12\x04\x90\x06\x02\n\n\r\n\x05\x04#\
    \x02\x02\x05\x12\x04\x90\x06\x0b\x11\n\r\n\x05\x04#\x02\x02\x01\x12\x04\
    \x90\x06\x12\x1a\n\r\n\x05\x04#\x02\x02\x03\x12\x04\x90\x06\x1d\x1e\n\
    \x98\x01\n\x04\x04#\x02\x03\x12\x04\x92\x06\x02!\x1a\x89\x01\x20clientUR\
    Ls\x20is\x20the\x20list\x20of\x20URLs\x20the\x20member\x20exposes\x20to\
    \x20clients\x20for\x20communication.\x20If\x20the\x20member\x20is\x20not\
    \x20started,\x20clientURLs\x20will\x20be\x20empty.\n\n\r\n\x05\x04#\x02\
    \x03\x04\x12\x04\x92\x06\x02\n\n\r\n\x05\x04#\x02\x03\x05\x12\x04\x92\
    \x06\x0b\x11\n\r\n\x05\x04#\x02\x03\x01\x12\x04\x92\x06\x12\x1c\n\r\n\
    \x05\x04#\x02\x03\x03\x12\x04\x92\x06\x1f\x20\n\x0c\n\x02\x04$\x12\x06\
    \x95\x06\0\x98\x06\x01\n\x0b\n\x03\x04$\x01\x12\x04\x95\x06\x08\x18\ng\n\
    \x04\x04$\x02\0\x12\x04\x97\x06\x02\x1f\x1aY\x20peerURLs\x20is\x20the\
    \x20list\x20of\x20URLs\x20the\x20added\x20member\x20will\x20use\x20to\
    \x20communicate\x20with\x20the\x20cluster.\n\n\r\n\x05\x04$\x02\0\x04\
    \x12\x04\x97\x06\x02\n\n\r\n\x05\x04$\x02\0\x05\x12\x04\x97\x06\x0b\x11\
    \n\r\n\x05\x04$\x02\0\x01\x12\x04\x97\x06\x12\x1a\n\r\n\x05\x04$\x02\0\
    \x03\x12\x04\x97\x06\x1d\x1e\n\x0c\n\x02\x04%\x12\x06\x9a\x06\0\xa0\x06\
    \x01\n\x0b\n\x03\x04%\x01\x12\x04\x9a\x06\x08\x19\n\x0c\n\x04\x04%\x02\0\
    \x12\x04\x9b\x06\x02\x1c\n\x0f\n\x05\x04%\x02\0\x04\x12\x06\x9b\x06\x02\
    \x9a\x06\x1b\n\r\n\x05\x04%\x02\0\x06\x12\x04\x9b\x06\x02\x10\n\r\n\x05\
    \x04%\x02\0\x01\x12\x04\x9b\x06\x11\x17\n\r\n\x05\x04%\x02\0\x03\x12\x04\
    \x9b\x06\x1a\x1b\nF\n\x04\x04%\x02\x01\x12\x04\x9d\x06\x02\x14\x1a8\x20m\
    ember\x20is\x20the\x20member\x20information\x20for\x20the\x20added\x20me\
    mber.\n\n\x0f\n\x05\x04%\x02\x01\x04\x12\x06\x9d\x06\x02\x9b\x06\x1c\n\r\
    \n\x05\x04%\x02\x01\x06\x12\x04\x9d\x06\x02\x08\n\r\n\x05\x04%\x02\x01\
    \x01\x12\x04\x9d\x06\t\x0f\n\r\n\x05\x04%\x02\x01\x03\x12\x04\x9d\x06\
    \x12\x13\nM\n\x04\x04%\x02\x02\x12\x04\x9f\x06\x02\x1e\x1a?\x20members\
    \x20is\x20a\x20list\x20of\x20all\x20members\x20after\x20adding\x20the\
    \x20new\x20member.\n\n\r\n\x05\x04%\x02\x02\x04\x12\x04\x9f\x06\x02\n\n\
    \r\n\x05\x04%\x02\x02\x06\x12\x04\x9f\x06\x0b\x11\n\r\n\x05\x04%\x02\x02\
    \x01\x12\x04\x9f\x06\x12\x19\n\r\n\x05\x04%\x02\x02\x03\x12\x04\x9f\x06\
    \x1c\x1d\n\x0c\n\x02\x04&\x12\x06\xa2\x06\0\xa5\x06\x01\n\x0b\n\x03\x04&\
    \x01\x12\x04\xa2\x06\x08\x1b\n<\n\x04\x04&\x02\0\x12\x04\xa4\x06\x02\x10\
    \x1a.\x20ID\x20is\x20the\x20member\x20ID\x20of\x20the\x20member\x20to\
    \x20remove.\n\n\x0f\n\x05\x04&\x02\0\x04\x12\x06\xa4\x06\x02\xa2\x06\x1d\
    \n\r\n\x05\x04&\x02\0\x05\x12\x04\xa4\x06\x02\x08\n\r\n\x05\x04&\x02\0\
    \x01\x12\x04\xa4\x06\t\x0b\n\r\n\x05\x04&\x02\0\x03\x12\x04\xa4\x06\x0e\
    \x0f\n\x0c\n\x02\x04'\x12\x06\xa7\x06\0\xab\x06\x01\n\x0b\n\x03\x04'\x01\
    \x12\x04\xa7\x06\x08\x1c\n\x0c\n\x04\x04'\x02\0\x12\x04\xa8\x06\x02\x1c\
    \n\x0f\n\x05\x04'\x02\0\x04\x12\x06\xa8\x06\x02\xa7\x06\x1e\n\r\n\x05\
    \x04'\x02\0\x06\x12\x04\xa8\x06\x02\x10\n\r\n\x05\x04'\x02\0\x01\x12\x04\
    \xa8\x06\x11\x17\n\r\n\x05\x04'\x02\0\x03\x12\x04\xa8\x06\x1a\x1b\nK\n\
    \x04\x04'\x02\x01\x12\x04\xaa\x06\x02\x1e\x1a=\x20members\x20is\x20a\x20\
    list\x20of\x20all\x20members\x20after\x20removing\x20the\x20member.\n\n\
    \r\n\x05\x04'\x02\x01\x04\x12\x04\xaa\x06\x02\n\n\r\n\x05\x04'\x02\x01\
    \x06\x12\x04\xaa\x06\x0b\x11\n\r\n\x05\x04'\x02\x01\x01\x12\x04\xaa\x06\
    \x12\x19\n\r\n\x05\x04'\x02\x01\x03\x12\x04\xaa\x06\x1c\x1d\n\x0c\n\x02\
    \x04(\x12\x06\xad\x06\0\xb2\x06\x01\n\x0b\n\x03\x04(\x01\x12\x04\xad\x06\
    \x08\x1b\n<\n\x04\x04(\x02\0\x12\x04\xaf\x06\x02\x10\x1a.\x20ID\x20is\
    \x20the\x20member\x20ID\x20of\x20the\x20member\x20to\x20update.\n\n\x0f\
    \n\x05\x04(\x02\0\x04\x12\x06\xaf\x06\x02\xad\x06\x1d\n\r\n\x05\x04(\x02\
    \0\x05\x12\x04\xaf\x06\x02\x08\n\r\n\x05\x04(\x02\0\x01\x12\x04\xaf\x06\
    \t\x0b\n\r\n\x05\x04(\x02\0\x03\x12\x04\xaf\x06\x0e\x0f\ne\n\x04\x04(\
    \x02\x01\x12\x04\xb1\x06\x02\x1f\x1aW\x20peerURLs\x20is\x20the\x20new\
    \x20list\x20of\x20URLs\x20the\x20member\x20will\x20use\x20to\x20communic\
    ate\x20with\x20the\x20cluster.\n\n\r\n\x05\x04(\x02\x01\x04\x12\x04\xb1\
    \x06\x02\n\n\r\n\x05\x04(\x02\x01\x05\x12\x04\xb1\x06\x0b\x11\n\r\n\x05\
    \x04(\x02\x01\x01\x12\x04\xb1\x06\x12\x1a\n\r\n\x05\x04(\x02\x01\x03\x12\
    \x04\xb1\x06\x1d\x1e\n\x0c\n\x02\x04)\x12\x06\xb4\x06\0\xb8\x06\x01\n\
    \x0b\n\x03\x04)\x01\x12\x04\xb4\x06\x08\x1c\n\x0c\n\x04\x04)\x02\0\x12\
    \x04\xb5\x06\x02\x1c\n\x0f\n\x05\x04)\x02\0\x04\x12\x06\xb5\x06\x02\xb4\
    \x06\x1d\n\r\n\x05\x04)\x02\0\x06\x12\x04\xb5\x06\x02\x10\n\r\n\x05\x04)\
    \x02\0\x01\x12\x04\xb5\x06\x11\x17\n\r\n\x05\x04)\x02\0\x03\x12\x04\xb5\
    \x06\x1a\x1b\nK\n\x04\x04)\x02\x01\x12\x04\xb7\x06\x02\x1e\x1a=\x20membe\
    rs\x20is\x20a\x20list\x20of\x20all\x20members\x20after\x20updating\x20th\
    e\x20member.\n\n\r\n\x05\x04)\x02\x01\x04\x12\x04\xb7\x06\x02\n\n\r\n\
    \x05\x04)\x02\x01\x06\x12\x04\xb7\x06\x0b\x11\n\r\n\x05\x04)\x02\x01\x01\
    \x12\x04\xb7\x06\x12\x19\n\r\n\x05\x04)\x02\x01\x03\x12\x04\xb7\x06\x1c\
    \x1d\n\x0c\n\x02\x04*\x12\x06\xba\x06\0\xbb\x06\x01\n\x0b\n\x03\x04*\x01\
    \x12\x04\xba\x06\x08\x19\n\x0c\n\x02\x04+\x12\x06\xbd\x06\0\xc1\x06\x01\
    \n\x0b\n\x03\x04+\x01\x12\x04\xbd\x06\x08\x1a\n\x0c\n\x04\x04+\x02\0\x12\
    \x04\xbe\x06\x02\x1c\n\x0f\n\x05\x04+\x02\0\x04\x12\x06\xbe\x06\x02\xbd\
    \x06\x1c\n\r\n\x05\x04+\x02\0\x06\x12\x04\xbe\x06\x02\x10\n\r\n\x05\x04+\
    \x02\0\x01\x12\x04\xbe\x06\x11\x17\n\r\n\x05\x04+\x02\0\x03\x12\x04\xbe\
    \x06\x1a\x1b\nM\n\x04\x04+\x02\x01\x12\x04\xc0\x06\x02\x1e\x1a?\x20membe\
    rs\x20is\x20a\x20list\x20of\x20all\x20members\x20associated\x20with\x20t\
    he\x20cluster.\n\n\r\n\x05\x04+\x02\x01\x04\x12\x04\xc0\x06\x02\n\n\r\n\
    \x05\x04+\x02\x01\x06\x12\x04\xc0\x06\x0b\x11\n\r\n\x05\x04+\x02\x01\x01\
    \x12\x04\xc0\x06\x12\x19\n\r\n\x05\x04+\x02\x01\x03\x12\x04\xc0\x06\x1c\
    \x1d\n\x0c\n\x02\x04,\x12\x06\xc3\x06\0\xc4\x06\x01\n\x0b\n\x03\x04,\x01\
    \x12\x04\xc3\x06\x08\x19\n\x0c\n\x02\x04-\x12\x06\xc6\x06\0\xc8\x06\x01\
    \n\x0b\n\x03\x04-\x01\x12\x04\xc6\x06\x08\x1a\n\x0c\n\x04\x04-\x02\0\x12\
    \x04\xc7\x06\x02\x1c\n\x0f\n\x05\x04-\x02\0\x04\x12\x06\xc7\x06\x02\xc6\
    \x06\x1c\n\r\n\x05\x04-\x02\0\x06\x12\x04\xc7\x06\x02\x10\n\r\n\x05\x04-\
    \x02\0\x01\x12\x04\xc7\x06\x11\x17\n\r\n\x05\x04-\x02\0\x03\x12\x04\xc7\
    \x06\x1a\x1b\n\x0c\n\x02\x04.\x12\x06\xca\x06\0\xcd\x06\x01\n\x0b\n\x03\
    \x04.\x01\x12\x04\xca\x06\x08\x19\n;\n\x04\x04.\x02\0\x12\x04\xcc\x06\
    \x02\x16\x1a-\x20targetID\x20is\x20the\x20node\x20ID\x20for\x20the\x20ne\
    w\x20leader.\n\n\x0f\n\x05\x04.\x02\0\x04\x12\x06\xcc\x06\x02\xca\x06\
    \x1b\n\r\n\x05\x04.\x02\0\x05\x12\x04\xcc\x06\x02\x08\n\r\n\x05\x04.\x02\
    \0\x01\x12\x04\xcc\x06\t\x11\n\r\n\x05\x04.\x02\0\x03\x12\x04\xcc\x06\
    \x14\x15\n\x0c\n\x02\x04/\x12\x06\xcf\x06\0\xd1\x06\x01\n\x0b\n\x03\x04/\
    \x01\x12\x04\xcf\x06\x08\x1a\n\x0c\n\x04\x04/\x02\0\x12\x04\xd0\x06\x02\
    \x1c\n\x0f\n\x05\x04/\x02\0\x04\x12\x06\xd0\x06\x02\xcf\x06\x1c\n\r\n\
    \x05\x04/\x02\0\x06\x12\x04\xd0\x06\x02\x10\n\r\n\x05\x04/\x02\0\x01\x12\
    \x04\xd0\x06\x11\x17\n\r\n\x05\x04/\x02\0\x03\x12\x04\xd0\x06\x1a\x1b\n\
    \x0c\n\x02\x05\0\x12\x06\xd3\x06\0\xd7\x06\x01\n\x0b\n\x03\x05\0\x01\x12\
    \x04\xd3\x06\x05\x0e\n=\n\x04\x05\0\x02\0\x12\x04\xd4\x06\x08\x11\"/\x20\
    default,\x20used\x20to\x20query\x20if\x20any\x20alarm\x20is\x20active\n\
    \n\r\n\x05\x05\0\x02\0\x01\x12\x04\xd4\x06\x08\x0c\n\r\n\x05\x05\0\x02\0\
    \x02\x12\x04\xd4\x06\x0f\x10\n(\n\x04\x05\0\x02\x01\x12\x04\xd5\x06\x08\
    \x14\"\x1a\x20space\x20quota\x20is\x20exhausted\n\n\r\n\x05\x05\0\x02\
    \x01\x01\x12\x04\xd5\x06\x08\x0f\n\r\n\x05\x05\0\x02\x01\x02\x12\x04\xd5\
    \x06\x12\x13\n,\n\x04\x05\0\x02\x02\x12\x04\xd6\x06\x08\x14\"\x1e\x20kv\
    \x20store\x20corruption\x20detected\n\n\r\n\x05\x05\0\x02\x02\x01\x12\
    \x04\xd6\x06\x08\x0f\n\r\n\x05\x05\0\x02\x02\x02\x12\x04\xd6\x06\x12\x13\
    \n\x0c\n\x02\x040\x12\x06\xd9\x06\0\xe8\x06\x01\n\x0b\n\x03\x040\x01\x12\
    \x04\xd9\x06\x08\x14\n\x0e\n\x04\x040\x04\0\x12\x06\xda\x06\x02\xde\x06\
    \x03\n\r\n\x05\x040\x04\0\x01\x12\x04\xda\x06\x07\x12\n\x0e\n\x06\x040\
    \x04\0\x02\0\x12\x04\xdb\x06\x08\x10\n\x0f\n\x07\x040\x04\0\x02\0\x01\
    \x12\x04\xdb\x06\x08\x0b\n\x0f\n\x07\x040\x04\0\x02\0\x02\x12\x04\xdb\
    \x06\x0e\x0f\n\x0e\n\x06\x040\x04\0\x02\x01\x12\x04\xdc\x06\x08\x15\n\
    \x0f\n\x07\x040\x04\0\x02\x01\x01\x12\x04\xdc\x06\x08\x10\n\x0f\n\x07\
    \x040\x04\0\x02\x01\x02\x12\x04\xdc\x06\x13\x14\n\x0e\n\x06\x040\x04\0\
    \x02\x02\x12\x04\xdd\x06\x08\x17\n\x0f\n\x07\x040\x04\0\x02\x02\x01\x12\
    \x04\xdd\x06\x08\x12\n\x0f\n\x07\x040\x04\0\x02\x02\x02\x12\x04\xdd\x06\
    \x15\x16\n\x94\x01\n\x04\x040\x02\0\x12\x04\xe2\x06\x02\x19\x1a\x85\x01\
    \x20action\x20is\x20the\x20kind\x20of\x20alarm\x20request\x20to\x20issue\
    .\x20The\x20action\n\x20may\x20GET\x20alarm\x20statuses,\x20ACTIVATE\x20\
    an\x20alarm,\x20or\x20DEACTIVATE\x20a\n\x20raised\x20alarm.\n\n\x0f\n\
    \x05\x040\x02\0\x04\x12\x06\xe2\x06\x02\xde\x06\x03\n\r\n\x05\x040\x02\0\
    \x06\x12\x04\xe2\x06\x02\r\n\r\n\x05\x040\x02\0\x01\x12\x04\xe2\x06\x0e\
    \x14\n\r\n\x05\x040\x02\0\x03\x12\x04\xe2\x06\x17\x18\n\x84\x01\n\x04\
    \x040\x02\x01\x12\x04\xe5\x06\x02\x16\x1av\x20memberID\x20is\x20the\x20I\
    D\x20of\x20the\x20member\x20associated\x20with\x20the\x20alarm.\x20If\
    \x20memberID\x20is\x200,\x20the\n\x20alarm\x20request\x20covers\x20all\
    \x20members.\n\n\x0f\n\x05\x040\x02\x01\x04\x12\x06\xe5\x06\x02\xe2\x06\
    \x19\n\r\n\x05\x040\x02\x01\x05\x12\x04\xe5\x06\x02\x08\n\r\n\x05\x040\
    \x02\x01\x01\x12\x04\xe5\x06\t\x11\n\r\n\x05\x040\x02\x01\x03\x12\x04\
    \xe5\x06\x14\x15\nH\n\x04\x040\x02\x02\x12\x04\xe7\x06\x02\x16\x1a:\x20a\
    larm\x20is\x20the\x20type\x20of\x20alarm\x20to\x20consider\x20for\x20thi\
    s\x20request.\n\n\x0f\n\x05\x040\x02\x02\x04\x12\x06\xe7\x06\x02\xe5\x06\
    \x16\n\r\n\x05\x040\x02\x02\x06\x12\x04\xe7\x06\x02\x0b\n\r\n\x05\x040\
    \x02\x02\x01\x12\x04\xe7\x06\x0c\x11\n\r\n\x05\x040\x02\x02\x03\x12\x04\
    \xe7\x06\x14\x15\n\x0c\n\x02\x041\x12\x06\xea\x06\0\xef\x06\x01\n\x0b\n\
    \x03\x041\x01\x12\x04\xea\x06\x08\x13\nR\n\x04\x041\x02\0\x12\x04\xec\
    \x06\x02\x16\x1aD\x20memberID\x20is\x20the\x20ID\x20of\x20the\x20member\
    \x20associated\x20with\x20the\x20raised\x20alarm.\n\n\x0f\n\x05\x041\x02\
    \0\x04\x12\x06\xec\x06\x02\xea\x06\x15\n\r\n\x05\x041\x02\0\x05\x12\x04\
    \xec\x06\x02\x08\n\r\n\x05\x041\x02\0\x01\x12\x04\xec\x06\t\x11\n\r\n\
    \x05\x041\x02\0\x03\x12\x04\xec\x06\x14\x15\nA\n\x04\x041\x02\x01\x12\
    \x04\xee\x06\x02\x16\x1a3\x20alarm\x20is\x20the\x20type\x20of\x20alarm\
    \x20which\x20has\x20been\x20raised.\n\n\x0f\n\x05\x041\x02\x01\x04\x12\
    \x06\xee\x06\x02\xec\x06\x16\n\r\n\x05\x041\x02\x01\x06\x12\x04\xee\x06\
    \x02\x0b\n\r\n\x05\x041\x02\x01\x01\x12\x04\xee\x06\x0c\x11\n\r\n\x05\
    \x041\x02\x01\x03\x12\x04\xee\x06\x14\x15\n\x0c\n\x02\x042\x12\x06\xf1\
    \x06\0\xf5\x06\x01\n\x0b\n\x03\x042\x01\x12\x04\xf1\x06\x08\x15\n\x0c\n\
    \x04\x042\x02\0\x12\x04\xf2\x06\x02\x1c\n\x0f\n\x05\x042\x02\0\x04\x12\
    \x06\xf2\x06\x02\xf1\x06\x17\n\r\n\x05\x042\x02\0\x06\x12\x04\xf2\x06\
    \x02\x10\n\r\n\x05\x042\x02\0\x01\x12\x04\xf2\x06\x11\x17\n\r\n\x05\x042\
    \x02\0\x03\x12\x04\xf2\x06\x1a\x1b\nM\n\x04\x042\x02\x01\x12\x04\xf4\x06\
    \x02\"\x1a?\x20alarms\x20is\x20a\x20list\x20of\x20alarms\x20associated\
    \x20with\x20the\x20alarm\x20request.\n\n\r\n\x05\x042\x02\x01\x04\x12\
    \x04\xf4\x06\x02\n\n\r\n\x05\x042\x02\x01\x06\x12\x04\xf4\x06\x0b\x16\n\
    \r\n\x05\x042\x02\x01\x01\x12\x04\xf4\x06\x17\x1d\n\r\n\x05\x042\x02\x01\
    \x03\x12\x04\xf4\x06\x20!\n\x0c\n\x02\x043\x12\x06\xf7\x06\0\xf8\x06\x01\
    \n\x0b\n\x03\x043\x01\x12\x04\xf7\x06\x08\x15\n\x0c\n\x02\x044\x12\x06\
    \xfa\x06\0\x86\x07\x01\n\x0b\n\x03\x044\x01\x12\x04\xfa\x06\x08\x16\n\
    \x0c\n\x04\x044\x02\0\x12\x04\xfb\x06\x02\x1c\n\x0f\n\x05\x044\x02\0\x04\
    \x12\x06\xfb\x06\x02\xfa\x06\x18\n\r\n\x05\x044\x02\0\x06\x12\x04\xfb\
    \x06\x02\x10\n\r\n\x05\x044\x02\0\x01\x12\x04\xfb\x06\x11\x17\n\r\n\x05\
    \x044\x02\0\x03\x12\x04\xfb\x06\x1a\x1b\nV\n\x04\x044\x02\x01\x12\x04\
    \xfd\x06\x02\x15\x1aH\x20version\x20is\x20the\x20cluster\x20protocol\x20\
    version\x20used\x20by\x20the\x20responding\x20member.\n\n\x0f\n\x05\x044\
    \x02\x01\x04\x12\x06\xfd\x06\x02\xfb\x06\x1c\n\r\n\x05\x044\x02\x01\x05\
    \x12\x04\xfd\x06\x02\x08\n\r\n\x05\x044\x02\x01\x01\x12\x04\xfd\x06\t\
    \x10\n\r\n\x05\x044\x02\x01\x03\x12\x04\xfd\x06\x13\x14\n_\n\x04\x044\
    \x02\x02\x12\x04\xff\x06\x02\x13\x1aQ\x20dbSize\x20is\x20the\x20size\x20\
    of\x20the\x20backend\x20database,\x20in\x20bytes,\x20of\x20the\x20respon\
    ding\x20member.\n\n\x0f\n\x05\x044\x02\x02\x04\x12\x06\xff\x06\x02\xfd\
    \x06\x15\n\r\n\x05\x044\x02\x02\x05\x12\x04\xff\x06\x02\x07\n\r\n\x05\
    \x044\x02\x02\x01\x12\x04\xff\x06\x08\x0e\n\r\n\x05\x044\x02\x02\x03\x12\
    \x04\xff\x06\x11\x12\nc\n\x04\x044\x02\x03\x12\x04\x81\x07\x02\x14\x1aU\
    \x20leader\x20is\x20the\x20member\x20ID\x20which\x20the\x20responding\
    \x20member\x20believes\x20is\x20the\x20current\x20leader.\n\n\x0f\n\x05\
    \x044\x02\x03\x04\x12\x06\x81\x07\x02\xff\x06\x13\n\r\n\x05\x044\x02\x03\
    \x05\x12\x04\x81\x07\x02\x08\n\r\n\x05\x044\x02\x03\x01\x12\x04\x81\x07\
    \t\x0f\n\r\n\x05\x044\x02\x03\x03\x12\x04\x81\x07\x12\x13\nM\n\x04\x044\
    \x02\x04\x12\x04\x83\x07\x02\x17\x1a?\x20raftIndex\x20is\x20the\x20curre\
    nt\x20raft\x20index\x20of\x20the\x20responding\x20member.\n\n\x0f\n\x05\
    \x044\x02\x04\x04\x12\x06\x83\x07\x02\x81\x07\x14\n\r\n\x05\x044\x02\x04\
    \x05\x12\x04\x83\x07\x02\x08\n\r\n\x05\x044\x02\x04\x01\x12\x04\x83\x07\
    \t\x12\n\r\n\x05\x044\x02\x04\x03\x12\x04\x83\x07\x15\x16\nK\n\x04\x044\
    \x02\x05\x12\x04\x85\x07\x02\x16\x1a=\x20raftTerm\x20is\x20the\x20curren\
    t\x20raft\x20term\x20of\x20the\x20responding\x20member.\n\n\x0f\n\x05\
    \x044\x02\x05\x04\x12\x06\x85\x07\x02\x83\x07\x17\n\r\n\x05\x044\x02\x05\
    \x05\x12\x04\x85\x07\x02\x08\n\r\n\x05\x044\x02\x05\x01\x12\x04\x85\x07\
    \t\x11\n\r\n\x05\x044\x02\x05\x03\x12\x04\x85\x07\x14\x15\n\x0c\n\x02\
    \x045\x12\x06\x88\x07\0\x89\x07\x01\n\x0b\n\x03\x045\x01\x12\x04\x88\x07\
    \x08\x19\n\x0c\n\x02\x046\x12\x06\x8b\x07\0\x8c\x07\x01\n\x0b\n\x03\x046\
    \x01\x12\x04\x8b\x07\x08\x1a\n\x0c\n\x02\x047\x12\x06\x8e\x07\0\x91\x07\
    \x01\n\x0b\n\x03\x047\x01\x12\x04\x8e\x07\x08\x1b\n\x0c\n\x04\x047\x02\0\
    \x12\x04\x8f\x07\x02\x12\n\x0f\n\x05\x047\x02\0\x04\x12\x06\x8f\x07\x02\
    \x8e\x07\x1d\n\r\n\x05\x047\x02\0\x05\x12\x04\x8f\x07\x02\x08\n\r\n\x05\
    \x047\x02\0\x01\x12\x04\x8f\x07\t\r\n\r\n\x05\x047\x02\0\x03\x12\x04\x8f\
    \x07\x10\x11\n\x0c\n\x04\x047\x02\x01\x12\x04\x90\x07\x02\x16\n\x0f\n\
    \x05\x047\x02\x01\x04\x12\x06\x90\x07\x02\x8f\x07\x12\n\r\n\x05\x047\x02\
    \x01\x05\x12\x04\x90\x07\x02\x08\n\r\n\x05\x047\x02\x01\x01\x12\x04\x90\
    \x07\t\x11\n\r\n\x05\x047\x02\x01\x03\x12\x04\x90\x07\x14\x15\n\x0c\n\
    \x02\x048\x12\x06\x93\x07\0\x96\x07\x01\n\x0b\n\x03\x048\x01\x12\x04\x93\
    \x07\x08\x1a\n\x0c\n\x04\x048\x02\0\x12\x04\x94\x07\x02\x12\n\x0f\n\x05\
    \x048\x02\0\x04\x12\x06\x94\x07\x02\x93\x07\x1c\n\r\n\x05\x048\x02\0\x05\
    \x12\x04\x94\x07\x02\x08\n\r\n\x05\x048\x02\0\x01\x12\x04\x94\x07\t\r\n\
    \r\n\x05\x048\x02\0\x03\x12\x04\x94\x07\x10\x11\n\x0c\n\x04\x048\x02\x01\
    \x12\x04\x95\x07\x02\x16\n\x0f\n\x05\x048\x02\x01\x04\x12\x06\x95\x07\
    \x02\x94\x07\x12\n\r\n\x05\x048\x02\x01\x05\x12\x04\x95\x07\x02\x08\n\r\
    \n\x05\x048\x02\x01\x01\x12\x04\x95\x07\t\x11\n\r\n\x05\x048\x02\x01\x03\
    \x12\x04\x95\x07\x14\x15\n\x0c\n\x02\x049\x12\x06\x98\x07\0\x9a\x07\x01\
    \n\x0b\n\x03\x049\x01\x12\x04\x98\x07\x08\x1a\n\x0c\n\x04\x049\x02\0\x12\
    \x04\x99\x07\x02\x12\n\x0f\n\x05\x049\x02\0\x04\x12\x06\x99\x07\x02\x98\
    \x07\x1c\n\r\n\x05\x049\x02\0\x05\x12\x04\x99\x07\x02\x08\n\r\n\x05\x049\
    \x02\0\x01\x12\x04\x99\x07\t\r\n\r\n\x05\x049\x02\0\x03\x12\x04\x99\x07\
    \x10\x11\n\x0c\n\x02\x04:\x12\x06\x9c\x07\0\x9f\x07\x01\n\x0b\n\x03\x04:\
    \x01\x12\x04\x9c\x07\x08\x1d\n7\n\x04\x04:\x02\0\x12\x04\x9e\x07\x02\x12\
    \x1a)\x20name\x20is\x20the\x20name\x20of\x20the\x20user\x20to\x20delete.\
    \n\n\x0f\n\x05\x04:\x02\0\x04\x12\x06\x9e\x07\x02\x9c\x07\x1f\n\r\n\x05\
    \x04:\x02\0\x05\x12\x04\x9e\x07\x02\x08\n\r\n\x05\x04:\x02\0\x01\x12\x04\
    \x9e\x07\t\r\n\r\n\x05\x04:\x02\0\x03\x12\x04\x9e\x07\x10\x11\n\x0c\n\
    \x02\x04;\x12\x06\xa1\x07\0\xa6\x07\x01\n\x0b\n\x03\x04;\x01\x12\x04\xa1\
    \x07\x08%\nM\n\x04\x04;\x02\0\x12\x04\xa3\x07\x02\x12\x1a?\x20name\x20is\
    \x20the\x20name\x20of\x20the\x20user\x20whose\x20password\x20is\x20being\
    \x20changed.\n\n\x0f\n\x05\x04;\x02\0\x04\x12\x06\xa3\x07\x02\xa1\x07'\n\
    \r\n\x05\x04;\x02\0\x05\x12\x04\xa3\x07\x02\x08\n\r\n\x05\x04;\x02\0\x01\
    \x12\x04\xa3\x07\t\r\n\r\n\x05\x04;\x02\0\x03\x12\x04\xa3\x07\x10\x11\n:\
    \n\x04\x04;\x02\x01\x12\x04\xa5\x07\x02\x16\x1a,\x20password\x20is\x20th\
    e\x20new\x20password\x20for\x20the\x20user.\n\n\x0f\n\x05\x04;\x02\x01\
    \x04\x12\x06\xa5\x07\x02\xa3\x07\x12\n\r\n\x05\x04;\x02\x01\x05\x12\x04\
    \xa5\x07\x02\x08\n\r\n\x05\x04;\x02\x01\x01\x12\x04\xa5\x07\t\x11\n\r\n\
    \x05\x04;\x02\x01\x03\x12\x04\xa5\x07\x14\x15\n\x0c\n\x02\x04<\x12\x06\
    \xa8\x07\0\xad\x07\x01\n\x0b\n\x03\x04<\x01\x12\x04\xa8\x07\x08\x20\nR\n\
    \x04\x04<\x02\0\x12\x04\xaa\x07\x02\x12\x1aD\x20user\x20is\x20the\x20nam\
    e\x20of\x20the\x20user\x20which\x20should\x20be\x20granted\x20a\x20given\
    \x20role.\n\n\x0f\n\x05\x04<\x02\0\x04\x12\x06\xaa\x07\x02\xa8\x07\"\n\r\
    \n\x05\x04<\x02\0\x05\x12\x04\xaa\x07\x02\x08\n\r\n\x05\x04<\x02\0\x01\
    \x12\x04\xaa\x07\t\r\n\r\n\x05\x04<\x02\0\x03\x12\x04\xaa\x07\x10\x11\nB\
    \n\x04\x04<\x02\x01\x12\x04\xac\x07\x02\x12\x1a4\x20role\x20is\x20the\
    \x20name\x20of\x20the\x20role\x20to\x20grant\x20to\x20the\x20user.\n\n\
    \x0f\n\x05\x04<\x02\x01\x04\x12\x06\xac\x07\x02\xaa\x07\x12\n\r\n\x05\
    \x04<\x02\x01\x05\x12\x04\xac\x07\x02\x08\n\r\n\x05\x04<\x02\x01\x01\x12\
    \x04\xac\x07\t\r\n\r\n\x05\x04<\x02\x01\x03\x12\x04\xac\x07\x10\x11\n\
    \x0c\n\x02\x04=\x12\x06\xaf\x07\0\xb2\x07\x01\n\x0b\n\x03\x04=\x01\x12\
    \x04\xaf\x07\x08!\n\x0c\n\x04\x04=\x02\0\x12\x04\xb0\x07\x02\x12\n\x0f\n\
    \x05\x04=\x02\0\x04\x12\x06\xb0\x07\x02\xaf\x07#\n\r\n\x05\x04=\x02\0\
    \x05\x12\x04\xb0\x07\x02\x08\n\r\n\x05\x04=\x02\0\x01\x12\x04\xb0\x07\t\
    \r\n\r\n\x05\x04=\x02\0\x03\x12\x04\xb0\x07\x10\x11\n\x0c\n\x04\x04=\x02\
    \x01\x12\x04\xb1\x07\x02\x12\n\x0f\n\x05\x04=\x02\x01\x04\x12\x06\xb1\
    \x07\x02\xb0\x07\x12\n\r\n\x05\x04=\x02\x01\x05\x12\x04\xb1\x07\x02\x08\
    \n\r\n\x05\x04=\x02\x01\x01\x12\x04\xb1\x07\t\r\n\r\n\x05\x04=\x02\x01\
    \x03\x12\x04\xb1\x07\x10\x11\n\x0c\n\x02\x04>\x12\x06\xb4\x07\0\xb7\x07\
    \x01\n\x0b\n\x03\x04>\x01\x12\x04\xb4\x07\x08\x1a\nQ\n\x04\x04>\x02\0\
    \x12\x04\xb6\x07\x02\x12\x1aC\x20name\x20is\x20the\x20name\x20of\x20the\
    \x20role\x20to\x20add\x20to\x20the\x20authentication\x20system.\n\n\x0f\
    \n\x05\x04>\x02\0\x04\x12\x06\xb6\x07\x02\xb4\x07\x1c\n\r\n\x05\x04>\x02\
    \0\x05\x12\x04\xb6\x07\x02\x08\n\r\n\x05\x04>\x02\0\x01\x12\x04\xb6\x07\
    \t\r\n\r\n\x05\x04>\x02\0\x03\x12\x04\xb6\x07\x10\x11\n\x0c\n\x02\x04?\
    \x12\x06\xb9\x07\0\xbb\x07\x01\n\x0b\n\x03\x04?\x01\x12\x04\xb9\x07\x08\
    \x1a\n\x0c\n\x04\x04?\x02\0\x12\x04\xba\x07\x02\x12\n\x0f\n\x05\x04?\x02\
    \0\x04\x12\x06\xba\x07\x02\xb9\x07\x1c\n\r\n\x05\x04?\x02\0\x05\x12\x04\
    \xba\x07\x02\x08\n\r\n\x05\x04?\x02\0\x01\x12\x04\xba\x07\t\r\n\r\n\x05\
    \x04?\x02\0\x03\x12\x04\xba\x07\x10\x11\n\x0c\n\x02\x04@\x12\x06\xbd\x07\
    \0\xbe\x07\x01\n\x0b\n\x03\x04@\x01\x12\x04\xbd\x07\x08\x1b\n\x0c\n\x02\
    \x04A\x12\x06\xc0\x07\0\xc1\x07\x01\n\x0b\n\x03\x04A\x01\x12\x04\xc0\x07\
    \x08\x1b\n\x0c\n\x02\x04B\x12\x06\xc3\x07\0\xc5\x07\x01\n\x0b\n\x03\x04B\
    \x01\x12\x04\xc3\x07\x08\x1d\n\x0c\n\x04\x04B\x02\0\x12\x04\xc4\x07\x02\
    \x12\n\x0f\n\x05\x04B\x02\0\x04\x12\x06\xc4\x07\x02\xc3\x07\x1f\n\r\n\
    \x05\x04B\x02\0\x05\x12\x04\xc4\x07\x02\x08\n\r\n\x05\x04B\x02\0\x01\x12\
    \x04\xc4\x07\t\r\n\r\n\x05\x04B\x02\0\x03\x12\x04\xc4\x07\x10\x11\n\x0c\
    \n\x02\x04C\x12\x06\xc7\x07\0\xcc\x07\x01\n\x0b\n\x03\x04C\x01\x12\x04\
    \xc7\x07\x08&\nR\n\x04\x04C\x02\0\x12\x04\xc9\x07\x02\x12\x1aD\x20name\
    \x20is\x20the\x20name\x20of\x20the\x20role\x20which\x20will\x20be\x20gra\
    nted\x20the\x20permission.\n\n\x0f\n\x05\x04C\x02\0\x04\x12\x06\xc9\x07\
    \x02\xc7\x07(\n\r\n\x05\x04C\x02\0\x05\x12\x04\xc9\x07\x02\x08\n\r\n\x05\
    \x04C\x02\0\x01\x12\x04\xc9\x07\t\r\n\r\n\x05\x04C\x02\0\x03\x12\x04\xc9\
    \x07\x10\x11\n<\n\x04\x04C\x02\x01\x12\x04\xcb\x07\x02\x1d\x1a.\x20perm\
    \x20is\x20the\x20permission\x20to\x20grant\x20to\x20the\x20role.\n\n\x0f\
    \n\x05\x04C\x02\x01\x04\x12\x06\xcb\x07\x02\xc9\x07\x12\n\r\n\x05\x04C\
    \x02\x01\x06\x12\x04\xcb\x07\x02\x13\n\r\n\x05\x04C\x02\x01\x01\x12\x04\
    \xcb\x07\x14\x18\n\r\n\x05\x04C\x02\x01\x03\x12\x04\xcb\x07\x1b\x1c\n\
    \x0c\n\x02\x04D\x12\x06\xce\x07\0\xd2\x07\x01\n\x0b\n\x03\x04D\x01\x12\
    \x04\xce\x07\x08'\n\x0c\n\x04\x04D\x02\0\x12\x04\xcf\x07\x02\x12\n\x0f\n\
    \x05\x04D\x02\0\x04\x12\x06\xcf\x07\x02\xce\x07)\n\r\n\x05\x04D\x02\0\
    \x05\x12\x04\xcf\x07\x02\x08\n\r\n\x05\x04D\x02\0\x01\x12\x04\xcf\x07\t\
    \r\n\r\n\x05\x04D\x02\0\x03\x12\x04\xcf\x07\x10\x11\n\x0c\n\x04\x04D\x02\
    \x01\x12\x04\xd0\x07\x02\x11\n\x0f\n\x05\x04D\x02\x01\x04\x12\x06\xd0\
    \x07\x02\xcf\x07\x12\n\r\n\x05\x04D\x02\x01\x05\x12\x04\xd0\x07\x02\x08\
    \n\r\n\x05\x04D\x02\x01\x01\x12\x04\xd0\x07\t\x0c\n\r\n\x05\x04D\x02\x01\
    \x03\x12\x04\xd0\x07\x0f\x10\n\x0c\n\x04\x04D\x02\x02\x12\x04\xd1\x07\
    \x02\x17\n\x0f\n\x05\x04D\x02\x02\x04\x12\x06\xd1\x07\x02\xd0\x07\x11\n\
    \r\n\x05\x04D\x02\x02\x05\x12\x04\xd1\x07\x02\x08\n\r\n\x05\x04D\x02\x02\
    \x01\x12\x04\xd1\x07\t\x12\n\r\n\x05\x04D\x02\x02\x03\x12\x04\xd1\x07\
    \x15\x16\n\x0c\n\x02\x04E\x12\x06\xd4\x07\0\xd6\x07\x01\n\x0b\n\x03\x04E\
    \x01\x12\x04\xd4\x07\x08\x1a\n\x0c\n\x04\x04E\x02\0\x12\x04\xd5\x07\x02\
    \x1c\n\x0f\n\x05\x04E\x02\0\x04\x12\x06\xd5\x07\x02\xd4\x07\x1c\n\r\n\
    \x05\x04E\x02\0\x06\x12\x04\xd5\x07\x02\x10\n\r\n\x05\x04E\x02\0\x01\x12\
    \x04\xd5\x07\x11\x17\n\r\n\x05\x04E\x02\0\x03\x12\x04\xd5\x07\x1a\x1b\n\
    \x0c\n\x02\x04F\x12\x06\xd8\x07\0\xda\x07\x01\n\x0b\n\x03\x04F\x01\x12\
    \x04\xd8\x07\x08\x1b\n\x0c\n\x04\x04F\x02\0\x12\x04\xd9\x07\x02\x1c\n\
    \x0f\n\x05\x04F\x02\0\x04\x12\x06\xd9\x07\x02\xd8\x07\x1d\n\r\n\x05\x04F\
    \x02\0\x06\x12\x04\xd9\x07\x02\x10\n\r\n\x05\x04F\x02\0\x01\x12\x04\xd9\
    \x07\x11\x17\n\r\n\x05\x04F\x02\0\x03\x12\x04\xd9\x07\x1a\x1b\n\x0c\n\
    \x02\x04G\x12\x06\xdc\x07\0\xe0\x07\x01\n\x0b\n\x03\x04G\x01\x12\x04\xdc\
    \x07\x08\x1c\n\x0c\n\x04\x04G\x02\0\x12\x04\xdd\x07\x02\x1c\n\x0f\n\x05\
    \x04G\x02\0\x04\x12\x06\xdd\x07\x02\xdc\x07\x1e\n\r\n\x05\x04G\x02\0\x06\
    \x12\x04\xdd\x07\x02\x10\n\r\n\x05\x04G\x02\0\x01\x12\x04\xdd\x07\x11\
    \x17\n\r\n\x05\x04G\x02\0\x03\x12\x04\xdd\x07\x1a\x1b\nP\n\x04\x04G\x02\
    \x01\x12\x04\xdf\x07\x02\x13\x1aB\x20token\x20is\x20an\x20authorized\x20\
    token\x20that\x20can\x20be\x20used\x20in\x20succeeding\x20RPCs\n\n\x0f\n\
    \x05\x04G\x02\x01\x04\x12\x06\xdf\x07\x02\xdd\x07\x1c\n\r\n\x05\x04G\x02\
    \x01\x05\x12\x04\xdf\x07\x02\x08\n\r\n\x05\x04G\x02\x01\x01\x12\x04\xdf\
    \x07\t\x0e\n\r\n\x05\x04G\x02\x01\x03\x12\x04\xdf\x07\x11\x12\n\x0c\n\
    \x02\x04H\x12\x06\xe2\x07\0\xe4\x07\x01\n\x0b\n\x03\x04H\x01\x12\x04\xe2\
    \x07\x08\x1b\n\x0c\n\x04\x04H\x02\0\x12\x04\xe3\x07\x02\x1c\n\x0f\n\x05\
    \x04H\x02\0\x04\x12\x06\xe3\x07\x02\xe2\x07\x1d\n\r\n\x05\x04H\x02\0\x06\
    \x12\x04\xe3\x07\x02\x10\n\r\n\x05\x04H\x02\0\x01\x12\x04\xe3\x07\x11\
    \x17\n\r\n\x05\x04H\x02\0\x03\x12\x04\xe3\x07\x1a\x1b\n\x0c\n\x02\x04I\
    \x12\x06\xe6\x07\0\xea\x07\x01\n\x0b\n\x03\x04I\x01\x12\x04\xe6\x07\x08\
    \x1b\n\x0c\n\x04\x04I\x02\0\x12\x04\xe7\x07\x02\x1c\n\x0f\n\x05\x04I\x02\
    \0\x04\x12\x06\xe7\x07\x02\xe6\x07\x1d\n\r\n\x05\x04I\x02\0\x06\x12\x04\
    \xe7\x07\x02\x10\n\r\n\x05\x04I\x02\0\x01\x12\x04\xe7\x07\x11\x17\n\r\n\
    \x05\x04I\x02\0\x03\x12\x04\xe7\x07\x1a\x1b\n\x0c\n\x04\x04I\x02\x01\x12\
    \x04\xe9\x07\x02\x1c\n\r\n\x05\x04I\x02\x01\x04\x12\x04\xe9\x07\x02\n\n\
    \r\n\x05\x04I\x02\x01\x05\x12\x04\xe9\x07\x0b\x11\n\r\n\x05\x04I\x02\x01\
    \x01\x12\x04\xe9\x07\x12\x17\n\r\n\x05\x04I\x02\x01\x03\x12\x04\xe9\x07\
    \x1a\x1b\n\x0c\n\x02\x04J\x12\x06\xec\x07\0\xee\x07\x01\n\x0b\n\x03\x04J\
    \x01\x12\x04\xec\x07\x08\x1e\n\x0c\n\x04\x04J\x02\0\x12\x04\xed\x07\x02\
    \x1c\n\x0f\n\x05\x04J\x02\0\x04\x12\x06\xed\x07\x02\xec\x07\x20\n\r\n\
    \x05\x04J\x02\0\x06\x12\x04\xed\x07\x02\x10\n\r\n\x05\x04J\x02\0\x01\x12\
    \x04\xed\x07\x11\x17\n\r\n\x05\x04J\x02\0\x03\x12\x04\xed\x07\x1a\x1b\n\
    \x0c\n\x02\x04K\x12\x06\xf0\x07\0\xf2\x07\x01\n\x0b\n\x03\x04K\x01\x12\
    \x04\xf0\x07\x08&\n\x0c\n\x04\x04K\x02\0\x12\x04\xf1\x07\x02\x1c\n\x0f\n\
    \x05\x04K\x02\0\x04\x12\x06\xf1\x07\x02\xf0\x07(\n\r\n\x05\x04K\x02\0\
    \x06\x12\x04\xf1\x07\x02\x10\n\r\n\x05\x04K\x02\0\x01\x12\x04\xf1\x07\
    \x11\x17\n\r\n\x05\x04K\x02\0\x03\x12\x04\xf1\x07\x1a\x1b\n\x0c\n\x02\
    \x04L\x12\x06\xf4\x07\0\xf6\x07\x01\n\x0b\n\x03\x04L\x01\x12\x04\xf4\x07\
    \x08!\n\x0c\n\x04\x04L\x02\0\x12\x04\xf5\x07\x02\x1c\n\x0f\n\x05\x04L\
    \x02\0\x04\x12\x06\xf5\x07\x02\xf4\x07#\n\r\n\x05\x04L\x02\0\x06\x12\x04\
    \xf5\x07\x02\x10\n\r\n\x05\x04L\x02\0\x01\x12\x04\xf5\x07\x11\x17\n\r\n\
    \x05\x04L\x02\0\x03\x12\x04\xf5\x07\x1a\x1b\n\x0c\n\x02\x04M\x12\x06\xf8\
    \x07\0\xfa\x07\x01\n\x0b\n\x03\x04M\x01\x12\x04\xf8\x07\x08\"\n\x0c\n\
    \x04\x04M\x02\0\x12\x04\xf9\x07\x02\x1c\n\x0f\n\x05\x04M\x02\0\x04\x12\
    \x06\xf9\x07\x02\xf8\x07$\n\r\n\x05\x04M\x02\0\x06\x12\x04\xf9\x07\x02\
    \x10\n\r\n\x05\x04M\x02\0\x01\x12\x04\xf9\x07\x11\x17\n\r\n\x05\x04M\x02\
    \0\x03\x12\x04\xf9\x07\x1a\x1b\n\x0c\n\x02\x04N\x12\x06\xfc\x07\0\xfe\
    \x07\x01\n\x0b\n\x03\x04N\x01\x12\x04\xfc\x07\x08\x1b\n\x0c\n\x04\x04N\
    \x02\0\x12\x04\xfd\x07\x02\x1c\n\x0f\n\x05\x04N\x02\0\x04\x12\x06\xfd\
    \x07\x02\xfc\x07\x1d\n\r\n\x05\x04N\x02\0\x06\x12\x04\xfd\x07\x02\x10\n\
    \r\n\x05\x04N\x02\0\x01\x12\x04\xfd\x07\x11\x17\n\r\n\x05\x04N\x02\0\x03\
    \x12\x04\xfd\x07\x1a\x1b\n\x0c\n\x02\x04O\x12\x06\x80\x08\0\x84\x08\x01\
    \n\x0b\n\x03\x04O\x01\x12\x04\x80\x08\x08\x1b\n\x0c\n\x04\x04O\x02\0\x12\
    \x04\x81\x08\x02\x1c\n\x0f\n\x05\x04O\x02\0\x04\x12\x06\x81\x08\x02\x80\
    \x08\x1d\n\r\n\x05\x04O\x02\0\x06\x12\x04\x81\x08\x02\x10\n\r\n\x05\x04O\
    \x02\0\x01\x12\x04\x81\x08\x11\x17\n\r\n\x05\x04O\x02\0\x03\x12\x04\x81\
    \x08\x1a\x1b\n\x0c\n\x04\x04O\x02\x01\x12\x04\x83\x08\x02&\n\r\n\x05\x04\
    O\x02\x01\x04\x12\x04\x83\x08\x02\n\n\r\n\x05\x04O\x02\x01\x06\x12\x04\
    \x83\x08\x0b\x1c\n\r\n\x05\x04O\x02\x01\x01\x12\x04\x83\x08\x1d!\n\r\n\
    \x05\x04O\x02\x01\x03\x12\x04\x83\x08$%\n\x0c\n\x02\x04P\x12\x06\x86\x08\
    \0\x8a\x08\x01\n\x0b\n\x03\x04P\x01\x12\x04\x86\x08\x08\x1c\n\x0c\n\x04\
    \x04P\x02\0\x12\x04\x87\x08\x02\x1c\n\x0f\n\x05\x04P\x02\0\x04\x12\x06\
    \x87\x08\x02\x86\x08\x1e\n\r\n\x05\x04P\x02\0\x06\x12\x04\x87\x08\x02\
    \x10\n\r\n\x05\x04P\x02\0\x01\x12\x04\x87\x08\x11\x17\n\r\n\x05\x04P\x02\
    \0\x03\x12\x04\x87\x08\x1a\x1b\n\x0c\n\x04\x04P\x02\x01\x12\x04\x89\x08\
    \x02\x1c\n\r\n\x05\x04P\x02\x01\x04\x12\x04\x89\x08\x02\n\n\r\n\x05\x04P\
    \x02\x01\x05\x12\x04\x89\x08\x0b\x11\n\r\n\x05\x04P\x02\x01\x01\x12\x04\
    \x89\x08\x12\x17\n\r\n\x05\x04P\x02\x01\x03\x12\x04\x89\x08\x1a\x1b\n\
    \x0c\n\x02\x04Q\x12\x06\x8c\x08\0\x90\x08\x01\n\x0b\n\x03\x04Q\x01\x12\
    \x04\x8c\x08\x08\x1c\n\x0c\n\x04\x04Q\x02\0\x12\x04\x8d\x08\x02\x1c\n\
    \x0f\n\x05\x04Q\x02\0\x04\x12\x06\x8d\x08\x02\x8c\x08\x1e\n\r\n\x05\x04Q\
    \x02\0\x06\x12\x04\x8d\x08\x02\x10\n\r\n\x05\x04Q\x02\0\x01\x12\x04\x8d\
    \x08\x11\x17\n\r\n\x05\x04Q\x02\0\x03\x12\x04\x8d\x08\x1a\x1b\n\x0c\n\
    \x04\x04Q\x02\x01\x12\x04\x8f\x08\x02\x1c\n\r\n\x05\x04Q\x02\x01\x04\x12\
    \x04\x8f\x08\x02\n\n\r\n\x05\x04Q\x02\x01\x05\x12\x04\x8f\x08\x0b\x11\n\
    \r\n\x05\x04Q\x02\x01\x01\x12\x04\x8f\x08\x12\x17\n\r\n\x05\x04Q\x02\x01\
    \x03\x12\x04\x8f\x08\x1a\x1b\n\x0c\n\x02\x04R\x12\x06\x92\x08\0\x94\x08\
    \x01\n\x0b\n\x03\x04R\x01\x12\x04\x92\x08\x08\x1e\n\x0c\n\x04\x04R\x02\0\
    \x12\x04\x93\x08\x02\x1c\n\x0f\n\x05\x04R\x02\0\x04\x12\x06\x93\x08\x02\
    \x92\x08\x20\n\r\n\x05\x04R\x02\0\x06\x12\x04\x93\x08\x02\x10\n\r\n\x05\
    \x04R\x02\0\x01\x12\x04\x93\x08\x11\x17\n\r\n\x05\x04R\x02\0\x03\x12\x04\
    \x93\x08\x1a\x1b\n\x0c\n\x02\x04S\x12\x06\x96\x08\0\x98\x08\x01\n\x0b\n\
    \x03\x04S\x01\x12\x04\x96\x08\x08'\n\x0c\n\x04\x04S\x02\0\x12\x04\x97\
    \x08\x02\x1c\n\x0f\n\x05\x04S\x02\0\x04\x12\x06\x97\x08\x02\x96\x08)\n\r\
    \n\x05\x04S\x02\0\x06\x12\x04\x97\x08\x02\x10\n\r\n\x05\x04S\x02\0\x01\
    \x12\x04\x97\x08\x11\x17\n\r\n\x05\x04S\x02\0\x03\x12\x04\x97\x08\x1a\
    \x1b\n\x0c\n\x02\x04T\x12\x06\x9a\x08\0\x9c\x08\x01\n\x0b\n\x03\x04T\x01\
    \x12\x04\x9a\x08\x08(\n\x0c\n\x04\x04T\x02\0\x12\x04\x9b\x08\x02\x1c\n\
    \x0f\n\x05\x04T\x02\0\x04\x12\x06\x9b\x08\x02\x9a\x08*\n\r\n\x05\x04T\
    \x02\0\x06\x12\x04\x9b\x08\x02\x10\n\r\n\x05\x04T\x02\0\x01\x12\x04\x9b\
    \x08\x11\x17\n\r\n\x05\x04T\x02\0\x03\x12\x04\x9b\x08\x1a\x1bb\x06proto3\
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
