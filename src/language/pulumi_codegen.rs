/// Pos represents a single position in a source file, by addressing the start byte of a unicode character
/// encoded in UTF-8.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pos {
    /// Line is the source code line where this position points. Lines are counted starting at 1 and
    /// incremented for each newline character encountered.
    #[prost(int64, tag = "1")]
    pub line: i64,
    /// Column is the source code column where this position points, in unicode characters, with counting
    /// starting at 1.
    ///
    /// Column counts characters as they appear visually, so for example a latin letter with a combining
    /// diacritic mark counts as one character. This is intended for rendering visual markers against source
    /// code in contexts where these diacritics would be rendered in a single character cell. Technically
    /// speaking, Column is counting grapheme clusters as used in unicode normalization.
    #[prost(int64, tag = "2")]
    pub column: i64,
    /// Byte is the byte offset into the file where the indicated character begins. This is a zero-based offset
    /// to the first byte of the first UTF-8 codepoint sequence in the character, and thus gives a position
    /// that can be resolved _without_ awareness of Unicode characters.
    #[prost(int64, tag = "3")]
    pub byte: i64,
}
/// Range represents a span of characters between two positions in a source file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    /// Filename is the name of the file into which this range's positions point.
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
    /// Start and End represent the bounds of this range. Start is inclusive and End is exclusive.
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<Pos>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<Pos>,
}
/// Diagnostic represents information to be presented to a user about an error or anomaly in parsing or evaluating configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Diagnostic {
    #[prost(enumeration = "DiagnosticSeverity", tag = "1")]
    pub severity: i32,
    /// Summary and Detail contain the English-language description of the
    /// problem. Summary is a terse description of the general problem and
    /// detail is a more elaborate, often-multi-sentence description of
    /// the problem and what might be done to solve it.
    #[prost(string, tag = "2")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub detail: ::prost::alloc::string::String,
    /// Subject and Context are both source ranges relating to the diagnostic.
    ///
    /// Subject is a tight range referring to exactly the construct that
    /// is problematic, while Context is an optional broader range (which should
    /// fully contain Subject) that ought to be shown around Subject when
    /// generating isolated source-code snippets in diagnostic messages.
    /// If Context is nil, the Subject is also the Context.
    ///
    /// Some diagnostics have no source ranges at all. If Context is set then
    /// Subject should always also be set.
    #[prost(message, optional, tag = "4")]
    pub subject: ::core::option::Option<Range>,
    #[prost(message, optional, tag = "5")]
    pub context: ::core::option::Option<Range>,
}
/// DiagnosticSeverity is the severity level of a diagnostic message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DiagnosticSeverity {
    /// DIAG_INVALID is the invalid zero value of DiagnosticSeverity
    DiagInvalid = 0,
    /// DIAG_ERROR indicates that the problem reported by a diagnostic prevents
    /// further progress in parsing and/or evaluating the subject.
    DiagError = 1,
    /// DIAG_WARNING indicates that the problem reported by a diagnostic warrants
    /// user attention but does not prevent further progress. It is most
    /// commonly used for showing deprecation notices.
    DiagWarning = 2,
}
impl DiagnosticSeverity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DiagnosticSeverity::DiagInvalid => "DIAG_INVALID",
            DiagnosticSeverity::DiagError => "DIAG_ERROR",
            DiagnosticSeverity::DiagWarning => "DIAG_WARNING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DIAG_INVALID" => Some(Self::DiagInvalid),
            "DIAG_ERROR" => Some(Self::DiagError),
            "DIAG_WARNING" => Some(Self::DiagWarning),
            _ => None,
        }
    }
}
