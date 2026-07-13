// @generated
/// Generated client implementations.
pub mod ble_manager_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BleManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BleManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> BleManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            BleManagerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn scan_ble(
            &mut self,
            request: impl tonic::IntoRequest<super::ScanBleRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::BluetoothDevice>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.BleManager/ScanBle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.BleManager",
                        "ScanBle",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn connect_ble(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectBleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConnectBleReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.BleManager/ConnectBle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.BleManager",
                        "ConnectBle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn write(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteRequest>,
        ) -> std::result::Result<tonic::Response<super::WriteReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.BleManager/Write",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cloud.lazycat.apis.localdevice.BleManager", "Write"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.BleManager/Read",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cloud.lazycat.apis.localdevice.BleManager", "Read"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn disconnect(
            &mut self,
            request: impl tonic::IntoRequest<super::DisconnectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisconnectReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.BleManager/Disconnect",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.BleManager",
                        "Disconnect",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_permission_status(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PermissionStatusReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.BleManager/GetPermissionStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.BleManager",
                        "GetPermissionStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_characteristic_notification(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeCharacteristicNotificationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::SubscribeCharacteristicNotificationResponse,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.BleManager/SubscribeCharacteristicNotification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.BleManager",
                        "SubscribeCharacteristicNotification",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ble_manager_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BleManagerServer.
    #[async_trait]
    pub trait BleManager: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the ScanBle method.
        type ScanBleStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::BluetoothDevice, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn scan_ble(
            &self,
            request: tonic::Request<super::ScanBleRequest>,
        ) -> std::result::Result<tonic::Response<Self::ScanBleStream>, tonic::Status>;
        async fn connect_ble(
            &self,
            request: tonic::Request<super::ConnectBleRequest>,
        ) -> std::result::Result<tonic::Response<super::ConnectBleReply>, tonic::Status>;
        async fn write(
            &self,
            request: tonic::Request<super::WriteRequest>,
        ) -> std::result::Result<tonic::Response<super::WriteReply>, tonic::Status>;
        async fn read(
            &self,
            request: tonic::Request<super::ReadRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadReply>, tonic::Status>;
        async fn disconnect(
            &self,
            request: tonic::Request<super::DisconnectRequest>,
        ) -> std::result::Result<tonic::Response<super::DisconnectReply>, tonic::Status>;
        async fn get_permission_status(
            &self,
            request: tonic::Request<super::PermissionStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PermissionStatusReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeCharacteristicNotification method.
        type SubscribeCharacteristicNotificationStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::SubscribeCharacteristicNotificationResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn subscribe_characteristic_notification(
            &self,
            request: tonic::Request<super::SubscribeCharacteristicNotificationRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeCharacteristicNotificationStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BleManagerServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> BleManagerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BleManagerServer<T>
    where
        T: BleManager,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.BleManager/ScanBle" => {
                    #[allow(non_camel_case_types)]
                    struct ScanBleSvc<T: BleManager>(pub Arc<T>);
                    impl<
                        T: BleManager,
                    > tonic::server::ServerStreamingService<super::ScanBleRequest>
                    for ScanBleSvc<T> {
                        type Response = super::BluetoothDevice;
                        type ResponseStream = T::ScanBleStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScanBleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BleManager>::scan_ble(&inner, request).await
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
                        let method = ScanBleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.BleManager/ConnectBle" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectBleSvc<T: BleManager>(pub Arc<T>);
                    impl<
                        T: BleManager,
                    > tonic::server::UnaryService<super::ConnectBleRequest>
                    for ConnectBleSvc<T> {
                        type Response = super::ConnectBleReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectBleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BleManager>::connect_ble(&inner, request).await
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
                        let method = ConnectBleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.BleManager/Write" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSvc<T: BleManager>(pub Arc<T>);
                    impl<T: BleManager> tonic::server::UnaryService<super::WriteRequest>
                    for WriteSvc<T> {
                        type Response = super::WriteReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BleManager>::write(&inner, request).await
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
                        let method = WriteSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.BleManager/Read" => {
                    #[allow(non_camel_case_types)]
                    struct ReadSvc<T: BleManager>(pub Arc<T>);
                    impl<T: BleManager> tonic::server::UnaryService<super::ReadRequest>
                    for ReadSvc<T> {
                        type Response = super::ReadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BleManager>::read(&inner, request).await
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
                        let method = ReadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.BleManager/Disconnect" => {
                    #[allow(non_camel_case_types)]
                    struct DisconnectSvc<T: BleManager>(pub Arc<T>);
                    impl<
                        T: BleManager,
                    > tonic::server::UnaryService<super::DisconnectRequest>
                    for DisconnectSvc<T> {
                        type Response = super::DisconnectReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisconnectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BleManager>::disconnect(&inner, request).await
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
                        let method = DisconnectSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.BleManager/GetPermissionStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetPermissionStatusSvc<T: BleManager>(pub Arc<T>);
                    impl<
                        T: BleManager,
                    > tonic::server::UnaryService<super::PermissionStatusRequest>
                    for GetPermissionStatusSvc<T> {
                        type Response = super::PermissionStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BleManager>::get_permission_status(&inner, request)
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
                        let method = GetPermissionStatusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.BleManager/SubscribeCharacteristicNotification" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeCharacteristicNotificationSvc<T: BleManager>(
                        pub Arc<T>,
                    );
                    impl<
                        T: BleManager,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeCharacteristicNotificationRequest,
                    > for SubscribeCharacteristicNotificationSvc<T> {
                        type Response = super::SubscribeCharacteristicNotificationResponse;
                        type ResponseStream = T::SubscribeCharacteristicNotificationStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeCharacteristicNotificationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BleManager>::subscribe_characteristic_notification(
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
                        let method = SubscribeCharacteristicNotificationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for BleManagerServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.BleManager";
    impl<T> tonic::server::NamedService for BleManagerServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod client_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ClientClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ClientClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> ClientClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ClientClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_client_info(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetClientInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.Client/GetClientInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.Client",
                        "GetClientInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn login_tv(
            &mut self,
            request: impl tonic::IntoRequest<super::TvLoginInfo>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.Client/LoginTV",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cloud.lazycat.apis.localdevice.Client", "LoginTV"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod client_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ClientServer.
    #[async_trait]
    pub trait Client: std::marker::Send + std::marker::Sync + 'static {
        async fn get_client_info(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetClientInfoResponse>,
            tonic::Status,
        >;
        async fn login_tv(
            &self,
            request: tonic::Request<super::TvLoginInfo>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ClientServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ClientServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ClientServer<T>
    where
        T: Client,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.Client/GetClientInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetClientInfoSvc<T: Client>(pub Arc<T>);
                    impl<T: Client> tonic::server::UnaryService<()>
                    for GetClientInfoSvc<T> {
                        type Response = super::GetClientInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Client>::get_client_info(&inner, request).await
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
                        let method = GetClientInfoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.Client/LoginTV" => {
                    #[allow(non_camel_case_types)]
                    struct LoginTVSvc<T: Client>(pub Arc<T>);
                    impl<T: Client> tonic::server::UnaryService<super::TvLoginInfo>
                    for LoginTVSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TvLoginInfo>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Client>::login_tv(&inner, request).await
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
                        let method = LoginTVSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ClientServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.Client";
    impl<T> tonic::server::NamedService for ClientServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod cloud_driver_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CloudDriverClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudDriverClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> CloudDriverClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            CloudDriverClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn enable_auto_mount(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableAutoMountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EnableAutoMountReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.CloudDriver/EnableAutoMount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.CloudDriver",
                        "EnableAutoMount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_enable_auto_mount(
            &mut self,
            request: impl tonic::IntoRequest<super::IsEnableAutoMountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsEnableAutoMountReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.CloudDriver/IsEnableAutoMount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.CloudDriver",
                        "IsEnableAutoMount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_file(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenFileRequest>,
        ) -> std::result::Result<tonic::Response<super::OpenFileReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.CloudDriver/OpenFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.CloudDriver",
                        "OpenFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod cloud_driver_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CloudDriverServer.
    #[async_trait]
    pub trait CloudDriver: std::marker::Send + std::marker::Sync + 'static {
        async fn enable_auto_mount(
            &self,
            request: tonic::Request<super::EnableAutoMountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EnableAutoMountReply>,
            tonic::Status,
        >;
        async fn is_enable_auto_mount(
            &self,
            request: tonic::Request<super::IsEnableAutoMountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsEnableAutoMountReply>,
            tonic::Status,
        >;
        async fn open_file(
            &self,
            request: tonic::Request<super::OpenFileRequest>,
        ) -> std::result::Result<tonic::Response<super::OpenFileReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CloudDriverServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CloudDriverServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CloudDriverServer<T>
    where
        T: CloudDriver,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.CloudDriver/EnableAutoMount" => {
                    #[allow(non_camel_case_types)]
                    struct EnableAutoMountSvc<T: CloudDriver>(pub Arc<T>);
                    impl<
                        T: CloudDriver,
                    > tonic::server::UnaryService<super::EnableAutoMountRequest>
                    for EnableAutoMountSvc<T> {
                        type Response = super::EnableAutoMountReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EnableAutoMountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudDriver>::enable_auto_mount(&inner, request).await
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
                        let method = EnableAutoMountSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.CloudDriver/IsEnableAutoMount" => {
                    #[allow(non_camel_case_types)]
                    struct IsEnableAutoMountSvc<T: CloudDriver>(pub Arc<T>);
                    impl<
                        T: CloudDriver,
                    > tonic::server::UnaryService<super::IsEnableAutoMountRequest>
                    for IsEnableAutoMountSvc<T> {
                        type Response = super::IsEnableAutoMountReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsEnableAutoMountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudDriver>::is_enable_auto_mount(&inner, request)
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
                        let method = IsEnableAutoMountSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.CloudDriver/OpenFile" => {
                    #[allow(non_camel_case_types)]
                    struct OpenFileSvc<T: CloudDriver>(pub Arc<T>);
                    impl<
                        T: CloudDriver,
                    > tonic::server::UnaryService<super::OpenFileRequest>
                    for OpenFileSvc<T> {
                        type Response = super::OpenFileReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudDriver>::open_file(&inner, request).await
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
                        let method = OpenFileSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for CloudDriverServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.CloudDriver";
    impl<T> tonic::server::NamedService for CloudDriverServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod user_config_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UserConfigClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserConfigClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> UserConfigClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            UserConfigClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_user_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserConfigResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.UserConfig/GetUserConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.UserConfig",
                        "GetUserConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_user_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUserConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetUserConfigResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.UserConfig/SetUserConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.UserConfig",
                        "SetUserConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_config_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserConfigServer.
    #[async_trait]
    pub trait UserConfig: std::marker::Send + std::marker::Sync + 'static {
        async fn get_user_config(
            &self,
            request: tonic::Request<super::GetUserConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserConfigResponse>,
            tonic::Status,
        >;
        async fn set_user_config(
            &self,
            request: tonic::Request<super::SetUserConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetUserConfigResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct UserConfigServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> UserConfigServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserConfigServer<T>
    where
        T: UserConfig,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.UserConfig/GetUserConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserConfigSvc<T: UserConfig>(pub Arc<T>);
                    impl<
                        T: UserConfig,
                    > tonic::server::UnaryService<super::GetUserConfigRequest>
                    for GetUserConfigSvc<T> {
                        type Response = super::GetUserConfigResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserConfigRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserConfig>::get_user_config(&inner, request).await
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
                        let method = GetUserConfigSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.UserConfig/SetUserConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetUserConfigSvc<T: UserConfig>(pub Arc<T>);
                    impl<
                        T: UserConfig,
                    > tonic::server::UnaryService<super::SetUserConfigRequest>
                    for SetUserConfigSvc<T> {
                        type Response = super::SetUserConfigResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetUserConfigRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserConfig>::set_user_config(&inner, request).await
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
                        let method = SetUserConfigSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for UserConfigServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.UserConfig";
    impl<T> tonic::server::NamedService for UserConfigServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod contacts_manager_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ContactsManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ContactsManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> ContactsManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ContactsManagerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_contacts(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListContactsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.ContactsManager/ListContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.ContactsManager",
                        "ListContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::AddContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddContactsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.ContactsManager/AddContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.ContactsManager",
                        "AddContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteContactsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.ContactsManager/DeleteContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.ContactsManager",
                        "DeleteContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateContactsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.ContactsManager/UpdateContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.ContactsManager",
                        "UpdateContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_contacts_count(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContactsCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetContactsCountReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.ContactsManager/GetContactsCount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.ContactsManager",
                        "GetContactsCount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_contacts_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContactsStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListContactsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.ContactsManager/ListContactsStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.ContactsManager",
                        "ListContactsStream",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod contacts_manager_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ContactsManagerServer.
    #[async_trait]
    pub trait ContactsManager: std::marker::Send + std::marker::Sync + 'static {
        async fn list_contacts(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListContactsReply>,
            tonic::Status,
        >;
        async fn add_contacts(
            &self,
            request: tonic::Request<super::AddContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddContactsReply>,
            tonic::Status,
        >;
        async fn delete_contacts(
            &self,
            request: tonic::Request<super::DeleteContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteContactsReply>,
            tonic::Status,
        >;
        async fn update_contacts(
            &self,
            request: tonic::Request<super::UpdateContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateContactsReply>,
            tonic::Status,
        >;
        async fn get_contacts_count(
            &self,
            request: tonic::Request<super::GetContactsCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetContactsCountReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListContactsStream method.
        type ListContactsStreamStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ListContactsReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_contacts_stream(
            &self,
            request: tonic::Request<super::ListContactsStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListContactsStreamStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ContactsManagerServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ContactsManagerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ContactsManagerServer<T>
    where
        T: ContactsManager,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.ContactsManager/ListContacts" => {
                    #[allow(non_camel_case_types)]
                    struct ListContactsSvc<T: ContactsManager>(pub Arc<T>);
                    impl<T: ContactsManager> tonic::server::UnaryService<()>
                    for ListContactsSvc<T> {
                        type Response = super::ListContactsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactsManager>::list_contacts(&inner, request).await
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
                        let method = ListContactsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.ContactsManager/AddContacts" => {
                    #[allow(non_camel_case_types)]
                    struct AddContactsSvc<T: ContactsManager>(pub Arc<T>);
                    impl<
                        T: ContactsManager,
                    > tonic::server::UnaryService<super::AddContactsRequest>
                    for AddContactsSvc<T> {
                        type Response = super::AddContactsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddContactsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactsManager>::add_contacts(&inner, request).await
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
                        let method = AddContactsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.ContactsManager/DeleteContacts" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteContactsSvc<T: ContactsManager>(pub Arc<T>);
                    impl<
                        T: ContactsManager,
                    > tonic::server::UnaryService<super::DeleteContactsRequest>
                    for DeleteContactsSvc<T> {
                        type Response = super::DeleteContactsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteContactsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactsManager>::delete_contacts(&inner, request)
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
                        let method = DeleteContactsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.ContactsManager/UpdateContacts" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateContactsSvc<T: ContactsManager>(pub Arc<T>);
                    impl<
                        T: ContactsManager,
                    > tonic::server::UnaryService<super::UpdateContactsRequest>
                    for UpdateContactsSvc<T> {
                        type Response = super::UpdateContactsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateContactsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactsManager>::update_contacts(&inner, request)
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
                        let method = UpdateContactsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.ContactsManager/GetContactsCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetContactsCountSvc<T: ContactsManager>(pub Arc<T>);
                    impl<
                        T: ContactsManager,
                    > tonic::server::UnaryService<super::GetContactsCountRequest>
                    for GetContactsCountSvc<T> {
                        type Response = super::GetContactsCountReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetContactsCountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactsManager>::get_contacts_count(&inner, request)
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
                        let method = GetContactsCountSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.ContactsManager/ListContactsStream" => {
                    #[allow(non_camel_case_types)]
                    struct ListContactsStreamSvc<T: ContactsManager>(pub Arc<T>);
                    impl<
                        T: ContactsManager,
                    > tonic::server::ServerStreamingService<
                        super::ListContactsStreamRequest,
                    > for ListContactsStreamSvc<T> {
                        type Response = super::ListContactsReply;
                        type ResponseStream = T::ListContactsStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListContactsStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactsManager>::list_contacts_stream(
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
                        let method = ListContactsStreamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ContactsManagerServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.ContactsManager";
    impl<T> tonic::server::NamedService for ContactsManagerServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod device_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DeviceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DeviceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> DeviceServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            DeviceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::DeviceInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.DeviceService/Query",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.DeviceService",
                        "Query",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_service_status(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryServiceStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryServiceStatusReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.DeviceService/QueryServiceStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.DeviceService",
                        "QueryServiceStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod device_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DeviceServiceServer.
    #[async_trait]
    pub trait DeviceService: std::marker::Send + std::marker::Sync + 'static {
        async fn query(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::DeviceInfo>, tonic::Status>;
        async fn query_service_status(
            &self,
            request: tonic::Request<super::QueryServiceStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryServiceStatusReply>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DeviceServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> DeviceServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DeviceServiceServer<T>
    where
        T: DeviceService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.DeviceService/Query" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<()>
                    for QuerySvc<T> {
                        type Response = super::DeviceInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::query(&inner, request).await
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
                        let method = QuerySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.DeviceService/QueryServiceStatus" => {
                    #[allow(non_camel_case_types)]
                    struct QueryServiceStatusSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::QueryServiceStatusRequest>
                    for QueryServiceStatusSvc<T> {
                        type Response = super::QueryServiceStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryServiceStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::query_service_status(&inner, request)
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
                        let method = QueryServiceStatusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for DeviceServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.DeviceService";
    impl<T> tonic::server::NamedService for DeviceServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod dialog_manager_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DialogManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DialogManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> DialogManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            DialogManagerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn question(
            &mut self,
            request: impl tonic::IntoRequest<super::QuestionRequest>,
        ) -> std::result::Result<tonic::Response<super::QuestionResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.DialogManager/Question",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.DialogManager",
                        "Question",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn message_box(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageBoxRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.DialogManager/MessageBox",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.DialogManager",
                        "MessageBox",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn password(
            &mut self,
            request: impl tonic::IntoRequest<super::PasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::PasswordResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.DialogManager/Password",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.DialogManager",
                        "Password",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_file_seletor(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenFileSeletorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenFileSeletorResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.DialogManager/OpenFileSeletor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.DialogManager",
                        "OpenFileSeletor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dialog_manager_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DialogManagerServer.
    #[async_trait]
    pub trait DialogManager: std::marker::Send + std::marker::Sync + 'static {
        async fn question(
            &self,
            request: tonic::Request<super::QuestionRequest>,
        ) -> std::result::Result<tonic::Response<super::QuestionResult>, tonic::Status>;
        async fn message_box(
            &self,
            request: tonic::Request<super::MessageBoxRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn password(
            &self,
            request: tonic::Request<super::PasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::PasswordResult>, tonic::Status>;
        async fn open_file_seletor(
            &self,
            request: tonic::Request<super::OpenFileSeletorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenFileSeletorResult>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DialogManagerServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> DialogManagerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DialogManagerServer<T>
    where
        T: DialogManager,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.DialogManager/Question" => {
                    #[allow(non_camel_case_types)]
                    struct QuestionSvc<T: DialogManager>(pub Arc<T>);
                    impl<
                        T: DialogManager,
                    > tonic::server::UnaryService<super::QuestionRequest>
                    for QuestionSvc<T> {
                        type Response = super::QuestionResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuestionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DialogManager>::question(&inner, request).await
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
                        let method = QuestionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.DialogManager/MessageBox" => {
                    #[allow(non_camel_case_types)]
                    struct MessageBoxSvc<T: DialogManager>(pub Arc<T>);
                    impl<
                        T: DialogManager,
                    > tonic::server::UnaryService<super::MessageBoxRequest>
                    for MessageBoxSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessageBoxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DialogManager>::message_box(&inner, request).await
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
                        let method = MessageBoxSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.DialogManager/Password" => {
                    #[allow(non_camel_case_types)]
                    struct PasswordSvc<T: DialogManager>(pub Arc<T>);
                    impl<
                        T: DialogManager,
                    > tonic::server::UnaryService<super::PasswordRequest>
                    for PasswordSvc<T> {
                        type Response = super::PasswordResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DialogManager>::password(&inner, request).await
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
                        let method = PasswordSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.DialogManager/OpenFileSeletor" => {
                    #[allow(non_camel_case_types)]
                    struct OpenFileSeletorSvc<T: DialogManager>(pub Arc<T>);
                    impl<
                        T: DialogManager,
                    > tonic::server::UnaryService<super::OpenFileSeletorRequest>
                    for OpenFileSeletorSvc<T> {
                        type Response = super::OpenFileSeletorResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenFileSeletorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DialogManager>::open_file_seletor(&inner, request)
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
                        let method = OpenFileSeletorSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for DialogManagerServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.DialogManager";
    impl<T> tonic::server::NamedService for DialogManagerServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod local_launch_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LocalLaunchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LocalLaunchServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> LocalLaunchServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            LocalLaunchServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn pin_app(
            &mut self,
            request: impl tonic::IntoRequest<super::PinAppRequest>,
        ) -> std::result::Result<tonic::Response<super::PinAppReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/PinApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.LocalLaunchService",
                        "PinApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn un_pin_app(
            &mut self,
            request: impl tonic::IntoRequest<super::UnPinAppRequest>,
        ) -> std::result::Result<tonic::Response<super::UnPinAppReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/UnPinApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.LocalLaunchService",
                        "UnPinApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_app(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenAppRequest>,
        ) -> std::result::Result<tonic::Response<super::OpenAppReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/OpenApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.LocalLaunchService",
                        "OpenApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_unsafe_app(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenUnsafeAppRequest>,
        ) -> std::result::Result<tonic::Response<super::OpenAppReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/OpenUnsafeApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.LocalLaunchService",
                        "OpenUnsafeApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_app_method(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenAppMethodRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenAppMethodReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/OpenAppMethod",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.LocalLaunchService",
                        "OpenAppMethod",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod local_launch_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LocalLaunchServiceServer.
    #[async_trait]
    pub trait LocalLaunchService: std::marker::Send + std::marker::Sync + 'static {
        async fn pin_app(
            &self,
            request: tonic::Request<super::PinAppRequest>,
        ) -> std::result::Result<tonic::Response<super::PinAppReply>, tonic::Status>;
        async fn un_pin_app(
            &self,
            request: tonic::Request<super::UnPinAppRequest>,
        ) -> std::result::Result<tonic::Response<super::UnPinAppReply>, tonic::Status>;
        async fn open_app(
            &self,
            request: tonic::Request<super::OpenAppRequest>,
        ) -> std::result::Result<tonic::Response<super::OpenAppReply>, tonic::Status>;
        async fn open_unsafe_app(
            &self,
            request: tonic::Request<super::OpenUnsafeAppRequest>,
        ) -> std::result::Result<tonic::Response<super::OpenAppReply>, tonic::Status>;
        async fn open_app_method(
            &self,
            request: tonic::Request<super::OpenAppMethodRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenAppMethodReply>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LocalLaunchServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> LocalLaunchServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LocalLaunchServiceServer<T>
    where
        T: LocalLaunchService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/PinApp" => {
                    #[allow(non_camel_case_types)]
                    struct PinAppSvc<T: LocalLaunchService>(pub Arc<T>);
                    impl<
                        T: LocalLaunchService,
                    > tonic::server::UnaryService<super::PinAppRequest>
                    for PinAppSvc<T> {
                        type Response = super::PinAppReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PinAppRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocalLaunchService>::pin_app(&inner, request).await
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
                        let method = PinAppSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/UnPinApp" => {
                    #[allow(non_camel_case_types)]
                    struct UnPinAppSvc<T: LocalLaunchService>(pub Arc<T>);
                    impl<
                        T: LocalLaunchService,
                    > tonic::server::UnaryService<super::UnPinAppRequest>
                    for UnPinAppSvc<T> {
                        type Response = super::UnPinAppReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnPinAppRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocalLaunchService>::un_pin_app(&inner, request).await
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
                        let method = UnPinAppSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/OpenApp" => {
                    #[allow(non_camel_case_types)]
                    struct OpenAppSvc<T: LocalLaunchService>(pub Arc<T>);
                    impl<
                        T: LocalLaunchService,
                    > tonic::server::UnaryService<super::OpenAppRequest>
                    for OpenAppSvc<T> {
                        type Response = super::OpenAppReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenAppRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocalLaunchService>::open_app(&inner, request).await
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
                        let method = OpenAppSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/OpenUnsafeApp" => {
                    #[allow(non_camel_case_types)]
                    struct OpenUnsafeAppSvc<T: LocalLaunchService>(pub Arc<T>);
                    impl<
                        T: LocalLaunchService,
                    > tonic::server::UnaryService<super::OpenUnsafeAppRequest>
                    for OpenUnsafeAppSvc<T> {
                        type Response = super::OpenAppReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenUnsafeAppRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocalLaunchService>::open_unsafe_app(&inner, request)
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
                        let method = OpenUnsafeAppSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.LocalLaunchService/OpenAppMethod" => {
                    #[allow(non_camel_case_types)]
                    struct OpenAppMethodSvc<T: LocalLaunchService>(pub Arc<T>);
                    impl<
                        T: LocalLaunchService,
                    > tonic::server::UnaryService<super::OpenAppMethodRequest>
                    for OpenAppMethodSvc<T> {
                        type Response = super::OpenAppMethodReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenAppMethodRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LocalLaunchService>::open_app_method(&inner, request)
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
                        let method = OpenAppMethodSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for LocalLaunchServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.LocalLaunchService";
    impl<T> tonic::server::NamedService for LocalLaunchServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod network_manager_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct NetworkManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NetworkManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> NetworkManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            NetworkManagerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::NetworkInformation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.NetworkManager/Query",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.NetworkManager",
                        "Query",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_wifi_networks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWifiNetworksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WifiNetwork>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.NetworkManager/ListWifiNetworks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.NetworkManager",
                        "ListWifiNetworks",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn ws_discovery(
            &mut self,
            request: impl tonic::IntoRequest<super::WsDiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WsDiscoveryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.NetworkManager/WSDiscovery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.NetworkManager",
                        "WSDiscovery",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn native_http_request(
            &mut self,
            request: impl tonic::IntoRequest<super::NativeHttpRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NativeHttpRequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.NetworkManager/NativeHttpRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.NetworkManager",
                        "NativeHttpRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod network_manager_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NetworkManagerServer.
    #[async_trait]
    pub trait NetworkManager: std::marker::Send + std::marker::Sync + 'static {
        async fn query(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::NetworkInformation>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListWifiNetworks method.
        type ListWifiNetworksStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::WifiNetwork, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_wifi_networks(
            &self,
            request: tonic::Request<super::ListWifiNetworksRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListWifiNetworksStream>,
            tonic::Status,
        >;
        async fn ws_discovery(
            &self,
            request: tonic::Request<super::WsDiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WsDiscoveryResponse>,
            tonic::Status,
        >;
        async fn native_http_request(
            &self,
            request: tonic::Request<super::NativeHttpRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NativeHttpRequestResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct NetworkManagerServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> NetworkManagerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NetworkManagerServer<T>
    where
        T: NetworkManager,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.NetworkManager/Query" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySvc<T: NetworkManager>(pub Arc<T>);
                    impl<T: NetworkManager> tonic::server::UnaryService<()>
                    for QuerySvc<T> {
                        type Response = super::NetworkInformation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NetworkManager>::query(&inner, request).await
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
                        let method = QuerySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.NetworkManager/ListWifiNetworks" => {
                    #[allow(non_camel_case_types)]
                    struct ListWifiNetworksSvc<T: NetworkManager>(pub Arc<T>);
                    impl<
                        T: NetworkManager,
                    > tonic::server::ServerStreamingService<
                        super::ListWifiNetworksRequest,
                    > for ListWifiNetworksSvc<T> {
                        type Response = super::WifiNetwork;
                        type ResponseStream = T::ListWifiNetworksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWifiNetworksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NetworkManager>::list_wifi_networks(&inner, request)
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
                        let method = ListWifiNetworksSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.NetworkManager/WSDiscovery" => {
                    #[allow(non_camel_case_types)]
                    struct WSDiscoverySvc<T: NetworkManager>(pub Arc<T>);
                    impl<
                        T: NetworkManager,
                    > tonic::server::UnaryService<super::WsDiscoveryRequest>
                    for WSDiscoverySvc<T> {
                        type Response = super::WsDiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WsDiscoveryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NetworkManager>::ws_discovery(&inner, request).await
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
                        let method = WSDiscoverySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.NetworkManager/NativeHttpRequest" => {
                    #[allow(non_camel_case_types)]
                    struct NativeHttpRequestSvc<T: NetworkManager>(pub Arc<T>);
                    impl<
                        T: NetworkManager,
                    > tonic::server::UnaryService<super::NativeHttpRequestRequest>
                    for NativeHttpRequestSvc<T> {
                        type Response = super::NativeHttpRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NativeHttpRequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NetworkManager>::native_http_request(&inner, request)
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
                        let method = NativeHttpRequestSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for NetworkManagerServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.NetworkManager";
    impl<T> tonic::server::NamedService for NetworkManagerServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod notification_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct NotificationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NotificationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> NotificationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            NotificationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn notify(
            &mut self,
            request: impl tonic::IntoRequest<super::NotifyRequest>,
        ) -> std::result::Result<tonic::Response<super::NotifyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.NotificationService/Notify",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.NotificationService",
                        "Notify",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod notification_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NotificationServiceServer.
    #[async_trait]
    pub trait NotificationService: std::marker::Send + std::marker::Sync + 'static {
        async fn notify(
            &self,
            request: tonic::Request<super::NotifyRequest>,
        ) -> std::result::Result<tonic::Response<super::NotifyResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct NotificationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> NotificationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NotificationServiceServer<T>
    where
        T: NotificationService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.NotificationService/Notify" => {
                    #[allow(non_camel_case_types)]
                    struct NotifySvc<T: NotificationService>(pub Arc<T>);
                    impl<
                        T: NotificationService,
                    > tonic::server::UnaryService<super::NotifyRequest>
                    for NotifySvc<T> {
                        type Response = super::NotifyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NotifyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NotificationService>::notify(&inner, request).await
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
                        let method = NotifySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for NotificationServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.NotificationService";
    impl<T> tonic::server::NamedService for NotificationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod oneway_sync_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OnewaySyncClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OnewaySyncClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> OnewaySyncClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            OnewaySyncClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_sync_dir(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestSyncDir>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseListSyncDir>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.OnewaySync/ListSyncDir",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.OnewaySync",
                        "ListSyncDir",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn new_sync_dir(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestSyncDir>,
        ) -> std::result::Result<tonic::Response<super::SyncDir>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.OnewaySync/NewSyncDir",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.OnewaySync",
                        "NewSyncDir",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn do_sync(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::SyncDir>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.OnewaySync/DoSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.OnewaySync",
                        "DoSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod oneway_sync_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OnewaySyncServer.
    #[async_trait]
    pub trait OnewaySync: std::marker::Send + std::marker::Sync + 'static {
        async fn list_sync_dir(
            &self,
            request: tonic::Request<super::RequestSyncDir>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseListSyncDir>,
            tonic::Status,
        >;
        async fn new_sync_dir(
            &self,
            request: tonic::Request<super::RequestSyncDir>,
        ) -> std::result::Result<tonic::Response<super::SyncDir>, tonic::Status>;
        async fn do_sync(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::SyncDir>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct OnewaySyncServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> OnewaySyncServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OnewaySyncServer<T>
    where
        T: OnewaySync,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.OnewaySync/ListSyncDir" => {
                    #[allow(non_camel_case_types)]
                    struct ListSyncDirSvc<T: OnewaySync>(pub Arc<T>);
                    impl<
                        T: OnewaySync,
                    > tonic::server::UnaryService<super::RequestSyncDir>
                    for ListSyncDirSvc<T> {
                        type Response = super::ResponseListSyncDir;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestSyncDir>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OnewaySync>::list_sync_dir(&inner, request).await
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
                        let method = ListSyncDirSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.OnewaySync/NewSyncDir" => {
                    #[allow(non_camel_case_types)]
                    struct NewSyncDirSvc<T: OnewaySync>(pub Arc<T>);
                    impl<
                        T: OnewaySync,
                    > tonic::server::UnaryService<super::RequestSyncDir>
                    for NewSyncDirSvc<T> {
                        type Response = super::SyncDir;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestSyncDir>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OnewaySync>::new_sync_dir(&inner, request).await
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
                        let method = NewSyncDirSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.OnewaySync/DoSync" => {
                    #[allow(non_camel_case_types)]
                    struct DoSyncSvc<T: OnewaySync>(pub Arc<T>);
                    impl<T: OnewaySync> tonic::server::UnaryService<()>
                    for DoSyncSvc<T> {
                        type Response = super::SyncDir;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OnewaySync>::do_sync(&inner, request).await
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
                        let method = DoSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for OnewaySyncServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.OnewaySync";
    impl<T> tonic::server::NamedService for OnewaySyncServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod permission_manager_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PermissionManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PermissionManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> PermissionManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            PermissionManagerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PermissionReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PermissionManager/GetPermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PermissionManager",
                        "GetPermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PermissionReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PermissionManager/RequestPermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PermissionManager",
                        "RequestPermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_permissions(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListPermissionsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PermissionManager/ListPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PermissionManager",
                        "ListPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_auth_token(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestAuthTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RequestAuthTokenResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PermissionManager/RequestAuthToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PermissionManager",
                        "RequestAuthToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod permission_manager_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PermissionManagerServer.
    #[async_trait]
    pub trait PermissionManager: std::marker::Send + std::marker::Sync + 'static {
        async fn get_permission(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> std::result::Result<tonic::Response<super::PermissionReply>, tonic::Status>;
        async fn request_permission(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> std::result::Result<tonic::Response<super::PermissionReply>, tonic::Status>;
        async fn list_permissions(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListPermissionsReply>,
            tonic::Status,
        >;
        async fn request_auth_token(
            &self,
            request: tonic::Request<super::RequestAuthTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RequestAuthTokenResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PermissionManagerServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> PermissionManagerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PermissionManagerServer<T>
    where
        T: PermissionManager,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.PermissionManager/GetPermission" => {
                    #[allow(non_camel_case_types)]
                    struct GetPermissionSvc<T: PermissionManager>(pub Arc<T>);
                    impl<
                        T: PermissionManager,
                    > tonic::server::UnaryService<super::PermissionRequest>
                    for GetPermissionSvc<T> {
                        type Response = super::PermissionReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PermissionManager>::get_permission(&inner, request)
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
                        let method = GetPermissionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.PermissionManager/RequestPermission" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPermissionSvc<T: PermissionManager>(pub Arc<T>);
                    impl<
                        T: PermissionManager,
                    > tonic::server::UnaryService<super::PermissionRequest>
                    for RequestPermissionSvc<T> {
                        type Response = super::PermissionReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PermissionManager>::request_permission(
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
                        let method = RequestPermissionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.PermissionManager/ListPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct ListPermissionsSvc<T: PermissionManager>(pub Arc<T>);
                    impl<T: PermissionManager> tonic::server::UnaryService<()>
                    for ListPermissionsSvc<T> {
                        type Response = super::ListPermissionsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PermissionManager>::list_permissions(&inner, request)
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
                        let method = ListPermissionsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.PermissionManager/RequestAuthToken" => {
                    #[allow(non_camel_case_types)]
                    struct RequestAuthTokenSvc<T: PermissionManager>(pub Arc<T>);
                    impl<
                        T: PermissionManager,
                    > tonic::server::UnaryService<super::RequestAuthTokenRequest>
                    for RequestAuthTokenSvc<T> {
                        type Response = super::RequestAuthTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestAuthTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PermissionManager>::request_auth_token(
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
                        let method = RequestAuthTokenSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for PermissionManagerServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.PermissionManager";
    impl<T> tonic::server::NamedService for PermissionManagerServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod photo_library_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PhotoLibraryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PhotoLibraryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> PhotoLibraryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            PhotoLibraryClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn make_album(
            &mut self,
            request: impl tonic::IntoRequest<super::MakeAlbumRequest>,
        ) -> std::result::Result<tonic::Response<super::Album>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/MakeAlbum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "MakeAlbum",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_albums(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAlbumsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAlbumsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAlbums",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "ListAlbums",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::PutPhotoRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PutPhotoReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/PutPhoto",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "PutPhoto",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn delete_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePhotoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeletePhotoReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/DeletePhoto",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "DeletePhoto",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_photo_metas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPhotoMetasRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PhotoMeta>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListPhotoMetas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "ListPhotoMetas",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PhotoMeta>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "ListAssets",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn query_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPhotoRequest>,
        ) -> std::result::Result<tonic::Response<super::PhotoMeta>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/QueryPhoto",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "QueryPhoto",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_asset_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListAssetStatsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAssetStats",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "ListAssetStats",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn list_assets_by_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsByIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PhotoMeta>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAssetsByIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "ListAssetsByIds",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn query_asset_url_path(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAssetUrlPathReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/QueryAssetUrlPath",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "QueryAssetUrlPath",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_album_asset_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAlbumAssetStatsRequet>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListAlbumAssetStatsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAlbumAssetStats",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "ListAlbumAssetStats",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn list_album_asset_stats_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAlbumAssetStatsV2Requet>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListAlbumAssetStatsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAlbumAssetStatsV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.PhotoLibrary",
                        "ListAlbumAssetStatsV2",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod photo_library_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PhotoLibraryServer.
    #[async_trait]
    pub trait PhotoLibrary: std::marker::Send + std::marker::Sync + 'static {
        async fn make_album(
            &self,
            request: tonic::Request<super::MakeAlbumRequest>,
        ) -> std::result::Result<tonic::Response<super::Album>, tonic::Status>;
        async fn list_albums(
            &self,
            request: tonic::Request<super::ListAlbumsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListAlbumsReply>, tonic::Status>;
        /// Server streaming response type for the PutPhoto method.
        type PutPhotoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PutPhotoReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn put_photo(
            &self,
            request: tonic::Request<super::PutPhotoRequest>,
        ) -> std::result::Result<tonic::Response<Self::PutPhotoStream>, tonic::Status>;
        async fn delete_photo(
            &self,
            request: tonic::Request<super::DeletePhotoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeletePhotoReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListPhotoMetas method.
        type ListPhotoMetasStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PhotoMeta, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_photo_metas(
            &self,
            request: tonic::Request<super::ListPhotoMetasRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListPhotoMetasStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListAssets method.
        type ListAssetsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PhotoMeta, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_assets(
            &self,
            request: tonic::Request<super::ListAssetsRequest>,
        ) -> std::result::Result<tonic::Response<Self::ListAssetsStream>, tonic::Status>;
        async fn query_photo(
            &self,
            request: tonic::Request<super::QueryPhotoRequest>,
        ) -> std::result::Result<tonic::Response<super::PhotoMeta>, tonic::Status>;
        /// Server streaming response type for the ListAssetStats method.
        type ListAssetStatsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ListAssetStatsReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_asset_stats(
            &self,
            request: tonic::Request<super::ListAssetStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListAssetStatsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListAssetsByIds method.
        type ListAssetsByIdsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PhotoMeta, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_assets_by_ids(
            &self,
            request: tonic::Request<super::ListAssetsByIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListAssetsByIdsStream>,
            tonic::Status,
        >;
        async fn query_asset_url_path(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAssetUrlPathReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListAlbumAssetStats method.
        type ListAlbumAssetStatsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ListAlbumAssetStatsReply,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn list_album_asset_stats(
            &self,
            request: tonic::Request<super::ListAlbumAssetStatsRequet>,
        ) -> std::result::Result<
            tonic::Response<Self::ListAlbumAssetStatsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListAlbumAssetStatsV2 method.
        type ListAlbumAssetStatsV2Stream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ListAlbumAssetStatsReply,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn list_album_asset_stats_v2(
            &self,
            request: tonic::Request<super::ListAlbumAssetStatsV2Requet>,
        ) -> std::result::Result<
            tonic::Response<Self::ListAlbumAssetStatsV2Stream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PhotoLibraryServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> PhotoLibraryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PhotoLibraryServer<T>
    where
        T: PhotoLibrary,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/MakeAlbum" => {
                    #[allow(non_camel_case_types)]
                    struct MakeAlbumSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::UnaryService<super::MakeAlbumRequest>
                    for MakeAlbumSvc<T> {
                        type Response = super::Album;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MakeAlbumRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::make_album(&inner, request).await
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
                        let method = MakeAlbumSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAlbums" => {
                    #[allow(non_camel_case_types)]
                    struct ListAlbumsSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::UnaryService<super::ListAlbumsRequest>
                    for ListAlbumsSvc<T> {
                        type Response = super::ListAlbumsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAlbumsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::list_albums(&inner, request).await
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
                        let method = ListAlbumsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/PutPhoto" => {
                    #[allow(non_camel_case_types)]
                    struct PutPhotoSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::ServerStreamingService<super::PutPhotoRequest>
                    for PutPhotoSvc<T> {
                        type Response = super::PutPhotoReply;
                        type ResponseStream = T::PutPhotoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutPhotoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::put_photo(&inner, request).await
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
                        let method = PutPhotoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/DeletePhoto" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePhotoSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::UnaryService<super::DeletePhotoRequest>
                    for DeletePhotoSvc<T> {
                        type Response = super::DeletePhotoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePhotoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::delete_photo(&inner, request).await
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
                        let method = DeletePhotoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListPhotoMetas" => {
                    #[allow(non_camel_case_types)]
                    struct ListPhotoMetasSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::ServerStreamingService<super::ListPhotoMetasRequest>
                    for ListPhotoMetasSvc<T> {
                        type Response = super::PhotoMeta;
                        type ResponseStream = T::ListPhotoMetasStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPhotoMetasRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::list_photo_metas(&inner, request).await
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
                        let method = ListPhotoMetasSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAssets" => {
                    #[allow(non_camel_case_types)]
                    struct ListAssetsSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::ServerStreamingService<super::ListAssetsRequest>
                    for ListAssetsSvc<T> {
                        type Response = super::PhotoMeta;
                        type ResponseStream = T::ListAssetsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAssetsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::list_assets(&inner, request).await
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
                        let method = ListAssetsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/QueryPhoto" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPhotoSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::UnaryService<super::QueryPhotoRequest>
                    for QueryPhotoSvc<T> {
                        type Response = super::PhotoMeta;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPhotoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::query_photo(&inner, request).await
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
                        let method = QueryPhotoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAssetStats" => {
                    #[allow(non_camel_case_types)]
                    struct ListAssetStatsSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::ServerStreamingService<super::ListAssetStatsRequest>
                    for ListAssetStatsSvc<T> {
                        type Response = super::ListAssetStatsReply;
                        type ResponseStream = T::ListAssetStatsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAssetStatsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::list_asset_stats(&inner, request).await
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
                        let method = ListAssetStatsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAssetsByIds" => {
                    #[allow(non_camel_case_types)]
                    struct ListAssetsByIdsSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::ServerStreamingService<
                        super::ListAssetsByIdsRequest,
                    > for ListAssetsByIdsSvc<T> {
                        type Response = super::PhotoMeta;
                        type ResponseStream = T::ListAssetsByIdsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAssetsByIdsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::list_assets_by_ids(&inner, request)
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
                        let method = ListAssetsByIdsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/QueryAssetUrlPath" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAssetUrlPathSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<T: PhotoLibrary> tonic::server::UnaryService<()>
                    for QueryAssetUrlPathSvc<T> {
                        type Response = super::QueryAssetUrlPathReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::query_asset_url_path(&inner, request)
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
                        let method = QueryAssetUrlPathSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAlbumAssetStats" => {
                    #[allow(non_camel_case_types)]
                    struct ListAlbumAssetStatsSvc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::ServerStreamingService<
                        super::ListAlbumAssetStatsRequet,
                    > for ListAlbumAssetStatsSvc<T> {
                        type Response = super::ListAlbumAssetStatsReply;
                        type ResponseStream = T::ListAlbumAssetStatsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAlbumAssetStatsRequet>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::list_album_asset_stats(&inner, request)
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
                        let method = ListAlbumAssetStatsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.PhotoLibrary/ListAlbumAssetStatsV2" => {
                    #[allow(non_camel_case_types)]
                    struct ListAlbumAssetStatsV2Svc<T: PhotoLibrary>(pub Arc<T>);
                    impl<
                        T: PhotoLibrary,
                    > tonic::server::ServerStreamingService<
                        super::ListAlbumAssetStatsV2Requet,
                    > for ListAlbumAssetStatsV2Svc<T> {
                        type Response = super::ListAlbumAssetStatsReply;
                        type ResponseStream = T::ListAlbumAssetStatsV2Stream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAlbumAssetStatsV2Requet>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PhotoLibrary>::list_album_asset_stats_v2(
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
                        let method = ListAlbumAssetStatsV2Svc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for PhotoLibraryServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.PhotoLibrary";
    impl<T> tonic::server::NamedService for PhotoLibraryServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod remote_control_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct RemoteControlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RemoteControlClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> RemoteControlClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            RemoteControlClient::new(InterceptedService::new(inner, interceptor))
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
        /** 发送键盘输入事件
*/
        pub async fn send_keyboard_event(
            &mut self,
            request: impl tonic::IntoRequest<super::SendKeyboardEventRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendKeyboardEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendKeyboardEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发送触摸板输入事件
 此接口发送的事件都是源事件，比如发送LeftPress，只会发起按下，不会发起松开
 如果是想要单击等操作使用其他接口
*/
        pub async fn send_touchpad_event(
            &mut self,
            request: impl tonic::IntoRequest<super::SendTouchpadEventRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendTouchpadEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发送左键
*/
        pub async fn send_touchpad_click(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendTouchpadClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发送右键
*/
        pub async fn send_touchpad_right_click(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadRightClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendTouchpadRightClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发送双击
*/
        pub async fn send_touchpad_double_click(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadDoubleClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendTouchpadDoubleClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发送触摸板滑动
*/
        pub async fn send_touchpad_move(
            &mut self,
            request: impl tonic::IntoRequest<super::SendTouchpadMoveRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadMove",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendTouchpadMove",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发起鼠标向左移动
*/
        pub async fn send_mouse_move_left(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMouseMoveByDirectionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMoveLeft",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseMoveLeft",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发起鼠标向右移动
*/
        pub async fn send_mouse_move_right(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMouseMoveByDirectionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMoveRight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseMoveRight",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发起鼠标向上移动
*/
        pub async fn send_mouse_move_up(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMouseMoveByDirectionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMoveUp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseMoveUp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发起鼠标向下移动
*/
        pub async fn send_mouse_move(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMouseMoveRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMove",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseMove",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 建立鼠标移动客户端流
*/
        pub async fn mouse_move_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendMouseMoveRequest,
            >,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/MouseMoveStream",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "MouseMoveStream",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        /** 发起鼠标左键单击事件
*/
        pub async fn send_mouse_left_click(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseLeftClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseLeftClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发起鼠标右键单击事件
*/
        pub async fn send_mouse_right_click(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseRightClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseRightClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发起鼠标中键单击事件
*/
        pub async fn send_mouse_middle_click(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMiddleClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseMiddleClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 发起鼠标滚动
*/
        pub async fn send_mouse_wheel(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMouseWheelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseWheel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseWheel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 建立流式鼠标滑动
*/
        pub async fn mouse_wheel_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendMouseWheelRequest,
            >,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/MouseWheelStream",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "MouseWheelStream",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        /** 发起鼠标双击左键事件
*/
        pub async fn send_mouse_double_click(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseDoubleClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SendMouseDoubleClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 设置远程屏幕的宽高(用于计算鼠标，触控板移动的同比例偏移)
*/
        pub async fn set_remote_screen_rect(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRemoteScreenRectRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetRemoteScreenRect",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SetRemoteScreenRect",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 写入剪贴板
*/
        pub async fn write_clipboard(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteClipboardRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/WriteClipboard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "WriteClipboard",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 读取剪贴板
*/
        pub async fn read_clipboard(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::ReadClipboardResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/ReadClipboard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "ReadClipboard",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 粘贴
*/
        pub async fn do_paste(
            &mut self,
            request: impl tonic::IntoRequest<super::PasteContent>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/DoPaste",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "DoPaste",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 浏览器操作
*/
        pub async fn browser_action(
            &mut self,
            request: impl tonic::IntoRequest<super::BrowserActionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BrowserAction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BrowserAction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 音频管理
 sink 输入音频设备 (可用于播放音频的音频设备)
 sink-input 输入音频流 (正在播放音频的程序)
 card 声卡，对应到某个物理设备
 card profile 声音可用的配置(立体声、环绕声)，影响sink和source
 列出正在播放的输出音频流
*/
        pub async fn list_sink_inputs(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListSinkInputsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/ListSinkInputs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "ListSinkInputs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 列出当前输入音频设备
*/
        pub async fn list_sinks(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListSinksRepsonse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/ListSinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "ListSinks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 列出当前可用的声卡
*/
        pub async fn list_cards(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListCardsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/ListCards",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "ListCards",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 设置默认的输出音频设备
*/
        pub async fn set_default_sink(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDefaultSinkRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetDefaultSink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SetDefaultSink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 设置默认的输入音频设备
*/
        pub async fn set_card_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCardProfileRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetCardProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SetCardProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 自动初始化音频输出选择，供智慧屏launcher界面使用
*/
        pub async fn auto_init_audio(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/AutoInitAudio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "AutoInitAudio",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 获取是否被静音
*/
        pub async fn get_mute(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetMuteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetMute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "GetMute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 切换静音状态
*/
        pub async fn toggle_mute(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetMuteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/ToggleMute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "ToggleMute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 获取音量
*/
        pub async fn get_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::SinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSinkVolumeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "GetVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 增加音量
*/
        pub async fn increase_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeVolumeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/IncreaseVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "IncreaseVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 减少音量
*/
        pub async fn decrease_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeVolumeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/DecreaseVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "DecreaseVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 设置默认音频输入源音量
*/
        pub async fn set_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::SetVolumeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SetVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 蓝牙管理
 停止扫描蓝牙设备
*/
        pub async fn ble_stop_scan(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleStopScan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BleStopScan",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 开始扫描蓝牙设备
*/
        pub async fn ble_start_scan(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleStartScan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BleStartScan",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 蓝牙设备扫描状态
*/
        pub async fn ble_scan_status(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::BleScanStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleScanStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BleScanStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 流式获取扫描蓝牙设备结果
*/
        pub async fn ble_scan_devices(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::BleScanDevicesResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleScanDevices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BleScanDevices",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /** 连接蓝牙设备
*/
        pub async fn ble_connect_device(
            &mut self,
            request: impl tonic::IntoRequest<super::BleDeviceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleConnectDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BleConnectDevice",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 断开所有连接
*/
        pub async fn ble_disconnect_device(
            &mut self,
            request: impl tonic::IntoRequest<super::BleDeviceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleDisconnectDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BleDisconnectDevice",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 删除蓝牙设备
*/
        pub async fn ble_remove_device(
            &mut self,
            request: impl tonic::IntoRequest<super::BleDeviceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleRemoveDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BleRemoveDevice",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 刷新蓝牙设备列表
*/
        pub async fn ble_refresh_devices(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleRefreshDevices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "BleRefreshDevices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 切换ScreenLayer
*/
        pub async fn set_screen_layer(
            &mut self,
            request: impl tonic::IntoRequest<super::ScreenLayer>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetScreenLayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "SetScreenLayer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 返回当前的ScreenLayer
*/
        pub async fn get_screen_layer(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::ScreenLayer>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetScreenLayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "GetScreenLayer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 登出
*/
        pub async fn logout(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/Logout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "Logout",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 调试测试
*/
        pub async fn debug_test(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DebugTestReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/DebugTest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "DebugTest",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /** 获取浏览器当前url
*/
        pub async fn get_browser_url(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetBrowserUrlResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetBrowserURL",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "GetBrowserURL",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Ocr点击操作
*/
        pub async fn ocr_action_click(
            &mut self,
            request: impl tonic::IntoRequest<super::OcrActionClickRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OcrActionClickResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/OcrActionClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "OcrActionClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Ocr进行扫描
*/
        pub async fn ocr_do_scan(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/OcrDoScan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "OcrDoScan",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Asr耗时记录
*/
        pub async fn asr_record_time(
            &mut self,
            request: impl tonic::IntoRequest<super::AsrRecordTimeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/AsrRecordTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "AsrRecordTime",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 重置显示画面
*/
        pub async fn reset_display(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/ResetDisplay",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "ResetDisplay",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 选择某个数字导航点击
*/
        pub async fn use_number_navigator(
            &mut self,
            request: impl tonic::IntoRequest<super::UseNumberNavigatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UseNumberNavigatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/UseNumberNavigator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "UseNumberNavigator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 打开某个app
*/
        pub async fn open_app(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenAppResquest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/OpenApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "OpenApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 建立流式HID事件
*/
        pub async fn hid_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::HidRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/HIDStream",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "HIDStream",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        /** 获取文本输入框内容
*/
        pub async fn get_text(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::ReadClipboardResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.RemoteControl",
                        "GetText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod remote_control_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RemoteControlServer.
    #[async_trait]
    pub trait RemoteControl: std::marker::Send + std::marker::Sync + 'static {
        /** 发送键盘输入事件
*/
        async fn send_keyboard_event(
            &self,
            request: tonic::Request<super::SendKeyboardEventRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发送触摸板输入事件
 此接口发送的事件都是源事件，比如发送LeftPress，只会发起按下，不会发起松开
 如果是想要单击等操作使用其他接口
*/
        async fn send_touchpad_event(
            &self,
            request: tonic::Request<super::SendTouchpadEventRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发送左键
*/
        async fn send_touchpad_click(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发送右键
*/
        async fn send_touchpad_right_click(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发送双击
*/
        async fn send_touchpad_double_click(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发送触摸板滑动
*/
        async fn send_touchpad_move(
            &self,
            request: tonic::Request<super::SendTouchpadMoveRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标向左移动
*/
        async fn send_mouse_move_left(
            &self,
            request: tonic::Request<super::SendMouseMoveByDirectionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标向右移动
*/
        async fn send_mouse_move_right(
            &self,
            request: tonic::Request<super::SendMouseMoveByDirectionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标向上移动
*/
        async fn send_mouse_move_up(
            &self,
            request: tonic::Request<super::SendMouseMoveByDirectionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标向下移动
*/
        async fn send_mouse_move(
            &self,
            request: tonic::Request<super::SendMouseMoveRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 建立鼠标移动客户端流
*/
        async fn mouse_move_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendMouseMoveRequest>>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标左键单击事件
*/
        async fn send_mouse_left_click(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标右键单击事件
*/
        async fn send_mouse_right_click(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标中键单击事件
*/
        async fn send_mouse_middle_click(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标滚动
*/
        async fn send_mouse_wheel(
            &self,
            request: tonic::Request<super::SendMouseWheelRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 建立流式鼠标滑动
*/
        async fn mouse_wheel_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendMouseWheelRequest>>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 发起鼠标双击左键事件
*/
        async fn send_mouse_double_click(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 设置远程屏幕的宽高(用于计算鼠标，触控板移动的同比例偏移)
*/
        async fn set_remote_screen_rect(
            &self,
            request: tonic::Request<super::SetRemoteScreenRectRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 写入剪贴板
*/
        async fn write_clipboard(
            &self,
            request: tonic::Request<super::WriteClipboardRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 读取剪贴板
*/
        async fn read_clipboard(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::ReadClipboardResponse>,
            tonic::Status,
        >;
        /** 粘贴
*/
        async fn do_paste(
            &self,
            request: tonic::Request<super::PasteContent>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 浏览器操作
*/
        async fn browser_action(
            &self,
            request: tonic::Request<super::BrowserActionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 音频管理
 sink 输入音频设备 (可用于播放音频的音频设备)
 sink-input 输入音频流 (正在播放音频的程序)
 card 声卡，对应到某个物理设备
 card profile 声音可用的配置(立体声、环绕声)，影响sink和source
 列出正在播放的输出音频流
*/
        async fn list_sink_inputs(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListSinkInputsResponse>,
            tonic::Status,
        >;
        /** 列出当前输入音频设备
*/
        async fn list_sinks(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListSinksRepsonse>,
            tonic::Status,
        >;
        /** 列出当前可用的声卡
*/
        async fn list_cards(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::ListCardsResponse>,
            tonic::Status,
        >;
        /** 设置默认的输出音频设备
*/
        async fn set_default_sink(
            &self,
            request: tonic::Request<super::SetDefaultSinkRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 设置默认的输入音频设备
*/
        async fn set_card_profile(
            &self,
            request: tonic::Request<super::SetCardProfileRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 自动初始化音频输出选择，供智慧屏launcher界面使用
*/
        async fn auto_init_audio(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 获取是否被静音
*/
        async fn get_mute(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::GetMuteResponse>, tonic::Status>;
        /** 切换静音状态
*/
        async fn toggle_mute(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::GetMuteResponse>, tonic::Status>;
        /** 获取音量
*/
        async fn get_volume(
            &self,
            request: tonic::Request<super::SinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSinkVolumeResponse>,
            tonic::Status,
        >;
        /** 增加音量
*/
        async fn increase_volume(
            &self,
            request: tonic::Request<super::ChangeVolumeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 减少音量
*/
        async fn decrease_volume(
            &self,
            request: tonic::Request<super::ChangeVolumeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 设置默认音频输入源音量
*/
        async fn set_volume(
            &self,
            request: tonic::Request<super::SetVolumeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 蓝牙管理
 停止扫描蓝牙设备
*/
        async fn ble_stop_scan(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 开始扫描蓝牙设备
*/
        async fn ble_start_scan(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 蓝牙设备扫描状态
*/
        async fn ble_scan_status(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::BleScanStatusResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the BleScanDevices method.
        type BleScanDevicesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::BleScanDevicesResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        /** 流式获取扫描蓝牙设备结果
*/
        async fn ble_scan_devices(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<Self::BleScanDevicesStream>,
            tonic::Status,
        >;
        /** 连接蓝牙设备
*/
        async fn ble_connect_device(
            &self,
            request: tonic::Request<super::BleDeviceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 断开所有连接
*/
        async fn ble_disconnect_device(
            &self,
            request: tonic::Request<super::BleDeviceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 删除蓝牙设备
*/
        async fn ble_remove_device(
            &self,
            request: tonic::Request<super::BleDeviceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 刷新蓝牙设备列表
*/
        async fn ble_refresh_devices(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 切换ScreenLayer
*/
        async fn set_screen_layer(
            &self,
            request: tonic::Request<super::ScreenLayer>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 返回当前的ScreenLayer
*/
        async fn get_screen_layer(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::ScreenLayer>, tonic::Status>;
        /** 登出
*/
        async fn logout(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Server streaming response type for the DebugTest method.
        type DebugTestStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DebugTestReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        /** 调试测试
*/
        async fn debug_test(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<Self::DebugTestStream>, tonic::Status>;
        /** 获取浏览器当前url
*/
        async fn get_browser_url(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetBrowserUrlResponse>,
            tonic::Status,
        >;
        /** Ocr点击操作
*/
        async fn ocr_action_click(
            &self,
            request: tonic::Request<super::OcrActionClickRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OcrActionClickResponse>,
            tonic::Status,
        >;
        /** Ocr进行扫描
*/
        async fn ocr_do_scan(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** Asr耗时记录
*/
        async fn asr_record_time(
            &self,
            request: tonic::Request<super::AsrRecordTimeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 重置显示画面
*/
        async fn reset_display(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 选择某个数字导航点击
*/
        async fn use_number_navigator(
            &self,
            request: tonic::Request<super::UseNumberNavigatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UseNumberNavigatorResponse>,
            tonic::Status,
        >;
        /** 打开某个app
*/
        async fn open_app(
            &self,
            request: tonic::Request<super::OpenAppResquest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 建立流式HID事件
*/
        async fn hid_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::HidRequest>>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** 获取文本输入框内容
*/
        async fn get_text(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::ReadClipboardResponse>,
            tonic::Status,
        >;
    }
    ///
    #[derive(Debug)]
    pub struct RemoteControlServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RemoteControlServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RemoteControlServer<T>
    where
        T: RemoteControl,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendKeyboardEvent" => {
                    #[allow(non_camel_case_types)]
                    struct SendKeyboardEventSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SendKeyboardEventRequest>
                    for SendKeyboardEventSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendKeyboardEventRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_keyboard_event(&inner, request)
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
                        let method = SendKeyboardEventSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadEvent" => {
                    #[allow(non_camel_case_types)]
                    struct SendTouchpadEventSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SendTouchpadEventRequest>
                    for SendTouchpadEventSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendTouchpadEventRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_touchpad_event(&inner, request)
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
                        let method = SendTouchpadEventSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadClick" => {
                    #[allow(non_camel_case_types)]
                    struct SendTouchpadClickSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for SendTouchpadClickSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_touchpad_click(&inner, request)
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
                        let method = SendTouchpadClickSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadRightClick" => {
                    #[allow(non_camel_case_types)]
                    struct SendTouchpadRightClickSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for SendTouchpadRightClickSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_touchpad_right_click(
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
                        let method = SendTouchpadRightClickSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadDoubleClick" => {
                    #[allow(non_camel_case_types)]
                    struct SendTouchpadDoubleClickSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for SendTouchpadDoubleClickSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_touchpad_double_click(
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
                        let method = SendTouchpadDoubleClickSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendTouchpadMove" => {
                    #[allow(non_camel_case_types)]
                    struct SendTouchpadMoveSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SendTouchpadMoveRequest>
                    for SendTouchpadMoveSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendTouchpadMoveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_touchpad_move(&inner, request)
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
                        let method = SendTouchpadMoveSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMoveLeft" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseMoveLeftSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SendMouseMoveByDirectionRequest>
                    for SendMouseMoveLeftSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SendMouseMoveByDirectionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_move_left(&inner, request)
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
                        let method = SendMouseMoveLeftSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMoveRight" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseMoveRightSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SendMouseMoveByDirectionRequest>
                    for SendMouseMoveRightSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SendMouseMoveByDirectionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_move_right(&inner, request)
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
                        let method = SendMouseMoveRightSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMoveUp" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseMoveUpSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SendMouseMoveByDirectionRequest>
                    for SendMouseMoveUpSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SendMouseMoveByDirectionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_move_up(&inner, request)
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
                        let method = SendMouseMoveUpSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMove" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseMoveSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SendMouseMoveRequest>
                    for SendMouseMoveSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMouseMoveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_move(&inner, request).await
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
                        let method = SendMouseMoveSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/MouseMoveStream" => {
                    #[allow(non_camel_case_types)]
                    struct MouseMoveStreamSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::ClientStreamingService<super::SendMouseMoveRequest>
                    for MouseMoveStreamSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendMouseMoveRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::mouse_move_stream(&inner, request)
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
                        let method = MouseMoveStreamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseLeftClick" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseLeftClickSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for SendMouseLeftClickSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_left_click(&inner, request)
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
                        let method = SendMouseLeftClickSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseRightClick" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseRightClickSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for SendMouseRightClickSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_right_click(
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
                        let method = SendMouseRightClickSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseMiddleClick" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseMiddleClickSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for SendMouseMiddleClickSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_middle_click(
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
                        let method = SendMouseMiddleClickSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseWheel" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseWheelSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SendMouseWheelRequest>
                    for SendMouseWheelSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMouseWheelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_wheel(&inner, request)
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
                        let method = SendMouseWheelSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/MouseWheelStream" => {
                    #[allow(non_camel_case_types)]
                    struct MouseWheelStreamSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::ClientStreamingService<super::SendMouseWheelRequest>
                    for MouseWheelStreamSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendMouseWheelRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::mouse_wheel_stream(&inner, request)
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
                        let method = MouseWheelStreamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.RemoteControl/SendMouseDoubleClick" => {
                    #[allow(non_camel_case_types)]
                    struct SendMouseDoubleClickSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for SendMouseDoubleClickSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::send_mouse_double_click(
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
                        let method = SendMouseDoubleClickSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetRemoteScreenRect" => {
                    #[allow(non_camel_case_types)]
                    struct SetRemoteScreenRectSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SetRemoteScreenRectRequest>
                    for SetRemoteScreenRectSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRemoteScreenRectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::set_remote_screen_rect(
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
                        let method = SetRemoteScreenRectSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/WriteClipboard" => {
                    #[allow(non_camel_case_types)]
                    struct WriteClipboardSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::WriteClipboardRequest>
                    for WriteClipboardSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WriteClipboardRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::write_clipboard(&inner, request).await
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
                        let method = WriteClipboardSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/ReadClipboard" => {
                    #[allow(non_camel_case_types)]
                    struct ReadClipboardSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for ReadClipboardSvc<T> {
                        type Response = super::ReadClipboardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::read_clipboard(&inner, request).await
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
                        let method = ReadClipboardSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/DoPaste" => {
                    #[allow(non_camel_case_types)]
                    struct DoPasteSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::PasteContent>
                    for DoPasteSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PasteContent>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::do_paste(&inner, request).await
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
                        let method = DoPasteSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/BrowserAction" => {
                    #[allow(non_camel_case_types)]
                    struct BrowserActionSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::BrowserActionRequest>
                    for BrowserActionSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BrowserActionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::browser_action(&inner, request).await
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
                        let method = BrowserActionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/ListSinkInputs" => {
                    #[allow(non_camel_case_types)]
                    struct ListSinkInputsSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for ListSinkInputsSvc<T> {
                        type Response = super::ListSinkInputsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::list_sink_inputs(&inner, request)
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
                        let method = ListSinkInputsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/ListSinks" => {
                    #[allow(non_camel_case_types)]
                    struct ListSinksSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for ListSinksSvc<T> {
                        type Response = super::ListSinksRepsonse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::list_sinks(&inner, request).await
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
                        let method = ListSinksSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/ListCards" => {
                    #[allow(non_camel_case_types)]
                    struct ListCardsSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for ListCardsSvc<T> {
                        type Response = super::ListCardsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::list_cards(&inner, request).await
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
                        let method = ListCardsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetDefaultSink" => {
                    #[allow(non_camel_case_types)]
                    struct SetDefaultSinkSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SetDefaultSinkRequest>
                    for SetDefaultSinkSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetDefaultSinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::set_default_sink(&inner, request)
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
                        let method = SetDefaultSinkSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetCardProfile" => {
                    #[allow(non_camel_case_types)]
                    struct SetCardProfileSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SetCardProfileRequest>
                    for SetCardProfileSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetCardProfileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::set_card_profile(&inner, request)
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
                        let method = SetCardProfileSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/AutoInitAudio" => {
                    #[allow(non_camel_case_types)]
                    struct AutoInitAudioSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for AutoInitAudioSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::auto_init_audio(&inner, request).await
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
                        let method = AutoInitAudioSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetMute" => {
                    #[allow(non_camel_case_types)]
                    struct GetMuteSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for GetMuteSvc<T> {
                        type Response = super::GetMuteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::get_mute(&inner, request).await
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
                        let method = GetMuteSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/ToggleMute" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleMuteSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for ToggleMuteSvc<T> {
                        type Response = super::GetMuteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::toggle_mute(&inner, request).await
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
                        let method = ToggleMuteSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetVolume" => {
                    #[allow(non_camel_case_types)]
                    struct GetVolumeSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SinkRequest>
                    for GetVolumeSvc<T> {
                        type Response = super::GetSinkVolumeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SinkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::get_volume(&inner, request).await
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
                        let method = GetVolumeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/IncreaseVolume" => {
                    #[allow(non_camel_case_types)]
                    struct IncreaseVolumeSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::ChangeVolumeRequest>
                    for IncreaseVolumeSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeVolumeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::increase_volume(&inner, request).await
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
                        let method = IncreaseVolumeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/DecreaseVolume" => {
                    #[allow(non_camel_case_types)]
                    struct DecreaseVolumeSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::ChangeVolumeRequest>
                    for DecreaseVolumeSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeVolumeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::decrease_volume(&inner, request).await
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
                        let method = DecreaseVolumeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetVolume" => {
                    #[allow(non_camel_case_types)]
                    struct SetVolumeSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::SetVolumeRequest>
                    for SetVolumeSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetVolumeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::set_volume(&inner, request).await
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
                        let method = SetVolumeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleStopScan" => {
                    #[allow(non_camel_case_types)]
                    struct BleStopScanSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for BleStopScanSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ble_stop_scan(&inner, request).await
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
                        let method = BleStopScanSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleStartScan" => {
                    #[allow(non_camel_case_types)]
                    struct BleStartScanSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for BleStartScanSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ble_start_scan(&inner, request).await
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
                        let method = BleStartScanSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleScanStatus" => {
                    #[allow(non_camel_case_types)]
                    struct BleScanStatusSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for BleScanStatusSvc<T> {
                        type Response = super::BleScanStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ble_scan_status(&inner, request).await
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
                        let method = BleScanStatusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleScanDevices" => {
                    #[allow(non_camel_case_types)]
                    struct BleScanDevicesSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::ServerStreamingService<()>
                    for BleScanDevicesSvc<T> {
                        type Response = super::BleScanDevicesResponse;
                        type ResponseStream = T::BleScanDevicesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ble_scan_devices(&inner, request)
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
                        let method = BleScanDevicesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleConnectDevice" => {
                    #[allow(non_camel_case_types)]
                    struct BleConnectDeviceSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::BleDeviceRequest>
                    for BleConnectDeviceSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BleDeviceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ble_connect_device(&inner, request)
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
                        let method = BleConnectDeviceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleDisconnectDevice" => {
                    #[allow(non_camel_case_types)]
                    struct BleDisconnectDeviceSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::BleDeviceRequest>
                    for BleDisconnectDeviceSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BleDeviceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ble_disconnect_device(&inner, request)
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
                        let method = BleDisconnectDeviceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleRemoveDevice" => {
                    #[allow(non_camel_case_types)]
                    struct BleRemoveDeviceSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::BleDeviceRequest>
                    for BleRemoveDeviceSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BleDeviceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ble_remove_device(&inner, request)
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
                        let method = BleRemoveDeviceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/BleRefreshDevices" => {
                    #[allow(non_camel_case_types)]
                    struct BleRefreshDevicesSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for BleRefreshDevicesSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ble_refresh_devices(&inner, request)
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
                        let method = BleRefreshDevicesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/SetScreenLayer" => {
                    #[allow(non_camel_case_types)]
                    struct SetScreenLayerSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::ScreenLayer>
                    for SetScreenLayerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScreenLayer>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::set_screen_layer(&inner, request)
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
                        let method = SetScreenLayerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetScreenLayer" => {
                    #[allow(non_camel_case_types)]
                    struct GetScreenLayerSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for GetScreenLayerSvc<T> {
                        type Response = super::ScreenLayer;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::get_screen_layer(&inner, request)
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
                        let method = GetScreenLayerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/Logout" => {
                    #[allow(non_camel_case_types)]
                    struct LogoutSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for LogoutSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::logout(&inner, request).await
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
                        let method = LogoutSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/DebugTest" => {
                    #[allow(non_camel_case_types)]
                    struct DebugTestSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::ServerStreamingService<()>
                    for DebugTestSvc<T> {
                        type Response = super::DebugTestReply;
                        type ResponseStream = T::DebugTestStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::debug_test(&inner, request).await
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
                        let method = DebugTestSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetBrowserURL" => {
                    #[allow(non_camel_case_types)]
                    struct GetBrowserURLSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for GetBrowserURLSvc<T> {
                        type Response = super::GetBrowserUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::get_browser_url(&inner, request).await
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
                        let method = GetBrowserURLSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/OcrActionClick" => {
                    #[allow(non_camel_case_types)]
                    struct OcrActionClickSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::OcrActionClickRequest>
                    for OcrActionClickSvc<T> {
                        type Response = super::OcrActionClickResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OcrActionClickRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ocr_action_click(&inner, request)
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
                        let method = OcrActionClickSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/OcrDoScan" => {
                    #[allow(non_camel_case_types)]
                    struct OcrDoScanSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for OcrDoScanSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::ocr_do_scan(&inner, request).await
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
                        let method = OcrDoScanSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/AsrRecordTime" => {
                    #[allow(non_camel_case_types)]
                    struct AsrRecordTimeSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::AsrRecordTimeRequest>
                    for AsrRecordTimeSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AsrRecordTimeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::asr_record_time(&inner, request).await
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
                        let method = AsrRecordTimeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/ResetDisplay" => {
                    #[allow(non_camel_case_types)]
                    struct ResetDisplaySvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for ResetDisplaySvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::reset_display(&inner, request).await
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
                        let method = ResetDisplaySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/UseNumberNavigator" => {
                    #[allow(non_camel_case_types)]
                    struct UseNumberNavigatorSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::UseNumberNavigatorRequest>
                    for UseNumberNavigatorSvc<T> {
                        type Response = super::UseNumberNavigatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UseNumberNavigatorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::use_number_navigator(&inner, request)
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
                        let method = UseNumberNavigatorSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/OpenApp" => {
                    #[allow(non_camel_case_types)]
                    struct OpenAppSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::UnaryService<super::OpenAppResquest>
                    for OpenAppSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenAppResquest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::open_app(&inner, request).await
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
                        let method = OpenAppSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.RemoteControl/HIDStream" => {
                    #[allow(non_camel_case_types)]
                    struct HIDStreamSvc<T: RemoteControl>(pub Arc<T>);
                    impl<
                        T: RemoteControl,
                    > tonic::server::ClientStreamingService<super::HidRequest>
                    for HIDStreamSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::HidRequest>>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::hid_stream(&inner, request).await
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
                        let method = HIDStreamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.RemoteControl/GetText" => {
                    #[allow(non_camel_case_types)]
                    struct GetTextSvc<T: RemoteControl>(pub Arc<T>);
                    impl<T: RemoteControl> tonic::server::UnaryService<()>
                    for GetTextSvc<T> {
                        type Response = super::ReadClipboardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteControl>::get_text(&inner, request).await
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
                        let method = GetTextSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for RemoteControlServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.RemoteControl";
    impl<T> tonic::server::NamedService for RemoteControlServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod rim_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct RimClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RimClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> RimClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            RimClient::new(InterceptedService::new(inner, interceptor))
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
        /** 监听输入框聚焦
*/
        pub async fn listen_input_focus(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::InputContentReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.Rim/ListenInputFocus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.Rim",
                        "ListenInputFocus",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /** 监听输入框失焦
*/
        pub async fn listen_input_blur(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::InputContentReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.Rim/ListenInputBlur",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.Rim",
                        "ListenInputBlur",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /** 获取当前聚焦输入框文本
*/
        pub async fn get_input_text(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::InputContentReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.Rim/GetInputText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cloud.lazycat.apis.localdevice.Rim", "GetInputText"),
                );
            self.inner.unary(req, path, codec).await
        }
        /** 设置当前聚焦输入框文本
*/
        pub async fn set_input_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInputTextRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.Rim/SetInputText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cloud.lazycat.apis.localdevice.Rim", "SetInputText"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_input_focus(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::IsInputFocusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.Rim/IsInputFocus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cloud.lazycat.apis.localdevice.Rim", "IsInputFocus"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_input_event_enter(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cloud.lazycat.apis.localdevice.Rim/SendInputEventEnter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cloud.lazycat.apis.localdevice.Rim",
                        "SendInputEventEnter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod rim_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RimServer.
    #[async_trait]
    pub trait Rim: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the ListenInputFocus method.
        type ListenInputFocusStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::InputContentReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        /** 监听输入框聚焦
*/
        async fn listen_input_focus(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<Self::ListenInputFocusStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListenInputBlur method.
        type ListenInputBlurStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::InputContentReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        /** 监听输入框失焦
*/
        async fn listen_input_blur(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<Self::ListenInputBlurStream>,
            tonic::Status,
        >;
        /** 获取当前聚焦输入框文本
*/
        async fn get_input_text(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::InputContentReply>,
            tonic::Status,
        >;
        /** 设置当前聚焦输入框文本
*/
        async fn set_input_text(
            &self,
            request: tonic::Request<super::SetInputTextRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn is_input_focus(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::IsInputFocusResponse>,
            tonic::Status,
        >;
        async fn send_input_event_enter(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct RimServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RimServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RimServer<T>
    where
        T: Rim,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/cloud.lazycat.apis.localdevice.Rim/ListenInputFocus" => {
                    #[allow(non_camel_case_types)]
                    struct ListenInputFocusSvc<T: Rim>(pub Arc<T>);
                    impl<T: Rim> tonic::server::ServerStreamingService<()>
                    for ListenInputFocusSvc<T> {
                        type Response = super::InputContentReply;
                        type ResponseStream = T::ListenInputFocusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Rim>::listen_input_focus(&inner, request).await
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
                        let method = ListenInputFocusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.Rim/ListenInputBlur" => {
                    #[allow(non_camel_case_types)]
                    struct ListenInputBlurSvc<T: Rim>(pub Arc<T>);
                    impl<T: Rim> tonic::server::ServerStreamingService<()>
                    for ListenInputBlurSvc<T> {
                        type Response = super::InputContentReply;
                        type ResponseStream = T::ListenInputBlurStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Rim>::listen_input_blur(&inner, request).await
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
                        let method = ListenInputBlurSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cloud.lazycat.apis.localdevice.Rim/GetInputText" => {
                    #[allow(non_camel_case_types)]
                    struct GetInputTextSvc<T: Rim>(pub Arc<T>);
                    impl<T: Rim> tonic::server::UnaryService<()> for GetInputTextSvc<T> {
                        type Response = super::InputContentReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Rim>::get_input_text(&inner, request).await
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
                        let method = GetInputTextSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.Rim/SetInputText" => {
                    #[allow(non_camel_case_types)]
                    struct SetInputTextSvc<T: Rim>(pub Arc<T>);
                    impl<T: Rim> tonic::server::UnaryService<super::SetInputTextRequest>
                    for SetInputTextSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetInputTextRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Rim>::set_input_text(&inner, request).await
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
                        let method = SetInputTextSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.Rim/IsInputFocus" => {
                    #[allow(non_camel_case_types)]
                    struct IsInputFocusSvc<T: Rim>(pub Arc<T>);
                    impl<T: Rim> tonic::server::UnaryService<()> for IsInputFocusSvc<T> {
                        type Response = super::IsInputFocusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Rim>::is_input_focus(&inner, request).await
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
                        let method = IsInputFocusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/cloud.lazycat.apis.localdevice.Rim/SendInputEventEnter" => {
                    #[allow(non_camel_case_types)]
                    struct SendInputEventEnterSvc<T: Rim>(pub Arc<T>);
                    impl<T: Rim> tonic::server::UnaryService<()>
                    for SendInputEventEnterSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Rim>::send_input_event_enter(&inner, request).await
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
                        let method = SendInputEventEnterSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for RimServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "cloud.lazycat.apis.localdevice.Rim";
    impl<T> tonic::server::NamedService for RimServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
