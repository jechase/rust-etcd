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
pub struct Entry {
    // message fields
    Term: ::std::option::Option<u64>,
    Index: ::std::option::Option<u64>,
    Type: ::std::option::Option<EntryType>,
    Data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Entry {}

impl Entry {
    pub fn new() -> Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Entry {
        static mut instance: ::protobuf::lazy::Lazy<Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Entry,
        };
        unsafe {
            instance.get(Entry::new)
        }
    }

    // optional uint64 Term = 2;

    pub fn clear_Term(&mut self) {
        self.Term = ::std::option::Option::None;
    }

    pub fn has_Term(&self) -> bool {
        self.Term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Term(&mut self, v: u64) {
        self.Term = ::std::option::Option::Some(v);
    }

    pub fn get_Term(&self) -> u64 {
        self.Term.unwrap_or(0)
    }

    fn get_Term_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.Term
    }

    fn mut_Term_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.Term
    }

    // optional uint64 Index = 3;

    pub fn clear_Index(&mut self) {
        self.Index = ::std::option::Option::None;
    }

    pub fn has_Index(&self) -> bool {
        self.Index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Index(&mut self, v: u64) {
        self.Index = ::std::option::Option::Some(v);
    }

    pub fn get_Index(&self) -> u64 {
        self.Index.unwrap_or(0)
    }

    fn get_Index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.Index
    }

    fn mut_Index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.Index
    }

    // optional .raftpb.EntryType Type = 1;

    pub fn clear_Type(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_Type(&self) -> bool {
        self.Type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Type(&mut self, v: EntryType) {
        self.Type = ::std::option::Option::Some(v);
    }

    pub fn get_Type(&self) -> EntryType {
        self.Type.unwrap_or(EntryType::EntryNormal)
    }

    fn get_Type_for_reflect(&self) -> &::std::option::Option<EntryType> {
        &self.Type
    }

    fn mut_Type_for_reflect(&mut self) -> &mut ::std::option::Option<EntryType> {
        &mut self.Type
    }

    // optional bytes Data = 4;

    pub fn clear_Data(&mut self) {
        self.Data.clear();
    }

    pub fn has_Data(&self) -> bool {
        self.Data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.Data.is_none() {
            self.Data.set_default();
        }
        self.Data.as_mut().unwrap()
    }

    // Take field
    pub fn take_Data(&mut self) -> ::std::vec::Vec<u8> {
        self.Data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Data(&self) -> &[u8] {
        match self.Data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_Data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.Data
    }

    fn mut_Data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.Data
    }
}

impl ::protobuf::Message for Entry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.Term = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.Index = ::std::option::Option::Some(tmp);
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.Type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.Data)?;
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
        if let Some(v) = self.Term {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.Index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.Type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.Data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Term {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.Index {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.Type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.Data.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for Entry {
    fn new() -> Entry {
        Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "Term",
                    Entry::get_Term_for_reflect,
                    Entry::mut_Term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "Index",
                    Entry::get_Index_for_reflect,
                    Entry::mut_Index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EntryType>>(
                    "Type",
                    Entry::get_Type_for_reflect,
                    Entry::mut_Type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "Data",
                    Entry::get_Data_for_reflect,
                    Entry::mut_Data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Entry>(
                    "Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Entry {
    fn clear(&mut self) {
        self.clear_Term();
        self.clear_Index();
        self.clear_Type();
        self.clear_Data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotMetadata {
    // message fields
    conf_state: ::protobuf::SingularPtrField<ConfState>,
    index: ::std::option::Option<u64>,
    term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotMetadata {}

impl SnapshotMetadata {
    pub fn new() -> SnapshotMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotMetadata {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotMetadata,
        };
        unsafe {
            instance.get(SnapshotMetadata::new)
        }
    }

    // optional .raftpb.ConfState conf_state = 1;

    pub fn clear_conf_state(&mut self) {
        self.conf_state.clear();
    }

    pub fn has_conf_state(&self) -> bool {
        self.conf_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conf_state(&mut self, v: ConfState) {
        self.conf_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conf_state(&mut self) -> &mut ConfState {
        if self.conf_state.is_none() {
            self.conf_state.set_default();
        }
        self.conf_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_conf_state(&mut self) -> ConfState {
        self.conf_state.take().unwrap_or_else(|| ConfState::new())
    }

    pub fn get_conf_state(&self) -> &ConfState {
        self.conf_state.as_ref().unwrap_or_else(|| ConfState::default_instance())
    }

    fn get_conf_state_for_reflect(&self) -> &::protobuf::SingularPtrField<ConfState> {
        &self.conf_state
    }

    fn mut_conf_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ConfState> {
        &mut self.conf_state
    }

    // optional uint64 index = 2;

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

    // optional uint64 term = 3;

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

impl ::protobuf::Message for SnapshotMetadata {
    fn is_initialized(&self) -> bool {
        for v in &self.conf_state {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.conf_state)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(ref v) = self.conf_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.term {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.conf_state.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.index {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.term {
            os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for SnapshotMetadata {
    fn new() -> SnapshotMetadata {
        SnapshotMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ConfState>>(
                    "conf_state",
                    SnapshotMetadata::get_conf_state_for_reflect,
                    SnapshotMetadata::mut_conf_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    SnapshotMetadata::get_index_for_reflect,
                    SnapshotMetadata::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    SnapshotMetadata::get_term_for_reflect,
                    SnapshotMetadata::mut_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotMetadata>(
                    "SnapshotMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotMetadata {
    fn clear(&mut self) {
        self.clear_conf_state();
        self.clear_index();
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Snapshot {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    metadata: ::protobuf::SingularPtrField<SnapshotMetadata>,
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

    // optional bytes data = 1;

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

    // optional .raftpb.SnapshotMetadata metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: SnapshotMetadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut SnapshotMetadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> SnapshotMetadata {
        self.metadata.take().unwrap_or_else(|| SnapshotMetadata::new())
    }

    pub fn get_metadata(&self) -> &SnapshotMetadata {
        self.metadata.as_ref().unwrap_or_else(|| SnapshotMetadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<SnapshotMetadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SnapshotMetadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for Snapshot {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Snapshot::get_data_for_reflect,
                    Snapshot::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapshotMetadata>>(
                    "metadata",
                    Snapshot::get_metadata_for_reflect,
                    Snapshot::mut_metadata_for_reflect,
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
        self.clear_data();
        self.clear_metadata();
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

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    field_type: ::std::option::Option<MessageType>,
    to: ::std::option::Option<u64>,
    from: ::std::option::Option<u64>,
    term: ::std::option::Option<u64>,
    logTerm: ::std::option::Option<u64>,
    index: ::std::option::Option<u64>,
    entries: ::protobuf::RepeatedField<Entry>,
    commit: ::std::option::Option<u64>,
    snapshot: ::protobuf::SingularPtrField<Snapshot>,
    reject: ::std::option::Option<bool>,
    rejectHint: ::std::option::Option<u64>,
    context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // optional .raftpb.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MessageType {
        self.field_type.unwrap_or(MessageType::MsgHup)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<MessageType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<MessageType> {
        &mut self.field_type
    }

    // optional uint64 to = 2;

    pub fn clear_to(&mut self) {
        self.to = ::std::option::Option::None;
    }

    pub fn has_to(&self) -> bool {
        self.to.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: u64) {
        self.to = ::std::option::Option::Some(v);
    }

    pub fn get_to(&self) -> u64 {
        self.to.unwrap_or(0)
    }

    fn get_to_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.to
    }

    fn mut_to_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.to
    }

    // optional uint64 from = 3;

    pub fn clear_from(&mut self) {
        self.from = ::std::option::Option::None;
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: u64) {
        self.from = ::std::option::Option::Some(v);
    }

    pub fn get_from(&self) -> u64 {
        self.from.unwrap_or(0)
    }

    fn get_from_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.from
    }

    fn mut_from_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.from
    }

    // optional uint64 term = 4;

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

    // optional uint64 logTerm = 5;

    pub fn clear_logTerm(&mut self) {
        self.logTerm = ::std::option::Option::None;
    }

    pub fn has_logTerm(&self) -> bool {
        self.logTerm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logTerm(&mut self, v: u64) {
        self.logTerm = ::std::option::Option::Some(v);
    }

    pub fn get_logTerm(&self) -> u64 {
        self.logTerm.unwrap_or(0)
    }

    fn get_logTerm_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.logTerm
    }

    fn mut_logTerm_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.logTerm
    }

    // optional uint64 index = 6;

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

    // repeated .raftpb.Entry entries = 7;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Entry> {
        &mut self.entries
    }

    // optional uint64 commit = 8;

    pub fn clear_commit(&mut self) {
        self.commit = ::std::option::Option::None;
    }

    pub fn has_commit(&self) -> bool {
        self.commit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: u64) {
        self.commit = ::std::option::Option::Some(v);
    }

    pub fn get_commit(&self) -> u64 {
        self.commit.unwrap_or(0)
    }

    fn get_commit_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.commit
    }

    fn mut_commit_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.commit
    }

    // optional .raftpb.Snapshot snapshot = 9;

    pub fn clear_snapshot(&mut self) {
        self.snapshot.clear();
    }

    pub fn has_snapshot(&self) -> bool {
        self.snapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshot(&mut self, v: Snapshot) {
        self.snapshot = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshot(&mut self) -> &mut Snapshot {
        if self.snapshot.is_none() {
            self.snapshot.set_default();
        }
        self.snapshot.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshot(&mut self) -> Snapshot {
        self.snapshot.take().unwrap_or_else(|| Snapshot::new())
    }

    pub fn get_snapshot(&self) -> &Snapshot {
        self.snapshot.as_ref().unwrap_or_else(|| Snapshot::default_instance())
    }

    fn get_snapshot_for_reflect(&self) -> &::protobuf::SingularPtrField<Snapshot> {
        &self.snapshot
    }

    fn mut_snapshot_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Snapshot> {
        &mut self.snapshot
    }

    // optional bool reject = 10;

    pub fn clear_reject(&mut self) {
        self.reject = ::std::option::Option::None;
    }

    pub fn has_reject(&self) -> bool {
        self.reject.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reject(&mut self, v: bool) {
        self.reject = ::std::option::Option::Some(v);
    }

    pub fn get_reject(&self) -> bool {
        self.reject.unwrap_or(false)
    }

    fn get_reject_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.reject
    }

    fn mut_reject_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.reject
    }

    // optional uint64 rejectHint = 11;

    pub fn clear_rejectHint(&mut self) {
        self.rejectHint = ::std::option::Option::None;
    }

    pub fn has_rejectHint(&self) -> bool {
        self.rejectHint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rejectHint(&mut self, v: u64) {
        self.rejectHint = ::std::option::Option::Some(v);
    }

    pub fn get_rejectHint(&self) -> u64 {
        self.rejectHint.unwrap_or(0)
    }

    fn get_rejectHint_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.rejectHint
    }

    fn mut_rejectHint_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.rejectHint
    }

    // optional bytes context = 12;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::vec::Vec<u8> {
        self.context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_context(&self) -> &[u8] {
        match self.context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.context
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.snapshot {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.to = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.from = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.term = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.logTerm = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.snapshot)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.reject = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.rejectHint = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.context)?;
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
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.to {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.from {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.term {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.logTerm {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.commit {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.snapshot.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.reject {
            my_size += 2;
        }
        if let Some(v) = self.rejectHint {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.context.as_ref() {
            my_size += ::protobuf::rt::bytes_size(12, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.to {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.from {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.term {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.logTerm {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.index {
            os.write_uint64(6, v)?;
        }
        for v in &self.entries {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.commit {
            os.write_uint64(8, v)?;
        }
        if let Some(ref v) = self.snapshot.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.reject {
            os.write_bool(10, v)?;
        }
        if let Some(v) = self.rejectHint {
            os.write_uint64(11, v)?;
        }
        if let Some(ref v) = self.context.as_ref() {
            os.write_bytes(12, &v)?;
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

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "type",
                    Message::get_field_type_for_reflect,
                    Message::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "to",
                    Message::get_to_for_reflect,
                    Message::mut_to_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "from",
                    Message::get_from_for_reflect,
                    Message::mut_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    Message::get_term_for_reflect,
                    Message::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logTerm",
                    Message::get_logTerm_for_reflect,
                    Message::mut_logTerm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    Message::get_index_for_reflect,
                    Message::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Entry>>(
                    "entries",
                    Message::get_entries_for_reflect,
                    Message::mut_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit",
                    Message::get_commit_for_reflect,
                    Message::mut_commit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Snapshot>>(
                    "snapshot",
                    Message::get_snapshot_for_reflect,
                    Message::mut_snapshot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reject",
                    Message::get_reject_for_reflect,
                    Message::mut_reject_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "rejectHint",
                    Message::get_rejectHint_for_reflect,
                    Message::mut_rejectHint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "context",
                    Message::get_context_for_reflect,
                    Message::mut_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_to();
        self.clear_from();
        self.clear_term();
        self.clear_logTerm();
        self.clear_index();
        self.clear_entries();
        self.clear_commit();
        self.clear_snapshot();
        self.clear_reject();
        self.clear_rejectHint();
        self.clear_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HardState {
    // message fields
    term: ::std::option::Option<u64>,
    vote: ::std::option::Option<u64>,
    commit: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HardState {}

impl HardState {
    pub fn new() -> HardState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HardState {
        static mut instance: ::protobuf::lazy::Lazy<HardState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HardState,
        };
        unsafe {
            instance.get(HardState::new)
        }
    }

    // optional uint64 term = 1;

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

    // optional uint64 vote = 2;

    pub fn clear_vote(&mut self) {
        self.vote = ::std::option::Option::None;
    }

    pub fn has_vote(&self) -> bool {
        self.vote.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vote(&mut self, v: u64) {
        self.vote = ::std::option::Option::Some(v);
    }

    pub fn get_vote(&self) -> u64 {
        self.vote.unwrap_or(0)
    }

    fn get_vote_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.vote
    }

    fn mut_vote_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.vote
    }

    // optional uint64 commit = 3;

    pub fn clear_commit(&mut self) {
        self.commit = ::std::option::Option::None;
    }

    pub fn has_commit(&self) -> bool {
        self.commit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: u64) {
        self.commit = ::std::option::Option::Some(v);
    }

    pub fn get_commit(&self) -> u64 {
        self.commit.unwrap_or(0)
    }

    fn get_commit_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.commit
    }

    fn mut_commit_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.commit
    }
}

impl ::protobuf::Message for HardState {
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
                    self.term = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.vote = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.term {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vote {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.commit {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.vote {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.commit {
            os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for HardState {
    fn new() -> HardState {
        HardState::new()
    }

    fn descriptor_static(_: ::std::option::Option<HardState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    HardState::get_term_for_reflect,
                    HardState::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "vote",
                    HardState::get_vote_for_reflect,
                    HardState::mut_vote_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit",
                    HardState::get_commit_for_reflect,
                    HardState::mut_commit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HardState>(
                    "HardState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HardState {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_vote();
        self.clear_commit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HardState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HardState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfState {
    // message fields
    nodes: ::std::vec::Vec<u64>,
    learners: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfState {}

impl ConfState {
    pub fn new() -> ConfState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfState {
        static mut instance: ::protobuf::lazy::Lazy<ConfState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfState,
        };
        unsafe {
            instance.get(ConfState::new)
        }
    }

    // repeated uint64 nodes = 1;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::std::vec::Vec<u64>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.nodes, ::std::vec::Vec::new())
    }

    pub fn get_nodes(&self) -> &[u64] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.nodes
    }

    // repeated uint64 learners = 2;

    pub fn clear_learners(&mut self) {
        self.learners.clear();
    }

    // Param is passed by value, moved
    pub fn set_learners(&mut self, v: ::std::vec::Vec<u64>) {
        self.learners = v;
    }

    // Mutable pointer to the field.
    pub fn mut_learners(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.learners
    }

    // Take field
    pub fn take_learners(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.learners, ::std::vec::Vec::new())
    }

    pub fn get_learners(&self) -> &[u64] {
        &self.learners
    }

    fn get_learners_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.learners
    }

    fn mut_learners_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.learners
    }
}

impl ::protobuf::Message for ConfState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.nodes)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.learners)?;
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
        for value in &self.nodes {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.learners {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.nodes {
            os.write_uint64(1, *v)?;
        };
        for v in &self.learners {
            os.write_uint64(2, *v)?;
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

impl ::protobuf::MessageStatic for ConfState {
    fn new() -> ConfState {
        ConfState::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "nodes",
                    ConfState::get_nodes_for_reflect,
                    ConfState::mut_nodes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "learners",
                    ConfState::get_learners_for_reflect,
                    ConfState::mut_learners_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfState>(
                    "ConfState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfState {
    fn clear(&mut self) {
        self.clear_nodes();
        self.clear_learners();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfChange {
    // message fields
    ID: ::std::option::Option<u64>,
    Type: ::std::option::Option<ConfChangeType>,
    NodeID: ::std::option::Option<u64>,
    Context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfChange {}

impl ConfChange {
    pub fn new() -> ConfChange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfChange {
        static mut instance: ::protobuf::lazy::Lazy<ConfChange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfChange,
        };
        unsafe {
            instance.get(ConfChange::new)
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

    // optional .raftpb.ConfChangeType Type = 2;

    pub fn clear_Type(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_Type(&self) -> bool {
        self.Type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Type(&mut self, v: ConfChangeType) {
        self.Type = ::std::option::Option::Some(v);
    }

    pub fn get_Type(&self) -> ConfChangeType {
        self.Type.unwrap_or(ConfChangeType::ConfChangeAddNode)
    }

    fn get_Type_for_reflect(&self) -> &::std::option::Option<ConfChangeType> {
        &self.Type
    }

    fn mut_Type_for_reflect(&mut self) -> &mut ::std::option::Option<ConfChangeType> {
        &mut self.Type
    }

    // optional uint64 NodeID = 3;

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

    // optional bytes Context = 4;

    pub fn clear_Context(&mut self) {
        self.Context.clear();
    }

    pub fn has_Context(&self) -> bool {
        self.Context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Context(&mut self, v: ::std::vec::Vec<u8>) {
        self.Context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.Context.is_none() {
            self.Context.set_default();
        }
        self.Context.as_mut().unwrap()
    }

    // Take field
    pub fn take_Context(&mut self) -> ::std::vec::Vec<u8> {
        self.Context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Context(&self) -> &[u8] {
        match self.Context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_Context_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.Context
    }

    fn mut_Context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.Context
    }
}

impl ::protobuf::Message for ConfChange {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.Type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.NodeID = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.Context)?;
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
        if let Some(v) = self.Type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.NodeID {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.Context.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ID {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.Type {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.NodeID {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.Context.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for ConfChange {
    fn new() -> ConfChange {
        ConfChange::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfChange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ID",
                    ConfChange::get_ID_for_reflect,
                    ConfChange::mut_ID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ConfChangeType>>(
                    "Type",
                    ConfChange::get_Type_for_reflect,
                    ConfChange::mut_Type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "NodeID",
                    ConfChange::get_NodeID_for_reflect,
                    ConfChange::mut_NodeID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "Context",
                    ConfChange::get_Context_for_reflect,
                    ConfChange::mut_Context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfChange>(
                    "ConfChange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfChange {
    fn clear(&mut self) {
        self.clear_ID();
        self.clear_Type();
        self.clear_NodeID();
        self.clear_Context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfChange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EntryType {
    EntryNormal = 0,
    EntryConfChange = 1,
}

impl ::protobuf::ProtobufEnum for EntryType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EntryType> {
        match value {
            0 => ::std::option::Option::Some(EntryType::EntryNormal),
            1 => ::std::option::Option::Some(EntryType::EntryConfChange),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EntryType] = &[
            EntryType::EntryNormal,
            EntryType::EntryConfChange,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EntryType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EntryType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EntryType {
}

impl ::protobuf::reflect::ProtobufValue for EntryType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    MsgHup = 0,
    MsgBeat = 1,
    MsgProp = 2,
    MsgApp = 3,
    MsgAppResp = 4,
    MsgVote = 5,
    MsgVoteResp = 6,
    MsgSnap = 7,
    MsgHeartbeat = 8,
    MsgHeartbeatResp = 9,
    MsgUnreachable = 10,
    MsgSnapStatus = 11,
    MsgCheckQuorum = 12,
    MsgTransferLeader = 13,
    MsgTimeoutNow = 14,
    MsgReadIndex = 15,
    MsgReadIndexResp = 16,
    MsgPreVote = 17,
    MsgPreVoteResp = 18,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            0 => ::std::option::Option::Some(MessageType::MsgHup),
            1 => ::std::option::Option::Some(MessageType::MsgBeat),
            2 => ::std::option::Option::Some(MessageType::MsgProp),
            3 => ::std::option::Option::Some(MessageType::MsgApp),
            4 => ::std::option::Option::Some(MessageType::MsgAppResp),
            5 => ::std::option::Option::Some(MessageType::MsgVote),
            6 => ::std::option::Option::Some(MessageType::MsgVoteResp),
            7 => ::std::option::Option::Some(MessageType::MsgSnap),
            8 => ::std::option::Option::Some(MessageType::MsgHeartbeat),
            9 => ::std::option::Option::Some(MessageType::MsgHeartbeatResp),
            10 => ::std::option::Option::Some(MessageType::MsgUnreachable),
            11 => ::std::option::Option::Some(MessageType::MsgSnapStatus),
            12 => ::std::option::Option::Some(MessageType::MsgCheckQuorum),
            13 => ::std::option::Option::Some(MessageType::MsgTransferLeader),
            14 => ::std::option::Option::Some(MessageType::MsgTimeoutNow),
            15 => ::std::option::Option::Some(MessageType::MsgReadIndex),
            16 => ::std::option::Option::Some(MessageType::MsgReadIndexResp),
            17 => ::std::option::Option::Some(MessageType::MsgPreVote),
            18 => ::std::option::Option::Some(MessageType::MsgPreVoteResp),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::MsgHup,
            MessageType::MsgBeat,
            MessageType::MsgProp,
            MessageType::MsgApp,
            MessageType::MsgAppResp,
            MessageType::MsgVote,
            MessageType::MsgVoteResp,
            MessageType::MsgSnap,
            MessageType::MsgHeartbeat,
            MessageType::MsgHeartbeatResp,
            MessageType::MsgUnreachable,
            MessageType::MsgSnapStatus,
            MessageType::MsgCheckQuorum,
            MessageType::MsgTransferLeader,
            MessageType::MsgTimeoutNow,
            MessageType::MsgReadIndex,
            MessageType::MsgReadIndexResp,
            MessageType::MsgPreVote,
            MessageType::MsgPreVoteResp,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

impl ::protobuf::reflect::ProtobufValue for MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ConfChangeType {
    ConfChangeAddNode = 0,
    ConfChangeRemoveNode = 1,
    ConfChangeUpdateNode = 2,
    ConfChangeAddLearnerNode = 3,
}

impl ::protobuf::ProtobufEnum for ConfChangeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ConfChangeType> {
        match value {
            0 => ::std::option::Option::Some(ConfChangeType::ConfChangeAddNode),
            1 => ::std::option::Option::Some(ConfChangeType::ConfChangeRemoveNode),
            2 => ::std::option::Option::Some(ConfChangeType::ConfChangeUpdateNode),
            3 => ::std::option::Option::Some(ConfChangeType::ConfChangeAddLearnerNode),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ConfChangeType] = &[
            ConfChangeType::ConfChangeAddNode,
            ConfChangeType::ConfChangeRemoveNode,
            ConfChangeType::ConfChangeUpdateNode,
            ConfChangeType::ConfChangeAddLearnerNode,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ConfChangeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ConfChangeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ConfChangeType {
}

impl ::protobuf::reflect::ProtobufValue for ConfChangeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nraft.proto\x12\x06raftpb\x1a\x14gogoproto/gogo.proto\"~\n\x05Entry\
    \x12\x18\n\x04Term\x18\x02\x20\x01(\x04R\x04TermB\x04\xc8\xde\x1f\0\x12\
    \x1a\n\x05Index\x18\x03\x20\x01(\x04R\x05IndexB\x04\xc8\xde\x1f\0\x12+\n\
    \x04Type\x18\x01\x20\x01(\x0e2\x11.raftpb.EntryTypeR\x04TypeB\x04\xc8\
    \xde\x1f\0\x12\x12\n\x04Data\x18\x04\x20\x01(\x0cR\x04Data\"\x80\x01\n\
    \x10SnapshotMetadata\x126\n\nconf_state\x18\x01\x20\x01(\x0b2\x11.raftpb\
    .ConfStateR\tconfStateB\x04\xc8\xde\x1f\0\x12\x1a\n\x05index\x18\x02\x20\
    \x01(\x04R\x05indexB\x04\xc8\xde\x1f\0\x12\x18\n\x04term\x18\x03\x20\x01\
    (\x04R\x04termB\x04\xc8\xde\x1f\0\"Z\n\x08Snapshot\x12\x12\n\x04data\x18\
    \x01\x20\x01(\x0cR\x04data\x12:\n\x08metadata\x18\x02\x20\x01(\x0b2\x18.\
    raftpb.SnapshotMetadataR\x08metadataB\x04\xc8\xde\x1f\0\"\x9d\x03\n\x07M\
    essage\x12-\n\x04type\x18\x01\x20\x01(\x0e2\x13.raftpb.MessageTypeR\x04t\
    ypeB\x04\xc8\xde\x1f\0\x12\x14\n\x02to\x18\x02\x20\x01(\x04R\x02toB\x04\
    \xc8\xde\x1f\0\x12\x18\n\x04from\x18\x03\x20\x01(\x04R\x04fromB\x04\xc8\
    \xde\x1f\0\x12\x18\n\x04term\x18\x04\x20\x01(\x04R\x04termB\x04\xc8\xde\
    \x1f\0\x12\x1e\n\x07logTerm\x18\x05\x20\x01(\x04R\x07logTermB\x04\xc8\
    \xde\x1f\0\x12\x1a\n\x05index\x18\x06\x20\x01(\x04R\x05indexB\x04\xc8\
    \xde\x1f\0\x12-\n\x07entries\x18\x07\x20\x03(\x0b2\r.raftpb.EntryR\x07en\
    triesB\x04\xc8\xde\x1f\0\x12\x1c\n\x06commit\x18\x08\x20\x01(\x04R\x06co\
    mmitB\x04\xc8\xde\x1f\0\x122\n\x08snapshot\x18\t\x20\x01(\x0b2\x10.raftp\
    b.SnapshotR\x08snapshotB\x04\xc8\xde\x1f\0\x12\x1c\n\x06reject\x18\n\x20\
    \x01(\x08R\x06rejectB\x04\xc8\xde\x1f\0\x12$\n\nrejectHint\x18\x0b\x20\
    \x01(\x04R\nrejectHintB\x04\xc8\xde\x1f\0\x12\x18\n\x07context\x18\x0c\
    \x20\x01(\x0cR\x07context\"]\n\tHardState\x12\x18\n\x04term\x18\x01\x20\
    \x01(\x04R\x04termB\x04\xc8\xde\x1f\0\x12\x18\n\x04vote\x18\x02\x20\x01(\
    \x04R\x04voteB\x04\xc8\xde\x1f\0\x12\x1c\n\x06commit\x18\x03\x20\x01(\
    \x04R\x06commitB\x04\xc8\xde\x1f\0\"=\n\tConfState\x12\x14\n\x05nodes\
    \x18\x01\x20\x03(\x04R\x05nodes\x12\x1a\n\x08learners\x18\x02\x20\x03(\
    \x04R\x08learners\"\x8c\x01\n\nConfChange\x12\x14\n\x02ID\x18\x01\x20\
    \x01(\x04R\x02IDB\x04\xc8\xde\x1f\0\x120\n\x04Type\x18\x02\x20\x01(\x0e2\
    \x16.raftpb.ConfChangeTypeR\x04TypeB\x04\xc8\xde\x1f\0\x12\x1c\n\x06Node\
    ID\x18\x03\x20\x01(\x04R\x06NodeIDB\x04\xc8\xde\x1f\0\x12\x18\n\x07Conte\
    xt\x18\x04\x20\x01(\x0cR\x07Context*1\n\tEntryType\x12\x0f\n\x0bEntryNor\
    mal\x10\0\x12\x13\n\x0fEntryConfChange\x10\x01*\xd3\x02\n\x0bMessageType\
    \x12\n\n\x06MsgHup\x10\0\x12\x0b\n\x07MsgBeat\x10\x01\x12\x0b\n\x07MsgPr\
    op\x10\x02\x12\n\n\x06MsgApp\x10\x03\x12\x0e\n\nMsgAppResp\x10\x04\x12\
    \x0b\n\x07MsgVote\x10\x05\x12\x0f\n\x0bMsgVoteResp\x10\x06\x12\x0b\n\x07\
    MsgSnap\x10\x07\x12\x10\n\x0cMsgHeartbeat\x10\x08\x12\x14\n\x10MsgHeartb\
    eatResp\x10\t\x12\x12\n\x0eMsgUnreachable\x10\n\x12\x11\n\rMsgSnapStatus\
    \x10\x0b\x12\x12\n\x0eMsgCheckQuorum\x10\x0c\x12\x15\n\x11MsgTransferLea\
    der\x10\r\x12\x11\n\rMsgTimeoutNow\x10\x0e\x12\x10\n\x0cMsgReadIndex\x10\
    \x0f\x12\x14\n\x10MsgReadIndexResp\x10\x10\x12\x0e\n\nMsgPreVote\x10\x11\
    \x12\x12\n\x0eMsgPreVoteResp\x10\x12*y\n\x0eConfChangeType\x12\x15\n\x11\
    ConfChangeAddNode\x10\0\x12\x18\n\x14ConfChangeRemoveNode\x10\x01\x12\
    \x18\n\x14ConfChangeUpdateNode\x10\x02\x12\x1c\n\x18ConfChangeAddLearner\
    Node\x10\x03B\x14\xd0\xe1\x1e\0\xc8\xe2\x1e\x01\xc8\xe1\x1e\0\xd0\xe2\
    \x1e\x01\xe0\xe2\x1e\x01J\x982\n\x06\x12\x04\0\0^\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0e\n\t\n\x02\x03\0\x12\
    \x03\x03\x07\x1d\n\x08\n\x01\x08\x12\x03\x05\0(\n\x0b\n\x04\x08\xe7\x07\
    \0\x12\x03\x05\0(\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x05\x07\x20\n\r\
    \n\x06\x08\xe7\x07\0\x02\0\x12\x03\x05\x07\x20\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x05\x08\x1f\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x05\
    #'\n\x08\n\x01\x08\x12\x03\x06\0$\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\
    \x06\0$\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x06\x07\x1c\n\r\n\x06\
    \x08\xe7\x07\x01\x02\0\x12\x03\x06\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x01\
    \x02\0\x01\x12\x03\x06\x08\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\
    \x06\x1f#\n\x08\n\x01\x08\x12\x03\x07\0*\n\x0b\n\x04\x08\xe7\x07\x02\x12\
    \x03\x07\0*\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x07\x07\"\n\r\n\x06\
    \x08\xe7\x07\x02\x02\0\x12\x03\x07\x07\"\n\x0e\n\x07\x08\xe7\x07\x02\x02\
    \0\x01\x12\x03\x07\x08!\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x07%)\n\
    \x08\n\x01\x08\x12\x03\x08\0/\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x08\0/\
    \n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x08\x07&\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x08\x07&\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x08\x08%\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x08).\n\x08\n\x01\
    \x08\x12\x03\t\03\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\t\03\n\x0c\n\x05\
    \x08\xe7\x07\x04\x02\x12\x03\t\x07*\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\
    \x03\t\x07*\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03\t\x08)\n\x0c\n\
    \x05\x08\xe7\x07\x04\x03\x12\x03\t-2\n\n\n\x02\x05\0\x12\x04\x0b\0\x0e\
    \x01\n\n\n\x03\x05\0\x01\x12\x03\x0b\x05\x0e\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03\x0c\x08\x1c\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x0c\x08\x13\n\x0c\n\
    \x05\x05\0\x02\0\x02\x12\x03\x0c\x1a\x1b\n\x0b\n\x04\x05\0\x02\x01\x12\
    \x03\r\x08\x1c\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\r\x08\x17\n\x0c\n\
    \x05\x05\0\x02\x01\x02\x12\x03\r\x1a\x1b\n\n\n\x02\x04\0\x12\x04\x10\0\
    \x15\x01\n\n\n\x03\x04\0\x01\x12\x03\x10\x08\r\n;\n\x04\x04\0\x02\0\x12\
    \x03\x11\x08E\".\x20must\x20be\x2064-bit\x20aligned\x20for\x20atomic\x20\
    operations\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x11\x08\x10\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x11\x11\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x11\x1c\x20\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x11$%\n\x0c\n\x05\x04\0\
    \x02\0\x08\x12\x03\x11&D\n\x0f\n\x08\x04\0\x02\0\x08\xe7\x07\0\x12\x03\
    \x11'C\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x02\x12\x03\x11';\n\x11\n\n\
    \x04\0\x02\0\x08\xe7\x07\0\x02\0\x12\x03\x11';\n\x12\n\x0b\x04\0\x02\0\
    \x08\xe7\x07\0\x02\0\x01\x12\x03\x11(:\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\
    \0\x03\x12\x03\x11>C\n;\n\x04\x04\0\x02\x01\x12\x03\x12\x08E\".\x20must\
    \x20be\x2064-bit\x20aligned\x20for\x20atomic\x20operations\n\n\x0c\n\x05\
    \x04\0\x02\x01\x04\x12\x03\x12\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x12\x11\x17\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x12\x1c!\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x12$%\n\x0c\n\x05\x04\0\x02\x01\x08\x12\
    \x03\x12&D\n\x0f\n\x08\x04\0\x02\x01\x08\xe7\x07\0\x12\x03\x12'C\n\x10\n\
    \t\x04\0\x02\x01\x08\xe7\x07\0\x02\x12\x03\x12';\n\x11\n\n\x04\0\x02\x01\
    \x08\xe7\x07\0\x02\0\x12\x03\x12';\n\x12\n\x0b\x04\0\x02\x01\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x12(:\n\x10\n\t\x04\0\x02\x01\x08\xe7\x07\0\x03\x12\
    \x03\x12>C\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x13\x08E\n\x0c\n\x05\x04\0\
    \x02\x02\x04\x12\x03\x13\x08\x10\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\
    \x13\x11\x1a\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x13\x1c\x20\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\x13$%\n\x0c\n\x05\x04\0\x02\x02\x08\x12\
    \x03\x13&D\n\x0f\n\x08\x04\0\x02\x02\x08\xe7\x07\0\x12\x03\x13'C\n\x10\n\
    \t\x04\0\x02\x02\x08\xe7\x07\0\x02\x12\x03\x13';\n\x11\n\n\x04\0\x02\x02\
    \x08\xe7\x07\0\x02\0\x12\x03\x13';\n\x12\n\x0b\x04\0\x02\x02\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x13(:\n\x10\n\t\x04\0\x02\x02\x08\xe7\x07\0\x03\x12\
    \x03\x13>C\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x14\x08&\n\x0c\n\x05\x04\0\
    \x02\x03\x04\x12\x03\x14\x08\x10\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\
    \x14\x11\x16\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x14\x1c\x20\n\x0c\n\
    \x05\x04\0\x02\x03\x03\x12\x03\x14$%\n\n\n\x02\x04\x01\x12\x04\x17\0\x1b\
    \x01\n\n\n\x03\x04\x01\x01\x12\x03\x17\x08\x18\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03\x18\x08I\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x18\x08\x10\n\
    \x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x18\x11\x1a\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03\x18\x1b%\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x18()\n\
    \x0c\n\x05\x04\x01\x02\0\x08\x12\x03\x18*H\n\x0f\n\x08\x04\x01\x02\0\x08\
    \xe7\x07\0\x12\x03\x18+G\n\x10\n\t\x04\x01\x02\0\x08\xe7\x07\0\x02\x12\
    \x03\x18+?\n\x11\n\n\x04\x01\x02\0\x08\xe7\x07\0\x02\0\x12\x03\x18+?\n\
    \x12\n\x0b\x04\x01\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03\x18,>\n\x10\n\t\
    \x04\x01\x02\0\x08\xe7\x07\0\x03\x12\x03\x18BG\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03\x19\x08I\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x19\x08\
    \x10\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x19\x11\x17\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03\x19\x1b\x20\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03\x19()\n\x0c\n\x05\x04\x01\x02\x01\x08\x12\x03\x19*H\n\x0f\n\x08\x04\
    \x01\x02\x01\x08\xe7\x07\0\x12\x03\x19+G\n\x10\n\t\x04\x01\x02\x01\x08\
    \xe7\x07\0\x02\x12\x03\x19+?\n\x11\n\n\x04\x01\x02\x01\x08\xe7\x07\0\x02\
    \0\x12\x03\x19+?\n\x12\n\x0b\x04\x01\x02\x01\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\x19,>\n\x10\n\t\x04\x01\x02\x01\x08\xe7\x07\0\x03\x12\x03\x19BG\n\
    \x0b\n\x04\x04\x01\x02\x02\x12\x03\x1a\x08I\n\x0c\n\x05\x04\x01\x02\x02\
    \x04\x12\x03\x1a\x08\x10\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x1a\x11\
    \x17\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1a\x1b\x1f\n\x0c\n\x05\x04\
    \x01\x02\x02\x03\x12\x03\x1a()\n\x0c\n\x05\x04\x01\x02\x02\x08\x12\x03\
    \x1a*H\n\x0f\n\x08\x04\x01\x02\x02\x08\xe7\x07\0\x12\x03\x1a+G\n\x10\n\t\
    \x04\x01\x02\x02\x08\xe7\x07\0\x02\x12\x03\x1a+?\n\x11\n\n\x04\x01\x02\
    \x02\x08\xe7\x07\0\x02\0\x12\x03\x1a+?\n\x12\n\x0b\x04\x01\x02\x02\x08\
    \xe7\x07\0\x02\0\x01\x12\x03\x1a,>\n\x10\n\t\x04\x01\x02\x02\x08\xe7\x07\
    \0\x03\x12\x03\x1aBG\n\n\n\x02\x04\x02\x12\x04\x1d\0\x20\x01\n\n\n\x03\
    \x04\x02\x01\x12\x03\x1d\x08\x10\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1e\
    \x08/\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x1e\x08\x10\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x1e\x11\x16\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x1e\"&\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1e-.\n\x0b\n\x04\x04\x02\
    \x02\x01\x12\x03\x1f\x08N\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\x1f\
    \x08\x10\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x1f\x11!\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03\x1f\"*\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x1f-.\n\x0c\n\x05\x04\x02\x02\x01\x08\x12\x03\x1f/M\n\x0f\n\x08\x04\
    \x02\x02\x01\x08\xe7\x07\0\x12\x03\x1f0L\n\x10\n\t\x04\x02\x02\x01\x08\
    \xe7\x07\0\x02\x12\x03\x1f0D\n\x11\n\n\x04\x02\x02\x01\x08\xe7\x07\0\x02\
    \0\x12\x03\x1f0D\n\x12\n\x0b\x04\x02\x02\x01\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\x1f1C\n\x10\n\t\x04\x02\x02\x01\x08\xe7\x07\0\x03\x12\x03\x1fGL\n\n\
    \n\x02\x05\x01\x12\x04\"\06\x01\n\n\n\x03\x05\x01\x01\x12\x03\"\x05\x10\
    \n\x0b\n\x04\x05\x01\x02\0\x12\x03#\x08\x1f\n\x0c\n\x05\x05\x01\x02\0\
    \x01\x12\x03#\x08\x0e\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03#\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\x01\x12\x03$\x08\x1f\n\x0c\n\x05\x05\x01\x02\x01\
    \x01\x12\x03$\x08\x0f\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03$\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\x02\x12\x03%\x08\x1f\n\x0c\n\x05\x05\x01\x02\x02\
    \x01\x12\x03%\x08\x0f\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03%\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\x03\x12\x03&\x08\x1f\n\x0c\n\x05\x05\x01\x02\x03\
    \x01\x12\x03&\x08\x0e\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x03&\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\x04\x12\x03'\x08\x1f\n\x0c\n\x05\x05\x01\x02\x04\
    \x01\x12\x03'\x08\x12\n\x0c\n\x05\x05\x01\x02\x04\x02\x12\x03'\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\x05\x12\x03(\x08\x1f\n\x0c\n\x05\x05\x01\x02\x05\
    \x01\x12\x03(\x08\x0f\n\x0c\n\x05\x05\x01\x02\x05\x02\x12\x03(\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\x06\x12\x03)\x08\x1f\n\x0c\n\x05\x05\x01\x02\x06\
    \x01\x12\x03)\x08\x13\n\x0c\n\x05\x05\x01\x02\x06\x02\x12\x03)\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\x07\x12\x03*\x08\x1f\n\x0c\n\x05\x05\x01\x02\x07\
    \x01\x12\x03*\x08\x0f\n\x0c\n\x05\x05\x01\x02\x07\x02\x12\x03*\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\x08\x12\x03+\x08\x1f\n\x0c\n\x05\x05\x01\x02\x08\
    \x01\x12\x03+\x08\x14\n\x0c\n\x05\x05\x01\x02\x08\x02\x12\x03+\x1d\x1e\n\
    \x0b\n\x04\x05\x01\x02\t\x12\x03,\x08\x1f\n\x0c\n\x05\x05\x01\x02\t\x01\
    \x12\x03,\x08\x18\n\x0c\n\x05\x05\x01\x02\t\x02\x12\x03,\x1d\x1e\n\x0b\n\
    \x04\x05\x01\x02\n\x12\x03-\x08\x20\n\x0c\n\x05\x05\x01\x02\n\x01\x12\
    \x03-\x08\x16\n\x0c\n\x05\x05\x01\x02\n\x02\x12\x03-\x1d\x1f\n\x0b\n\x04\
    \x05\x01\x02\x0b\x12\x03.\x08\x20\n\x0c\n\x05\x05\x01\x02\x0b\x01\x12\
    \x03.\x08\x15\n\x0c\n\x05\x05\x01\x02\x0b\x02\x12\x03.\x1d\x1f\n\x0b\n\
    \x04\x05\x01\x02\x0c\x12\x03/\x08\x20\n\x0c\n\x05\x05\x01\x02\x0c\x01\
    \x12\x03/\x08\x16\n\x0c\n\x05\x05\x01\x02\x0c\x02\x12\x03/\x1d\x1f\n\x0b\
    \n\x04\x05\x01\x02\r\x12\x030\x08\x20\n\x0c\n\x05\x05\x01\x02\r\x01\x12\
    \x030\x08\x19\n\x0c\n\x05\x05\x01\x02\r\x02\x12\x030\x1d\x1f\n\x0b\n\x04\
    \x05\x01\x02\x0e\x12\x031\x08\x20\n\x0c\n\x05\x05\x01\x02\x0e\x01\x12\
    \x031\x08\x15\n\x0c\n\x05\x05\x01\x02\x0e\x02\x12\x031\x1d\x1f\n\x0b\n\
    \x04\x05\x01\x02\x0f\x12\x032\x08\x20\n\x0c\n\x05\x05\x01\x02\x0f\x01\
    \x12\x032\x08\x14\n\x0c\n\x05\x05\x01\x02\x0f\x02\x12\x032\x1d\x1f\n\x0b\
    \n\x04\x05\x01\x02\x10\x12\x033\x08\x20\n\x0c\n\x05\x05\x01\x02\x10\x01\
    \x12\x033\x08\x18\n\x0c\n\x05\x05\x01\x02\x10\x02\x12\x033\x1d\x1f\n\x0b\
    \n\x04\x05\x01\x02\x11\x12\x034\x08\x20\n\x0c\n\x05\x05\x01\x02\x11\x01\
    \x12\x034\x08\x12\n\x0c\n\x05\x05\x01\x02\x11\x02\x12\x034\x1d\x1f\n\x0b\
    \n\x04\x05\x01\x02\x12\x12\x035\x08\x20\n\x0c\n\x05\x05\x01\x02\x12\x01\
    \x12\x035\x08\x16\n\x0c\n\x05\x05\x01\x02\x12\x02\x12\x035\x1d\x1f\n\n\n\
    \x02\x04\x03\x12\x048\0E\x01\n\n\n\x03\x04\x03\x01\x12\x038\x08\x0f\n\
    \x0b\n\x04\x04\x03\x02\0\x12\x039\x08M\n\x0c\n\x05\x04\x03\x02\0\x04\x12\
    \x039\x08\x10\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x039\x11\x1c\n\x0c\n\x05\
    \x04\x03\x02\0\x01\x12\x039\x1d!\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x039+\
    ,\n\x0c\n\x05\x04\x03\x02\0\x08\x12\x039.L\n\x0f\n\x08\x04\x03\x02\0\x08\
    \xe7\x07\0\x12\x039/K\n\x10\n\t\x04\x03\x02\0\x08\xe7\x07\0\x02\x12\x039\
    /C\n\x11\n\n\x04\x03\x02\0\x08\xe7\x07\0\x02\0\x12\x039/C\n\x12\n\x0b\
    \x04\x03\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x0390B\n\x10\n\t\x04\x03\x02\
    \0\x08\xe7\x07\0\x03\x12\x039FK\n\x0b\n\x04\x04\x03\x02\x01\x12\x03:\x08\
    M\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03:\x08\x10\n\x0c\n\x05\x04\x03\
    \x02\x01\x05\x12\x03:\x11\x17\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03:\
    \x1d\x1f\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03:+,\n\x0c\n\x05\x04\x03\
    \x02\x01\x08\x12\x03:.L\n\x0f\n\x08\x04\x03\x02\x01\x08\xe7\x07\0\x12\
    \x03:/K\n\x10\n\t\x04\x03\x02\x01\x08\xe7\x07\0\x02\x12\x03:/C\n\x11\n\n\
    \x04\x03\x02\x01\x08\xe7\x07\0\x02\0\x12\x03:/C\n\x12\n\x0b\x04\x03\x02\
    \x01\x08\xe7\x07\0\x02\0\x01\x12\x03:0B\n\x10\n\t\x04\x03\x02\x01\x08\
    \xe7\x07\0\x03\x12\x03:FK\n\x0b\n\x04\x04\x03\x02\x02\x12\x03;\x08M\n\
    \x0c\n\x05\x04\x03\x02\x02\x04\x12\x03;\x08\x10\n\x0c\n\x05\x04\x03\x02\
    \x02\x05\x12\x03;\x11\x17\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03;\x1d!\
    \n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03;+,\n\x0c\n\x05\x04\x03\x02\x02\
    \x08\x12\x03;.L\n\x0f\n\x08\x04\x03\x02\x02\x08\xe7\x07\0\x12\x03;/K\n\
    \x10\n\t\x04\x03\x02\x02\x08\xe7\x07\0\x02\x12\x03;/C\n\x11\n\n\x04\x03\
    \x02\x02\x08\xe7\x07\0\x02\0\x12\x03;/C\n\x12\n\x0b\x04\x03\x02\x02\x08\
    \xe7\x07\0\x02\0\x01\x12\x03;0B\n\x10\n\t\x04\x03\x02\x02\x08\xe7\x07\0\
    \x03\x12\x03;FK\n\x0b\n\x04\x04\x03\x02\x03\x12\x03<\x08M\n\x0c\n\x05\
    \x04\x03\x02\x03\x04\x12\x03<\x08\x10\n\x0c\n\x05\x04\x03\x02\x03\x05\
    \x12\x03<\x11\x17\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03<\x1d!\n\x0c\n\
    \x05\x04\x03\x02\x03\x03\x12\x03<+,\n\x0c\n\x05\x04\x03\x02\x03\x08\x12\
    \x03<.L\n\x0f\n\x08\x04\x03\x02\x03\x08\xe7\x07\0\x12\x03</K\n\x10\n\t\
    \x04\x03\x02\x03\x08\xe7\x07\0\x02\x12\x03</C\n\x11\n\n\x04\x03\x02\x03\
    \x08\xe7\x07\0\x02\0\x12\x03</C\n\x12\n\x0b\x04\x03\x02\x03\x08\xe7\x07\
    \0\x02\0\x01\x12\x03<0B\n\x10\n\t\x04\x03\x02\x03\x08\xe7\x07\0\x03\x12\
    \x03<FK\n\x0b\n\x04\x04\x03\x02\x04\x12\x03=\x08M\n\x0c\n\x05\x04\x03\
    \x02\x04\x04\x12\x03=\x08\x10\n\x0c\n\x05\x04\x03\x02\x04\x05\x12\x03=\
    \x11\x17\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\x03=\x1d$\n\x0c\n\x05\x04\
    \x03\x02\x04\x03\x12\x03=+,\n\x0c\n\x05\x04\x03\x02\x04\x08\x12\x03=.L\n\
    \x0f\n\x08\x04\x03\x02\x04\x08\xe7\x07\0\x12\x03=/K\n\x10\n\t\x04\x03\
    \x02\x04\x08\xe7\x07\0\x02\x12\x03=/C\n\x11\n\n\x04\x03\x02\x04\x08\xe7\
    \x07\0\x02\0\x12\x03=/C\n\x12\n\x0b\x04\x03\x02\x04\x08\xe7\x07\0\x02\0\
    \x01\x12\x03=0B\n\x10\n\t\x04\x03\x02\x04\x08\xe7\x07\0\x03\x12\x03=FK\n\
    \x0b\n\x04\x04\x03\x02\x05\x12\x03>\x08M\n\x0c\n\x05\x04\x03\x02\x05\x04\
    \x12\x03>\x08\x10\n\x0c\n\x05\x04\x03\x02\x05\x05\x12\x03>\x11\x17\n\x0c\
    \n\x05\x04\x03\x02\x05\x01\x12\x03>\x1d\"\n\x0c\n\x05\x04\x03\x02\x05\
    \x03\x12\x03>+,\n\x0c\n\x05\x04\x03\x02\x05\x08\x12\x03>.L\n\x0f\n\x08\
    \x04\x03\x02\x05\x08\xe7\x07\0\x12\x03>/K\n\x10\n\t\x04\x03\x02\x05\x08\
    \xe7\x07\0\x02\x12\x03>/C\n\x11\n\n\x04\x03\x02\x05\x08\xe7\x07\0\x02\0\
    \x12\x03>/C\n\x12\n\x0b\x04\x03\x02\x05\x08\xe7\x07\0\x02\0\x01\x12\x03>\
    0B\n\x10\n\t\x04\x03\x02\x05\x08\xe7\x07\0\x03\x12\x03>FK\n\x0b\n\x04\
    \x04\x03\x02\x06\x12\x03?\x08M\n\x0c\n\x05\x04\x03\x02\x06\x04\x12\x03?\
    \x08\x10\n\x0c\n\x05\x04\x03\x02\x06\x06\x12\x03?\x11\x16\n\x0c\n\x05\
    \x04\x03\x02\x06\x01\x12\x03?\x1d$\n\x0c\n\x05\x04\x03\x02\x06\x03\x12\
    \x03?+,\n\x0c\n\x05\x04\x03\x02\x06\x08\x12\x03?.L\n\x0f\n\x08\x04\x03\
    \x02\x06\x08\xe7\x07\0\x12\x03?/K\n\x10\n\t\x04\x03\x02\x06\x08\xe7\x07\
    \0\x02\x12\x03?/C\n\x11\n\n\x04\x03\x02\x06\x08\xe7\x07\0\x02\0\x12\x03?\
    /C\n\x12\n\x0b\x04\x03\x02\x06\x08\xe7\x07\0\x02\0\x01\x12\x03?0B\n\x10\
    \n\t\x04\x03\x02\x06\x08\xe7\x07\0\x03\x12\x03?FK\n\x0b\n\x04\x04\x03\
    \x02\x07\x12\x03@\x08M\n\x0c\n\x05\x04\x03\x02\x07\x04\x12\x03@\x08\x10\
    \n\x0c\n\x05\x04\x03\x02\x07\x05\x12\x03@\x11\x17\n\x0c\n\x05\x04\x03\
    \x02\x07\x01\x12\x03@\x1d#\n\x0c\n\x05\x04\x03\x02\x07\x03\x12\x03@+,\n\
    \x0c\n\x05\x04\x03\x02\x07\x08\x12\x03@.L\n\x0f\n\x08\x04\x03\x02\x07\
    \x08\xe7\x07\0\x12\x03@/K\n\x10\n\t\x04\x03\x02\x07\x08\xe7\x07\0\x02\
    \x12\x03@/C\n\x11\n\n\x04\x03\x02\x07\x08\xe7\x07\0\x02\0\x12\x03@/C\n\
    \x12\n\x0b\x04\x03\x02\x07\x08\xe7\x07\0\x02\0\x01\x12\x03@0B\n\x10\n\t\
    \x04\x03\x02\x07\x08\xe7\x07\0\x03\x12\x03@FK\n\x0b\n\x04\x04\x03\x02\
    \x08\x12\x03A\x08M\n\x0c\n\x05\x04\x03\x02\x08\x04\x12\x03A\x08\x10\n\
    \x0c\n\x05\x04\x03\x02\x08\x06\x12\x03A\x11\x19\n\x0c\n\x05\x04\x03\x02\
    \x08\x01\x12\x03A\x1d%\n\x0c\n\x05\x04\x03\x02\x08\x03\x12\x03A+,\n\x0c\
    \n\x05\x04\x03\x02\x08\x08\x12\x03A.L\n\x0f\n\x08\x04\x03\x02\x08\x08\
    \xe7\x07\0\x12\x03A/K\n\x10\n\t\x04\x03\x02\x08\x08\xe7\x07\0\x02\x12\
    \x03A/C\n\x11\n\n\x04\x03\x02\x08\x08\xe7\x07\0\x02\0\x12\x03A/C\n\x12\n\
    \x0b\x04\x03\x02\x08\x08\xe7\x07\0\x02\0\x01\x12\x03A0B\n\x10\n\t\x04\
    \x03\x02\x08\x08\xe7\x07\0\x03\x12\x03AFK\n\x0b\n\x04\x04\x03\x02\t\x12\
    \x03B\x08M\n\x0c\n\x05\x04\x03\x02\t\x04\x12\x03B\x08\x10\n\x0c\n\x05\
    \x04\x03\x02\t\x05\x12\x03B\x11\x15\n\x0c\n\x05\x04\x03\x02\t\x01\x12\
    \x03B\x1d#\n\x0c\n\x05\x04\x03\x02\t\x03\x12\x03B+-\n\x0c\n\x05\x04\x03\
    \x02\t\x08\x12\x03B.L\n\x0f\n\x08\x04\x03\x02\t\x08\xe7\x07\0\x12\x03B/K\
    \n\x10\n\t\x04\x03\x02\t\x08\xe7\x07\0\x02\x12\x03B/C\n\x11\n\n\x04\x03\
    \x02\t\x08\xe7\x07\0\x02\0\x12\x03B/C\n\x12\n\x0b\x04\x03\x02\t\x08\xe7\
    \x07\0\x02\0\x01\x12\x03B0B\n\x10\n\t\x04\x03\x02\t\x08\xe7\x07\0\x03\
    \x12\x03BFK\n\x0b\n\x04\x04\x03\x02\n\x12\x03C\x08M\n\x0c\n\x05\x04\x03\
    \x02\n\x04\x12\x03C\x08\x10\n\x0c\n\x05\x04\x03\x02\n\x05\x12\x03C\x11\
    \x17\n\x0c\n\x05\x04\x03\x02\n\x01\x12\x03C\x1d'\n\x0c\n\x05\x04\x03\x02\
    \n\x03\x12\x03C+-\n\x0c\n\x05\x04\x03\x02\n\x08\x12\x03C.L\n\x0f\n\x08\
    \x04\x03\x02\n\x08\xe7\x07\0\x12\x03C/K\n\x10\n\t\x04\x03\x02\n\x08\xe7\
    \x07\0\x02\x12\x03C/C\n\x11\n\n\x04\x03\x02\n\x08\xe7\x07\0\x02\0\x12\
    \x03C/C\n\x12\n\x0b\x04\x03\x02\n\x08\xe7\x07\0\x02\0\x01\x12\x03C0B\n\
    \x10\n\t\x04\x03\x02\n\x08\xe7\x07\0\x03\x12\x03CFK\n\x0b\n\x04\x04\x03\
    \x02\x0b\x12\x03D\x08.\n\x0c\n\x05\x04\x03\x02\x0b\x04\x12\x03D\x08\x10\
    \n\x0c\n\x05\x04\x03\x02\x0b\x05\x12\x03D\x11\x16\n\x0c\n\x05\x04\x03\
    \x02\x0b\x01\x12\x03D\x1d$\n\x0c\n\x05\x04\x03\x02\x0b\x03\x12\x03D+-\n\
    \n\n\x02\x04\x04\x12\x04G\0K\x01\n\n\n\x03\x04\x04\x01\x12\x03G\x08\x11\
    \n\x0b\n\x04\x04\x04\x02\0\x12\x03H\x08B\n\x0c\n\x05\x04\x04\x02\0\x04\
    \x12\x03H\x08\x10\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03H\x11\x17\n\x0c\n\
    \x05\x04\x04\x02\0\x01\x12\x03H\x18\x1c\n\x0c\n\x05\x04\x04\x02\0\x03\
    \x12\x03H!\"\n\x0c\n\x05\x04\x04\x02\0\x08\x12\x03H#A\n\x0f\n\x08\x04\
    \x04\x02\0\x08\xe7\x07\0\x12\x03H$@\n\x10\n\t\x04\x04\x02\0\x08\xe7\x07\
    \0\x02\x12\x03H$8\n\x11\n\n\x04\x04\x02\0\x08\xe7\x07\0\x02\0\x12\x03H$8\
    \n\x12\n\x0b\x04\x04\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03H%7\n\x10\n\t\
    \x04\x04\x02\0\x08\xe7\x07\0\x03\x12\x03H;@\n\x0b\n\x04\x04\x04\x02\x01\
    \x12\x03I\x08B\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03I\x08\x10\n\x0c\n\
    \x05\x04\x04\x02\x01\x05\x12\x03I\x11\x17\n\x0c\n\x05\x04\x04\x02\x01\
    \x01\x12\x03I\x18\x1c\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03I!\"\n\x0c\
    \n\x05\x04\x04\x02\x01\x08\x12\x03I#A\n\x0f\n\x08\x04\x04\x02\x01\x08\
    \xe7\x07\0\x12\x03I$@\n\x10\n\t\x04\x04\x02\x01\x08\xe7\x07\0\x02\x12\
    \x03I$8\n\x11\n\n\x04\x04\x02\x01\x08\xe7\x07\0\x02\0\x12\x03I$8\n\x12\n\
    \x0b\x04\x04\x02\x01\x08\xe7\x07\0\x02\0\x01\x12\x03I%7\n\x10\n\t\x04\
    \x04\x02\x01\x08\xe7\x07\0\x03\x12\x03I;@\n\x0b\n\x04\x04\x04\x02\x02\
    \x12\x03J\x08B\n\x0c\n\x05\x04\x04\x02\x02\x04\x12\x03J\x08\x10\n\x0c\n\
    \x05\x04\x04\x02\x02\x05\x12\x03J\x11\x17\n\x0c\n\x05\x04\x04\x02\x02\
    \x01\x12\x03J\x18\x1e\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03J!\"\n\x0c\
    \n\x05\x04\x04\x02\x02\x08\x12\x03J#A\n\x0f\n\x08\x04\x04\x02\x02\x08\
    \xe7\x07\0\x12\x03J$@\n\x10\n\t\x04\x04\x02\x02\x08\xe7\x07\0\x02\x12\
    \x03J$8\n\x11\n\n\x04\x04\x02\x02\x08\xe7\x07\0\x02\0\x12\x03J$8\n\x12\n\
    \x0b\x04\x04\x02\x02\x08\xe7\x07\0\x02\0\x01\x12\x03J%7\n\x10\n\t\x04\
    \x04\x02\x02\x08\xe7\x07\0\x03\x12\x03J;@\n\n\n\x02\x04\x05\x12\x04M\0P\
    \x01\n\n\n\x03\x04\x05\x01\x12\x03M\x08\x11\n\x0b\n\x04\x04\x05\x02\0\
    \x12\x03N\x08%\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03N\x08\x10\n\x0c\n\
    \x05\x04\x05\x02\0\x05\x12\x03N\x11\x17\n\x0c\n\x05\x04\x05\x02\0\x01\
    \x12\x03N\x18\x1d\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03N#$\n\x0b\n\x04\
    \x04\x05\x02\x01\x12\x03O\x08%\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03O\
    \x08\x10\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03O\x11\x17\n\x0c\n\x05\
    \x04\x05\x02\x01\x01\x12\x03O\x18\x20\n\x0c\n\x05\x04\x05\x02\x01\x03\
    \x12\x03O#$\n\n\n\x02\x05\x02\x12\x04R\0W\x01\n\n\n\x03\x05\x02\x01\x12\
    \x03R\x05\x13\n\x0b\n\x04\x05\x02\x02\0\x12\x03S\x08%\n\x0c\n\x05\x05\
    \x02\x02\0\x01\x12\x03S\x08\x19\n\x0c\n\x05\x05\x02\x02\0\x02\x12\x03S#$\
    \n\x0b\n\x04\x05\x02\x02\x01\x12\x03T\x08%\n\x0c\n\x05\x05\x02\x02\x01\
    \x01\x12\x03T\x08\x1c\n\x0c\n\x05\x05\x02\x02\x01\x02\x12\x03T#$\n\x0b\n\
    \x04\x05\x02\x02\x02\x12\x03U\x08%\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\
    \x03U\x08\x1c\n\x0c\n\x05\x05\x02\x02\x02\x02\x12\x03U#$\n\x0b\n\x04\x05\
    \x02\x02\x03\x12\x03V\x08%\n\x0c\n\x05\x05\x02\x02\x03\x01\x12\x03V\x08\
    \x20\n\x0c\n\x05\x05\x02\x02\x03\x02\x12\x03V#$\n\n\n\x02\x04\x06\x12\
    \x04Y\0^\x01\n\n\n\x03\x04\x06\x01\x12\x03Y\x08\x12\n\x0b\n\x04\x04\x06\
    \x02\0\x12\x03Z\x08L\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03Z\x08\x10\n\
    \x0c\n\x05\x04\x06\x02\0\x05\x12\x03Z\x11\x17\n\x0c\n\x05\x04\x06\x02\0\
    \x01\x12\x03Z!#\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03Z+,\n\x0c\n\x05\x04\
    \x06\x02\0\x08\x12\x03Z-K\n\x0f\n\x08\x04\x06\x02\0\x08\xe7\x07\0\x12\
    \x03Z.J\n\x10\n\t\x04\x06\x02\0\x08\xe7\x07\0\x02\x12\x03Z.B\n\x11\n\n\
    \x04\x06\x02\0\x08\xe7\x07\0\x02\0\x12\x03Z.B\n\x12\n\x0b\x04\x06\x02\0\
    \x08\xe7\x07\0\x02\0\x01\x12\x03Z/A\n\x10\n\t\x04\x06\x02\0\x08\xe7\x07\
    \0\x03\x12\x03ZEJ\n\x0b\n\x04\x04\x06\x02\x01\x12\x03[\x08L\n\x0c\n\x05\
    \x04\x06\x02\x01\x04\x12\x03[\x08\x10\n\x0c\n\x05\x04\x06\x02\x01\x06\
    \x12\x03[\x11\x1f\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03[!%\n\x0c\n\x05\
    \x04\x06\x02\x01\x03\x12\x03[+,\n\x0c\n\x05\x04\x06\x02\x01\x08\x12\x03[\
    -K\n\x0f\n\x08\x04\x06\x02\x01\x08\xe7\x07\0\x12\x03[.J\n\x10\n\t\x04\
    \x06\x02\x01\x08\xe7\x07\0\x02\x12\x03[.B\n\x11\n\n\x04\x06\x02\x01\x08\
    \xe7\x07\0\x02\0\x12\x03[.B\n\x12\n\x0b\x04\x06\x02\x01\x08\xe7\x07\0\
    \x02\0\x01\x12\x03[/A\n\x10\n\t\x04\x06\x02\x01\x08\xe7\x07\0\x03\x12\
    \x03[EJ\n\x0b\n\x04\x04\x06\x02\x02\x12\x03\\\x08L\n\x0c\n\x05\x04\x06\
    \x02\x02\x04\x12\x03\\\x08\x10\n\x0c\n\x05\x04\x06\x02\x02\x05\x12\x03\\\
    \x11\x17\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x03\\!'\n\x0c\n\x05\x04\x06\
    \x02\x02\x03\x12\x03\\+,\n\x0c\n\x05\x04\x06\x02\x02\x08\x12\x03\\-K\n\
    \x0f\n\x08\x04\x06\x02\x02\x08\xe7\x07\0\x12\x03\\.J\n\x10\n\t\x04\x06\
    \x02\x02\x08\xe7\x07\0\x02\x12\x03\\.B\n\x11\n\n\x04\x06\x02\x02\x08\xe7\
    \x07\0\x02\0\x12\x03\\.B\n\x12\n\x0b\x04\x06\x02\x02\x08\xe7\x07\0\x02\0\
    \x01\x12\x03\\/A\n\x10\n\t\x04\x06\x02\x02\x08\xe7\x07\0\x03\x12\x03\\EJ\
    \n\x0b\n\x04\x04\x06\x02\x03\x12\x03]\x08-\n\x0c\n\x05\x04\x06\x02\x03\
    \x04\x12\x03]\x08\x10\n\x0c\n\x05\x04\x06\x02\x03\x05\x12\x03]\x11\x16\n\
    \x0c\n\x05\x04\x06\x02\x03\x01\x12\x03]!(\n\x0c\n\x05\x04\x06\x02\x03\
    \x03\x12\x03]+,\
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
