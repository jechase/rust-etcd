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
pub struct Lease {
    // message fields
    pub ID: i64,
    pub TTL: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Lease {}

impl Lease {
    pub fn new() -> Lease {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Lease {
        static mut instance: ::protobuf::lazy::Lazy<Lease> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Lease,
        };
        unsafe {
            instance.get(Lease::new)
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

    // int64 TTL = 2;

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

impl ::protobuf::Message for Lease {
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
        if self.ID != 0 {
            my_size += ::protobuf::rt::value_size(1, self.ID, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.TTL != 0 {
            my_size += ::protobuf::rt::value_size(2, self.TTL, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.ID != 0 {
            os.write_int64(1, self.ID)?;
        }
        if self.TTL != 0 {
            os.write_int64(2, self.TTL)?;
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

impl ::protobuf::MessageStatic for Lease {
    fn new() -> Lease {
        Lease::new()
    }

    fn descriptor_static(_: ::std::option::Option<Lease>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ID",
                    Lease::get_ID_for_reflect,
                    Lease::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "TTL",
                    Lease::get_TTL_for_reflect,
                    Lease::mut_TTL_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Lease>(
                    "Lease",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Lease {
    fn clear(&mut self) {
        self.clear_ID();
        self.clear_TTL();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Lease {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Lease {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseInternalRequest {
    // message fields
    pub LeaseTimeToLiveRequest: ::protobuf::SingularPtrField<super::rpc::LeaseTimeToLiveRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseInternalRequest {}

impl LeaseInternalRequest {
    pub fn new() -> LeaseInternalRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseInternalRequest {
        static mut instance: ::protobuf::lazy::Lazy<LeaseInternalRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseInternalRequest,
        };
        unsafe {
            instance.get(LeaseInternalRequest::new)
        }
    }

    // .etcdserverpb.LeaseTimeToLiveRequest LeaseTimeToLiveRequest = 1;

    pub fn clear_LeaseTimeToLiveRequest(&mut self) {
        self.LeaseTimeToLiveRequest.clear();
    }

    pub fn has_LeaseTimeToLiveRequest(&self) -> bool {
        self.LeaseTimeToLiveRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_LeaseTimeToLiveRequest(&mut self, v: super::rpc::LeaseTimeToLiveRequest) {
        self.LeaseTimeToLiveRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_LeaseTimeToLiveRequest(&mut self) -> &mut super::rpc::LeaseTimeToLiveRequest {
        if self.LeaseTimeToLiveRequest.is_none() {
            self.LeaseTimeToLiveRequest.set_default();
        }
        self.LeaseTimeToLiveRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_LeaseTimeToLiveRequest(&mut self) -> super::rpc::LeaseTimeToLiveRequest {
        self.LeaseTimeToLiveRequest.take().unwrap_or_else(|| super::rpc::LeaseTimeToLiveRequest::new())
    }

    pub fn get_LeaseTimeToLiveRequest(&self) -> &super::rpc::LeaseTimeToLiveRequest {
        self.LeaseTimeToLiveRequest.as_ref().unwrap_or_else(|| super::rpc::LeaseTimeToLiveRequest::default_instance())
    }

    fn get_LeaseTimeToLiveRequest_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::LeaseTimeToLiveRequest> {
        &self.LeaseTimeToLiveRequest
    }

    fn mut_LeaseTimeToLiveRequest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::LeaseTimeToLiveRequest> {
        &mut self.LeaseTimeToLiveRequest
    }
}

impl ::protobuf::Message for LeaseInternalRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.LeaseTimeToLiveRequest {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.LeaseTimeToLiveRequest)?;
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
        if let Some(ref v) = self.LeaseTimeToLiveRequest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.LeaseTimeToLiveRequest.as_ref() {
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

impl ::protobuf::MessageStatic for LeaseInternalRequest {
    fn new() -> LeaseInternalRequest {
        LeaseInternalRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseInternalRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::LeaseTimeToLiveRequest>>(
                    "LeaseTimeToLiveRequest",
                    LeaseInternalRequest::get_LeaseTimeToLiveRequest_for_reflect,
                    LeaseInternalRequest::mut_LeaseTimeToLiveRequest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseInternalRequest>(
                    "LeaseInternalRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseInternalRequest {
    fn clear(&mut self) {
        self.clear_LeaseTimeToLiveRequest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseInternalRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseInternalRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LeaseInternalResponse {
    // message fields
    pub LeaseTimeToLiveResponse: ::protobuf::SingularPtrField<super::rpc::LeaseTimeToLiveResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeaseInternalResponse {}

impl LeaseInternalResponse {
    pub fn new() -> LeaseInternalResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseInternalResponse {
        static mut instance: ::protobuf::lazy::Lazy<LeaseInternalResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseInternalResponse,
        };
        unsafe {
            instance.get(LeaseInternalResponse::new)
        }
    }

    // .etcdserverpb.LeaseTimeToLiveResponse LeaseTimeToLiveResponse = 1;

    pub fn clear_LeaseTimeToLiveResponse(&mut self) {
        self.LeaseTimeToLiveResponse.clear();
    }

    pub fn has_LeaseTimeToLiveResponse(&self) -> bool {
        self.LeaseTimeToLiveResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_LeaseTimeToLiveResponse(&mut self, v: super::rpc::LeaseTimeToLiveResponse) {
        self.LeaseTimeToLiveResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_LeaseTimeToLiveResponse(&mut self) -> &mut super::rpc::LeaseTimeToLiveResponse {
        if self.LeaseTimeToLiveResponse.is_none() {
            self.LeaseTimeToLiveResponse.set_default();
        }
        self.LeaseTimeToLiveResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_LeaseTimeToLiveResponse(&mut self) -> super::rpc::LeaseTimeToLiveResponse {
        self.LeaseTimeToLiveResponse.take().unwrap_or_else(|| super::rpc::LeaseTimeToLiveResponse::new())
    }

    pub fn get_LeaseTimeToLiveResponse(&self) -> &super::rpc::LeaseTimeToLiveResponse {
        self.LeaseTimeToLiveResponse.as_ref().unwrap_or_else(|| super::rpc::LeaseTimeToLiveResponse::default_instance())
    }

    fn get_LeaseTimeToLiveResponse_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rpc::LeaseTimeToLiveResponse> {
        &self.LeaseTimeToLiveResponse
    }

    fn mut_LeaseTimeToLiveResponse_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rpc::LeaseTimeToLiveResponse> {
        &mut self.LeaseTimeToLiveResponse
    }
}

impl ::protobuf::Message for LeaseInternalResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.LeaseTimeToLiveResponse {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.LeaseTimeToLiveResponse)?;
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
        if let Some(ref v) = self.LeaseTimeToLiveResponse.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.LeaseTimeToLiveResponse.as_ref() {
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

impl ::protobuf::MessageStatic for LeaseInternalResponse {
    fn new() -> LeaseInternalResponse {
        LeaseInternalResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseInternalResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::LeaseTimeToLiveResponse>>(
                    "LeaseTimeToLiveResponse",
                    LeaseInternalResponse::get_LeaseTimeToLiveResponse_for_reflect,
                    LeaseInternalResponse::mut_LeaseTimeToLiveResponse_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseInternalResponse>(
                    "LeaseInternalResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseInternalResponse {
    fn clear(&mut self) {
        self.clear_LeaseTimeToLiveResponse();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LeaseInternalResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LeaseInternalResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0blease.proto\x12\x07leasepb\x1a\x14gogoproto/gogo.proto\x1a&etcd/et\
    cdserver/etcdserverpb/rpc.proto\")\n\x05Lease\x12\x0e\n\x02ID\x18\x01\
    \x20\x01(\x03R\x02ID\x12\x10\n\x03TTL\x18\x02\x20\x01(\x03R\x03TTL\"t\n\
    \x14LeaseInternalRequest\x12\\\n\x16LeaseTimeToLiveRequest\x18\x01\x20\
    \x01(\x0b2$.etcdserverpb.LeaseTimeToLiveRequestR\x16LeaseTimeToLiveReque\
    st\"x\n\x15LeaseInternalResponse\x12_\n\x17LeaseTimeToLiveResponse\x18\
    \x01\x20\x01(\x0b2%.etcdserverpb.LeaseTimeToLiveResponseR\x17LeaseTimeTo\
    LiveResponseB\x14\xc8\xe2\x1e\x01\xe0\xe2\x1e\x01\xd0\xe2\x1e\x01\xc8\
    \xe1\x1e\0\xd0\xe1\x1e\0J\xac\x06\n\x06\x12\x04\0\0\x17\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0f\n\t\n\x02\x03\0\
    \x12\x03\x03\x07\x1d\n\t\n\x02\x03\x01\x12\x03\x04\x07/\n\x08\n\x01\x08\
    \x12\x03\x06\0(\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x06\0(\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x06\x07\x20\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x06\x07\x20\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x06\x08\x1f\n\
    \x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x06#'\n\x08\n\x01\x08\x12\x03\x07\0\
    $\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x07\0$\n\x0c\n\x05\x08\xe7\x07\x01\
    \x02\x12\x03\x07\x07\x1c\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x07\x07\
    \x1c\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x07\x08\x1b\n\x0c\n\
    \x05\x08\xe7\x07\x01\x03\x12\x03\x07\x1f#\n\x08\n\x01\x08\x12\x03\x08\0*\
    \n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x08\0*\n\x0c\n\x05\x08\xe7\x07\x02\
    \x02\x12\x03\x08\x07\"\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x08\x07\"\
    \n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x08\x08!\n\x0c\n\x05\x08\
    \xe7\x07\x02\x03\x12\x03\x08%)\n\x08\n\x01\x08\x12\x03\t\0/\n\x0b\n\x04\
    \x08\xe7\x07\x03\x12\x03\t\0/\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\t\
    \x07&\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\t\x07&\n\x0e\n\x07\x08\xe7\
    \x07\x03\x02\0\x01\x12\x03\t\x08%\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\
    \x03\t).\n\x08\n\x01\x08\x12\x03\n\03\n\x0b\n\x04\x08\xe7\x07\x04\x12\
    \x03\n\03\n\x0c\n\x05\x08\xe7\x07\x04\x02\x12\x03\n\x07*\n\r\n\x06\x08\
    \xe7\x07\x04\x02\0\x12\x03\n\x07*\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\
    \x12\x03\n\x08)\n\x0c\n\x05\x08\xe7\x07\x04\x03\x12\x03\n-2\n\n\n\x02\
    \x04\0\x12\x04\x0c\0\x0f\x01\n\n\n\x03\x04\0\x01\x12\x03\x0c\x08\r\n\x0b\
    \n\x04\x04\0\x02\0\x12\x03\r\x02\x0f\n\r\n\x05\x04\0\x02\0\x04\x12\x04\r\
    \x02\x0c\x0f\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\r\x02\x07\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\r\x08\n\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\r\
    \x0e\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0e\x02\x10\n\r\n\x05\x04\0\x02\
    \x01\x04\x12\x04\x0e\x02\r\x0f\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0e\
    \x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0e\x08\x0b\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x0e\x0e\x0f\n\n\n\x02\x04\x01\x12\x04\x11\0\
    \x13\x01\n\n\n\x03\x04\x01\x01\x12\x03\x11\x08\x1c\n\x0b\n\x04\x04\x01\
    \x02\0\x12\x03\x12\x02A\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x12\x02\x11\
    \x1e\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x12\x02%\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x12&<\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x12?@\n\n\
    \n\x02\x04\x02\x12\x04\x15\0\x17\x01\n\n\n\x03\x04\x02\x01\x12\x03\x15\
    \x08\x1d\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x16\x02C\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04\x16\x02\x15\x1f\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\
    \x16\x02&\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x16'>\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03\x16ABb\x06proto3\
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
