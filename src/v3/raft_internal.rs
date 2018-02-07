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
pub struct RequestHeader {
    // message fields
    pub ID: u64,
    pub username: ::std::string::String,
    pub auth_revision: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestHeader {}

impl RequestHeader {
    pub fn new() -> RequestHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestHeader {
        static mut instance: ::protobuf::lazy::Lazy<RequestHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestHeader,
        };
        unsafe {
            instance.get(RequestHeader::new)
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

    // string username = 2;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // uint64 auth_revision = 3;

    pub fn clear_auth_revision(&mut self) {
        self.auth_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_auth_revision(&mut self, v: u64) {
        self.auth_revision = v;
    }

    pub fn get_auth_revision(&self) -> u64 {
        self.auth_revision
    }

    fn get_auth_revision_for_reflect(&self) -> &u64 {
        &self.auth_revision
    }

    fn mut_auth_revision_for_reflect(&mut self) -> &mut u64 {
        &mut self.auth_revision
    }
}

impl ::protobuf::Message for RequestHeader {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.auth_revision = tmp;
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
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.username);
        }
        if self.auth_revision != 0 {
            my_size += ::protobuf::rt::value_size(3, self.auth_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_uint64(1, self.ID)?;
        }
        if !self.username.is_empty() {
            os.write_string(2, &self.username)?;
        }
        if self.auth_revision != 0 {
            os.write_uint64(3, self.auth_revision)?;
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

impl ::protobuf::MessageStatic for RequestHeader {
    fn new() -> RequestHeader {
        RequestHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ID",
                    RequestHeader::get_ID_for_reflect,
                    RequestHeader::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    RequestHeader::get_username_for_reflect,
                    RequestHeader::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "auth_revision",
                    RequestHeader::get_auth_revision_for_reflect,
                    RequestHeader::mut_auth_revision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestHeader>(
                    "RequestHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestHeader {
    fn clear(&mut self) {
        self.clear_ID();
        self.clear_username();
        self.clear_auth_revision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InternalRaftRequest {
    // message fields
    pub header: ::protobuf::SingularPtrField<RequestHeader>,
    pub ID: u64,
    pub v2: ::protobuf::SingularPtrField<super::etcdserver::Request>,
    pub range: ::protobuf::SingularPtrField<super::rpc::RangeRequest>,
    pub put: ::protobuf::SingularPtrField<super::rpc::PutRequest>,
    pub delete_range: ::protobuf::SingularPtrField<super::rpc::DeleteRangeRequest>,
    pub txn: ::protobuf::SingularPtrField<super::rpc::TxnRequest>,
    pub compaction: ::protobuf::SingularPtrField<super::rpc::CompactionRequest>,
    pub lease_grant: ::protobuf::SingularPtrField<super::rpc::LeaseGrantRequest>,
    pub lease_revoke: ::protobuf::SingularPtrField<super::rpc::LeaseRevokeRequest>,
    pub alarm: ::protobuf::SingularPtrField<super::rpc::AlarmRequest>,
    pub auth_enable: ::protobuf::SingularPtrField<super::rpc::AuthEnableRequest>,
    pub auth_disable: ::protobuf::SingularPtrField<super::rpc::AuthDisableRequest>,
    pub authenticate: ::protobuf::SingularPtrField<InternalAuthenticateRequest>,
    pub auth_user_add: ::protobuf::SingularPtrField<super::rpc::AuthUserAddRequest>,
    pub auth_user_delete: ::protobuf::SingularPtrField<super::rpc::AuthUserDeleteRequest>,
    pub auth_user_get: ::protobuf::SingularPtrField<super::rpc::AuthUserGetRequest>,
    pub auth_user_change_password: ::protobuf::SingularPtrField<super::rpc::AuthUserChangePasswordRequest>,
    pub auth_user_grant_role: ::protobuf::SingularPtrField<super::rpc::AuthUserGrantRoleRequest>,
    pub auth_user_revoke_role: ::protobuf::SingularPtrField<super::rpc::AuthUserRevokeRoleRequest>,
    pub auth_user_list: ::protobuf::SingularPtrField<super::rpc::AuthUserListRequest>,
    pub auth_role_list: ::protobuf::SingularPtrField<super::rpc::AuthRoleListRequest>,
    pub auth_role_add: ::protobuf::SingularPtrField<super::rpc::AuthRoleAddRequest>,
    pub auth_role_delete: ::protobuf::SingularPtrField<super::rpc::AuthRoleDeleteRequest>,
    pub auth_role_get: ::protobuf::SingularPtrField<super::rpc::AuthRoleGetRequest>,
    pub auth_role_grant_permission: ::protobuf::SingularPtrField<super::rpc::AuthRoleGrantPermissionRequest>,
    pub auth_role_revoke_permission: ::protobuf::SingularPtrField<super::rpc::AuthRoleRevokePermissionRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InternalRaftRequest {}

impl InternalRaftRequest {
    pub fn new() -> InternalRaftRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalRaftRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalRaftRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalRaftRequest,
        };
        unsafe {
            instance.get(InternalRaftRequest::new)
        }
    }

    // .etcdserverpb.RequestHeader header = 100;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header(&self) -> &RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<RequestHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RequestHeader> {
        &mut self.header
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

    // .etcdserverpb.Request v2 = 2;

    pub fn clear_v2(&mut self) {
        self.v2.clear();
    }

    pub fn has_v2(&self) -> bool {
        self.v2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v2(&mut self, v: super::etcdserver::Request) {
        self.v2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v2(&mut self) -> &mut super::etcdserver::Request {
        if self.v2.is_none() {
            self.v2.set_default();
        }
        self.v2.as_mut().unwrap()
    }

    // Take field
    pub fn take_v2(&mut self) -> super::etcdserver::Request {
        self.v2.take().unwrap_or_else(|| super::etcdserver::Request::new())
    }

    pub fn get_v2(&self) -> &super::etcdserver::Request {
        self.v2.as_ref().unwrap_or_else(|| super::etcdserver::Request::default_instance())
    }

    fn get_v2_for_reflect(&self) -> &::protobuf::SingularPtrField<super::etcdserver::Request> {
        &self.v2
    }

    fn mut_v2_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::etcdserver::Request> {
        &mut self.v2
    }

    // .etcdserverpb.RangeRequest range = 3;

    pub fn clear_range(&mut self) {
        self.range.clear();
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: super::rpc::RangeRequest) {
        self.range = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range(&mut self) -> &mut super::rpc::RangeRequest {
        if self.range.is_none() {
            self.range.set_default();
        }
        self.range.as_mut().unwrap()
    }

    // Take field
    pub fn take_range(&mut self) -> super::rpc::RangeRequest {
        self.range.take().unwrap_or_else(|| super::rpc::RangeRequest::new())
    }

    pub fn get_range(&self) -> &super::rpc::RangeRequest {
        self.range.as_ref().unwrap_or_else(|| super::rpc::RangeRequest::default_instance())
    }

    fn get_range_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::RangeRequest> {
        &self.range
    }

    fn mut_range_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::RangeRequest> {
        &mut self.range
    }

    // .etcdserverpb.PutRequest put = 4;

    pub fn clear_put(&mut self) {
        self.put.clear();
    }

    pub fn has_put(&self) -> bool {
        self.put.is_some()
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: super::rpc::PutRequest) {
        self.put = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put(&mut self) -> &mut super::rpc::PutRequest {
        if self.put.is_none() {
            self.put.set_default();
        }
        self.put.as_mut().unwrap()
    }

    // Take field
    pub fn take_put(&mut self) -> super::rpc::PutRequest {
        self.put.take().unwrap_or_else(|| super::rpc::PutRequest::new())
    }

    pub fn get_put(&self) -> &super::rpc::PutRequest {
        self.put.as_ref().unwrap_or_else(|| super::rpc::PutRequest::default_instance())
    }

    fn get_put_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::PutRequest> {
        &self.put
    }

    fn mut_put_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::PutRequest> {
        &mut self.put
    }

    // .etcdserverpb.DeleteRangeRequest delete_range = 5;

    pub fn clear_delete_range(&mut self) {
        self.delete_range.clear();
    }

    pub fn has_delete_range(&self) -> bool {
        self.delete_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete_range(&mut self, v: super::rpc::DeleteRangeRequest) {
        self.delete_range = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_range(&mut self) -> &mut super::rpc::DeleteRangeRequest {
        if self.delete_range.is_none() {
            self.delete_range.set_default();
        }
        self.delete_range.as_mut().unwrap()
    }

    // Take field
    pub fn take_delete_range(&mut self) -> super::rpc::DeleteRangeRequest {
        self.delete_range.take().unwrap_or_else(|| super::rpc::DeleteRangeRequest::new())
    }

    pub fn get_delete_range(&self) -> &super::rpc::DeleteRangeRequest {
        self.delete_range.as_ref().unwrap_or_else(|| super::rpc::DeleteRangeRequest::default_instance())
    }

    fn get_delete_range_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::DeleteRangeRequest> {
        &self.delete_range
    }

    fn mut_delete_range_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::DeleteRangeRequest> {
        &mut self.delete_range
    }

    // .etcdserverpb.TxnRequest txn = 6;

    pub fn clear_txn(&mut self) {
        self.txn.clear();
    }

    pub fn has_txn(&self) -> bool {
        self.txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn(&mut self, v: super::rpc::TxnRequest) {
        self.txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txn(&mut self) -> &mut super::rpc::TxnRequest {
        if self.txn.is_none() {
            self.txn.set_default();
        }
        self.txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_txn(&mut self) -> super::rpc::TxnRequest {
        self.txn.take().unwrap_or_else(|| super::rpc::TxnRequest::new())
    }

    pub fn get_txn(&self) -> &super::rpc::TxnRequest {
        self.txn.as_ref().unwrap_or_else(|| super::rpc::TxnRequest::default_instance())
    }

    fn get_txn_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::TxnRequest> {
        &self.txn
    }

    fn mut_txn_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::TxnRequest> {
        &mut self.txn
    }

    // .etcdserverpb.CompactionRequest compaction = 7;

    pub fn clear_compaction(&mut self) {
        self.compaction.clear();
    }

    pub fn has_compaction(&self) -> bool {
        self.compaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compaction(&mut self, v: super::rpc::CompactionRequest) {
        self.compaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_compaction(&mut self) -> &mut super::rpc::CompactionRequest {
        if self.compaction.is_none() {
            self.compaction.set_default();
        }
        self.compaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_compaction(&mut self) -> super::rpc::CompactionRequest {
        self.compaction.take().unwrap_or_else(|| super::rpc::CompactionRequest::new())
    }

    pub fn get_compaction(&self) -> &super::rpc::CompactionRequest {
        self.compaction.as_ref().unwrap_or_else(|| super::rpc::CompactionRequest::default_instance())
    }

    fn get_compaction_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::CompactionRequest> {
        &self.compaction
    }

    fn mut_compaction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::CompactionRequest> {
        &mut self.compaction
    }

    // .etcdserverpb.LeaseGrantRequest lease_grant = 8;

    pub fn clear_lease_grant(&mut self) {
        self.lease_grant.clear();
    }

    pub fn has_lease_grant(&self) -> bool {
        self.lease_grant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lease_grant(&mut self, v: super::rpc::LeaseGrantRequest) {
        self.lease_grant = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lease_grant(&mut self) -> &mut super::rpc::LeaseGrantRequest {
        if self.lease_grant.is_none() {
            self.lease_grant.set_default();
        }
        self.lease_grant.as_mut().unwrap()
    }

    // Take field
    pub fn take_lease_grant(&mut self) -> super::rpc::LeaseGrantRequest {
        self.lease_grant.take().unwrap_or_else(|| super::rpc::LeaseGrantRequest::new())
    }

    pub fn get_lease_grant(&self) -> &super::rpc::LeaseGrantRequest {
        self.lease_grant.as_ref().unwrap_or_else(|| super::rpc::LeaseGrantRequest::default_instance())
    }

    fn get_lease_grant_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::LeaseGrantRequest> {
        &self.lease_grant
    }

    fn mut_lease_grant_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::LeaseGrantRequest> {
        &mut self.lease_grant
    }

    // .etcdserverpb.LeaseRevokeRequest lease_revoke = 9;

    pub fn clear_lease_revoke(&mut self) {
        self.lease_revoke.clear();
    }

    pub fn has_lease_revoke(&self) -> bool {
        self.lease_revoke.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lease_revoke(&mut self, v: super::rpc::LeaseRevokeRequest) {
        self.lease_revoke = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lease_revoke(&mut self) -> &mut super::rpc::LeaseRevokeRequest {
        if self.lease_revoke.is_none() {
            self.lease_revoke.set_default();
        }
        self.lease_revoke.as_mut().unwrap()
    }

    // Take field
    pub fn take_lease_revoke(&mut self) -> super::rpc::LeaseRevokeRequest {
        self.lease_revoke.take().unwrap_or_else(|| super::rpc::LeaseRevokeRequest::new())
    }

    pub fn get_lease_revoke(&self) -> &super::rpc::LeaseRevokeRequest {
        self.lease_revoke.as_ref().unwrap_or_else(|| super::rpc::LeaseRevokeRequest::default_instance())
    }

    fn get_lease_revoke_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::LeaseRevokeRequest> {
        &self.lease_revoke
    }

    fn mut_lease_revoke_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::LeaseRevokeRequest> {
        &mut self.lease_revoke
    }

    // .etcdserverpb.AlarmRequest alarm = 10;

    pub fn clear_alarm(&mut self) {
        self.alarm.clear();
    }

    pub fn has_alarm(&self) -> bool {
        self.alarm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alarm(&mut self, v: super::rpc::AlarmRequest) {
        self.alarm = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alarm(&mut self) -> &mut super::rpc::AlarmRequest {
        if self.alarm.is_none() {
            self.alarm.set_default();
        }
        self.alarm.as_mut().unwrap()
    }

    // Take field
    pub fn take_alarm(&mut self) -> super::rpc::AlarmRequest {
        self.alarm.take().unwrap_or_else(|| super::rpc::AlarmRequest::new())
    }

    pub fn get_alarm(&self) -> &super::rpc::AlarmRequest {
        self.alarm.as_ref().unwrap_or_else(|| super::rpc::AlarmRequest::default_instance())
    }

    fn get_alarm_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AlarmRequest> {
        &self.alarm
    }

    fn mut_alarm_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AlarmRequest> {
        &mut self.alarm
    }

    // .etcdserverpb.AuthEnableRequest auth_enable = 1000;

    pub fn clear_auth_enable(&mut self) {
        self.auth_enable.clear();
    }

    pub fn has_auth_enable(&self) -> bool {
        self.auth_enable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_enable(&mut self, v: super::rpc::AuthEnableRequest) {
        self.auth_enable = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_enable(&mut self) -> &mut super::rpc::AuthEnableRequest {
        if self.auth_enable.is_none() {
            self.auth_enable.set_default();
        }
        self.auth_enable.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_enable(&mut self) -> super::rpc::AuthEnableRequest {
        self.auth_enable.take().unwrap_or_else(|| super::rpc::AuthEnableRequest::new())
    }

    pub fn get_auth_enable(&self) -> &super::rpc::AuthEnableRequest {
        self.auth_enable.as_ref().unwrap_or_else(|| super::rpc::AuthEnableRequest::default_instance())
    }

    fn get_auth_enable_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthEnableRequest> {
        &self.auth_enable
    }

    fn mut_auth_enable_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthEnableRequest> {
        &mut self.auth_enable
    }

    // .etcdserverpb.AuthDisableRequest auth_disable = 1011;

    pub fn clear_auth_disable(&mut self) {
        self.auth_disable.clear();
    }

    pub fn has_auth_disable(&self) -> bool {
        self.auth_disable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_disable(&mut self, v: super::rpc::AuthDisableRequest) {
        self.auth_disable = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_disable(&mut self) -> &mut super::rpc::AuthDisableRequest {
        if self.auth_disable.is_none() {
            self.auth_disable.set_default();
        }
        self.auth_disable.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_disable(&mut self) -> super::rpc::AuthDisableRequest {
        self.auth_disable.take().unwrap_or_else(|| super::rpc::AuthDisableRequest::new())
    }

    pub fn get_auth_disable(&self) -> &super::rpc::AuthDisableRequest {
        self.auth_disable.as_ref().unwrap_or_else(|| super::rpc::AuthDisableRequest::default_instance())
    }

    fn get_auth_disable_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthDisableRequest> {
        &self.auth_disable
    }

    fn mut_auth_disable_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthDisableRequest> {
        &mut self.auth_disable
    }

    // .etcdserverpb.InternalAuthenticateRequest authenticate = 1012;

    pub fn clear_authenticate(&mut self) {
        self.authenticate.clear();
    }

    pub fn has_authenticate(&self) -> bool {
        self.authenticate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_authenticate(&mut self, v: InternalAuthenticateRequest) {
        self.authenticate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_authenticate(&mut self) -> &mut InternalAuthenticateRequest {
        if self.authenticate.is_none() {
            self.authenticate.set_default();
        }
        self.authenticate.as_mut().unwrap()
    }

    // Take field
    pub fn take_authenticate(&mut self) -> InternalAuthenticateRequest {
        self.authenticate.take().unwrap_or_else(|| InternalAuthenticateRequest::new())
    }

    pub fn get_authenticate(&self) -> &InternalAuthenticateRequest {
        self.authenticate.as_ref().unwrap_or_else(|| InternalAuthenticateRequest::default_instance())
    }

    fn get_authenticate_for_reflect(&self) -> &::protobuf::SingularPtrField<InternalAuthenticateRequest> {
        &self.authenticate
    }

    fn mut_authenticate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<InternalAuthenticateRequest> {
        &mut self.authenticate
    }

    // .etcdserverpb.AuthUserAddRequest auth_user_add = 1100;

    pub fn clear_auth_user_add(&mut self) {
        self.auth_user_add.clear();
    }

    pub fn has_auth_user_add(&self) -> bool {
        self.auth_user_add.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_user_add(&mut self, v: super::rpc::AuthUserAddRequest) {
        self.auth_user_add = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_user_add(&mut self) -> &mut super::rpc::AuthUserAddRequest {
        if self.auth_user_add.is_none() {
            self.auth_user_add.set_default();
        }
        self.auth_user_add.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_user_add(&mut self) -> super::rpc::AuthUserAddRequest {
        self.auth_user_add.take().unwrap_or_else(|| super::rpc::AuthUserAddRequest::new())
    }

    pub fn get_auth_user_add(&self) -> &super::rpc::AuthUserAddRequest {
        self.auth_user_add.as_ref().unwrap_or_else(|| super::rpc::AuthUserAddRequest::default_instance())
    }

    fn get_auth_user_add_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthUserAddRequest> {
        &self.auth_user_add
    }

    fn mut_auth_user_add_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthUserAddRequest> {
        &mut self.auth_user_add
    }

    // .etcdserverpb.AuthUserDeleteRequest auth_user_delete = 1101;

    pub fn clear_auth_user_delete(&mut self) {
        self.auth_user_delete.clear();
    }

    pub fn has_auth_user_delete(&self) -> bool {
        self.auth_user_delete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_user_delete(&mut self, v: super::rpc::AuthUserDeleteRequest) {
        self.auth_user_delete = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_user_delete(&mut self) -> &mut super::rpc::AuthUserDeleteRequest {
        if self.auth_user_delete.is_none() {
            self.auth_user_delete.set_default();
        }
        self.auth_user_delete.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_user_delete(&mut self) -> super::rpc::AuthUserDeleteRequest {
        self.auth_user_delete.take().unwrap_or_else(|| super::rpc::AuthUserDeleteRequest::new())
    }

    pub fn get_auth_user_delete(&self) -> &super::rpc::AuthUserDeleteRequest {
        self.auth_user_delete.as_ref().unwrap_or_else(|| super::rpc::AuthUserDeleteRequest::default_instance())
    }

    fn get_auth_user_delete_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthUserDeleteRequest> {
        &self.auth_user_delete
    }

    fn mut_auth_user_delete_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthUserDeleteRequest> {
        &mut self.auth_user_delete
    }

    // .etcdserverpb.AuthUserGetRequest auth_user_get = 1102;

    pub fn clear_auth_user_get(&mut self) {
        self.auth_user_get.clear();
    }

    pub fn has_auth_user_get(&self) -> bool {
        self.auth_user_get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_user_get(&mut self, v: super::rpc::AuthUserGetRequest) {
        self.auth_user_get = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_user_get(&mut self) -> &mut super::rpc::AuthUserGetRequest {
        if self.auth_user_get.is_none() {
            self.auth_user_get.set_default();
        }
        self.auth_user_get.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_user_get(&mut self) -> super::rpc::AuthUserGetRequest {
        self.auth_user_get.take().unwrap_or_else(|| super::rpc::AuthUserGetRequest::new())
    }

    pub fn get_auth_user_get(&self) -> &super::rpc::AuthUserGetRequest {
        self.auth_user_get.as_ref().unwrap_or_else(|| super::rpc::AuthUserGetRequest::default_instance())
    }

    fn get_auth_user_get_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthUserGetRequest> {
        &self.auth_user_get
    }

    fn mut_auth_user_get_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthUserGetRequest> {
        &mut self.auth_user_get
    }

    // .etcdserverpb.AuthUserChangePasswordRequest auth_user_change_password = 1103;

    pub fn clear_auth_user_change_password(&mut self) {
        self.auth_user_change_password.clear();
    }

    pub fn has_auth_user_change_password(&self) -> bool {
        self.auth_user_change_password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_user_change_password(&mut self, v: super::rpc::AuthUserChangePasswordRequest) {
        self.auth_user_change_password = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_user_change_password(&mut self) -> &mut super::rpc::AuthUserChangePasswordRequest {
        if self.auth_user_change_password.is_none() {
            self.auth_user_change_password.set_default();
        }
        self.auth_user_change_password.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_user_change_password(&mut self) -> super::rpc::AuthUserChangePasswordRequest {
        self.auth_user_change_password.take().unwrap_or_else(|| super::rpc::AuthUserChangePasswordRequest::new())
    }

    pub fn get_auth_user_change_password(&self) -> &super::rpc::AuthUserChangePasswordRequest {
        self.auth_user_change_password.as_ref().unwrap_or_else(|| super::rpc::AuthUserChangePasswordRequest::default_instance())
    }

    fn get_auth_user_change_password_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthUserChangePasswordRequest> {
        &self.auth_user_change_password
    }

    fn mut_auth_user_change_password_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthUserChangePasswordRequest> {
        &mut self.auth_user_change_password
    }

    // .etcdserverpb.AuthUserGrantRoleRequest auth_user_grant_role = 1104;

    pub fn clear_auth_user_grant_role(&mut self) {
        self.auth_user_grant_role.clear();
    }

    pub fn has_auth_user_grant_role(&self) -> bool {
        self.auth_user_grant_role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_user_grant_role(&mut self, v: super::rpc::AuthUserGrantRoleRequest) {
        self.auth_user_grant_role = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_user_grant_role(&mut self) -> &mut super::rpc::AuthUserGrantRoleRequest {
        if self.auth_user_grant_role.is_none() {
            self.auth_user_grant_role.set_default();
        }
        self.auth_user_grant_role.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_user_grant_role(&mut self) -> super::rpc::AuthUserGrantRoleRequest {
        self.auth_user_grant_role.take().unwrap_or_else(|| super::rpc::AuthUserGrantRoleRequest::new())
    }

    pub fn get_auth_user_grant_role(&self) -> &super::rpc::AuthUserGrantRoleRequest {
        self.auth_user_grant_role.as_ref().unwrap_or_else(|| super::rpc::AuthUserGrantRoleRequest::default_instance())
    }

    fn get_auth_user_grant_role_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthUserGrantRoleRequest> {
        &self.auth_user_grant_role
    }

    fn mut_auth_user_grant_role_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthUserGrantRoleRequest> {
        &mut self.auth_user_grant_role
    }

    // .etcdserverpb.AuthUserRevokeRoleRequest auth_user_revoke_role = 1105;

    pub fn clear_auth_user_revoke_role(&mut self) {
        self.auth_user_revoke_role.clear();
    }

    pub fn has_auth_user_revoke_role(&self) -> bool {
        self.auth_user_revoke_role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_user_revoke_role(&mut self, v: super::rpc::AuthUserRevokeRoleRequest) {
        self.auth_user_revoke_role = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_user_revoke_role(&mut self) -> &mut super::rpc::AuthUserRevokeRoleRequest {
        if self.auth_user_revoke_role.is_none() {
            self.auth_user_revoke_role.set_default();
        }
        self.auth_user_revoke_role.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_user_revoke_role(&mut self) -> super::rpc::AuthUserRevokeRoleRequest {
        self.auth_user_revoke_role.take().unwrap_or_else(|| super::rpc::AuthUserRevokeRoleRequest::new())
    }

    pub fn get_auth_user_revoke_role(&self) -> &super::rpc::AuthUserRevokeRoleRequest {
        self.auth_user_revoke_role.as_ref().unwrap_or_else(|| super::rpc::AuthUserRevokeRoleRequest::default_instance())
    }

    fn get_auth_user_revoke_role_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthUserRevokeRoleRequest> {
        &self.auth_user_revoke_role
    }

    fn mut_auth_user_revoke_role_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthUserRevokeRoleRequest> {
        &mut self.auth_user_revoke_role
    }

    // .etcdserverpb.AuthUserListRequest auth_user_list = 1106;

    pub fn clear_auth_user_list(&mut self) {
        self.auth_user_list.clear();
    }

    pub fn has_auth_user_list(&self) -> bool {
        self.auth_user_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_user_list(&mut self, v: super::rpc::AuthUserListRequest) {
        self.auth_user_list = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_user_list(&mut self) -> &mut super::rpc::AuthUserListRequest {
        if self.auth_user_list.is_none() {
            self.auth_user_list.set_default();
        }
        self.auth_user_list.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_user_list(&mut self) -> super::rpc::AuthUserListRequest {
        self.auth_user_list.take().unwrap_or_else(|| super::rpc::AuthUserListRequest::new())
    }

    pub fn get_auth_user_list(&self) -> &super::rpc::AuthUserListRequest {
        self.auth_user_list.as_ref().unwrap_or_else(|| super::rpc::AuthUserListRequest::default_instance())
    }

    fn get_auth_user_list_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthUserListRequest> {
        &self.auth_user_list
    }

    fn mut_auth_user_list_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthUserListRequest> {
        &mut self.auth_user_list
    }

    // .etcdserverpb.AuthRoleListRequest auth_role_list = 1107;

    pub fn clear_auth_role_list(&mut self) {
        self.auth_role_list.clear();
    }

    pub fn has_auth_role_list(&self) -> bool {
        self.auth_role_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_role_list(&mut self, v: super::rpc::AuthRoleListRequest) {
        self.auth_role_list = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_role_list(&mut self) -> &mut super::rpc::AuthRoleListRequest {
        if self.auth_role_list.is_none() {
            self.auth_role_list.set_default();
        }
        self.auth_role_list.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_role_list(&mut self) -> super::rpc::AuthRoleListRequest {
        self.auth_role_list.take().unwrap_or_else(|| super::rpc::AuthRoleListRequest::new())
    }

    pub fn get_auth_role_list(&self) -> &super::rpc::AuthRoleListRequest {
        self.auth_role_list.as_ref().unwrap_or_else(|| super::rpc::AuthRoleListRequest::default_instance())
    }

    fn get_auth_role_list_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthRoleListRequest> {
        &self.auth_role_list
    }

    fn mut_auth_role_list_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthRoleListRequest> {
        &mut self.auth_role_list
    }

    // .etcdserverpb.AuthRoleAddRequest auth_role_add = 1200;

    pub fn clear_auth_role_add(&mut self) {
        self.auth_role_add.clear();
    }

    pub fn has_auth_role_add(&self) -> bool {
        self.auth_role_add.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_role_add(&mut self, v: super::rpc::AuthRoleAddRequest) {
        self.auth_role_add = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_role_add(&mut self) -> &mut super::rpc::AuthRoleAddRequest {
        if self.auth_role_add.is_none() {
            self.auth_role_add.set_default();
        }
        self.auth_role_add.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_role_add(&mut self) -> super::rpc::AuthRoleAddRequest {
        self.auth_role_add.take().unwrap_or_else(|| super::rpc::AuthRoleAddRequest::new())
    }

    pub fn get_auth_role_add(&self) -> &super::rpc::AuthRoleAddRequest {
        self.auth_role_add.as_ref().unwrap_or_else(|| super::rpc::AuthRoleAddRequest::default_instance())
    }

    fn get_auth_role_add_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthRoleAddRequest> {
        &self.auth_role_add
    }

    fn mut_auth_role_add_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthRoleAddRequest> {
        &mut self.auth_role_add
    }

    // .etcdserverpb.AuthRoleDeleteRequest auth_role_delete = 1201;

    pub fn clear_auth_role_delete(&mut self) {
        self.auth_role_delete.clear();
    }

    pub fn has_auth_role_delete(&self) -> bool {
        self.auth_role_delete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_role_delete(&mut self, v: super::rpc::AuthRoleDeleteRequest) {
        self.auth_role_delete = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_role_delete(&mut self) -> &mut super::rpc::AuthRoleDeleteRequest {
        if self.auth_role_delete.is_none() {
            self.auth_role_delete.set_default();
        }
        self.auth_role_delete.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_role_delete(&mut self) -> super::rpc::AuthRoleDeleteRequest {
        self.auth_role_delete.take().unwrap_or_else(|| super::rpc::AuthRoleDeleteRequest::new())
    }

    pub fn get_auth_role_delete(&self) -> &super::rpc::AuthRoleDeleteRequest {
        self.auth_role_delete.as_ref().unwrap_or_else(|| super::rpc::AuthRoleDeleteRequest::default_instance())
    }

    fn get_auth_role_delete_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthRoleDeleteRequest> {
        &self.auth_role_delete
    }

    fn mut_auth_role_delete_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthRoleDeleteRequest> {
        &mut self.auth_role_delete
    }

    // .etcdserverpb.AuthRoleGetRequest auth_role_get = 1202;

    pub fn clear_auth_role_get(&mut self) {
        self.auth_role_get.clear();
    }

    pub fn has_auth_role_get(&self) -> bool {
        self.auth_role_get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_role_get(&mut self, v: super::rpc::AuthRoleGetRequest) {
        self.auth_role_get = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_role_get(&mut self) -> &mut super::rpc::AuthRoleGetRequest {
        if self.auth_role_get.is_none() {
            self.auth_role_get.set_default();
        }
        self.auth_role_get.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_role_get(&mut self) -> super::rpc::AuthRoleGetRequest {
        self.auth_role_get.take().unwrap_or_else(|| super::rpc::AuthRoleGetRequest::new())
    }

    pub fn get_auth_role_get(&self) -> &super::rpc::AuthRoleGetRequest {
        self.auth_role_get.as_ref().unwrap_or_else(|| super::rpc::AuthRoleGetRequest::default_instance())
    }

    fn get_auth_role_get_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthRoleGetRequest> {
        &self.auth_role_get
    }

    fn mut_auth_role_get_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthRoleGetRequest> {
        &mut self.auth_role_get
    }

    // .etcdserverpb.AuthRoleGrantPermissionRequest auth_role_grant_permission = 1203;

    pub fn clear_auth_role_grant_permission(&mut self) {
        self.auth_role_grant_permission.clear();
    }

    pub fn has_auth_role_grant_permission(&self) -> bool {
        self.auth_role_grant_permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_role_grant_permission(&mut self, v: super::rpc::AuthRoleGrantPermissionRequest) {
        self.auth_role_grant_permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_role_grant_permission(&mut self) -> &mut super::rpc::AuthRoleGrantPermissionRequest {
        if self.auth_role_grant_permission.is_none() {
            self.auth_role_grant_permission.set_default();
        }
        self.auth_role_grant_permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_role_grant_permission(&mut self) -> super::rpc::AuthRoleGrantPermissionRequest {
        self.auth_role_grant_permission.take().unwrap_or_else(|| super::rpc::AuthRoleGrantPermissionRequest::new())
    }

    pub fn get_auth_role_grant_permission(&self) -> &super::rpc::AuthRoleGrantPermissionRequest {
        self.auth_role_grant_permission.as_ref().unwrap_or_else(|| super::rpc::AuthRoleGrantPermissionRequest::default_instance())
    }

    fn get_auth_role_grant_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthRoleGrantPermissionRequest> {
        &self.auth_role_grant_permission
    }

    fn mut_auth_role_grant_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthRoleGrantPermissionRequest> {
        &mut self.auth_role_grant_permission
    }

    // .etcdserverpb.AuthRoleRevokePermissionRequest auth_role_revoke_permission = 1204;

    pub fn clear_auth_role_revoke_permission(&mut self) {
        self.auth_role_revoke_permission.clear();
    }

    pub fn has_auth_role_revoke_permission(&self) -> bool {
        self.auth_role_revoke_permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_role_revoke_permission(&mut self, v: super::rpc::AuthRoleRevokePermissionRequest) {
        self.auth_role_revoke_permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_role_revoke_permission(&mut self) -> &mut super::rpc::AuthRoleRevokePermissionRequest {
        if self.auth_role_revoke_permission.is_none() {
            self.auth_role_revoke_permission.set_default();
        }
        self.auth_role_revoke_permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_role_revoke_permission(&mut self) -> super::rpc::AuthRoleRevokePermissionRequest {
        self.auth_role_revoke_permission.take().unwrap_or_else(|| super::rpc::AuthRoleRevokePermissionRequest::new())
    }

    pub fn get_auth_role_revoke_permission(&self) -> &super::rpc::AuthRoleRevokePermissionRequest {
        self.auth_role_revoke_permission.as_ref().unwrap_or_else(|| super::rpc::AuthRoleRevokePermissionRequest::default_instance())
    }

    fn get_auth_role_revoke_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::AuthRoleRevokePermissionRequest> {
        &self.auth_role_revoke_permission
    }

    fn mut_auth_role_revoke_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::AuthRoleRevokePermissionRequest> {
        &mut self.auth_role_revoke_permission
    }
}

impl ::protobuf::Message for InternalRaftRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.v2 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.range {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.put {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.delete_range {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.txn {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.compaction {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lease_grant {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lease_revoke {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.alarm {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_enable {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_disable {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.authenticate {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_user_add {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_user_delete {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_user_get {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_user_change_password {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_user_grant_role {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_user_revoke_role {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_user_list {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_role_list {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_role_add {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_role_delete {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_role_get {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_role_grant_permission {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.auth_role_revoke_permission {
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
                100 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ID = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.v2)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.range)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.delete_range)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.txn)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.compaction)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lease_grant)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lease_revoke)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.alarm)?;
                },
                1000 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_enable)?;
                },
                1011 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_disable)?;
                },
                1012 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.authenticate)?;
                },
                1100 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_user_add)?;
                },
                1101 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_user_delete)?;
                },
                1102 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_user_get)?;
                },
                1103 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_user_change_password)?;
                },
                1104 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_user_grant_role)?;
                },
                1105 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_user_revoke_role)?;
                },
                1106 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_user_list)?;
                },
                1107 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_role_list)?;
                },
                1200 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_role_add)?;
                },
                1201 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_role_delete)?;
                },
                1202 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_role_get)?;
                },
                1203 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_role_grant_permission)?;
                },
                1204 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_role_revoke_permission)?;
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
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.v2.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.range.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.put.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.delete_range.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.txn.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.compaction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.lease_grant.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.lease_revoke.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.alarm.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_enable.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_disable.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.authenticate.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_user_add.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_user_delete.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_user_get.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_user_change_password.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_user_grant_role.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_user_revoke_role.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_user_list.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_role_list.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_role_add.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_role_delete.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_role_get.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_role_grant_permission.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.auth_role_revoke_permission.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(100, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.ID != 0 {
            os.write_uint64(1, self.ID)?;
        }
        if let Some(ref v) = self.v2.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.range.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.put.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.delete_range.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.txn.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.compaction.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.lease_grant.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.lease_revoke.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.alarm.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_enable.as_ref() {
            os.write_tag(1000, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_disable.as_ref() {
            os.write_tag(1011, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.authenticate.as_ref() {
            os.write_tag(1012, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_user_add.as_ref() {
            os.write_tag(1100, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_user_delete.as_ref() {
            os.write_tag(1101, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_user_get.as_ref() {
            os.write_tag(1102, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_user_change_password.as_ref() {
            os.write_tag(1103, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_user_grant_role.as_ref() {
            os.write_tag(1104, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_user_revoke_role.as_ref() {
            os.write_tag(1105, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_user_list.as_ref() {
            os.write_tag(1106, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_role_list.as_ref() {
            os.write_tag(1107, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_role_add.as_ref() {
            os.write_tag(1200, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_role_delete.as_ref() {
            os.write_tag(1201, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_role_get.as_ref() {
            os.write_tag(1202, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_role_grant_permission.as_ref() {
            os.write_tag(1203, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.auth_role_revoke_permission.as_ref() {
            os.write_tag(1204, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for InternalRaftRequest {
    fn new() -> InternalRaftRequest {
        InternalRaftRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalRaftRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestHeader>>(
                    "header",
                    InternalRaftRequest::get_header_for_reflect,
                    InternalRaftRequest::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ID",
                    InternalRaftRequest::get_ID_for_reflect,
                    InternalRaftRequest::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::etcdserver::Request>>(
                    "v2",
                    InternalRaftRequest::get_v2_for_reflect,
                    InternalRaftRequest::mut_v2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::RangeRequest>>(
                    "range",
                    InternalRaftRequest::get_range_for_reflect,
                    InternalRaftRequest::mut_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::PutRequest>>(
                    "put",
                    InternalRaftRequest::get_put_for_reflect,
                    InternalRaftRequest::mut_put_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::DeleteRangeRequest>>(
                    "delete_range",
                    InternalRaftRequest::get_delete_range_for_reflect,
                    InternalRaftRequest::mut_delete_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::TxnRequest>>(
                    "txn",
                    InternalRaftRequest::get_txn_for_reflect,
                    InternalRaftRequest::mut_txn_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::CompactionRequest>>(
                    "compaction",
                    InternalRaftRequest::get_compaction_for_reflect,
                    InternalRaftRequest::mut_compaction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::LeaseGrantRequest>>(
                    "lease_grant",
                    InternalRaftRequest::get_lease_grant_for_reflect,
                    InternalRaftRequest::mut_lease_grant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::LeaseRevokeRequest>>(
                    "lease_revoke",
                    InternalRaftRequest::get_lease_revoke_for_reflect,
                    InternalRaftRequest::mut_lease_revoke_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AlarmRequest>>(
                    "alarm",
                    InternalRaftRequest::get_alarm_for_reflect,
                    InternalRaftRequest::mut_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthEnableRequest>>(
                    "auth_enable",
                    InternalRaftRequest::get_auth_enable_for_reflect,
                    InternalRaftRequest::mut_auth_enable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthDisableRequest>>(
                    "auth_disable",
                    InternalRaftRequest::get_auth_disable_for_reflect,
                    InternalRaftRequest::mut_auth_disable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<InternalAuthenticateRequest>>(
                    "authenticate",
                    InternalRaftRequest::get_authenticate_for_reflect,
                    InternalRaftRequest::mut_authenticate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthUserAddRequest>>(
                    "auth_user_add",
                    InternalRaftRequest::get_auth_user_add_for_reflect,
                    InternalRaftRequest::mut_auth_user_add_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthUserDeleteRequest>>(
                    "auth_user_delete",
                    InternalRaftRequest::get_auth_user_delete_for_reflect,
                    InternalRaftRequest::mut_auth_user_delete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthUserGetRequest>>(
                    "auth_user_get",
                    InternalRaftRequest::get_auth_user_get_for_reflect,
                    InternalRaftRequest::mut_auth_user_get_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthUserChangePasswordRequest>>(
                    "auth_user_change_password",
                    InternalRaftRequest::get_auth_user_change_password_for_reflect,
                    InternalRaftRequest::mut_auth_user_change_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthUserGrantRoleRequest>>(
                    "auth_user_grant_role",
                    InternalRaftRequest::get_auth_user_grant_role_for_reflect,
                    InternalRaftRequest::mut_auth_user_grant_role_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthUserRevokeRoleRequest>>(
                    "auth_user_revoke_role",
                    InternalRaftRequest::get_auth_user_revoke_role_for_reflect,
                    InternalRaftRequest::mut_auth_user_revoke_role_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthUserListRequest>>(
                    "auth_user_list",
                    InternalRaftRequest::get_auth_user_list_for_reflect,
                    InternalRaftRequest::mut_auth_user_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthRoleListRequest>>(
                    "auth_role_list",
                    InternalRaftRequest::get_auth_role_list_for_reflect,
                    InternalRaftRequest::mut_auth_role_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthRoleAddRequest>>(
                    "auth_role_add",
                    InternalRaftRequest::get_auth_role_add_for_reflect,
                    InternalRaftRequest::mut_auth_role_add_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthRoleDeleteRequest>>(
                    "auth_role_delete",
                    InternalRaftRequest::get_auth_role_delete_for_reflect,
                    InternalRaftRequest::mut_auth_role_delete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthRoleGetRequest>>(
                    "auth_role_get",
                    InternalRaftRequest::get_auth_role_get_for_reflect,
                    InternalRaftRequest::mut_auth_role_get_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthRoleGrantPermissionRequest>>(
                    "auth_role_grant_permission",
                    InternalRaftRequest::get_auth_role_grant_permission_for_reflect,
                    InternalRaftRequest::mut_auth_role_grant_permission_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::AuthRoleRevokePermissionRequest>>(
                    "auth_role_revoke_permission",
                    InternalRaftRequest::get_auth_role_revoke_permission_for_reflect,
                    InternalRaftRequest::mut_auth_role_revoke_permission_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalRaftRequest>(
                    "InternalRaftRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalRaftRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_ID();
        self.clear_v2();
        self.clear_range();
        self.clear_put();
        self.clear_delete_range();
        self.clear_txn();
        self.clear_compaction();
        self.clear_lease_grant();
        self.clear_lease_revoke();
        self.clear_alarm();
        self.clear_auth_enable();
        self.clear_auth_disable();
        self.clear_authenticate();
        self.clear_auth_user_add();
        self.clear_auth_user_delete();
        self.clear_auth_user_get();
        self.clear_auth_user_change_password();
        self.clear_auth_user_grant_role();
        self.clear_auth_user_revoke_role();
        self.clear_auth_user_list();
        self.clear_auth_role_list();
        self.clear_auth_role_add();
        self.clear_auth_role_delete();
        self.clear_auth_role_get();
        self.clear_auth_role_grant_permission();
        self.clear_auth_role_revoke_permission();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InternalRaftRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InternalRaftRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EmptyResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EmptyResponse {}

impl EmptyResponse {
    pub fn new() -> EmptyResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EmptyResponse {
        static mut instance: ::protobuf::lazy::Lazy<EmptyResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EmptyResponse,
        };
        unsafe {
            instance.get(EmptyResponse::new)
        }
    }
}

impl ::protobuf::Message for EmptyResponse {
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

impl ::protobuf::MessageStatic for EmptyResponse {
    fn new() -> EmptyResponse {
        EmptyResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EmptyResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<EmptyResponse>(
                    "EmptyResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EmptyResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EmptyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EmptyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InternalAuthenticateRequest {
    // message fields
    pub name: ::std::string::String,
    pub password: ::std::string::String,
    pub simple_token: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InternalAuthenticateRequest {}

impl InternalAuthenticateRequest {
    pub fn new() -> InternalAuthenticateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalAuthenticateRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalAuthenticateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalAuthenticateRequest,
        };
        unsafe {
            instance.get(InternalAuthenticateRequest::new)
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

    // string simple_token = 3;

    pub fn clear_simple_token(&mut self) {
        self.simple_token.clear();
    }

    // Param is passed by value, moved
    pub fn set_simple_token(&mut self, v: ::std::string::String) {
        self.simple_token = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_simple_token(&mut self) -> &mut ::std::string::String {
        &mut self.simple_token
    }

    // Take field
    pub fn take_simple_token(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.simple_token, ::std::string::String::new())
    }

    pub fn get_simple_token(&self) -> &str {
        &self.simple_token
    }

    fn get_simple_token_for_reflect(&self) -> &::std::string::String {
        &self.simple_token
    }

    fn mut_simple_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.simple_token
    }
}

impl ::protobuf::Message for InternalAuthenticateRequest {
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
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.simple_token)?;
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
        if !self.simple_token.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.simple_token);
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
        if !self.simple_token.is_empty() {
            os.write_string(3, &self.simple_token)?;
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

impl ::protobuf::MessageStatic for InternalAuthenticateRequest {
    fn new() -> InternalAuthenticateRequest {
        InternalAuthenticateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalAuthenticateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    InternalAuthenticateRequest::get_name_for_reflect,
                    InternalAuthenticateRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "password",
                    InternalAuthenticateRequest::get_password_for_reflect,
                    InternalAuthenticateRequest::mut_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "simple_token",
                    InternalAuthenticateRequest::get_simple_token_for_reflect,
                    InternalAuthenticateRequest::mut_simple_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalAuthenticateRequest>(
                    "InternalAuthenticateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalAuthenticateRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_password();
        self.clear_simple_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InternalAuthenticateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InternalAuthenticateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13raft_internal.proto\x12\x0cetcdserverpb\x1a\x14gogoproto/gogo.prot\
    o\x1a\x10etcdserver.proto\x1a\trpc.proto\"`\n\rRequestHeader\x12\x0e\n\
    \x02ID\x18\x01\x20\x01(\x04R\x02ID\x12\x1a\n\x08username\x18\x02\x20\x01\
    (\tR\x08username\x12#\n\rauth_revision\x18\x03\x20\x01(\x04R\x0cauthRevi\
    sion\"\xee\x0e\n\x13InternalRaftRequest\x123\n\x06header\x18d\x20\x01(\
    \x0b2\x1b.etcdserverpb.RequestHeaderR\x06header\x12\x0e\n\x02ID\x18\x01\
    \x20\x01(\x04R\x02ID\x12%\n\x02v2\x18\x02\x20\x01(\x0b2\x15.etcdserverpb\
    .RequestR\x02v2\x120\n\x05range\x18\x03\x20\x01(\x0b2\x1a.etcdserverpb.R\
    angeRequestR\x05range\x12*\n\x03put\x18\x04\x20\x01(\x0b2\x18.etcdserver\
    pb.PutRequestR\x03put\x12C\n\x0cdelete_range\x18\x05\x20\x01(\x0b2\x20.e\
    tcdserverpb.DeleteRangeRequestR\x0bdeleteRange\x12*\n\x03txn\x18\x06\x20\
    \x01(\x0b2\x18.etcdserverpb.TxnRequestR\x03txn\x12?\n\ncompaction\x18\
    \x07\x20\x01(\x0b2\x1f.etcdserverpb.CompactionRequestR\ncompaction\x12@\
    \n\x0blease_grant\x18\x08\x20\x01(\x0b2\x1f.etcdserverpb.LeaseGrantReque\
    stR\nleaseGrant\x12C\n\x0clease_revoke\x18\t\x20\x01(\x0b2\x20.etcdserve\
    rpb.LeaseRevokeRequestR\x0bleaseRevoke\x120\n\x05alarm\x18\n\x20\x01(\
    \x0b2\x1a.etcdserverpb.AlarmRequestR\x05alarm\x12A\n\x0bauth_enable\x18\
    \xe8\x07\x20\x01(\x0b2\x1f.etcdserverpb.AuthEnableRequestR\nauthEnable\
    \x12D\n\x0cauth_disable\x18\xf3\x07\x20\x01(\x0b2\x20.etcdserverpb.AuthD\
    isableRequestR\x0bauthDisable\x12N\n\x0cauthenticate\x18\xf4\x07\x20\x01\
    (\x0b2).etcdserverpb.InternalAuthenticateRequestR\x0cauthenticate\x12E\n\
    \rauth_user_add\x18\xcc\x08\x20\x01(\x0b2\x20.etcdserverpb.AuthUserAddRe\
    questR\x0bauthUserAdd\x12N\n\x10auth_user_delete\x18\xcd\x08\x20\x01(\
    \x0b2#.etcdserverpb.AuthUserDeleteRequestR\x0eauthUserDelete\x12E\n\raut\
    h_user_get\x18\xce\x08\x20\x01(\x0b2\x20.etcdserverpb.AuthUserGetRequest\
    R\x0bauthUserGet\x12g\n\x19auth_user_change_password\x18\xcf\x08\x20\x01\
    (\x0b2+.etcdserverpb.AuthUserChangePasswordRequestR\x16authUserChangePas\
    sword\x12X\n\x14auth_user_grant_role\x18\xd0\x08\x20\x01(\x0b2&.etcdserv\
    erpb.AuthUserGrantRoleRequestR\x11authUserGrantRole\x12[\n\x15auth_user_\
    revoke_role\x18\xd1\x08\x20\x01(\x0b2'.etcdserverpb.AuthUserRevokeRoleRe\
    questR\x12authUserRevokeRole\x12H\n\x0eauth_user_list\x18\xd2\x08\x20\
    \x01(\x0b2!.etcdserverpb.AuthUserListRequestR\x0cauthUserList\x12H\n\x0e\
    auth_role_list\x18\xd3\x08\x20\x01(\x0b2!.etcdserverpb.AuthRoleListReque\
    stR\x0cauthRoleList\x12E\n\rauth_role_add\x18\xb0\t\x20\x01(\x0b2\x20.et\
    cdserverpb.AuthRoleAddRequestR\x0bauthRoleAdd\x12N\n\x10auth_role_delete\
    \x18\xb1\t\x20\x01(\x0b2#.etcdserverpb.AuthRoleDeleteRequestR\x0eauthRol\
    eDelete\x12E\n\rauth_role_get\x18\xb2\t\x20\x01(\x0b2\x20.etcdserverpb.A\
    uthRoleGetRequestR\x0bauthRoleGet\x12j\n\x1aauth_role_grant_permission\
    \x18\xb3\t\x20\x01(\x0b2,.etcdserverpb.AuthRoleGrantPermissionRequestR\
    \x17authRoleGrantPermission\x12m\n\x1bauth_role_revoke_permission\x18\
    \xb4\t\x20\x01(\x0b2-.etcdserverpb.AuthRoleRevokePermissionRequestR\x18a\
    uthRoleRevokePermission\"\x0f\n\rEmptyResponse\"p\n\x1bInternalAuthentic\
    ateRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x1a\n\x08pa\
    ssword\x18\x02\x20\x01(\tR\x08password\x12!\n\x0csimple_token\x18\x03\
    \x20\x01(\tR\x0bsimpleTokenB\x10\xe0\xe2\x1e\x01\xc8\xe1\x1e\0\xc8\xe2\
    \x1e\x01\xd0\xe2\x1e\x01J\xd4\x1a\n\x06\x12\x04\0\0H\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x14\n\t\n\x02\x03\0\x12\
    \x03\x03\x07\x1d\n\t\n\x02\x03\x01\x12\x03\x04\x07\x19\n\t\n\x02\x03\x02\
    \x12\x03\x05\x07\x12\n\x08\n\x01\x08\x12\x03\x07\0(\n\x0b\n\x04\x08\xe7\
    \x07\0\x12\x03\x07\0(\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x07\x07\x20\
    \n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x07\x07\x20\n\x0e\n\x07\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\x07\x08\x1f\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\
    \x03\x07#'\n\x08\n\x01\x08\x12\x03\x08\0$\n\x0b\n\x04\x08\xe7\x07\x01\
    \x12\x03\x08\0$\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x08\x07\x1c\n\r\
    \n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x08\x07\x1c\n\x0e\n\x07\x08\xe7\x07\
    \x01\x02\0\x01\x12\x03\x08\x08\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\
    \x03\x08\x1f#\n\x08\n\x01\x08\x12\x03\t\0*\n\x0b\n\x04\x08\xe7\x07\x02\
    \x12\x03\t\0*\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\t\x07\"\n\r\n\x06\
    \x08\xe7\x07\x02\x02\0\x12\x03\t\x07\"\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\
    \x01\x12\x03\t\x08!\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\t%)\n\x08\n\
    \x01\x08\x12\x03\n\0/\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\n\0/\n\x0c\n\
    \x05\x08\xe7\x07\x03\x02\x12\x03\n\x07&\n\r\n\x06\x08\xe7\x07\x03\x02\0\
    \x12\x03\n\x07&\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\n\x08%\n\
    \x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\n).\n\n\n\x02\x04\0\x12\x04\x0c\0\
    \x12\x01\n\n\n\x03\x04\0\x01\x12\x03\x0c\x08\x15\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\r\x02\x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\r\x02\x0c\x17\n\x0c\
    \n\x05\x04\0\x02\0\x05\x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\r\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x0e\x0f\n^\n\x04\x04\
    \0\x02\x01\x12\x03\x0f\x02\x16\x1aQ\x20username\x20is\x20a\x20username\
    \x20that\x20is\x20associated\x20with\x20an\x20auth\x20token\x20of\x20gRP\
    C\x20connection\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0f\x02\r\x10\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x0f\t\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0f\x14\
    \x15\n^\n\x04\x04\0\x02\x02\x12\x03\x11\x02\x1b\x1aQ\x20auth_revision\
    \x20is\x20a\x20revision\x20number\x20of\x20auth.authStore.\x20It\x20is\
    \x20not\x20related\x20to\x20mvcc\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\
    \x11\x02\x0f\x16\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x11\x02\x08\n\x0c\
    \n\x05\x04\0\x02\x02\x01\x12\x03\x11\t\x16\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x11\x19\x1a\n^\n\x02\x04\x01\x12\x04\x16\0:\x01\x1aR\x20An\x20I\
    nternalRaftRequest\x20is\x20the\x20union\x20of\x20all\x20requests\x20whi\
    ch\x20can\x20be\n\x20sent\x20via\x20raft.\n\n\n\n\x03\x04\x01\x01\x12\
    \x03\x16\x08\x1b\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x17\x02\x1d\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04\x17\x02\x16\x1d\n\x0c\n\x05\x04\x01\x02\0\x06\
    \x12\x03\x17\x02\x0f\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x17\x10\x16\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x17\x19\x1c\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03\x18\x02\x10\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x18\x02\
    \x17\x1d\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x18\x02\x08\n\x0c\n\x05\
    \x04\x01\x02\x01\x01\x12\x03\x18\t\x0b\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03\x18\x0e\x0f\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x1a\x02\x11\n\r\
    \n\x05\x04\x01\x02\x02\x04\x12\x04\x1a\x02\x18\x10\n\x0c\n\x05\x04\x01\
    \x02\x02\x06\x12\x03\x1a\x02\t\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\
    \x1a\n\x0c\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1a\x0f\x10\n\x0b\n\
    \x04\x04\x01\x02\x03\x12\x03\x1c\x02\x19\n\r\n\x05\x04\x01\x02\x03\x04\
    \x12\x04\x1c\x02\x1a\x11\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03\x1c\x02\
    \x0e\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x1c\x0f\x14\n\x0c\n\x05\x04\
    \x01\x02\x03\x03\x12\x03\x1c\x17\x18\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\
    \x1d\x02\x15\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04\x1d\x02\x1c\x19\n\x0c\
    \n\x05\x04\x01\x02\x04\x06\x12\x03\x1d\x02\x0c\n\x0c\n\x05\x04\x01\x02\
    \x04\x01\x12\x03\x1d\r\x10\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x1d\
    \x13\x14\n\x0b\n\x04\x04\x01\x02\x05\x12\x03\x1e\x02&\n\r\n\x05\x04\x01\
    \x02\x05\x04\x12\x04\x1e\x02\x1d\x15\n\x0c\n\x05\x04\x01\x02\x05\x06\x12\
    \x03\x1e\x02\x14\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\x1e\x15!\n\x0c\
    \n\x05\x04\x01\x02\x05\x03\x12\x03\x1e$%\n\x0b\n\x04\x04\x01\x02\x06\x12\
    \x03\x1f\x02\x15\n\r\n\x05\x04\x01\x02\x06\x04\x12\x04\x1f\x02\x1e&\n\
    \x0c\n\x05\x04\x01\x02\x06\x06\x12\x03\x1f\x02\x0c\n\x0c\n\x05\x04\x01\
    \x02\x06\x01\x12\x03\x1f\r\x10\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03\
    \x1f\x13\x14\n\x0b\n\x04\x04\x01\x02\x07\x12\x03\x20\x02#\n\r\n\x05\x04\
    \x01\x02\x07\x04\x12\x04\x20\x02\x1f\x15\n\x0c\n\x05\x04\x01\x02\x07\x06\
    \x12\x03\x20\x02\x13\n\x0c\n\x05\x04\x01\x02\x07\x01\x12\x03\x20\x14\x1e\
    \n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x03\x20!\"\n\x0b\n\x04\x04\x01\x02\
    \x08\x12\x03\"\x02$\n\r\n\x05\x04\x01\x02\x08\x04\x12\x04\"\x02\x20#\n\
    \x0c\n\x05\x04\x01\x02\x08\x06\x12\x03\"\x02\x13\n\x0c\n\x05\x04\x01\x02\
    \x08\x01\x12\x03\"\x14\x1f\n\x0c\n\x05\x04\x01\x02\x08\x03\x12\x03\"\"#\
    \n\x0b\n\x04\x04\x01\x02\t\x12\x03#\x02&\n\r\n\x05\x04\x01\x02\t\x04\x12\
    \x04#\x02\"$\n\x0c\n\x05\x04\x01\x02\t\x06\x12\x03#\x02\x14\n\x0c\n\x05\
    \x04\x01\x02\t\x01\x12\x03#\x15!\n\x0c\n\x05\x04\x01\x02\t\x03\x12\x03#$\
    %\n\x0b\n\x04\x04\x01\x02\n\x12\x03%\x02\x1a\n\r\n\x05\x04\x01\x02\n\x04\
    \x12\x04%\x02#&\n\x0c\n\x05\x04\x01\x02\n\x06\x12\x03%\x02\x0e\n\x0c\n\
    \x05\x04\x01\x02\n\x01\x12\x03%\x0f\x14\n\x0c\n\x05\x04\x01\x02\n\x03\
    \x12\x03%\x17\x19\n\x0b\n\x04\x04\x01\x02\x0b\x12\x03'\x02'\n\r\n\x05\
    \x04\x01\x02\x0b\x04\x12\x04'\x02%\x1a\n\x0c\n\x05\x04\x01\x02\x0b\x06\
    \x12\x03'\x02\x13\n\x0c\n\x05\x04\x01\x02\x0b\x01\x12\x03'\x14\x1f\n\x0c\
    \n\x05\x04\x01\x02\x0b\x03\x12\x03'\"&\n\x0b\n\x04\x04\x01\x02\x0c\x12\
    \x03(\x02)\n\r\n\x05\x04\x01\x02\x0c\x04\x12\x04(\x02''\n\x0c\n\x05\x04\
    \x01\x02\x0c\x06\x12\x03(\x02\x14\n\x0c\n\x05\x04\x01\x02\x0c\x01\x12\
    \x03(\x15!\n\x0c\n\x05\x04\x01\x02\x0c\x03\x12\x03($(\n\x0b\n\x04\x04\
    \x01\x02\r\x12\x03*\x022\n\r\n\x05\x04\x01\x02\r\x04\x12\x04*\x02()\n\
    \x0c\n\x05\x04\x01\x02\r\x06\x12\x03*\x02\x1d\n\x0c\n\x05\x04\x01\x02\r\
    \x01\x12\x03*\x1e*\n\x0c\n\x05\x04\x01\x02\r\x03\x12\x03*-1\n\x0b\n\x04\
    \x04\x01\x02\x0e\x12\x03,\x02*\n\r\n\x05\x04\x01\x02\x0e\x04\x12\x04,\
    \x02*2\n\x0c\n\x05\x04\x01\x02\x0e\x06\x12\x03,\x02\x14\n\x0c\n\x05\x04\
    \x01\x02\x0e\x01\x12\x03,\x15\"\n\x0c\n\x05\x04\x01\x02\x0e\x03\x12\x03,\
    %)\n\x0b\n\x04\x04\x01\x02\x0f\x12\x03-\x020\n\r\n\x05\x04\x01\x02\x0f\
    \x04\x12\x04-\x02,*\n\x0c\n\x05\x04\x01\x02\x0f\x06\x12\x03-\x02\x17\n\
    \x0c\n\x05\x04\x01\x02\x0f\x01\x12\x03-\x18(\n\x0c\n\x05\x04\x01\x02\x0f\
    \x03\x12\x03-+/\n\x0b\n\x04\x04\x01\x02\x10\x12\x03.\x02*\n\r\n\x05\x04\
    \x01\x02\x10\x04\x12\x04.\x02-0\n\x0c\n\x05\x04\x01\x02\x10\x06\x12\x03.\
    \x02\x14\n\x0c\n\x05\x04\x01\x02\x10\x01\x12\x03.\x15\"\n\x0c\n\x05\x04\
    \x01\x02\x10\x03\x12\x03.%)\n\x0b\n\x04\x04\x01\x02\x11\x12\x03/\x02A\n\
    \r\n\x05\x04\x01\x02\x11\x04\x12\x04/\x02.*\n\x0c\n\x05\x04\x01\x02\x11\
    \x06\x12\x03/\x02\x1f\n\x0c\n\x05\x04\x01\x02\x11\x01\x12\x03/\x209\n\
    \x0c\n\x05\x04\x01\x02\x11\x03\x12\x03/<@\n\x0b\n\x04\x04\x01\x02\x12\
    \x12\x030\x027\n\r\n\x05\x04\x01\x02\x12\x04\x12\x040\x02/A\n\x0c\n\x05\
    \x04\x01\x02\x12\x06\x12\x030\x02\x1a\n\x0c\n\x05\x04\x01\x02\x12\x01\
    \x12\x030\x1b/\n\x0c\n\x05\x04\x01\x02\x12\x03\x12\x03026\n\x0b\n\x04\
    \x04\x01\x02\x13\x12\x031\x029\n\r\n\x05\x04\x01\x02\x13\x04\x12\x041\
    \x0207\n\x0c\n\x05\x04\x01\x02\x13\x06\x12\x031\x02\x1b\n\x0c\n\x05\x04\
    \x01\x02\x13\x01\x12\x031\x1c1\n\x0c\n\x05\x04\x01\x02\x13\x03\x12\x0314\
    8\n\x0b\n\x04\x04\x01\x02\x14\x12\x032\x02,\n\r\n\x05\x04\x01\x02\x14\
    \x04\x12\x042\x0219\n\x0c\n\x05\x04\x01\x02\x14\x06\x12\x032\x02\x15\n\
    \x0c\n\x05\x04\x01\x02\x14\x01\x12\x032\x16$\n\x0c\n\x05\x04\x01\x02\x14\
    \x03\x12\x032'+\n\x0b\n\x04\x04\x01\x02\x15\x12\x033\x02,\n\r\n\x05\x04\
    \x01\x02\x15\x04\x12\x043\x022,\n\x0c\n\x05\x04\x01\x02\x15\x06\x12\x033\
    \x02\x15\n\x0c\n\x05\x04\x01\x02\x15\x01\x12\x033\x16$\n\x0c\n\x05\x04\
    \x01\x02\x15\x03\x12\x033'+\n\x0b\n\x04\x04\x01\x02\x16\x12\x035\x02*\n\
    \r\n\x05\x04\x01\x02\x16\x04\x12\x045\x023,\n\x0c\n\x05\x04\x01\x02\x16\
    \x06\x12\x035\x02\x14\n\x0c\n\x05\x04\x01\x02\x16\x01\x12\x035\x15\"\n\
    \x0c\n\x05\x04\x01\x02\x16\x03\x12\x035%)\n\x0b\n\x04\x04\x01\x02\x17\
    \x12\x036\x020\n\r\n\x05\x04\x01\x02\x17\x04\x12\x046\x025*\n\x0c\n\x05\
    \x04\x01\x02\x17\x06\x12\x036\x02\x17\n\x0c\n\x05\x04\x01\x02\x17\x01\
    \x12\x036\x18(\n\x0c\n\x05\x04\x01\x02\x17\x03\x12\x036+/\n\x0b\n\x04\
    \x04\x01\x02\x18\x12\x037\x02*\n\r\n\x05\x04\x01\x02\x18\x04\x12\x047\
    \x0260\n\x0c\n\x05\x04\x01\x02\x18\x06\x12\x037\x02\x14\n\x0c\n\x05\x04\
    \x01\x02\x18\x01\x12\x037\x15\"\n\x0c\n\x05\x04\x01\x02\x18\x03\x12\x037\
    %)\n\x0b\n\x04\x04\x01\x02\x19\x12\x038\x02C\n\r\n\x05\x04\x01\x02\x19\
    \x04\x12\x048\x027*\n\x0c\n\x05\x04\x01\x02\x19\x06\x12\x038\x02\x20\n\
    \x0c\n\x05\x04\x01\x02\x19\x01\x12\x038!;\n\x0c\n\x05\x04\x01\x02\x19\
    \x03\x12\x038>B\n\x0b\n\x04\x04\x01\x02\x1a\x12\x039\x02E\n\r\n\x05\x04\
    \x01\x02\x1a\x04\x12\x049\x028C\n\x0c\n\x05\x04\x01\x02\x1a\x06\x12\x039\
    \x02!\n\x0c\n\x05\x04\x01\x02\x1a\x01\x12\x039\"=\n\x0c\n\x05\x04\x01\
    \x02\x1a\x03\x12\x039@D\n\n\n\x02\x04\x02\x12\x04<\0=\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03<\x08\x15\n\xb4\x02\n\x02\x04\x03\x12\x04B\0H\x01\x1a\
    \xa7\x02\x20What\x20is\x20the\x20difference\x20between\x20AuthenticateRe\
    quest\x20(defined\x20in\x20rpc.proto)\x20and\x20InternalAuthenticateRequ\
    est?\n\x20InternalAuthenticateRequest\x20has\x20a\x20member\x20that\x20i\
    s\x20filled\x20by\x20etcdserver\x20and\x20shouldn't\x20be\x20user-facing\
    .\n\x20For\x20avoiding\x20misusage\x20the\x20field,\x20we\x20have\x20an\
    \x20internal\x20version\x20of\x20AuthenticateRequest.\n\n\n\n\x03\x04\
    \x03\x01\x12\x03B\x08#\n\x0b\n\x04\x04\x03\x02\0\x12\x03C\x02\x12\n\r\n\
    \x05\x04\x03\x02\0\x04\x12\x04C\x02B%\n\x0c\n\x05\x04\x03\x02\0\x05\x12\
    \x03C\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03C\t\r\n\x0c\n\x05\x04\
    \x03\x02\0\x03\x12\x03C\x10\x11\n\x0b\n\x04\x04\x03\x02\x01\x12\x03D\x02\
    \x16\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04D\x02C\x12\n\x0c\n\x05\x04\x03\
    \x02\x01\x05\x12\x03D\x02\x08\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03D\t\
    \x11\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03D\x14\x15\nO\n\x04\x04\x03\
    \x02\x02\x12\x03G\x02\x1a\x1aB\x20simple_token\x20is\x20generated\x20in\
    \x20API\x20layer\x20(etcdserver/v3_server.go)\n\n\r\n\x05\x04\x03\x02\
    \x02\x04\x12\x04G\x02D\x16\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x03G\x02\
    \x08\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03G\t\x15\n\x0c\n\x05\x04\x03\
    \x02\x02\x03\x12\x03G\x18\x19b\x06proto3\
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
