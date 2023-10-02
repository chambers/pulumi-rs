/// PluginInfo is meta-information about a plugin that is used by the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginInfo {
    /// the semver for this plugin.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
/// PluginDependency is information about a plugin that a program may depend upon.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginDependency {
    /// the name of the plugin.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// the kind of plugin (e.g., language, etc).
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
    /// the semver for this plugin.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// the URL of a server that can be used to download this plugin, if needed.
    #[prost(string, tag = "4")]
    pub server: ::prost::alloc::string::String,
    /// a map of the checksums for the plugin, will be empty from old language runtimes. The keys should match
    /// the os and architecture names used in pulumi releases, e.g. "darwin-amd64", "windows-arm64".
    #[prost(map = "string, bytes", tag = "5")]
    pub checksums:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
/// PluginAttach is used to attach an already running plugin to the engine.
///
/// Normally the engine starts the plugin process itself and passes the engine address as the first argumnent.
/// But when debugging it can be useful to have an already running provider that the engine instead attaches
/// to, this message is used so the provider can still be passed the engine address to communicate with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginAttach {
    /// the grpc address for the engine
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// AboutResponse returns runtime information about the language.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AboutResponse {
    /// the primary executable for the runtime of this language.
    #[prost(string, tag = "1")]
    pub executable: ::prost::alloc::string::String,
    /// the version of the runtime for this language.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// other information about this language.
    #[prost(map = "string, string", tag = "3")]
    pub metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProgramDependenciesRequest {
    /// the project name.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// the program's working directory.
    #[prost(string, tag = "2")]
    pub pwd: ::prost::alloc::string::String,
    /// the path to the program.
    #[prost(string, tag = "3")]
    pub program: ::prost::alloc::string::String,
    /// if transitive dependencies should be included in the result.
    #[prost(bool, tag = "4")]
    pub transitive_dependencies: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DependencyInfo {
    /// The name of the dependency.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The version of the dependency.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProgramDependenciesResponse {
    /// the dependencies of this program
    #[prost(message, repeated, tag = "1")]
    pub dependencies: ::prost::alloc::vec::Vec<DependencyInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequiredPluginsRequest {
    /// the project name.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// the program's working directory.
    #[prost(string, tag = "2")]
    pub pwd: ::prost::alloc::string::String,
    /// the path to the program.
    #[prost(string, tag = "3")]
    pub program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequiredPluginsResponse {
    /// a list of plugins required by this program.
    #[prost(message, repeated, tag = "1")]
    pub plugins: ::prost::alloc::vec::Vec<PluginDependency>,
}
/// RunRequest asks the interpreter to execute a program.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunRequest {
    /// the project name.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// the name of the stack being deployed into.
    #[prost(string, tag = "2")]
    pub stack: ::prost::alloc::string::String,
    /// the program's working directory.
    #[prost(string, tag = "3")]
    pub pwd: ::prost::alloc::string::String,
    /// the path to the program to execute.
    #[prost(string, tag = "4")]
    pub program: ::prost::alloc::string::String,
    /// any arguments to pass to the program.
    #[prost(string, repeated, tag = "5")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the configuration variables to apply before running.
    #[prost(map = "string, string", tag = "6")]
    pub config:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// true if we're only doing a dryrun (preview).
    #[prost(bool, tag = "7")]
    pub dry_run: bool,
    /// the degree of parallelism for resource operations (<=1 for serial).
    #[prost(int32, tag = "8")]
    pub parallel: i32,
    /// the address for communicating back to the resource monitor.
    #[prost(string, tag = "9")]
    pub monitor_address: ::prost::alloc::string::String,
    /// true if we're only doing a query.
    #[prost(bool, tag = "10")]
    pub query_mode: bool,
    /// the configuration keys that have secret values.
    #[prost(string, repeated, tag = "11")]
    pub config_secret_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the organization of the stack being deployed into.
    #[prost(string, tag = "12")]
    pub organization: ::prost::alloc::string::String,
}
/// RunResponse is the response back from the interpreter/source back to the monitor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunResponse {
    /// An unhandled error if any occurred.
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    /// An error happened.  And it was reported to the user.  Work should stop immediately
    /// with nothing further to print to the user.  This corresponds to a "result.Bail()"
    /// value in the 'go' layer.
    #[prost(bool, tag = "2")]
    pub bail: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallDependenciesRequest {
    /// the program's working directory.
    #[prost(string, tag = "1")]
    pub directory: ::prost::alloc::string::String,
    /// if we are running in a terminal and should use ANSI codes
    #[prost(bool, tag = "2")]
    pub is_terminal: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallDependenciesResponse {
    /// a line of stdout text.
    #[prost(bytes = "vec", tag = "1")]
    pub stdout: ::prost::alloc::vec::Vec<u8>,
    /// a line of stderr text.
    #[prost(bytes = "vec", tag = "2")]
    pub stderr: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPluginRequest {
    /// the program's working directory.
    #[prost(string, tag = "1")]
    pub pwd: ::prost::alloc::string::String,
    /// the path to the program to execute.
    #[prost(string, tag = "2")]
    pub program: ::prost::alloc::string::String,
    /// any arguments to pass to the program.
    #[prost(string, repeated, tag = "3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// any environment variables to set as part of the program.
    #[prost(string, repeated, tag = "4")]
    pub env: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPluginResponse {
    #[prost(oneof = "run_plugin_response::Output", tags = "1, 2, 3")]
    pub output: ::core::option::Option<run_plugin_response::Output>,
}
/// Nested message and enum types in `RunPluginResponse`.
pub mod run_plugin_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        /// a line of stdout text.
        #[prost(bytes, tag = "1")]
        Stdout(::prost::alloc::vec::Vec<u8>),
        /// a line of stderr text.
        #[prost(bytes, tag = "2")]
        Stderr(::prost::alloc::vec::Vec<u8>),
        /// the exit code of the provider.
        #[prost(int32, tag = "3")]
        Exitcode(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProgramRequest {
    /// the PCL source of the project.
    #[prost(map = "string, string", tag = "1")]
    pub source:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The target of a codegen.LoaderServer to use for loading schemas.
    #[prost(string, tag = "2")]
    pub loader_target: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProgramResponse {
    /// any diagnostics from code generation.
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<super::pulumi_codegen::Diagnostic>,
    /// the generated program source code.
    #[prost(map = "string, bytes", tag = "2")]
    pub source:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProjectRequest {
    /// the directory to generate the project from.
    #[prost(string, tag = "1")]
    pub source_directory: ::prost::alloc::string::String,
    /// the directory to generate the project in.
    #[prost(string, tag = "2")]
    pub target_directory: ::prost::alloc::string::String,
    /// the JSON-encoded pulumi project file.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
    /// if PCL binding should be strict or not.
    #[prost(bool, tag = "4")]
    pub strict: bool,
    /// The target of a codegen.LoaderServer to use for loading schemas.
    #[prost(string, tag = "5")]
    pub loader_target: ::prost::alloc::string::String,
    /// local dependencies to use instead of using the package system. This is a map of package name to a local
    /// path of a language specific artifact to use for the SDK for that package.
    #[prost(map = "string, string", tag = "6")]
    pub local_dependencies:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProjectResponse {
    /// any diagnostics from code generation.
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<super::pulumi_codegen::Diagnostic>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratePackageRequest {
    /// the directory to generate the package in.
    #[prost(string, tag = "1")]
    pub directory: ::prost::alloc::string::String,
    /// the JSON-encoded schema.
    #[prost(string, tag = "2")]
    pub schema: ::prost::alloc::string::String,
    /// extra files to copy to the package output.
    #[prost(map = "string, bytes", tag = "3")]
    pub extra_files:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    /// The target of a codegen.LoaderServer to use for loading schemas.
    #[prost(string, tag = "4")]
    pub loader_target: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratePackageResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackRequest {
    /// the directory of a package to pack.
    #[prost(string, tag = "1")]
    pub package_directory: ::prost::alloc::string::String,
    /// the version to tag the artifact with.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// the directory to write the packed artifact to.
    #[prost(string, tag = "3")]
    pub destination_directory: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackResponse {
    /// the full path of the packed artifact.
    #[prost(string, tag = "1")]
    pub artifact_path: ::prost::alloc::string::String,
}
/// Generated server implementations.
pub mod language_runtime_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LanguageRuntimeServer.
    #[async_trait]
    pub trait LanguageRuntime: Send + Sync + 'static {
        /// GetRequiredPlugins computes the complete set of anticipated plugins required by a program.
        async fn get_required_plugins(
            &self,
            request: tonic::Request<super::GetRequiredPluginsRequest>,
        ) -> std::result::Result<tonic::Response<super::GetRequiredPluginsResponse>, tonic::Status>;
        /// Run executes a program and returns its result.
        async fn run(
            &self,
            request: tonic::Request<super::RunRequest>,
        ) -> std::result::Result<tonic::Response<super::RunResponse>, tonic::Status>;
        /// GetPluginInfo returns generic information about this plugin, like its version.
        async fn get_plugin_info(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::PluginInfo>, tonic::Status>;
        /// Server streaming response type for the InstallDependencies method.
        type InstallDependenciesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::InstallDependenciesResponse, tonic::Status>,
            > + Send
            + 'static;
        /// InstallDependencies will install dependencies for the project, e.g. by running `npm install` for nodejs projects.
        async fn install_dependencies(
            &self,
            request: tonic::Request<super::InstallDependenciesRequest>,
        ) -> std::result::Result<tonic::Response<Self::InstallDependenciesStream>, tonic::Status>;
        /// About returns information about the runtime for this language.
        async fn about(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::AboutResponse>, tonic::Status>;
        /// GetProgramDependencies returns the set of dependencies required by the program.
        async fn get_program_dependencies(
            &self,
            request: tonic::Request<super::GetProgramDependenciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProgramDependenciesResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the RunPlugin method.
        type RunPluginStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::RunPluginResponse, tonic::Status>,
            > + Send
            + 'static;
        /// RunPlugin executes a plugin program and returns its result asynchronously.
        async fn run_plugin(
            &self,
            request: tonic::Request<super::RunPluginRequest>,
        ) -> std::result::Result<tonic::Response<Self::RunPluginStream>, tonic::Status>;
        /// GenerateProgram generates a given PCL program into a program for this language.
        async fn generate_program(
            &self,
            request: tonic::Request<super::GenerateProgramRequest>,
        ) -> std::result::Result<tonic::Response<super::GenerateProgramResponse>, tonic::Status>;
        /// GenerateProject generates a given PCL program into a project for this language.
        async fn generate_project(
            &self,
            request: tonic::Request<super::GenerateProjectRequest>,
        ) -> std::result::Result<tonic::Response<super::GenerateProjectResponse>, tonic::Status>;
        /// GeneratePackage generates a given pulumi package into a package for this language.
        async fn generate_package(
            &self,
            request: tonic::Request<super::GeneratePackageRequest>,
        ) -> std::result::Result<tonic::Response<super::GeneratePackageResponse>, tonic::Status>;
        /// Pack packs a package into a language specific artifact.
        async fn pack(
            &self,
            request: tonic::Request<super::PackRequest>,
        ) -> std::result::Result<tonic::Response<super::PackResponse>, tonic::Status>;
    }
    /// LanguageRuntime is the interface that the planning monitor uses to drive execution of an interpreter responsible
    /// for confguring and creating resource objects.
    #[derive(Debug)]
    pub struct LanguageRuntimeServer<T: LanguageRuntime> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LanguageRuntime> LanguageRuntimeServer<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LanguageRuntimeServer<T>
    where
        T: LanguageRuntime,
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
                "/pulumirpc.LanguageRuntime/GetRequiredPlugins" => {
                    #[allow(non_camel_case_types)]
                    struct GetRequiredPluginsSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime>
                        tonic::server::UnaryService<super::GetRequiredPluginsRequest>
                        for GetRequiredPluginsSvc<T>
                    {
                        type Response = super::GetRequiredPluginsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequiredPluginsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LanguageRuntime>::get_required_plugins(&inner, request).await
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
                        let method = GetRequiredPluginsSvc(inner);
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
                "/pulumirpc.LanguageRuntime/Run" => {
                    #[allow(non_camel_case_types)]
                    struct RunSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime> tonic::server::UnaryService<super::RunRequest> for RunSvc<T> {
                        type Response = super::RunResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RunRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as LanguageRuntime>::run(&inner, request).await };
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
                        let method = RunSvc(inner);
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
                "/pulumirpc.LanguageRuntime/GetPluginInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetPluginInfoSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime> tonic::server::UnaryService<()> for GetPluginInfoSvc<T> {
                        type Response = super::PluginInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LanguageRuntime>::get_plugin_info(&inner, request).await
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
                        let method = GetPluginInfoSvc(inner);
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
                "/pulumirpc.LanguageRuntime/InstallDependencies" => {
                    #[allow(non_camel_case_types)]
                    struct InstallDependenciesSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime>
                        tonic::server::ServerStreamingService<super::InstallDependenciesRequest>
                        for InstallDependenciesSvc<T>
                    {
                        type Response = super::InstallDependenciesResponse;
                        type ResponseStream = T::InstallDependenciesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InstallDependenciesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LanguageRuntime>::install_dependencies(&inner, request).await
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
                        let method = InstallDependenciesSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pulumirpc.LanguageRuntime/About" => {
                    #[allow(non_camel_case_types)]
                    struct AboutSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime> tonic::server::UnaryService<()> for AboutSvc<T> {
                        type Response = super::AboutResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as LanguageRuntime>::about(&inner, request).await };
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
                        let method = AboutSvc(inner);
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
                "/pulumirpc.LanguageRuntime/GetProgramDependencies" => {
                    #[allow(non_camel_case_types)]
                    struct GetProgramDependenciesSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime>
                        tonic::server::UnaryService<super::GetProgramDependenciesRequest>
                        for GetProgramDependenciesSvc<T>
                    {
                        type Response = super::GetProgramDependenciesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProgramDependenciesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LanguageRuntime>::get_program_dependencies(&inner, request)
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
                        let method = GetProgramDependenciesSvc(inner);
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
                "/pulumirpc.LanguageRuntime/RunPlugin" => {
                    #[allow(non_camel_case_types)]
                    struct RunPluginSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime>
                        tonic::server::ServerStreamingService<super::RunPluginRequest>
                        for RunPluginSvc<T>
                    {
                        type Response = super::RunPluginResponse;
                        type ResponseStream = T::RunPluginStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RunPluginRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LanguageRuntime>::run_plugin(&inner, request).await
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
                        let method = RunPluginSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pulumirpc.LanguageRuntime/GenerateProgram" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateProgramSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime>
                        tonic::server::UnaryService<super::GenerateProgramRequest>
                        for GenerateProgramSvc<T>
                    {
                        type Response = super::GenerateProgramResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateProgramRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LanguageRuntime>::generate_program(&inner, request).await
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
                        let method = GenerateProgramSvc(inner);
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
                "/pulumirpc.LanguageRuntime/GenerateProject" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateProjectSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime>
                        tonic::server::UnaryService<super::GenerateProjectRequest>
                        for GenerateProjectSvc<T>
                    {
                        type Response = super::GenerateProjectResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateProjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LanguageRuntime>::generate_project(&inner, request).await
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
                        let method = GenerateProjectSvc(inner);
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
                "/pulumirpc.LanguageRuntime/GeneratePackage" => {
                    #[allow(non_camel_case_types)]
                    struct GeneratePackageSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime>
                        tonic::server::UnaryService<super::GeneratePackageRequest>
                        for GeneratePackageSvc<T>
                    {
                        type Response = super::GeneratePackageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GeneratePackageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LanguageRuntime>::generate_package(&inner, request).await
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
                        let method = GeneratePackageSvc(inner);
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
                "/pulumirpc.LanguageRuntime/Pack" => {
                    #[allow(non_camel_case_types)]
                    struct PackSvc<T: LanguageRuntime>(pub Arc<T>);
                    impl<T: LanguageRuntime> tonic::server::UnaryService<super::PackRequest> for PackSvc<T> {
                        type Response = super::PackResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PackRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as LanguageRuntime>::pack(&inner, request).await };
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
                        let method = PackSvc(inner);
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
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: LanguageRuntime> Clone for LanguageRuntimeServer<T> {
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
    impl<T: LanguageRuntime> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LanguageRuntime> tonic::server::NamedService for LanguageRuntimeServer<T> {
        const NAME: &'static str = "pulumirpc.LanguageRuntime";
    }
}
