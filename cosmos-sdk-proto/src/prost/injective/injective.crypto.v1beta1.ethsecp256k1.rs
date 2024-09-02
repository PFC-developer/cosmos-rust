// @generated
/// PubKey defines a type alias for an ecdsa.PublicKey that implements
/// Tendermint's PubKey interface. It represents the 33-byte compressed public
/// key format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PubKey {
    const NAME: &'static str = "PubKey";
    const PACKAGE: &'static str = "injective.crypto.v1beta1.ethsecp256k1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.crypto.v1beta1.ethsecp256k1.{}", Self::NAME)
    }
}
/// PrivKey defines a type alias for an ecdsa.PrivateKey that implements
/// Tendermint's PrivateKey interface.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PrivKey {
    const NAME: &'static str = "PrivKey";
    const PACKAGE: &'static str = "injective.crypto.v1beta1.ethsecp256k1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.crypto.v1beta1.ethsecp256k1.{}", Self::NAME)
    }
}
include!("injective.crypto.v1beta1.ethsecp256k1.serde.rs");
// @@protoc_insertion_point(module)
