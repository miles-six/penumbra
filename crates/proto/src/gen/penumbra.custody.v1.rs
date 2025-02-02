#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeRequest {
    /// The transaction plan to authorize.
    #[prost(message, optional, tag = "1")]
    pub plan: ::core::option::Option<
        super::super::core::transaction::v1::TransactionPlan,
    >,
    /// Optionally, pre-authorization data, if required by the custodian.
    ///
    /// Pre-authorization data is backend-specific, and backends are free to ignore it.
    ///
    /// Multiple `PreAuthorization` packets can be included in a single request,
    /// to support multi-party pre-authorizations.
    #[prost(message, repeated, tag = "3")]
    pub pre_authorizations: ::prost::alloc::vec::Vec<PreAuthorization>,
}
impl ::prost::Name for AuthorizeRequest {
    const NAME: &'static str = "AuthorizeRequest";
    const PACKAGE: &'static str = "penumbra.custody.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.custody.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeResponse {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<
        super::super::core::transaction::v1::AuthorizationData,
    >,
}
impl ::prost::Name for AuthorizeResponse {
    const NAME: &'static str = "AuthorizeResponse";
    const PACKAGE: &'static str = "penumbra.custody.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.custody.v1.{}", Self::NAME)
    }
}
/// A pre-authorization packet.  This allows a custodian to delegate (partial)
/// signing authority to other authorization mechanisms.  Details of how a
/// custodian manages those keys are out-of-scope for the custody protocol and
/// are custodian-specific.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreAuthorization {
    #[prost(oneof = "pre_authorization::PreAuthorization", tags = "1")]
    pub pre_authorization: ::core::option::Option<pre_authorization::PreAuthorization>,
}
/// Nested message and enum types in `PreAuthorization`.
pub mod pre_authorization {
    /// An Ed25519-based preauthorization, containing an Ed25519 signature over the
    /// `TransactionPlan`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ed25519 {
        /// The Ed25519 verification key used to verify the signature.
        #[prost(bytes = "vec", tag = "1")]
        pub vk: ::prost::alloc::vec::Vec<u8>,
        /// The Ed25519 signature over the `TransactionPlan`.
        #[prost(bytes = "vec", tag = "2")]
        pub sig: ::prost::alloc::vec::Vec<u8>,
    }
    impl ::prost::Name for Ed25519 {
        const NAME: &'static str = "Ed25519";
        const PACKAGE: &'static str = "penumbra.custody.v1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "penumbra.custody.v1.PreAuthorization.{}", Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PreAuthorization {
        #[prost(message, tag = "1")]
        Ed25519(Ed25519),
    }
}
impl ::prost::Name for PreAuthorization {
    const NAME: &'static str = "PreAuthorization";
    const PACKAGE: &'static str = "penumbra.custody.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.custody.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFullViewingKeyRequest {}
impl ::prost::Name for ExportFullViewingKeyRequest {
    const NAME: &'static str = "ExportFullViewingKeyRequest";
    const PACKAGE: &'static str = "penumbra.custody.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.custody.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFullViewingKeyResponse {
    /// The full viewing key.
    #[prost(message, optional, tag = "1")]
    pub full_viewing_key: ::core::option::Option<
        super::super::core::keys::v1::FullViewingKey,
    >,
}
impl ::prost::Name for ExportFullViewingKeyResponse {
    const NAME: &'static str = "ExportFullViewingKeyResponse";
    const PACKAGE: &'static str = "penumbra.custody.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.custody.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmAddressRequest {
    #[prost(message, optional, tag = "1")]
    pub address_index: ::core::option::Option<
        super::super::core::keys::v1::AddressIndex,
    >,
}
impl ::prost::Name for ConfirmAddressRequest {
    const NAME: &'static str = "ConfirmAddressRequest";
    const PACKAGE: &'static str = "penumbra.custody.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.custody.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmAddressResponse {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::core::keys::v1::Address>,
}
impl ::prost::Name for ConfirmAddressResponse {
    const NAME: &'static str = "ConfirmAddressResponse";
    const PACKAGE: &'static str = "penumbra.custody.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("penumbra.custody.v1.{}", Self::NAME)
    }
}
/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod custody_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The custody protocol is used by a wallet client to request authorization for
    /// a transaction they've constructed.
    ///
    /// Modeling transaction authorization as an asynchronous RPC call encourages
    /// software to be written in a way that has a compatible data flow with a "soft
    /// HSM", threshold signing, a hardware wallet, etc.
    ///
    /// The custody protocol does not trust the client to authorize spends, so
    /// custody requests must contain sufficient information for the custodian to
    /// understand the transaction and determine whether or not it should be
    /// authorized.
    #[derive(Debug, Clone)]
    pub struct CustodyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CustodyServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CustodyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CustodyServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CustodyServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Requests authorization of the transaction with the given description.
        pub async fn authorize(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthorizeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthorizeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.custody.v1.CustodyService/Authorize",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("penumbra.custody.v1.CustodyService", "Authorize"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Requests the full viewing key from the custodian.
        ///
        /// Custody backends should decide whether to honor this request, and how to
        /// control access to it.
        pub async fn export_full_viewing_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportFullViewingKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportFullViewingKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.custody.v1.CustodyService/ExportFullViewingKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "penumbra.custody.v1.CustodyService",
                        "ExportFullViewingKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Displays an address to a user for confirmation.
        ///
        /// Custody backends with user interaction should present the address to the
        /// user and wait for explicit confirmation before returning.
        ///
        /// Non-interactive custody backends may return immediately.
        pub async fn confirm_address(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.custody.v1.CustodyService/ConfirmAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "penumbra.custody.v1.CustodyService",
                        "ConfirmAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod custody_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CustodyServiceServer.
    #[async_trait]
    pub trait CustodyService: Send + Sync + 'static {
        /// Requests authorization of the transaction with the given description.
        async fn authorize(
            &self,
            request: tonic::Request<super::AuthorizeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthorizeResponse>,
            tonic::Status,
        >;
        /// Requests the full viewing key from the custodian.
        ///
        /// Custody backends should decide whether to honor this request, and how to
        /// control access to it.
        async fn export_full_viewing_key(
            &self,
            request: tonic::Request<super::ExportFullViewingKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportFullViewingKeyResponse>,
            tonic::Status,
        >;
        /// Displays an address to a user for confirmation.
        ///
        /// Custody backends with user interaction should present the address to the
        /// user and wait for explicit confirmation before returning.
        ///
        /// Non-interactive custody backends may return immediately.
        async fn confirm_address(
            &self,
            request: tonic::Request<super::ConfirmAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmAddressResponse>,
            tonic::Status,
        >;
    }
    /// The custody protocol is used by a wallet client to request authorization for
    /// a transaction they've constructed.
    ///
    /// Modeling transaction authorization as an asynchronous RPC call encourages
    /// software to be written in a way that has a compatible data flow with a "soft
    /// HSM", threshold signing, a hardware wallet, etc.
    ///
    /// The custody protocol does not trust the client to authorize spends, so
    /// custody requests must contain sufficient information for the custodian to
    /// understand the transaction and determine whether or not it should be
    /// authorized.
    #[derive(Debug)]
    pub struct CustodyServiceServer<T: CustodyService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CustodyService> CustodyServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CustodyServiceServer<T>
    where
        T: CustodyService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/penumbra.custody.v1.CustodyService/Authorize" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeSvc<T: CustodyService>(pub Arc<T>);
                    impl<
                        T: CustodyService,
                    > tonic::server::UnaryService<super::AuthorizeRequest>
                    for AuthorizeSvc<T> {
                        type Response = super::AuthorizeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthorizeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustodyService>::authorize(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthorizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.custody.v1.CustodyService/ExportFullViewingKey" => {
                    #[allow(non_camel_case_types)]
                    struct ExportFullViewingKeySvc<T: CustodyService>(pub Arc<T>);
                    impl<
                        T: CustodyService,
                    > tonic::server::UnaryService<super::ExportFullViewingKeyRequest>
                    for ExportFullViewingKeySvc<T> {
                        type Response = super::ExportFullViewingKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportFullViewingKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustodyService>::export_full_viewing_key(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExportFullViewingKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.custody.v1.CustodyService/ConfirmAddress" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmAddressSvc<T: CustodyService>(pub Arc<T>);
                    impl<
                        T: CustodyService,
                    > tonic::server::UnaryService<super::ConfirmAddressRequest>
                    for ConfirmAddressSvc<T> {
                        type Response = super::ConfirmAddressResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CustodyService>::confirm_address(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfirmAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: CustodyService> Clone for CustodyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: CustodyService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CustodyService> tonic::server::NamedService for CustodyServiceServer<T> {
        const NAME: &'static str = "penumbra.custody.v1.CustodyService";
    }
}
