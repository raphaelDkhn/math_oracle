#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MathRequest {
    #[prost(enumeration = "Operation", tag = "1")]
    pub operation: i32,
    #[prost(int64, repeated, tag = "2")]
    pub operands: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MathResponse {
    #[prost(int64, tag = "1")]
    pub result: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Operation {
    Unknown = 0,
    Sqrt = 1,
    Exp = 2,
}
impl Operation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Operation::Unknown => "UNKNOWN",
            Operation::Sqrt => "SQRT",
            Operation::Exp => "EXP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "SQRT" => Some(Self::Sqrt),
            "EXP" => Some(Self::Exp),
            _ => None,
        }
    }
}
