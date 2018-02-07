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


// interface

pub trait Lock {
    fn lock(&self, o: ::grpc::RequestOptions, p: super::v3lock::LockRequest) -> ::grpc::SingleResponse<super::v3lock::LockResponse>;

    fn unlock(&self, o: ::grpc::RequestOptions, p: super::v3lock::UnlockRequest) -> ::grpc::SingleResponse<super::v3lock::UnlockResponse>;
}

// client

pub struct LockClient {
    grpc_client: ::grpc::Client,
    method_Lock: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::v3lock::LockRequest, super::v3lock::LockResponse>>,
    method_Unlock: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::v3lock::UnlockRequest, super::v3lock::UnlockResponse>>,
}

impl LockClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        LockClient {
            grpc_client: grpc_client,
            method_Lock: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/v3lockpb.Lock/Lock".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Unlock: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/v3lockpb.Lock/Unlock".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            LockClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            LockClient::with_client(c)
        })
    }
}

impl Lock for LockClient {
    fn lock(&self, o: ::grpc::RequestOptions, p: super::v3lock::LockRequest) -> ::grpc::SingleResponse<super::v3lock::LockResponse> {
        self.grpc_client.call_unary(o, p, self.method_Lock.clone())
    }

    fn unlock(&self, o: ::grpc::RequestOptions, p: super::v3lock::UnlockRequest) -> ::grpc::SingleResponse<super::v3lock::UnlockResponse> {
        self.grpc_client.call_unary(o, p, self.method_Unlock.clone())
    }
}

// server

pub struct LockServer;


impl LockServer {
    pub fn new_service_def<H : Lock + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/v3lockpb.Lock",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/v3lockpb.Lock/Lock".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.lock(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/v3lockpb.Lock/Unlock".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlock(o, p))
                    },
                ),
            ],
        )
    }
}
