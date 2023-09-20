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
    pub checksums: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
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
/// A SourcePosition represents a position in a source file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourcePosition {
    /// The URI of the file. Currently only the file scheme with an absolute path is supported.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// The line in the file
    #[prost(int32, tag = "2")]
    pub line: i32,
    /// The column in the line
    #[prost(int32, tag = "3")]
    pub column: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    /// the schema version.
    #[prost(int32, tag = "1")]
    pub version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaResponse {
    /// the JSON-encoded schema.
    #[prost(string, tag = "1")]
    pub schema: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureRequest {
    /// a map of configuration keys to values.
    #[prost(map = "string, string", tag = "1")]
    pub variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// the input properties for the provider. Only filled in for newer providers.
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<::prost_types::Struct>,
    /// when true, operations should return secrets as strongly typed.
    #[prost(bool, tag = "3")]
    pub accept_secrets: bool,
    /// when true, operations should return resources as strongly typed values to the provider.
    #[prost(bool, tag = "4")]
    pub accept_resources: bool,
    /// when true, diff and update will be called with the old outputs and the old inputs.
    #[prost(bool, tag = "5")]
    pub sends_old_inputs: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureResponse {
    /// when true, the engine should pass secrets as strongly typed values to the provider.
    #[prost(bool, tag = "1")]
    pub accept_secrets: bool,
    /// when true, the engine should invoke create and update with preview=true during previews.
    #[prost(bool, tag = "2")]
    pub supports_preview: bool,
    /// when true, the engine should pass resources as strongly typed values to the provider.
    #[prost(bool, tag = "3")]
    pub accept_resources: bool,
    /// when true, the engine should pass output values to the provider.
    #[prost(bool, tag = "4")]
    pub accept_outputs: bool,
}
/// ConfigureErrorMissingKeys is sent as a Detail on an error returned from `ResourceProvider.Configure`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureErrorMissingKeys {
    /// a list of required configuration keys that were not supplied.
    #[prost(message, repeated, tag = "1")]
    pub missing_keys: ::prost::alloc::vec::Vec<configure_error_missing_keys::MissingKey>,
}
/// Nested message and enum types in `ConfigureErrorMissingKeys`.
pub mod configure_error_missing_keys {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MissingKey {
        /// the Pulumi name (not the provider name!) of the missing config key.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// a description of the missing config key, as reported by the provider.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokeRequest {
    /// the function token to invoke.
    #[prost(string, tag = "1")]
    pub tok: ::prost::alloc::string::String,
    /// the arguments for the function invocation.
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvokeResponse {
    /// the returned values, if invoke was successful.
    #[prost(message, optional, tag = "1")]
    pub r#return: ::core::option::Option<::prost_types::Struct>,
    /// the failures if any arguments didn't pass verification.
    #[prost(message, repeated, tag = "2")]
    pub failures: ::prost::alloc::vec::Vec<CheckFailure>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallRequest {
    /// the function token to invoke.
    #[prost(string, tag = "1")]
    pub tok: ::prost::alloc::string::String,
    /// the arguments for the function invocation.
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<::prost_types::Struct>,
    /// a map from argument keys to the dependencies of the argument.
    #[prost(map = "string, message", tag = "3")]
    pub arg_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        call_request::ArgumentDependencies,
    >,
    /// an optional reference to the provider to use for this invoke.
    #[prost(string, tag = "4")]
    pub provider: ::prost::alloc::string::String,
    /// the version of the provider to use when servicing this request.
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// the pluginDownloadURL of the provider to use when servicing this request.
    #[prost(string, tag = "13")]
    pub plugin_download_url: ::prost::alloc::string::String,
    /// a map of checksums of the provider to use when servicing this request.
    #[prost(map = "string, bytes", tag = "16")]
    pub plugin_checksums: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    /// the project name.
    #[prost(string, tag = "6")]
    pub project: ::prost::alloc::string::String,
    /// the name of the stack being deployed into.
    #[prost(string, tag = "7")]
    pub stack: ::prost::alloc::string::String,
    /// the configuration variables to apply before running.
    #[prost(map = "string, string", tag = "8")]
    pub config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// the configuration keys that have secret values.
    #[prost(string, repeated, tag = "9")]
    pub config_secret_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// true if we're only doing a dryrun (preview).
    #[prost(bool, tag = "10")]
    pub dry_run: bool,
    /// the degree of parallelism for resource operations (<=1 for serial).
    #[prost(int32, tag = "11")]
    pub parallel: i32,
    /// the address for communicating back to the resource monitor.
    #[prost(string, tag = "12")]
    pub monitor_endpoint: ::prost::alloc::string::String,
    /// the organization of the stack being deployed into.
    #[prost(string, tag = "14")]
    pub organization: ::prost::alloc::string::String,
    /// the optional source position of the user code that initiated the call.
    #[prost(message, optional, tag = "15")]
    pub source_position: ::core::option::Option<SourcePosition>,
}
/// Nested message and enum types in `CallRequest`.
pub mod call_request {
    /// ArgumentDependencies describes the resources that a particular argument depends on.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArgumentDependencies {
        /// A list of URNs this argument depends on.
        #[prost(string, repeated, tag = "1")]
        pub urns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallResponse {
    /// the returned values, if call was successful.
    #[prost(message, optional, tag = "1")]
    pub r#return: ::core::option::Option<::prost_types::Struct>,
    /// a map from return value keys to the dependencies of the return value.
    #[prost(map = "string, message", tag = "2")]
    pub return_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        call_response::ReturnDependencies,
    >,
    /// the failures if any arguments didn't pass verification.
    #[prost(message, repeated, tag = "3")]
    pub failures: ::prost::alloc::vec::Vec<CheckFailure>,
}
/// Nested message and enum types in `CallResponse`.
pub mod call_response {
    /// ReturnDependencies describes the resources that a particular return value depends on.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReturnDependencies {
        /// A list of URNs this return value depends on.
        #[prost(string, repeated, tag = "1")]
        pub urns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRequest {
    /// the Pulumi URN for this resource.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// the old Pulumi inputs for this resource, if any.
    #[prost(message, optional, tag = "2")]
    pub olds: ::core::option::Option<::prost_types::Struct>,
    /// the new Pulumi inputs for this resource.
    #[prost(message, optional, tag = "3")]
    pub news: ::core::option::Option<::prost_types::Struct>,
    /// a deterministically random hash, primarily intended for global unique naming.
    #[prost(bytes = "vec", tag = "5")]
    pub random_seed: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
    /// the provider inputs for this resource.
    #[prost(message, optional, tag = "1")]
    pub inputs: ::core::option::Option<::prost_types::Struct>,
    /// any validation failures that occurred.
    #[prost(message, repeated, tag = "2")]
    pub failures: ::prost::alloc::vec::Vec<CheckFailure>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckFailure {
    /// the property that failed validation.
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// the reason that the property failed validation.
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffRequest {
    /// the ID of the resource to diff.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// the Pulumi URN for this resource.
    #[prost(string, tag = "2")]
    pub urn: ::prost::alloc::string::String,
    /// the old output values of resource to diff.
    #[prost(message, optional, tag = "3")]
    pub olds: ::core::option::Option<::prost_types::Struct>,
    /// the new input values of resource to diff.
    #[prost(message, optional, tag = "4")]
    pub news: ::core::option::Option<::prost_types::Struct>,
    /// a set of property paths that should be treated as unchanged.
    #[prost(string, repeated, tag = "5")]
    pub ignore_changes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the old input values of the resource to diff.
    #[prost(message, optional, tag = "6")]
    pub old_inputs: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyDiff {
    /// The kind of diff asdsociated with this property.
    #[prost(enumeration = "property_diff::Kind", tag = "1")]
    pub kind: i32,
    /// The difference is between old and new inputs, not old and new state.
    #[prost(bool, tag = "2")]
    pub input_diff: bool,
}
/// Nested message and enum types in `PropertyDiff`.
pub mod property_diff {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Kind {
        /// this property was added
        Add = 0,
        /// this property was added, and this change requires a replace
        AddReplace = 1,
        /// this property was removed
        Delete = 2,
        /// this property was removed, and this change requires a replace
        DeleteReplace = 3,
        /// this property's value was changed
        Update = 4,
        /// this property's value was changed, and this change requires a replace
        UpdateReplace = 5,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Add => "ADD",
                Kind::AddReplace => "ADD_REPLACE",
                Kind::Delete => "DELETE",
                Kind::DeleteReplace => "DELETE_REPLACE",
                Kind::Update => "UPDATE",
                Kind::UpdateReplace => "UPDATE_REPLACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADD" => Some(Self::Add),
                "ADD_REPLACE" => Some(Self::AddReplace),
                "DELETE" => Some(Self::Delete),
                "DELETE_REPLACE" => Some(Self::DeleteReplace),
                "UPDATE" => Some(Self::Update),
                "UPDATE_REPLACE" => Some(Self::UpdateReplace),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffResponse {
    /// if this update requires a replacement, the set of properties triggering it.
    #[prost(string, repeated, tag = "1")]
    pub replaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// an optional list of properties that will not ever change.
    #[prost(string, repeated, tag = "2")]
    pub stables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// if true, this resource must be deleted before replacing it.
    #[prost(bool, tag = "3")]
    pub delete_before_replace: bool,
    /// if true, this diff represents an actual difference and thus requires an update.
    #[prost(enumeration = "diff_response::DiffChanges", tag = "4")]
    pub changes: i32,
    /// a list of the properties that changed.
    #[prost(string, repeated, tag = "5")]
    pub diffs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// detailedDiff is an optional field that contains map from each changed property to the type of the change.
    ///
    /// The keys of this map are property paths. These paths are essentially Javascript property access expressions
    /// in which all elements are literals, and obey the following EBNF-ish grammar:
    ///
    ///    propertyName := \[a-zA-Z_$\] { \[a-zA-Z0-9_$\] }
    ///    quotedPropertyName := '"' ( '\' '"' | \[^"\] ) { ( '\' '"' | \[^"\] ) } '"'
    ///    arrayIndex := { \[0-9\] }
    ///
    ///    propertyIndex := '\[' ( quotedPropertyName | arrayIndex ) '\]'
    ///    rootProperty := ( propertyName | propertyIndex )
    ///    propertyAccessor := ( ( '.' propertyName ) |  propertyIndex )
    ///    path := rootProperty { propertyAccessor }
    ///
    /// Examples of valid keys:
    /// - root
    /// - root.nested
    /// - root\["nested"\]
    /// - root.double.nest
    /// - root\["double"\].nest
    /// - root["double"]["nest"]
    /// - root.array\[0\]
    /// - root.array\[100\]
    /// - root.array\[0\].nested
    /// - root.array[0][1].nested
    /// - root.nested.array\[0\].double\[1\]
    /// - root\["key with \"escaped\" quotes"\]
    /// - root\["key with a ."\]
    /// - \["root key with \"escaped\" quotes"\].nested
    /// - ["root key with a ."][100]
    ///
    /// a detailed diff appropriate for display.
    #[prost(map = "string, message", tag = "6")]
    pub detailed_diff: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        PropertyDiff,
    >,
    /// true if this response contains a detailed diff.
    #[prost(bool, tag = "7")]
    pub has_detailed_diff: bool,
}
/// Nested message and enum types in `DiffResponse`.
pub mod diff_response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DiffChanges {
        /// unknown whether there are changes or not (legacy behavior).
        DiffUnknown = 0,
        /// the diff was performed, and no changes were detected that require an update.
        DiffNone = 1,
        /// the diff was performed, and changes were detected that require an update or replacement.
        DiffSome = 2,
    }
    impl DiffChanges {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DiffChanges::DiffUnknown => "DIFF_UNKNOWN",
                DiffChanges::DiffNone => "DIFF_NONE",
                DiffChanges::DiffSome => "DIFF_SOME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIFF_UNKNOWN" => Some(Self::DiffUnknown),
                "DIFF_NONE" => Some(Self::DiffNone),
                "DIFF_SOME" => Some(Self::DiffSome),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    /// the Pulumi URN for this resource.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// the provider inputs to set during creation.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// the create request timeout represented in seconds.
    #[prost(double, tag = "3")]
    pub timeout: f64,
    /// true if this is a preview and the provider should not actually create the resource.
    #[prost(bool, tag = "4")]
    pub preview: bool,
}
/// NOTE: The partial-update-error equivalent of this message is `ErrorResourceInitFailed`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
    /// the ID of the created resource.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// any properties that were computed during creation.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    /// the ID of the resource to read.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// the Pulumi URN for this resource.
    #[prost(string, tag = "2")]
    pub urn: ::prost::alloc::string::String,
    /// the current state (sufficiently complete to identify the resource).
    #[prost(message, optional, tag = "3")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// the current inputs, if any (only populated during refresh).
    #[prost(message, optional, tag = "4")]
    pub inputs: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    /// the ID of the resource read back (or empty if missing).
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// the state of the resource read from the live environment.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// the inputs for this resource that would be returned from Check.
    #[prost(message, optional, tag = "3")]
    pub inputs: ::core::option::Option<::prost_types::Struct>,
}
/// NOTE: The partial-update-error equivalent of this message is `ErrorResourceInitFailed`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    /// the ID of the resource to update.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// the Pulumi URN for this resource.
    #[prost(string, tag = "2")]
    pub urn: ::prost::alloc::string::String,
    /// the old values of provider inputs for the resource to update.
    #[prost(message, optional, tag = "3")]
    pub olds: ::core::option::Option<::prost_types::Struct>,
    /// the new values of provider inputs for the resource to update.
    #[prost(message, optional, tag = "4")]
    pub news: ::core::option::Option<::prost_types::Struct>,
    /// the update request timeout represented in seconds.
    #[prost(double, tag = "5")]
    pub timeout: f64,
    /// a set of property paths that should be treated as unchanged.
    #[prost(string, repeated, tag = "6")]
    pub ignore_changes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// true if this is a preview and the provider should not actually create the resource.
    #[prost(bool, tag = "7")]
    pub preview: bool,
    /// the old input values of the resource to diff.
    #[prost(message, optional, tag = "8")]
    pub old_inputs: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    /// any properties that were computed during updating.
    #[prost(message, optional, tag = "1")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    /// the ID of the resource to delete.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// the Pulumi URN for this resource.
    #[prost(string, tag = "2")]
    pub urn: ::prost::alloc::string::String,
    /// the current properties on the resource.
    #[prost(message, optional, tag = "3")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// the delete request timeout represented in seconds.
    #[prost(double, tag = "4")]
    pub timeout: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConstructRequest {
    /// the project name.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// the name of the stack being deployed into.
    #[prost(string, tag = "2")]
    pub stack: ::prost::alloc::string::String,
    /// the configuration variables to apply before running.
    #[prost(map = "string, string", tag = "3")]
    pub config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// true if we're only doing a dryrun (preview).
    #[prost(bool, tag = "4")]
    pub dry_run: bool,
    /// the degree of parallelism for resource operations (<=1 for serial).
    #[prost(int32, tag = "5")]
    pub parallel: i32,
    /// the address for communicating back to the resource monitor.
    #[prost(string, tag = "6")]
    pub monitor_endpoint: ::prost::alloc::string::String,
    /// the type of the object allocated.
    #[prost(string, tag = "7")]
    pub r#type: ::prost::alloc::string::String,
    /// the name, for URN purposes, of the object.
    #[prost(string, tag = "8")]
    pub name: ::prost::alloc::string::String,
    /// an optional parent URN that this child resource belongs to.
    #[prost(string, tag = "9")]
    pub parent: ::prost::alloc::string::String,
    /// the inputs to the resource constructor.
    #[prost(message, optional, tag = "10")]
    pub inputs: ::core::option::Option<::prost_types::Struct>,
    /// a map from property keys to the dependencies of the property.
    #[prost(map = "string, message", tag = "11")]
    pub input_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        construct_request::PropertyDependencies,
    >,
    /// the map of providers to use for this resource's children.
    #[prost(map = "string, string", tag = "13")]
    pub providers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// a list of URNs that this resource depends on, as observed by the language host.
    #[prost(string, repeated, tag = "15")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the configuration keys that have secret values.
    #[prost(string, repeated, tag = "16")]
    pub config_secret_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the organization of the stack being deployed into.
    #[prost(string, tag = "17")]
    pub organization: ::prost::alloc::string::String,
    /// Resource options:
    /// Do not change field IDs. Add new fields at the end.
    ///
    /// true if the resource should be marked protected.
    #[prost(bool, tag = "12")]
    pub protect: bool,
    /// a list of additional URNs that shoud be considered the same.
    #[prost(string, repeated, tag = "14")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// additional output properties that should be treated as secrets.
    #[prost(string, repeated, tag = "18")]
    pub additional_secret_outputs: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// default timeouts for CRUD operations on this resource.
    #[prost(message, optional, tag = "19")]
    pub custom_timeouts: ::core::option::Option<construct_request::CustomTimeouts>,
    /// URN of a resource that, if deleted, also deletes this resource.
    #[prost(string, tag = "20")]
    pub deleted_with: ::prost::alloc::string::String,
    /// whether to delete the resource before replacing it.
    #[prost(bool, tag = "21")]
    pub delete_before_replace: bool,
    /// properties that should be ignored during a diff
    #[prost(string, repeated, tag = "22")]
    pub ignore_changes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// properties that, when changed, trigger a replacement
    #[prost(string, repeated, tag = "23")]
    pub replace_on_changes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// whether to retain the resource in the cloud provider when it is deleted
    #[prost(bool, tag = "24")]
    pub retain_on_delete: bool,
}
/// Nested message and enum types in `ConstructRequest`.
pub mod construct_request {
    /// PropertyDependencies describes the resources that a particular property depends on.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PropertyDependencies {
        /// A list of URNs this property depends on.
        #[prost(string, repeated, tag = "1")]
        pub urns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// CustomTimeouts specifies timeouts for resource provisioning operations.
    /// Use it with the \[Timeouts\] option when creating new resources
    /// to override default timeouts.
    ///
    /// Each timeout is specified as a duration string such as,
    /// "5ms" (5 milliseconds), "40s" (40 seconds),
    /// and "1m30s" (1 minute, 30 seconds).
    ///
    /// The following units are accepted.
    ///
    ///    - ns: nanoseconds
    ///    - us: microseconds
    ///    - µs: microseconds
    ///    - ms: milliseconds
    ///    - s: seconds
    ///    - m: minutes
    ///    - h: hours
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomTimeouts {
        #[prost(string, tag = "1")]
        pub create: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub update: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub delete: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConstructResponse {
    /// the URN of the component resource.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// any properties that were computed during construction.
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<::prost_types::Struct>,
    /// a map from property keys to the dependencies of the property.
    #[prost(map = "string, message", tag = "3")]
    pub state_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        construct_response::PropertyDependencies,
    >,
}
/// Nested message and enum types in `ConstructResponse`.
pub mod construct_response {
    /// PropertyDependencies describes the resources that a particular property depends on.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PropertyDependencies {
        /// A list of URNs this property depends on.
        #[prost(string, repeated, tag = "1")]
        pub urns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// ErrorResourceInitFailed is sent as a Detail `ResourceProvider.{Create, Update}` fail because a
/// resource was created successfully, but failed to initialize.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResourceInitFailed {
    /// the ID of the created resource.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// any properties that were computed during updating.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// error messages associated with initialization failure.
    #[prost(string, repeated, tag = "3")]
    pub reasons: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the current inputs to this resource (only applicable for Read)
    #[prost(message, optional, tag = "4")]
    pub inputs: ::core::option::Option<::prost_types::Struct>,
}
/// GetMappingRequest allows providers to return ecosystem specific information to allow the provider to be
/// converted from a source markup to Pulumi. It's expected that provider bridges that target a given ecosystem
/// (e.g. Terraform, Kubernetes) would also publish a conversion plugin to convert markup from that ecosystem
/// to Pulumi, using the bridged providers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMappingRequest {
    /// the conversion key for the mapping being requested.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// GetMappingResponse returns convert plugin specific data for this provider. This will normally be human
/// readable JSON, but the engine doesn't mandate any form.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMappingResponse {
    /// the provider key this is mapping for. For example the Pulumi provider "terraform-template" would return "template" for this.
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
    /// the conversion plugin specific data.
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod resource_provider_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ResourceProvider is a service that understands how to create, read, update, or delete resources for types defined
    /// within a single package.  It is driven by the overall planning engine in response to resource diffs.
    #[derive(Debug, Clone)]
    pub struct ResourceProviderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResourceProviderClient<tonic::transport::Channel> {
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
    impl<T> ResourceProviderClient<T>
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
        ) -> ResourceProviderClient<InterceptedService<T, F>>
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
            ResourceProviderClient::new(InterceptedService::new(inner, interceptor))
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
        /// GetSchema fetches the schema for this resource provider.
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSchemaResponse>,
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
                "/pulumirpc.ResourceProvider/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "GetSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// CheckConfig validates the configuration for this resource provider.
        pub async fn check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckRequest>,
        ) -> std::result::Result<tonic::Response<super::CheckResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/CheckConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "CheckConfig"));
            self.inner.unary(req, path, codec).await
        }
        /// DiffConfig checks the impact a hypothetical change to this provider's configuration will have on the provider.
        pub async fn diff_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DiffRequest>,
        ) -> std::result::Result<tonic::Response<super::DiffResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/DiffConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "DiffConfig"));
            self.inner.unary(req, path, codec).await
        }
        /// Configure configures the resource provider with "globals" that control its behavior.
        pub async fn configure(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigureResponse>,
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
                "/pulumirpc.ResourceProvider/Configure",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Configure"));
            self.inner.unary(req, path, codec).await
        }
        /// Invoke dynamically executes a built-in function in the provider.
        pub async fn invoke(
            &mut self,
            request: impl tonic::IntoRequest<super::InvokeRequest>,
        ) -> std::result::Result<tonic::Response<super::InvokeResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Invoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Invoke"));
            self.inner.unary(req, path, codec).await
        }
        /// StreamInvoke dynamically executes a built-in function in the provider, which returns a stream
        /// of responses.
        pub async fn stream_invoke(
            &mut self,
            request: impl tonic::IntoRequest<super::InvokeRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::InvokeResponse>>,
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
                "/pulumirpc.ResourceProvider/StreamInvoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "StreamInvoke"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Call dynamically executes a method in the provider associated with a component resource.
        pub async fn call(
            &mut self,
            request: impl tonic::IntoRequest<super::CallRequest>,
        ) -> std::result::Result<tonic::Response<super::CallResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Call",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Call"));
            self.inner.unary(req, path, codec).await
        }
        /// Check validates that the given property bag is valid for a resource of the given type and returns the inputs
        /// that should be passed to successive calls to Diff, Create, or Update for this resource. As a rule, the provider
        /// inputs returned by a call to Check should preserve the original representation of the properties as present in
        /// the program inputs. Though this rule is not required for correctness, violations thereof can negatively impact
        /// the end-user experience, as the provider inputs are using for detecting and rendering diffs.
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckRequest>,
        ) -> std::result::Result<tonic::Response<super::CheckResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Check",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Check"));
            self.inner.unary(req, path, codec).await
        }
        /// Diff checks what impacts a hypothetical update will have on the resource's properties.
        pub async fn diff(
            &mut self,
            request: impl tonic::IntoRequest<super::DiffRequest>,
        ) -> std::result::Result<tonic::Response<super::DiffResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Diff",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Diff"));
            self.inner.unary(req, path, codec).await
        }
        /// Create allocates a new instance of the provided resource and returns its unique ID afterwards.  (The input ID
        /// must be blank.)  If this call fails, the resource must not have been created (i.e., it is "transactional").
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Create",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Create"));
            self.inner.unary(req, path, codec).await
        }
        /// Read the current live state associated with a resource.  Enough state must be include in the inputs to uniquely
        /// identify the resource; this is typically just the resource ID, but may also include some properties.
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Read",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Read"));
            self.inner.unary(req, path, codec).await
        }
        /// Update updates an existing resource with new values.
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Update",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Update"));
            self.inner.unary(req, path, codec).await
        }
        /// Delete tears down an existing resource with the given ID.  If it fails, the resource is assumed to still exist.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Delete"));
            self.inner.unary(req, path, codec).await
        }
        /// Construct creates a new instance of the provided component resource and returns its state.
        pub async fn construct(
            &mut self,
            request: impl tonic::IntoRequest<super::ConstructRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConstructResponse>,
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
                "/pulumirpc.ResourceProvider/Construct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Construct"));
            self.inner.unary(req, path, codec).await
        }
        /// Cancel signals the provider to gracefully shut down and abort any ongoing resource operations.
        /// Operations aborted in this way will return an error (e.g., `Update` and `Create` will either return a
        /// creation error or an initialization error). Since Cancel is advisory and non-blocking, it is up
        /// to the host to decide how long to wait after Cancel is called before (e.g.)
        /// hard-closing any gRPC connection.
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Cancel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Cancel"));
            self.inner.unary(req, path, codec).await
        }
        /// GetPluginInfo returns generic information about this plugin, like its version.
        pub async fn get_plugin_info(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::PluginInfo>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/GetPluginInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "GetPluginInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// Attach sends the engine address to an already running plugin.
        pub async fn attach(
            &mut self,
            request: impl tonic::IntoRequest<super::PluginAttach>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/pulumirpc.ResourceProvider/Attach",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "Attach"));
            self.inner.unary(req, path, codec).await
        }
        /// GetMapping fetches the mapping for this resource provider, if any. A provider should return an empty
        /// response (not an error) if it doesn't have a mapping for the given key.
        pub async fn get_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMappingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMappingResponse>,
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
                "/pulumirpc.ResourceProvider/GetMapping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceProvider", "GetMapping"));
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alias {
    #[prost(oneof = "alias::Alias", tags = "1, 2")]
    pub alias: ::core::option::Option<alias::Alias>,
}
/// Nested message and enum types in `Alias`.
pub mod alias {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Spec {
        /// The previous name of the resource.  If none is provided, we will use the current name.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The previous type of the resource. If none is provided, we will use the current resoource type.
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// The previous stack of the resource. If not set, the current stack of the resource is used.
        #[prost(string, tag = "3")]
        pub stack: ::prost::alloc::string::String,
        /// The previous project of the resource. If not set, the current project of the resource is used.
        #[prost(string, tag = "4")]
        pub project: ::prost::alloc::string::String,
        /// The previous parent of the resource. If not set, the current parent of the resource is used.
        #[prost(oneof = "spec::Parent", tags = "5, 6")]
        pub parent: ::core::option::Option<spec::Parent>,
    }
    /// Nested message and enum types in `Spec`.
    pub mod spec {
        /// The previous parent of the resource. If not set, the current parent of the resource is used.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Parent {
            /// The urn of the previous parent.
            #[prost(string, tag = "5")]
            ParentUrn(::prost::alloc::string::String),
            /// Used to indicate the resource previously had no parent. If false this property is ignored.
            #[prost(bool, tag = "6")]
            NoParent(bool),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Alias {
        /// The previous urn to alias to.
        #[prost(string, tag = "1")]
        Urn(::prost::alloc::string::String),
        /// An alias specification.
        #[prost(message, tag = "2")]
        Spec(Spec),
    }
}
/// SupportsFeatureRequest allows a client to test if the resource monitor supports a certain feature, which it may use
/// to control the format or types of messages it sends.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupportsFeatureRequest {
    /// the ID of the feature to test support for.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupportsFeatureResponse {
    /// true when the resource monitor supports this feature.
    #[prost(bool, tag = "1")]
    pub has_support: bool,
}
/// ReadResourceRequest contains enough information to uniquely qualify and read a resource's state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResourceRequest {
    /// the ID of the resource to read.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// the type of the resource object.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// the name, for URN purposes, of the object.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// an optional parent URN that this child resource belongs to.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// optional state sufficient to uniquely identify the resource.
    #[prost(message, optional, tag = "5")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// a list of URNs that this read depends on, as observed by the language host.
    #[prost(string, repeated, tag = "6")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// an optional reference to the provider to use for this read.
    #[prost(string, tag = "7")]
    pub provider: ::prost::alloc::string::String,
    /// the version of the provider to use when servicing this request.
    #[prost(string, tag = "8")]
    pub version: ::prost::alloc::string::String,
    /// when true operations should return secrets as strongly typed.
    #[prost(bool, tag = "9")]
    pub accept_secrets: bool,
    /// a list of output properties that should also be treated as secret, in addition to ones we detect.
    #[prost(string, repeated, tag = "10")]
    pub additional_secret_outputs: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// when true operations should return resource references as strongly typed.
    #[prost(bool, tag = "12")]
    pub accept_resources: bool,
    /// the server url of the provider to use when servicing this request.
    #[prost(string, tag = "13")]
    pub plugin_download_url: ::prost::alloc::string::String,
    /// a map of checksums of the provider to use when servicing this request.
    #[prost(map = "string, bytes", tag = "15")]
    pub plugin_checksums: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    /// the optional source position of the user code that initiated the read.
    #[prost(message, optional, tag = "14")]
    pub source_position: ::core::option::Option<SourcePosition>,
}
/// ReadResourceResponse contains the result of reading a resource's state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResourceResponse {
    /// the URN for this resource.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// the state of the resource read from the live environment.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
}
/// RegisterResourceRequest contains information about a resource object that was newly allocated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResourceRequest {
    /// the type of the object allocated.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// the name, for URN purposes, of the object.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// an optional parent URN that this child resource belongs to.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// true if the resource is a custom, managed by a plugin's CRUD operations.
    #[prost(bool, tag = "4")]
    pub custom: bool,
    /// an object produced by the interpreter/source.
    #[prost(message, optional, tag = "5")]
    pub object: ::core::option::Option<::prost_types::Struct>,
    /// true if the resource should be marked protected.
    #[prost(bool, tag = "6")]
    pub protect: bool,
    /// a list of URNs that this resource depends on, as observed by the language host.
    #[prost(string, repeated, tag = "7")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// an optional reference to the provider to manage this resource's CRUD operations.
    #[prost(string, tag = "8")]
    pub provider: ::prost::alloc::string::String,
    /// a map from property keys to the dependencies of the property.
    #[prost(map = "string, message", tag = "9")]
    pub property_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        register_resource_request::PropertyDependencies,
    >,
    /// true if this resource should be deleted before replacement.
    #[prost(bool, tag = "10")]
    pub delete_before_replace: bool,
    /// the version of the provider to use when servicing this request.
    #[prost(string, tag = "11")]
    pub version: ::prost::alloc::string::String,
    /// a list of property selectors to ignore during updates.
    #[prost(string, repeated, tag = "12")]
    pub ignore_changes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// when true operations should return secrets as strongly typed.
    #[prost(bool, tag = "13")]
    pub accept_secrets: bool,
    /// a list of output properties that should also be treated as secret, in addition to ones we detect.
    #[prost(string, repeated, tag = "14")]
    pub additional_secret_outputs: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// a list of additional URNs that should be considered the same.
    #[prost(string, repeated, tag = "15")]
    pub alias_ur_ns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// if set, this resource's state should be imported from the given ID.
    #[prost(string, tag = "16")]
    pub import_id: ::prost::alloc::string::String,
    /// ability to pass a custom Timeout block.
    #[prost(message, optional, tag = "17")]
    pub custom_timeouts: ::core::option::Option<
        register_resource_request::CustomTimeouts,
    >,
    /// true if the deleteBeforeReplace property should be treated as defined even if it is false.
    #[prost(bool, tag = "18")]
    pub delete_before_replace_defined: bool,
    /// true if the request is from an SDK that supports partially-known properties during preview.
    #[prost(bool, tag = "19")]
    pub supports_partial_values: bool,
    /// true if the resource is a plugin-managed component resource.
    #[prost(bool, tag = "20")]
    pub remote: bool,
    /// when true operations should return resource references as strongly typed.
    #[prost(bool, tag = "21")]
    pub accept_resources: bool,
    /// an optional reference to the provider map to manage this resource's CRUD operations.
    #[prost(map = "string, string", tag = "22")]
    pub providers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// a list of properties that if changed should force a replacement.
    #[prost(string, repeated, tag = "23")]
    pub replace_on_changes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the server URL of the provider to use when servicing this request.
    #[prost(string, tag = "24")]
    pub plugin_download_url: ::prost::alloc::string::String,
    /// a map of checksums expected for the provider plugin.
    #[prost(map = "string, bytes", tag = "30")]
    pub plugin_checksums: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    /// if true the engine will not call the resource providers delete method for this resource.
    #[prost(bool, tag = "25")]
    pub retain_on_delete: bool,
    /// a list of additional aliases that should be considered the same.
    #[prost(message, repeated, tag = "26")]
    pub aliases: ::prost::alloc::vec::Vec<Alias>,
    /// if set the engine will not call the resource providers delete method for this resource when specified resource is deleted.
    #[prost(string, tag = "27")]
    pub deleted_with: ::prost::alloc::string::String,
    /// Indicates that alias specs are specified correctly according to the spec.
    /// Older versions of the Node.js SDK did not send alias specs correctly.
    /// If this is not set to true and the engine detects the request is from the
    /// Node.js runtime, the engine will transform incorrect alias specs into
    /// correct ones.
    /// Other SDKs that are correctly specifying alias specs could set this to
    /// true, but it's not necessary.
    #[prost(bool, tag = "28")]
    pub alias_specs: bool,
    /// the optional source position of the user code that initiated the register.
    #[prost(message, optional, tag = "29")]
    pub source_position: ::core::option::Option<SourcePosition>,
}
/// Nested message and enum types in `RegisterResourceRequest`.
pub mod register_resource_request {
    /// PropertyDependencies describes the resources that a particular property depends on.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PropertyDependencies {
        /// A list of URNs this property depends on.
        #[prost(string, repeated, tag = "1")]
        pub urns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// CustomTimeouts allows a user to be able to create a set of custom timeout parameters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomTimeouts {
        /// The create resource timeout represented as a string e.g. 5m.
        #[prost(string, tag = "1")]
        pub create: ::prost::alloc::string::String,
        /// The update resource timeout represented as a string e.g. 5m.
        #[prost(string, tag = "2")]
        pub update: ::prost::alloc::string::String,
        /// The delete resource timeout represented as a string e.g. 5m.
        #[prost(string, tag = "3")]
        pub delete: ::prost::alloc::string::String,
    }
}
/// RegisterResourceResponse is returned by the engine after a resource has finished being initialized.  It includes the
/// auto-assigned URN, the provider-assigned ID, and any other properties initialized by the engine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResourceResponse {
    /// the URN assigned by the engine.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// the unique ID assigned by the provider.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// the resulting object properties, including provider defaults.
    #[prost(message, optional, tag = "3")]
    pub object: ::core::option::Option<::prost_types::Struct>,
    /// if true, the object's state is stable and may be trusted not to change.
    #[prost(bool, tag = "4")]
    pub stable: bool,
    /// an optional list of guaranteed-stable properties.
    #[prost(string, repeated, tag = "5")]
    pub stables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// a map from property keys to the dependencies of the property.
    #[prost(map = "string, message", tag = "6")]
    pub property_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        register_resource_response::PropertyDependencies,
    >,
}
/// Nested message and enum types in `RegisterResourceResponse`.
pub mod register_resource_response {
    /// PropertyDependencies describes the resources that a particular property depends on.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PropertyDependencies {
        /// A list of URNs this property depends on.
        #[prost(string, repeated, tag = "1")]
        pub urns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// RegisterResourceOutputsRequest adds extra resource outputs created by the program after registration has occurred.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResourceOutputsRequest {
    /// the URN for the resource to attach output properties to.
    #[prost(string, tag = "1")]
    pub urn: ::prost::alloc::string::String,
    /// additional output properties to add to the existing resource.
    #[prost(message, optional, tag = "2")]
    pub outputs: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceInvokeRequest {
    /// the function token to invoke.
    #[prost(string, tag = "1")]
    pub tok: ::prost::alloc::string::String,
    /// the arguments for the function invocation.
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<::prost_types::Struct>,
    /// an optional reference to the provider version to use for this invoke.
    #[prost(string, tag = "3")]
    pub provider: ::prost::alloc::string::String,
    /// the version of the provider to use when servicing this request.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// when true operations should return resource references as strongly typed.
    #[prost(bool, tag = "5")]
    pub accept_resources: bool,
    /// an optional reference to the provider url to use for this invoke.
    #[prost(string, tag = "6")]
    pub plugin_download_url: ::prost::alloc::string::String,
    /// a map of checksums expected for the provider plugin.
    #[prost(map = "string, bytes", tag = "8")]
    pub plugin_checksums: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    /// the optional source position of the user code that initiated the invoke.
    #[prost(message, optional, tag = "7")]
    pub source_position: ::core::option::Option<SourcePosition>,
}
/// Generated client implementations.
pub mod resource_monitor_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ResourceMonitor is the interface a source uses to talk back to the planning monitor orchestrating the execution.
    #[derive(Debug, Clone)]
    pub struct ResourceMonitorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResourceMonitorClient<tonic::transport::Channel> {
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
    impl<T> ResourceMonitorClient<T>
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
        ) -> ResourceMonitorClient<InterceptedService<T, F>>
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
            ResourceMonitorClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn supports_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::SupportsFeatureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SupportsFeatureResponse>,
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
                "/pulumirpc.ResourceMonitor/SupportsFeature",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceMonitor", "SupportsFeature"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn invoke(
            &mut self,
            request: impl tonic::IntoRequest<super::ResourceInvokeRequest>,
        ) -> std::result::Result<tonic::Response<super::InvokeResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceMonitor/Invoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceMonitor", "Invoke"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_invoke(
            &mut self,
            request: impl tonic::IntoRequest<super::ResourceInvokeRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::InvokeResponse>>,
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
                "/pulumirpc.ResourceMonitor/StreamInvoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceMonitor", "StreamInvoke"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn call(
            &mut self,
            request: impl tonic::IntoRequest<super::CallRequest>,
        ) -> std::result::Result<tonic::Response<super::CallResponse>, tonic::Status> {
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
                "/pulumirpc.ResourceMonitor/Call",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceMonitor", "Call"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadResourceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadResourceResponse>,
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
                "/pulumirpc.ResourceMonitor/ReadResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.ResourceMonitor", "ReadResource"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterResourceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterResourceResponse>,
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
                "/pulumirpc.ResourceMonitor/RegisterResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pulumirpc.ResourceMonitor", "RegisterResource"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_resource_outputs(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterResourceOutputsRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/pulumirpc.ResourceMonitor/RegisterResourceOutputs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pulumirpc.ResourceMonitor",
                        "RegisterResourceOutputs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorCause {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stack_trace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeRequest {
    /// the type token of the resource.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// the full properties to use for validation.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// the URN of the resource.
    #[prost(string, tag = "3")]
    pub urn: ::prost::alloc::string::String,
    /// the name for the resource's URN.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// the resource options.
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<AnalyzerResourceOptions>,
    /// the resource's provider.
    #[prost(message, optional, tag = "6")]
    pub provider: ::core::option::Option<AnalyzerProviderResource>,
}
/// AnalyzerResource defines the view of a Pulumi-managed resource as sent to Analyzers. The properties
/// of the resource are specific to the type of analysis being performed. See the Analyzer
/// service definition for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzerResource {
    /// the type token of the resource.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// the full properties to use for validation.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// the URN of the resource.
    #[prost(string, tag = "3")]
    pub urn: ::prost::alloc::string::String,
    /// the name for the resource's URN.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// the resource options.
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<AnalyzerResourceOptions>,
    /// the resource's provider.
    #[prost(message, optional, tag = "6")]
    pub provider: ::core::option::Option<AnalyzerProviderResource>,
    /// an optional parent URN that this child resource belongs to.
    #[prost(string, tag = "7")]
    pub parent: ::prost::alloc::string::String,
    /// a list of URNs that this resource depends on.
    #[prost(string, repeated, tag = "8")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// a map from property keys to the dependencies of the property.
    #[prost(map = "string, message", tag = "9")]
    pub property_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        AnalyzerPropertyDependencies,
    >,
}
/// AnalyzerResourceOptions defines the options associated with a resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzerResourceOptions {
    /// true if the resource should be marked protected.
    #[prost(bool, tag = "1")]
    pub protect: bool,
    /// a list of property names to ignore during changes.
    #[prost(string, repeated, tag = "2")]
    pub ignore_changes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// true if this resource should be deleted before replacement.
    #[prost(bool, tag = "3")]
    pub delete_before_replace: bool,
    /// true if the deleteBeforeReplace property should be treated as defined even if it is false.
    #[prost(bool, tag = "4")]
    pub delete_before_replace_defined: bool,
    /// a list of output properties that should also be treated as secret, in addition to ones we detect.
    #[prost(string, repeated, tag = "5")]
    pub additional_secret_outputs: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// a list of additional URNs that shoud be considered the same.
    #[prost(string, repeated, tag = "6")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// a config block that will be used to configure timeouts for CRUD operations.
    #[prost(message, optional, tag = "7")]
    pub custom_timeouts: ::core::option::Option<
        analyzer_resource_options::CustomTimeouts,
    >,
}
/// Nested message and enum types in `AnalyzerResourceOptions`.
pub mod analyzer_resource_options {
    /// CustomTimeouts allows a user to be able to create a set of custom timeout parameters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomTimeouts {
        /// The create resource timeout in seconds.
        #[prost(double, tag = "1")]
        pub create: f64,
        /// The update resource timeout in seconds.
        #[prost(double, tag = "2")]
        pub update: f64,
        /// The delete resource timeout in seconds.
        #[prost(double, tag = "3")]
        pub delete: f64,
    }
}
/// AnalyzerProviderResource provides information about a resource's provider.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzerProviderResource {
    /// the type token of the resource.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// the full properties to use for validation.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// the URN of the resource.
    #[prost(string, tag = "3")]
    pub urn: ::prost::alloc::string::String,
    /// the name for the resource's URN.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// AnalyzerPropertyDependencies describes the resources that a particular property depends on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzerPropertyDependencies {
    /// A list of URNs this property depends on.
    #[prost(string, repeated, tag = "1")]
    pub urns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeStackRequest {
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<AnalyzerResource>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeResponse {
    /// information about policy violations.
    #[prost(message, repeated, tag = "2")]
    pub diagnostics: ::prost::alloc::vec::Vec<AnalyzeDiagnostic>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeDiagnostic {
    /// Name of the violated policy.
    #[prost(string, tag = "1")]
    pub policy_name: ::prost::alloc::string::String,
    /// Name of the policy pack the policy is in.
    #[prost(string, tag = "2")]
    pub policy_pack_name: ::prost::alloc::string::String,
    /// Version of the policy pack.
    #[prost(string, tag = "3")]
    pub policy_pack_version: ::prost::alloc::string::String,
    /// Description of policy rule. e.g., "encryption enabled."
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Message to display on policy violation, e.g., remediation steps.
    #[prost(string, tag = "5")]
    pub message: ::prost::alloc::string::String,
    /// Keywords/terms to associate with a policy, e.g., "cost".
    #[prost(string, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Severity of the policy violation.
    #[prost(enumeration = "EnforcementLevel", tag = "7")]
    pub enforcement_level: i32,
    /// URN of the resource that violates the policy.
    #[prost(string, tag = "8")]
    pub urn: ::prost::alloc::string::String,
}
/// AnalyzerInfo provides metadata about a PolicyPack inside an analyzer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzerInfo {
    /// Name of the PolicyPack.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Pretty name for the PolicyPack.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Metadata about policies contained in PolicyPack.
    #[prost(message, repeated, tag = "3")]
    pub policies: ::prost::alloc::vec::Vec<PolicyInfo>,
    /// Version of the Policy Pack.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Whether the Policy Pack supports config.
    #[prost(bool, tag = "5")]
    pub supports_config: bool,
    /// Map of policy name to config.
    #[prost(map = "string, message", tag = "6")]
    pub initial_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        PolicyConfig,
    >,
}
/// PolicyInfo provides metadata about a policy within a Policy Pack.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyInfo {
    /// Name of the policy.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Pretty name for the policy.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of policy rule. e.g., "encryption enabled."
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Message to display on policy violation, e.g., remediation steps.
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    /// Severity of the policy violation.
    #[prost(enumeration = "EnforcementLevel", tag = "5")]
    pub enforcement_level: i32,
    /// Config schema for the policy.
    #[prost(message, optional, tag = "6")]
    pub config_schema: ::core::option::Option<PolicyConfigSchema>,
}
/// PolicyConfigSchema provides the schema for a policy's configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyConfigSchema {
    /// JSON schema for each property.
    #[prost(message, optional, tag = "1")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
    /// Required properties.
    #[prost(string, repeated, tag = "2")]
    pub required: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PolicyConfig provides configuration for a policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyConfig {
    /// Enforcement level of the policy.
    #[prost(enumeration = "EnforcementLevel", tag = "1")]
    pub enforcement_level: i32,
    /// Configuration properties of the policy.
    #[prost(message, optional, tag = "2")]
    pub properties: ::core::option::Option<::prost_types::Struct>,
}
/// ConfigureAnalyzerRequest provides configuration information to the analyzer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureAnalyzerRequest {
    /// Map of policy name to config.
    #[prost(map = "string, message", tag = "1")]
    pub policy_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        PolicyConfig,
    >,
}
/// EnforcementLevel indicates the severity of a policy violation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnforcementLevel {
    /// Displayed to users, but does not block deployment.
    Advisory = 0,
    /// Stops deployment, cannot be overridden.
    Mandatory = 1,
    /// Disabled policies do not run during a deployment.
    Disabled = 2,
}
impl EnforcementLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EnforcementLevel::Advisory => "ADVISORY",
            EnforcementLevel::Mandatory => "MANDATORY",
            EnforcementLevel::Disabled => "DISABLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ADVISORY" => Some(Self::Advisory),
            "MANDATORY" => Some(Self::Mandatory),
            "DISABLED" => Some(Self::Disabled),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod analyzer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Analyzer provides a pluggable interface for checking resource definitions against some number of
    /// resource policies. It is intentionally open-ended, allowing for implementations that check
    /// everything from raw resource definitions to entire projects/stacks/snapshots for arbitrary
    /// issues -- style, policy, correctness, security, and so on.
    #[derive(Debug, Clone)]
    pub struct AnalyzerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AnalyzerClient<tonic::transport::Channel> {
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
    impl<T> AnalyzerClient<T>
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
        ) -> AnalyzerClient<InterceptedService<T, F>>
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
            AnalyzerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Analyze analyzes a single resource object, and returns any errors that it finds.
        /// Called with the "inputs" to the resource, before it is updated.
        pub async fn analyze(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnalyzeResponse>,
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
                "/pulumirpc.Analyzer/Analyze",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.Analyzer", "Analyze"));
            self.inner.unary(req, path, codec).await
        }
        /// AnalyzeStack analyzes all resources within a stack, at the end of a successful
        /// preview or update. The provided resources are the "outputs", after any mutations
        /// have taken place.
        pub async fn analyze_stack(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeStackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnalyzeResponse>,
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
                "/pulumirpc.Analyzer/AnalyzeStack",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.Analyzer", "AnalyzeStack"));
            self.inner.unary(req, path, codec).await
        }
        /// GetAnalyzerInfo returns metadata about the analyzer (e.g., list of policies contained).
        pub async fn get_analyzer_info(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::AnalyzerInfo>, tonic::Status> {
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
                "/pulumirpc.Analyzer/GetAnalyzerInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.Analyzer", "GetAnalyzerInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// GetPluginInfo returns generic information about this plugin, like its version.
        pub async fn get_plugin_info(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::PluginInfo>, tonic::Status> {
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
                "/pulumirpc.Analyzer/GetPluginInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.Analyzer", "GetPluginInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// Configure configures the analyzer, passing configuration properties for each policy.
        pub async fn configure(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureAnalyzerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/pulumirpc.Analyzer/Configure",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.Analyzer", "Configure"));
            self.inner.unary(req, path, codec).await
        }
    }
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
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
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
    pub config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
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
    pub source: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The target of a codegen.LoaderServer to use for loading schemas.
    #[prost(string, tag = "2")]
    pub loader_target: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProgramResponse {
    /// any diagnostics from code generation.
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<codegen::Diagnostic>,
    /// the generated program source code.
    #[prost(map = "string, bytes", tag = "2")]
    pub source: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
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
    pub local_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProjectResponse {
    /// any diagnostics from code generation.
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<codegen::Diagnostic>,
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
    pub extra_files: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
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
/// Generated client implementations.
pub mod language_runtime_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// LanguageRuntime is the interface that the planning monitor uses to drive execution of an interpreter responsible
    /// for confguring and creating resource objects.
    #[derive(Debug, Clone)]
    pub struct LanguageRuntimeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LanguageRuntimeClient<tonic::transport::Channel> {
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
    impl<T> LanguageRuntimeClient<T>
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
        ) -> LanguageRuntimeClient<InterceptedService<T, F>>
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
            LanguageRuntimeClient::new(InterceptedService::new(inner, interceptor))
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
        /// GetRequiredPlugins computes the complete set of anticipated plugins required by a program.
        pub async fn get_required_plugins(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequiredPluginsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRequiredPluginsResponse>,
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
                "/pulumirpc.LanguageRuntime/GetRequiredPlugins",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pulumirpc.LanguageRuntime", "GetRequiredPlugins"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Run executes a program and returns its result.
        pub async fn run(
            &mut self,
            request: impl tonic::IntoRequest<super::RunRequest>,
        ) -> std::result::Result<tonic::Response<super::RunResponse>, tonic::Status> {
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
                "/pulumirpc.LanguageRuntime/Run",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.LanguageRuntime", "Run"));
            self.inner.unary(req, path, codec).await
        }
        /// GetPluginInfo returns generic information about this plugin, like its version.
        pub async fn get_plugin_info(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::PluginInfo>, tonic::Status> {
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
                "/pulumirpc.LanguageRuntime/GetPluginInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.LanguageRuntime", "GetPluginInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// InstallDependencies will install dependencies for the project, e.g. by running `npm install` for nodejs projects.
        pub async fn install_dependencies(
            &mut self,
            request: impl tonic::IntoRequest<super::InstallDependenciesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::InstallDependenciesResponse>>,
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
                "/pulumirpc.LanguageRuntime/InstallDependencies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("pulumirpc.LanguageRuntime", "InstallDependencies"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// About returns information about the runtime for this language.
        pub async fn about(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::AboutResponse>, tonic::Status> {
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
                "/pulumirpc.LanguageRuntime/About",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.LanguageRuntime", "About"));
            self.inner.unary(req, path, codec).await
        }
        /// GetProgramDependencies returns the set of dependencies required by the program.
        pub async fn get_program_dependencies(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProgramDependenciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProgramDependenciesResponse>,
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
                "/pulumirpc.LanguageRuntime/GetProgramDependencies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "pulumirpc.LanguageRuntime",
                        "GetProgramDependencies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RunPlugin executes a plugin program and returns its result asynchronously.
        pub async fn run_plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::RunPluginRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::RunPluginResponse>>,
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
                "/pulumirpc.LanguageRuntime/RunPlugin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.LanguageRuntime", "RunPlugin"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// GenerateProgram generates a given PCL program into a program for this language.
        pub async fn generate_program(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateProgramRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateProgramResponse>,
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
                "/pulumirpc.LanguageRuntime/GenerateProgram",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.LanguageRuntime", "GenerateProgram"));
            self.inner.unary(req, path, codec).await
        }
        /// GenerateProject generates a given PCL program into a project for this language.
        pub async fn generate_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateProjectResponse>,
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
                "/pulumirpc.LanguageRuntime/GenerateProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.LanguageRuntime", "GenerateProject"));
            self.inner.unary(req, path, codec).await
        }
        /// GeneratePackage generates a given pulumi package into a package for this language.
        pub async fn generate_package(
            &mut self,
            request: impl tonic::IntoRequest<super::GeneratePackageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GeneratePackageResponse>,
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
                "/pulumirpc.LanguageRuntime/GeneratePackage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.LanguageRuntime", "GeneratePackage"));
            self.inner.unary(req, path, codec).await
        }
        /// Pack packs a package into a language specific artifact.
        pub async fn pack(
            &mut self,
            request: impl tonic::IntoRequest<super::PackRequest>,
        ) -> std::result::Result<tonic::Response<super::PackResponse>, tonic::Status> {
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
                "/pulumirpc.LanguageRuntime/Pack",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.LanguageRuntime", "Pack"));
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertStateRequest {
    /// the gRPC target of the mapper service.
    #[prost(string, tag = "1")]
    pub mapper_target: ::prost::alloc::string::String,
    /// the args passed to `pulumi import` for this conversion. Normally used to specifiy a state file to
    /// import from.
    #[prost(string, repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A ResourceImport specifies a resource to import.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceImport {
    /// the type token for the resource.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// the name of the resource.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// the ID of the resource.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    /// the provider version to use for the resource, if any.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// the provider PluginDownloadURL to use for the resource, if any.
    #[prost(string, tag = "5")]
    pub plugin_download_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertStateResponse {
    /// a list of resources to import.
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<ResourceImport>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertProgramRequest {
    /// the source directory containing the program to convert from.
    #[prost(string, tag = "1")]
    pub source_directory: ::prost::alloc::string::String,
    /// a target directory to write the resulting PCL code and project file to.
    #[prost(string, tag = "2")]
    pub target_directory: ::prost::alloc::string::String,
    /// the gRPC target of the mapper service.
    #[prost(string, tag = "3")]
    pub mapper_target: ::prost::alloc::string::String,
    /// The target of a codegen.LoaderServer to use for loading schemas.
    #[prost(string, tag = "4")]
    pub loader_target: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertProgramResponse {
    /// any diagnostics from code generation.
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<codegen::Diagnostic>,
}
/// Generated client implementations.
pub mod converter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Converter is a service for converting between other ecosystems and Pulumi.
    /// This is currently unstable and experimental.
    #[derive(Debug, Clone)]
    pub struct ConverterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConverterClient<tonic::transport::Channel> {
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
    impl<T> ConverterClient<T>
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
        ) -> ConverterClient<InterceptedService<T, F>>
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
            ConverterClient::new(InterceptedService::new(inner, interceptor))
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
        /// ConvertState converts state from the target ecosystem into a form that can be imported into Pulumi.
        pub async fn convert_state(
            &mut self,
            request: impl tonic::IntoRequest<super::ConvertStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConvertStateResponse>,
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
                "/pulumirpc.Converter/ConvertState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.Converter", "ConvertState"));
            self.inner.unary(req, path, codec).await
        }
        /// ConvertProgram converts a program from the target ecosystem into a form that can be used with Pulumi.
        pub async fn convert_program(
            &mut self,
            request: impl tonic::IntoRequest<super::ConvertProgramRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConvertProgramResponse>,
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
                "/pulumirpc.Converter/ConvertProgram",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("pulumirpc.Converter", "ConvertProgram"));
            self.inner.unary(req, path, codec).await
        }
    }
}
