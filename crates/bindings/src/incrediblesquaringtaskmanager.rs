///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
    struct G2Point { uint256[2] X; uint256[2] Y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct G1Point { uint256 X; uint256 Y; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub Y: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { X: tuple.0, Y: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G1Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct G2Point { uint256[2] X; uint256[2] Y; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        #[allow(missing_docs)]
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
        pub Y: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<256>,
                2usize,
            >,
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<256>,
                2usize,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { X: tuple.0, Y: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G2Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G2Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "G2Point(uint256[2] X,uint256[2] Y)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G2Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<T, P, N> {
        BN254Instance::<T, P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BN254`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BN254Instance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

See the [wrapper's documentation](`BN254Instance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> BN254Instance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<T, P, N> {
            BN254Instance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IBLSSignatureCheckerTypes {
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBLSSignatureCheckerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerStakesAndSignature {
        #[allow(missing_docs)]
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerPubkeys: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub quorumApks: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerStakeIndices: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<u32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            BN254::G2Point,
            BN254::G1Point,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
            <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NonSignerStakesAndSignature>
        for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerStakesAndSignature) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.nonSignerPubkeys,
                    value.quorumApks,
                    value.apkG2,
                    value.sigma,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NonSignerStakesAndSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    nonSignerPubkeys: tuple.1,
                    quorumApks: tuple.2,
                    apkG2: tuple.3,
                    sigma: tuple.4,
                    quorumApkIndices: tuple.5,
                    totalStakeIndices: tuple.6,
                    nonSignerStakeIndices: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for NonSignerStakesAndSignature {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for NonSignerStakesAndSignature {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerPubkeys),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApks),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApkIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerStakeIndices),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for NonSignerStakesAndSignature {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for NonSignerStakesAndSignature {
            const NAME: &'static str = "NonSignerStakesAndSignature";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "NonSignerStakesAndSignature(uint32[] nonSignerQuorumBitmapIndices,BN254.G1Point[] nonSignerPubkeys,BN254.G1Point[] quorumApks,BN254.G2Point apkG2,BN254.G1Point sigma,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(4);
                components
                    .push(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BN254::G2Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BN254::G2Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerPubkeys,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumApks)
                        .0,
                    <BN254::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.apkG2,
                        )
                        .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sigma,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumApkIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerStakeIndices,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for NonSignerStakesAndSignature {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerPubkeys,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApks,
                    )
                    + <BN254::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.apkG2,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigma,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApkIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerStakeIndices,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerQuorumBitmapIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerPubkeys,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApks,
                    out,
                );
                <BN254::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.apkG2,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigma,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApkIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumStakeTotals {
        #[allow(missing_docs)]
        pub signedStakeForQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
        #[allow(missing_docs)]
        pub totalStakeForQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U96,
            >,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U96,
            >,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumStakeTotals> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumStakeTotals) -> Self {
                (value.signedStakeForQuorum, value.totalStakeForQuorum)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumStakeTotals {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signedStakeForQuorum: tuple.0,
                    totalStakeForQuorum: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumStakeTotals {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumStakeTotals {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.signedStakeForQuorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeForQuorum),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for QuorumStakeTotals {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for QuorumStakeTotals {
            const NAME: &'static str = "QuorumStakeTotals";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumStakeTotals(uint96[] signedStakeForQuorum,uint96[] totalStakeForQuorum)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signedStakeForQuorum,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeForQuorum,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumStakeTotals {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signedStakeForQuorum,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeForQuorum,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signedStakeForQuorum,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeForQuorum,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBLSSignatureCheckerTypesInstance<T, P, N> {
        IBLSSignatureCheckerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBLSSignatureCheckerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IBLSSignatureCheckerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBLSSignatureCheckerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBLSSignatureCheckerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSSignatureCheckerTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBLSSignatureCheckerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IBLSSignatureCheckerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBLSSignatureCheckerTypesInstance<T, P, N> {
            IBLSSignatureCheckerTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBLSSignatureCheckerTypesInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IBLSSignatureCheckerTypesInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IIncredibleSquaringTaskManager {
    struct Task { uint256 numberToBeSquared; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    struct TaskResponse { uint32 referenceTaskIndex; uint256 numberSquared; }
    struct TaskResponseMetadata { uint32 taskResponsedBlock; bytes32 hashOfNonSigners; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IIncredibleSquaringTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Task { uint256 numberToBeSquared; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Task {
        #[allow(missing_docs)]
        pub numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub taskCreatedBlock: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub quorumThresholdPercentage: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            alloy::sol_types::private::Bytes,
            u32,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Task> for UnderlyingRustTuple<'_> {
            fn from(value: Task) -> Self {
                (
                    value.numberToBeSquared,
                    value.taskCreatedBlock,
                    value.quorumNumbers,
                    value.quorumThresholdPercentage,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Task {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    numberToBeSquared: tuple.0,
                    taskCreatedBlock: tuple.1,
                    quorumNumbers: tuple.2,
                    quorumThresholdPercentage: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Task {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Task {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.numberToBeSquared),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.taskCreatedBlock),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Task {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Task {
            const NAME: &'static str = "Task";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Task(uint256 numberToBeSquared,uint32 taskCreatedBlock,bytes quorumNumbers,uint32 quorumThresholdPercentage)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.numberToBeSquared,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskCreatedBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumNumbers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumThresholdPercentage,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Task {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numberToBeSquared,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskCreatedBlock,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumNumbers,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumThresholdPercentage,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numberToBeSquared,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.taskCreatedBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumNumbers,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumThresholdPercentage,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct TaskResponse { uint32 referenceTaskIndex; uint256 numberSquared; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    pub struct TaskResponse {
        #[allow(missing_docs)]
        pub referenceTaskIndex: u32,
        #[allow(missing_docs)]
        pub numberSquared: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TaskResponse> for UnderlyingRustTuple<'_> {
            fn from(value: TaskResponse) -> Self {
                (value.referenceTaskIndex, value.numberSquared)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponse {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    referenceTaskIndex: tuple.0,
                    numberSquared: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskResponse {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskResponse {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTaskIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.numberSquared),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TaskResponse {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for TaskResponse {
            const NAME: &'static str = "TaskResponse";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponse(uint32 referenceTaskIndex,uint256 numberSquared)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.referenceTaskIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.numberSquared)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskResponse {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.referenceTaskIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numberSquared,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.referenceTaskIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numberSquared,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct TaskResponseMetadata { uint32 taskResponsedBlock; bytes32 hashOfNonSigners; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TaskResponseMetadata {
        #[allow(missing_docs)]
        pub taskResponsedBlock: u32,
        #[allow(missing_docs)]
        pub hashOfNonSigners: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TaskResponseMetadata> for UnderlyingRustTuple<'_> {
            fn from(value: TaskResponseMetadata) -> Self {
                (value.taskResponsedBlock, value.hashOfNonSigners)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponseMetadata {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    taskResponsedBlock: tuple.0,
                    hashOfNonSigners: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskResponseMetadata {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskResponseMetadata {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.taskResponsedBlock),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hashOfNonSigners),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TaskResponseMetadata {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for TaskResponseMetadata {
            const NAME: &'static str = "TaskResponseMetadata";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponseMetadata(uint32 taskResponsedBlock,bytes32 hashOfNonSigners)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskResponsedBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hashOfNonSigners,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskResponseMetadata {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskResponsedBlock,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hashOfNonSigners,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.taskResponsedBlock,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hashOfNonSigners,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IIncredibleSquaringTaskManager`](self) contract instance.

See the [wrapper's documentation](`IIncredibleSquaringTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IIncredibleSquaringTaskManagerInstance<T, P, N> {
        IIncredibleSquaringTaskManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IIncredibleSquaringTaskManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IIncredibleSquaringTaskManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IIncredibleSquaringTaskManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug
    for IIncredibleSquaringTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IIncredibleSquaringTaskManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IIncredibleSquaringTaskManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IIncredibleSquaringTaskManager`](self) contract instance.

See the [wrapper's documentation](`IIncredibleSquaringTaskManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<
        T,
        P: ::core::clone::Clone,
        N,
    > IIncredibleSquaringTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> IIncredibleSquaringTaskManagerInstance<T, P, N> {
            IIncredibleSquaringTaskManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IIncredibleSquaringTaskManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IIncredibleSquaringTaskManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library OperatorStateRetriever {
    struct CheckSignaturesIndices { uint32[] nonSignerQuorumBitmapIndices; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct Operator { address operator; bytes32 operatorId; uint96 stake; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod OperatorStateRetriever {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct CheckSignaturesIndices { uint32[] nonSignerQuorumBitmapIndices; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckSignaturesIndices {
        #[allow(missing_docs)]
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerStakeIndices: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<u32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CheckSignaturesIndices> for UnderlyingRustTuple<'_> {
            fn from(value: CheckSignaturesIndices) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CheckSignaturesIndices {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    quorumApkIndices: tuple.1,
                    totalStakeIndices: tuple.2,
                    nonSignerStakeIndices: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CheckSignaturesIndices {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CheckSignaturesIndices {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApkIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerStakeIndices),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for CheckSignaturesIndices {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for CheckSignaturesIndices {
            const NAME: &'static str = "CheckSignaturesIndices";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CheckSignaturesIndices(uint32[] nonSignerQuorumBitmapIndices,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumApkIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerStakeIndices,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CheckSignaturesIndices {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApkIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerStakeIndices,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerQuorumBitmapIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApkIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Operator { address operator; bytes32 operatorId; uint96 stake; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Operator {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub stake: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Operator> for UnderlyingRustTuple<'_> {
            fn from(value: Operator) -> Self {
                (value.operator, value.operatorId, value.stake)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Operator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    operatorId: tuple.1,
                    stake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Operator {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Operator {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Operator {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Operator {
            const NAME: &'static str = "Operator";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Operator(address operator,bytes32 operatorId,uint96 stake)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stake)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Operator {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.stake)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stake,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OperatorStateRetrieverInstance<T, P, N> {
        OperatorStateRetrieverInstance::<T, P, N>::new(address, provider)
    }
    /**A [`OperatorStateRetriever`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OperatorStateRetriever`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OperatorStateRetrieverInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OperatorStateRetrieverInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorStateRetrieverInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OperatorStateRetrieverInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> OperatorStateRetrieverInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OperatorStateRetrieverInstance<T, P, N> {
            OperatorStateRetrieverInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OperatorStateRetrieverInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OperatorStateRetrieverInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library BN254 {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

library IBLSSignatureCheckerTypes {
    struct NonSignerStakesAndSignature {
        uint32[] nonSignerQuorumBitmapIndices;
        BN254.G1Point[] nonSignerPubkeys;
        BN254.G1Point[] quorumApks;
        BN254.G2Point apkG2;
        BN254.G1Point sigma;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct QuorumStakeTotals {
        uint96[] signedStakeForQuorum;
        uint96[] totalStakeForQuorum;
    }
}

library IIncredibleSquaringTaskManager {
    struct Task {
        uint256 numberToBeSquared;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        uint32 quorumThresholdPercentage;
    }
    struct TaskResponse {
        uint32 referenceTaskIndex;
        uint256 numberSquared;
    }
    struct TaskResponseMetadata {
        uint32 taskResponsedBlock;
        bytes32 hashOfNonSigners;
    }
}

library OperatorStateRetriever {
    struct CheckSignaturesIndices {
        uint32[] nonSignerQuorumBitmapIndices;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct Operator {
        address operator;
        bytes32 operatorId;
        uint96 stake;
    }
}

interface IncredibleSquaringTaskManager {
    error BitmapValueTooLarge();
    error BytesArrayLengthTooLong();
    error BytesArrayNotOrdered();
    error CurrentlyPaused();
    error ECAddFailed();
    error ECMulFailed();
    error ExpModFailed();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InputEmptyQuorumNumbers();
    error InputNonSignerLengthMismatch();
    error InvalidBLSPairingKey();
    error InvalidBLSSignature();
    error InvalidNewPausedStatus();
    error InvalidQuorumApkHash();
    error InvalidReferenceBlocknumber();
    error NonSignerPubkeysNotSorted();
    error OnlyPauser();
    error OnlyRegistryCoordinatorOwner();
    error OnlyUnpauser();
    error OperatorNotRegistered();
    error ScalarTooLarge();

    event Initialized(uint8 version);
    event NewTaskCreated(uint32 indexed taskIndex, IIncredibleSquaringTaskManager.Task task);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskCompleted(uint32 indexed taskIndex);
    event TaskResponded(IIncredibleSquaringTaskManager.TaskResponse taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata taskResponseMetadata);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _slashingRegistryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock);

    function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
    function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
    function WADS_TO_SLASH() external view returns (uint256);
    function aggregator() external view returns (address);
    function allTaskHashes(uint32) external view returns (bytes32);
    function allTaskResponses(uint32) external view returns (bytes32);
    function allocationManager() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
    function createNewTask(uint256 numberToBeSquared, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
    function delegation() external view returns (address);
    function generator() external view returns (address);
    function getBatchOperatorFromId(address registryCoordinator, bytes32[] memory operatorIds) external view returns (address[] memory operators);
    function getBatchOperatorId(address registryCoordinator, address[] memory operators) external view returns (bytes32[] memory operatorIds);
    function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes memory quorumNumbers, bytes32[] memory nonSignerOperatorIds) external view returns (OperatorStateRetriever.CheckSignaturesIndices memory);
    function getOperatorState(address registryCoordinator, bytes memory quorumNumbers, uint32 blockNumber) external view returns (OperatorStateRetriever.Operator[][] memory);
    function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) external view returns (uint256, OperatorStateRetriever.Operator[][] memory);
    function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] memory operatorIds, uint32 blockNumber) external view returns (uint256[] memory);
    function getTaskResponseWindowBlock() external view returns (uint32);
    function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) external;
    function instantSlasher() external view returns (address);
    function latestTaskNum() external view returns (uint32);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function raiseAndResolveChallenge(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    function registryCoordinator() external view returns (address);
    function renounceOwnership() external;
    function respondToTask(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
    function serviceManager() external view returns (address);
    function stakeRegistry() external view returns (address);
    function taskNumber() external view returns (uint32);
    function taskSuccesfullyChallenged(uint32) external view returns (bool);
    function transferOwnership(address newOwner) external;
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    function unpause(uint256 newPausedStatus) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_slashingRegistryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "_taskResponseWindowBlock",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "TASK_CHALLENGE_WINDOW_BLOCK",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "TASK_RESPONSE_WINDOW_BLOCK",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "WADS_TO_SLASH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "aggregator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allTaskHashes",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allTaskResponses",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allocationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "blsApkRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkSignatures",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.QuorumStakeTotals",
        "components": [
          {
            "name": "signedStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          },
          {
            "name": "totalStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          }
        ]
      },
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createNewTask",
    "inputs": [
      {
        "name": "numberToBeSquared",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "quorumThresholdPercentage",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "generator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBatchOperatorFromId",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBatchOperatorId",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCheckSignaturesIndices",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "nonSignerOperatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OperatorStateRetriever.CheckSignaturesIndices",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[][]",
        "internalType": "struct OperatorStateRetriever.Operator[][]",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "tuple[][]",
        "internalType": "struct OperatorStateRetriever.Operator[][]",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapsAtBlockNumber",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTaskResponseWindowBlock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_aggregator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_generator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_slasher",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "instantSlasher",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "latestTaskNum",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pauseAll",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [
      {
        "name": "index",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pauserRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "raiseAndResolveChallenge",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.Task",
        "components": [
          {
            "name": "numberToBeSquared",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "taskResponse",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numberSquared",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponseMetadata",
        "components": [
          {
            "name": "taskResponsedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "hashOfNonSigners",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "pubkeysOfNonSigningOperators",
        "type": "tuple[]",
        "internalType": "struct BN254.G1Point[]",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registryCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "respondToTask",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.Task",
        "components": [
          {
            "name": "numberToBeSquared",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "taskResponse",
        "type": "tuple",
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numberSquared",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "nonSignerStakesAndSignature",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "serviceManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakeRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "taskNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "taskSuccesfullyChallenged",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "trySignatureAndApkVerification",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "apk",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "apkG2",
        "type": "tuple",
        "internalType": "struct BN254.G2Point",
        "components": [
          {
            "name": "X",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "Y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "sigma",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "pairingSuccessful",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "siganatureIsValid",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NewTaskCreated",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "task",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleSquaringTaskManager.Task",
        "components": [
          {
            "name": "numberToBeSquared",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskChallengedSuccessfully",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskChallengedUnsuccessfully",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskCompleted",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskResponded",
    "inputs": [
      {
        "name": "taskResponse",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numberSquared",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IIncredibleSquaringTaskManager.TaskResponseMetadata",
        "components": [
          {
            "name": "taskResponsedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "hashOfNonSigners",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "BitmapValueTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayLengthTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayNotOrdered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECAddFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECMulFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpModFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputEmptyQuorumNumbers",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputNonSignerLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBLSPairingKey",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBLSSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorumApkHash",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlocknumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonSignerPubkeysNotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRegistryCoordinatorOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyUnpauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ScalarTooLarge",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IncredibleSquaringTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61014080604052346101d157606081614d688038038091610020828561027c565b8339810103126101d15780516001600160a01b038116908181036101d15760208301516001600160a01b038116938482036101d157604001519363ffffffff851685036101d1571561026d5760805260a052604051636830483560e01b8152602081600481855afa9081156101dd575f9161022a575b5060c052604051632efa2ca360e11b815290602090829060049082905afa9081156101dd575f916101e8575b5060e05260c05160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156101dd575f91610197575b506101005261012052604051614ab490816102b482396080518181816102aa015281816110e801528181611cf7015261213b015260a051818181611b24015281816134e801528181613b170152613bed015260c05181818161190101528181613e7b0152613fb7015260e0518181816118bd015281816134090152613db90152610100518161203701526101205181818161058e01526115370152f35b90506020813d6020116101d5575b816101b26020938361027c565b810103126101d157516001600160a01b03811681036101d1575f6100fa565b5f80fd5b3d91506101a5565b6040513d5f823e3d90fd5b90506020813d602011610222575b816102036020938361027c565b810103126101d157516001600160a01b03811681036101d1575f6100c2565b3d91506101f6565b90506020813d602011610265575b816102456020938361027c565b810103126101d157516001600160a01b03811681036101d1576004610096565b3d9150610238565b6339b190bb60e11b5f5260045ffd5b601f909101601f19168101906001600160401b0382119082101761029f57604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8063136439dd1461027a578063171f1d5b146102755780631ad43189146101d0578063245a7bfc146102705780632cb223d51461026b5780632d89f6fc1461026657806331b36bd9146102615780633563b0d11461025c5780633998fdd3146102575780634d2b57fe146102525780634f739f741461024d578063595c6a67146102485780635a2d7f02146102435780635ac86ab71461023e5780635baec9a0146102395780635c155662146102345780635c975abb1461022f5780635decc3f51461022a5780635df459461461022557806368304835146102205780636b532e9e1461021b5780636b92787e146102165780636d14a987146102115780636efb46361461020c578063715018a61461020757806372d18e8d146101f85780637afa1eed14610202578063886f1195146101fd5780638b00ce7c146101f85780638da5cb5b146101f35780639b290e98146101ee578063ca8aa7c7146101e9578063cc2a9a5b146101e4578063cefdc1d4146101df578063df5cf723146101da578063f2fde38b146101d5578063f5c9899d146101d0578063f63c5bab146101cb5763fabc1cbc146101c6575f80fd5b612112565b6120f7565b610572565b612066565b612022565b611ede565b611d9e565b611d76565b611d4e565b611d26565b611c97565b611ce2565b611cba565b611c3c565b611b8f565b611b0f565b6119a1565b611930565b6118ec565b6118a8565b61186a565b61184d565b6116f6565b61140c565b61115d565b611130565b6110bd565b610bda565b6109bf565b6108f8565b61087e565b6106d4565b61062c565b6105f3565b6105b2565b610500565b3461033a57602036600319011261033a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561033557610304926102f0915f91610306575b50612217565b6102ff6066548281161461222d565b614205565b005b610328915060203d60201161032e575b610320818361038d565b8101906121f4565b5f6102ea565b503d610316565b61220c565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761036d57604052565b61033e565b608081019081106001600160401b0382111761036d57604052565b90601f801991011681019081106001600160401b0382111761036d57604052565b604051906103be6101008361038d565b565b604051906103be60408361038d565b604051906103be60608361038d565b604051906103be60a08361038d565b906103be604051928361038d565b60409060e319011261033a576040519061041482610352565b60e4358252610104356020830152565b919082604091031261033a5760405161043c81610352565b6020808294803584520135910152565b9080601f8301121561033a576040519161046760408461038d565b82906040810192831161033a57905b8282106104835750505090565b8135815260209182019101610476565b90608060631983011261033a576040516104ac81610352565b60206104c782946104be81606461044c565b845260a461044c565b910152565b919060808382031261033a5760206104c7604051926104ea84610352565b604084966104f8838261044c565b86520161044c565b3461033a5761012036600319011261033a57600435604036602319011261033a57610558604091825161053281610352565b6024358152604435602082015261054836610493565b90610552366103fb565b92612281565b8251911515825215156020820152f35b5f91031261033a57565b3461033a575f36600319011261033a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461033a575f36600319011261033a5760ce546040516001600160a01b039091168152602090f35b63ffffffff81160361033a57565b35906103be826105da565b3461033a57602036600319011261033a5763ffffffff600435610615816105da565b165f5260cb602052602060405f2054604051908152f35b3461033a57602036600319011261033a5763ffffffff60043561064e816105da565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361033a57565b6001600160401b03811161036d5760051b60200190565b90602080835192838152019201905f5b8181106106aa5750505090565b825184526020938401939092019160010161069d565b9060206106d192818152019061068d565b90565b3461033a57604036600319011261033a576004356106f181610665565b602435906001600160401b03821161033a573660238301121561033a5781600401359161071d83610676565b9261072b604051948561038d565b8084526024602085019160051b8301019136831161033a57602401905b82821061076c5761076861075c86866123eb565b604051918291826106c0565b0390f35b60208091833561077b81610665565b815201910190610748565b6001600160401b03811161036d57601f01601f191660200190565b9291926107ad82610786565b916107bb604051938461038d565b82948184528183011161033a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b83831061080257505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b80821061084257505050602080600192970193019301919392906107f3565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610823565b3461033a57606036600319011261033a5760043561089b81610665565b6024356001600160401b03811161033a573660238201121561033a57610768916108d26108e49236906024816004013591016107a1565b604435916108df836105da565b612629565b6040519182916020835260208301906107d7565b3461033a575f36600319011261033a5760cd546040516001600160a01b039091168152602090f35b9080601f8301121561033a57813561093781610676565b92610945604051948561038d565b81845260208085019260051b82010192831161033a57602001905b82821061096d5750505090565b8135815260209182019101610960565b60206040818301928281528451809452019201905f5b8181106109a05750505090565b82516001600160a01b0316845260209384019390920191600101610993565b3461033a57604036600319011261033a576004356109dc81610665565b6024356001600160401b03811161033a576109fb903690600401610920565b610a058151612389565b916001600160a01b03165f5b8251811015610ab257806020610a2a610a4a93866123c8565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561033557600192610a7e915f91610a84575b50610a6f83886123c8565b6001600160a01b039091169052565b01610a11565b610aa5915060203d8111610aab575b610a9d818361038d565b8101906124b0565b5f610a64565b503d610a93565b60405180610768868261097d565b9181601f8401121561033a578235916001600160401b03831161033a576020838186019501011161033a57565b90602080835192838152019201905f5b818110610b0a5750505090565b825163ffffffff16845260209384019390920191600101610afd565b90602082526060610b74610b5f610b4984516080602088015260a0870190610aed565b6020850151868203601f19016040880152610aed565b6040840151858203601f190184870152610aed565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610bad57505050505090565b9091929394602080610bcb600193601f198682030187528951610aed565b97019301930191939290610b9e565b3461033a57608036600319011261033a57600435610bf781610665565b60243590610c04826105da565b6044356001600160401b03811161033a57610c23903690600401610ac0565b906064356001600160401b03811161033a57610c43903690600401610920565b92610c4c612953565b50604051636830483560e01b81526001600160a01b039190911691602082600481865afa918215610335575f9261109c575b50610c87612953565b916040516361c8a12f60e11b81525f8180610ca68a8c600484016129fa565b0381885afa908115610335575f91611082575b5083526040516340e03a8160e11b81526001600160a01b039190911691905f8180610ce989868d60048501612a37565b0381865afa908115610335575f91611068575b506040840152610d0b856124c5565b94606084019586525f965b60ff88169482861015610fa8575f96610d4687610d338551612389565b8b5190610d4083836123c8565b526123c8565b505f5b8351811015610f3f57610d5c81856123c8565b5160208d610d78610d6e8588516123c8565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff918216602485015216604483015281606481865afa908115610335576001610e1181938c8a8c610e1e965f94610eff575b50610dfb92610df592610de792610de2898060c01b0388161515612a9d565b612ab3565b356001600160f81b03191690565b60f81c90565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14610e2c575b600101610d49565b978b6020610e3a8b876123c8565b51610e4c610df5610de78d8b8d612ab3565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff929092166044830152816064818b5afa91821561033557610eba8c600194838d610ebf965f93610ec7575b50610ea990610eaf9394516123c8565b516123c8565b9063ffffffff169052565b612ad4565b989050610e24565b610eaf935090610ef0610ea99260203d8111610ef8575b610ee8818361038d565b810190612abf565b935090610e99565b503d610ede565b610de791945092610df592610f2d610dfb9560203d8111610f38575b610f25818361038d565b810190612a7e565b959250925092610dc3565b503d610f1b565b5096989198959095610f5081612389565b905f5b8a828210610f8057610f7a95949250610f749391505190610d4083836123c8565b50612a68565b96610d16565b90610fa2610f98610d6e83610ea989600197516123c8565b610eaf83876123c8565b01610f53565b604051632efa2ca360e11b81528491908b856020836004818e5afa90811561033557610ff5955f948593611047575b5060405163354952a360e21b81529687948593849360048501612ae2565b03916001600160a01b03165afa801561033557610768925f91611025575b50602082015260405191829182610b26565b61104191503d805f833e611039818361038d565b810190612977565b83611013565b61106191935060203d602011610aab57610a9d818361038d565b9187610fd7565b61107c91503d805f833e611039818361038d565b5f610cfc565b61109691503d805f833e611039818361038d565b5f610cb9565b6110b691925060203d602011610aab57610a9d818361038d565b905f610c7e565b3461033a575f36600319011261033a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561033557611128915f916103065750612217565b6103046141d1565b3461033a575f36600319011261033a57602060405167016345785d8a00008152f35b60ff81160361033a57565b3461033a57602036600319011261033a576020600160ff60043561118081611152565b161b806066541614604051908152f35b9081608091031261033a5790565b604090602319011261033a57602490565b9080601f8301121561033a5781356111c681610676565b926111d4604051948561038d565b81845260208085019260051b82010192831161033a57602001905b8282106111fc5750505090565b60208091833561120b816105da565b8152019101906111ef565b81601f8201121561033a57803561122c81610676565b9261123a604051948561038d565b81845260208085019260061b8401019281841161033a57602001915b838310611264575050505090565b60206040916112738486610424565b815201920191611256565b9080601f8301121561033a57813561129581610676565b926112a3604051948561038d565b81845260208085019260051b8201019183831161033a5760208201905b8382106112cf57505050505090565b81356001600160401b03811161033a576020916112f1878480948801016111af565b8152019101906112c0565b9190916101808184031261033a576113126103ae565b9281356001600160401b03811161033a578161132f9184016111af565b845260208201356001600160401b03811161033a5781611350918401611216565b602085015260408201356001600160401b03811161033a5781611374918401611216565b604085015261138681606084016104cc565b60608501526113988160e08401610424565b60808501526101208201356001600160401b03811161033a57816113bd9184016111af565b60a08501526101408201356001600160401b03811161033a57816113e29184016111af565b60c08501526101608201356001600160401b03811161033a57611405920161127e565b60e0830152565b3461033a57608036600319011261033a576004356001600160401b03811161033a5761143c903690600401611190565b6114453661119e565b906064356001600160401b03811161033a576114659036906004016112fc565b60ce549092906001600160a01b0316330361167857611488602083949301612b07565b9161158b6114996040860186612b11565b9290946114f96114ab60608901612b07565b976040516114cf816114c1602082019485612b43565b03601f19810183528261038d565b5190206114f26114de88612b07565b63ffffffff165f5260ca60205260405f2090565b5414612bca565b61152361151c61150887612b07565b63ffffffff165f5260cb60205260405f2090565b5415612c3c565b8363ffffffff43169661156d61156561155c7f000000000000000000000000000000000000000000000000000000000000000086612ccd565b63ffffffff1690565b891115612ce7565b6040516020810190611583816114c18b85612d67565b519020613a3a565b919060ff5f9616955b828110611616577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a8686866115d66115ca6103c0565b63ffffffff9094168452565b602083015260405160208101906115f2816114c1868686612e4a565b51902061160161150883612b07565b5561161160405192839283612e4a565b0390a1005b8061167261164e61164961163d61163060019688516123c8565b516001600160601b031690565b6001600160601b031690565b612d77565b61166b61163d8b6116666116308760208b01516123c8565b612db6565b1115612dd9565b01611594565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b60206040818301928281528451809452019201905f5b8181106116e05750505090565b82518452602093840193909201916001016116d3565b3461033a57606036600319011261033a5760043561171381610665565b6024356001600160401b03811161033a57611732903690600401610920565b6044359161173f836105da565b6040516361c8a12f60e11b8152906001600160a01b03165f82806117678688600484016129fa565b0381845afa918215610335575f92611831575b506117858351612389565b935f5b84518110156118235761179b81866123c8565b51906020836117ad610d6e84896123c8565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610335576001925f91611805575b50828060c01b03166117fe82896123c8565b5201611788565b61181d915060203d8111610f3857610f25818361038d565b5f6117ec565b6040518061076888826116bd565b6118469192503d805f833e611039818361038d565b905f61177a565b3461033a575f36600319011261033a576020606654604051908152f35b3461033a57602036600319011261033a5763ffffffff60043561188c816105da565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033a5760c036600319011261033a576004356001600160401b03811161033a57611960903690600401611190565b6119693661119e565b90604036606319011261033a5760a4356001600160401b03811161033a576103049261199b6064923690600401611216565b926132a5565b3461033a57606036600319011261033a576024356004356119c1826105da565b6044356001600160401b03811161033a576119e0903690600401610ac0565b60cf5491939092916001600160a01b03163303611ac05761030493611aaa93611a2a611a3193611a0e6138a5565b9586524363ffffffff16602087015263ffffffff166060860152565b36916107a1565b60408201526040516020810190611a4c816114c185856138c9565b519020611a616114de60c95463ffffffff1690565b5560c95463ffffffff16907f1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c4860405180611aa263ffffffff861694826138c9565b0390a2612c9d565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b90602080835192838152019201905f5b818110611b705750505090565b82516001600160601b0316845260209384019390920191600101611b63565b3461033a57608036600319011261033a576004356024356001600160401b03811161033a57611bc2903690600401610ac0565b9091604435611bd0816105da565b606435926001600160401b03841161033a57611c3294611bf7611bfd9536906004016112fc565b93613a3a565b604051928392604084526020611c1e82516040808801526080870190611b53565b910151848203603f19016060860152611b53565b9060208301520390f35b3461033a575f36600319011261033a57611c5461487c565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461033a575f36600319011261033a57602063ffffffff60c95416604051908152f35b3461033a575f36600319011261033a5760cf546040516001600160a01b039091168152602090f35b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033a575f36600319011261033a576033546040516001600160a01b039091168152602090f35b3461033a575f36600319011261033a5760d0546040516001600160a01b039091168152602090f35b3461033a575f36600319011261033a5760d1546040516001600160a01b039091168152602090f35b3461033a5760c036600319011261033a57600435611dbb81610665565b611e3b602435611dca81610665565b604435611dd681610665565b606435611de281610665565b60843591611def83610665565b60a43593611dfc85610665565b5f5496611e2160ff60088a901c16158099819a611eb9575b8115611e99575b50614105565b87611e32600160ff195f5416175f55565b611e8257614168565b611e4157005b611e4f61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498908060208101611611565b611e9461010061ff00195f5416175f55565b614168565b303b15915081611eab575b505f611e1b565b60ff1660011490505f611ea4565b600160ff8216109150611e14565b6040906106d19392815281602082015201906107d7565b3461033a57606036600319011261033a57600435611efb81610665565b602435604435611f0a816105da565b611f4b611f15612367565b9280611f20856123bb565b526040516361c8a12f60e11b81526001600160a01b0386169490925f918491829187600484016129fa565b0381875afa9384156103355783611f7561155c610d6e611faa986020975f91612008575b506123bb565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561033557611fd9925f91611fe9575b506001600160c01b031692611fd38461491c565b90612629565b9061076860405192839283611ec7565b612002915060203d602011610f3857610f25818361038d565b5f611fbf565b61201c91503d805f833e611039818361038d565b5f611f6f565b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033a57602036600319011261033a5760043561208381610665565b61208b61487c565b6001600160a01b038116156120a357610304906148d4565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461033a575f36600319011261033a57602060405160648152f35b3461033a57602036600319011261033a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610335575f916121d5575b506001600160a01b031633036121c65761219460665419821981161461222d565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b6121ee915060203d602011610aab57610a9d818361038d565b5f612173565b9081602091031261033a5751801515810361033a5790565b6040513d5f823e3d90fd5b1561221e57565b631d77d47760e21b5f5260045ffd5b1561223457565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b9060028110156122685760051b0190565b612243565b634e487b7160e01b5f52601260045260245ffd5b61235d61233a6123639561233461232d85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e084015261010083015261230481610120840103601f19810183528261038d565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b809661427b565b906142c1565b9261233461234f612349614323565b9461441a565b91612358614536565b61427b565b916145b0565b9091565b60408051909190612378838261038d565b6001815291601f1901366020840137565b9061239382610676565b6123a0604051918261038d565b82815280926123b1601f1991610676565b0190602036910137565b8051156122685760200190565b80518210156122685760209160051b010190565b9081602091031261033a575190565b9190916123f88351612389565b925f5b81518110156124ab5780602061242461241761244d94866123c8565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610335576001925f9161247d575b5061247682886123c8565b52016123fb565b61249e915060203d81116124a4575b612496818361038d565b8101906123dc565b5f61246b565b503d61248c565b505050565b9081602091031261033a57516106d181610665565b906124cf82610676565b6124dc604051918261038d565b828152602081936124ef601f1991610676565b0191015f5b82811061250057505050565b6060828201526020016124f4565b908151811015612268570160200190565b60208183031261033a578051906001600160401b03821161033a57019080601f8301121561033a57815161255281610676565b92612560604051948561038d565b81845260208085019260051b82010192831161033a57602001905b8282106125885750505090565b815181526020918201910161257b565b906125a282610676565b6125af604051918261038d565b82815280926125c0601f1991610676565b015f5b8181106125cf57505050565b6040519060608201918083106001600160401b0384111761036d576020926040525f81525f838201525f6040820152828286010152016125c3565b9081602091031261033a57516001600160601b038116810361033a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610335575f95612932575b50604051634f4c91e160e11b815294602086600481855afa918215610335576004965f93612910575b5060209060405197888092632efa2ca360e11b82525afa958615610335575f966128ef575b506126b785939295516124c5565b945f935b80518510156128e5576126e2610df56126d4878461250e565b516001600160f81b03191690565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610335575f946128c1575b506127328451612598565b61273c888b6123c8565b52612747878a6123c8565b505f5b84518110156128b05780602061276361278593886123c8565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610335575f92612890575b506127ab81876123c8565b518a60208a6127ba858b6123c8565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa9384156103355761284d8c8f6128486001986128599789975f92612860575b506128336128246103cf565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b6123c8565b5190610d4083836123c8565b500161274a565b61288291925060203d8111612889575b61287a818361038d565b81019061260a565b905f612818565b503d612870565b6128a991925060203d8111610aab57610a9d818361038d565b905f6127a0565b5060019096019590945091506126bb565b6128de9194503d805f833e6128d6818361038d565b81019061251f565b925f612727565b5050509350505090565b61290991965060203d602011610aab57610a9d818361038d565b945f6126a9565b602091935061292b90823d8411610aab57610a9d818361038d565b9290612684565b61294c91955060203d602011610aab57610a9d818361038d565b935f61265b565b6040519061296082610372565b606080838181528160208201528160408201520152565b60208183031261033a578051906001600160401b03821161033a57019080601f8301121561033a5781516129aa81610676565b926129b8604051948561038d565b81845260208085019260051b82010192831161033a57602001905b8282106129e05750505090565b6020809183516129ef816105da565b8152019101906129d3565b60409063ffffffff6106d19493168152816020820152019061068d565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106d195931681528160208201520191612a17565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff8114612a795760010190565b612a54565b9081602091031261033a57516001600160c01b038116810361033a5790565b15612aa457565b6325ec6c1f60e01b5f5260045ffd5b90821015612268570190565b9081602091031261033a57516106d1816105da565b5f198114612a795760010190565b91612b0060209263ffffffff92969596604086526040860191612a17565b9416910152565b356106d1816105da565b903590601e198136030182121561033a57018035906001600160401b03821161033a5760200191813603831361033a57565b602081528135602082015263ffffffff6020830135612b61816105da565b1660408201526040820135601e198336030181121561033a578201906020823592016001600160401b03831161033a57823603811361033a57612bbf6060612bb86080936106d196858488015260a0870191612a17565b95016105e8565b63ffffffff16910152565b15612bd157565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15612c4357565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b63ffffffff60019116019063ffffffff8211612a7957565b63ffffffff60649116019063ffffffff8211612a7957565b9063ffffffff8091169116019063ffffffff8211612a7957565b15612cee57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6020809163ffffffff8135612d5d816105da565b1684520135910152565b6040810192916103be9190612d49565b90606482029180830460641490151715612a7957565b90600682029180830460061490151715612a7957565b81810292918115918404141715612a7957565b906001600160601b03809116911602906001600160601b038216918203612a7957565b15612de057565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091612e60846080810197612d49565b63ffffffff81511660408501520151910152565b15612e7b57565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b9092916020606091612ee0846080810197612d49565b63ffffffff8135612ef0816105da565b1660408501520135910152565b15612f0457565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15612f7657565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b15612ff457565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106130895750505090565b825184526020938401939092019160010161307c565b156130a657565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b60208183031261033a578051906001600160401b03821161033a57019080601f8301121561033a57815161315d81610676565b9261316b604051948561038d565b81845260208085019260051b82010192831161033a57602001905b8282106131935750505090565b6020809183516131a281610665565b815201910190613186565b604051906131bc60408361038d565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b8181106132865750505060806132716106d194956060850151601f19858303018486015261068d565b9201519060a0601f19828503019101526131dd565b82516001600160a01b0316875260209687019690920191600101613248565b90929160016132b385612b07565b94602061336f85356132de6132d68a63ffffffff165f5260cb60205260405f2090565b541515612e74565b6133196132f98a63ffffffff165f5260cb60205260405f2090565b5460405185810190613310816114c18c8b86612eca565b51902014612efd565b61334461333e6133378b63ffffffff165f5260cc60205260405f2090565b5460ff1690565b15612f6f565b61336961335b61155c61335689612b07565b612cb5565b63ffffffff43161115612fed565b80612da3565b9101351414613874576133828351612389565b935f5b84518110156133c257806133b161339e600193886123c8565b5180515f526020015160205260405f2090565b6133bb82896123c8565b5201613385565b5090929391946133fc602087019460206133db87612b07565b6040516133f0816114c18a868301958661305f565b5190209101351461309f565b6134068551612389565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b87518110156134b65780602061344d61346d93896123c8565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561033557600192613492915f91613498575b50610a6f838d6123c8565b01613434565b6134b0915060203d8111610aab57610a9d818361038d565b5f613487565b50929695509250926135156134de6134e660408601946134d68688612b11565b939091612b07565b9236916107a1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612629565b945f915b8651831015613816575f979697955b613532848a6123c8565b51518710156138065797613573986020806135518a610ea989876123c8565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a15610335575f9a6137e6575b506001985f5b85518110156137d8576135b16135a561241783896123c8565b6001600160a01b031690565b6001600160a01b038d16146135c85760010161358c565b5099909791985060015f5b1515146135e8575b5060010195979697613528565b98969786985f8795939861364d60ff613625610df5610de78c61361f6136a29f61361960cd5460018060a01b031690565b98612b11565b90612ab3565b61363f6136306103c0565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d154613664906135a5906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa968715610335575f976137b4575b506136bf8751612389565b985f5b8a518110156136e8578067016345785d8a00006136e16001938e6123c8565b52016136c2565b509a92949661372660ff61370d610df5610de78f9d979f969e9661361f8e8e92612b11565b6137186128246103de565b1663ffffffff166020860152565b604084015260608301526137386131ad565b608083015260d054613754906135a5906001600160a01b031681565b803b1561033a57604051636a669b4160e01b8152925f9184918290849082906137809060048301613201565b03925af19182156103355760019261379a575b50906135db565b806137a85f6137ae9361038d565b80610568565b5f613793565b6137d19197503d805f833e6137c9818361038d565b81019061312a565b955f6136b4565b5099909791986001906135d3565b6137ff919a5060203d8111610aab57610a9d818361038d565b985f613586565b9697969550600190920191613519565b50505050509190506138466138398263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b604051906138b282610372565b5f6060838281528260208201528160408201520152565b602081528151602082015263ffffffff6020830151166040820152608063ffffffff60606139046040860151848387015260a08601906131dd565b9401511691015290565b6040519061391b82610352565b60606020838281520152565b1561392e57565b62f8202d60e51b5f5260045ffd5b1561394357565b6343714afd60e01b5f5260045ffd5b1561395957565b635f832f4160e01b5f5260045ffd5b1561396f57565b634b874f4560e01b5f5260045ffd5b9081602091031261033a57516106d181611152565b5f19810191908211612a7957565b156139a857565b633fdc650560e21b5f5260045ffd5b9081602091031261033a575167ffffffffffffffff198116810361033a5790565b156139df57565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b038211612a7957565b15613a1557565b6367988d3360e01b5f5260045ffd5b15613a2b57565b63ab1b236b60e01b5f5260045ffd5b949392909193613a4861390e565b50613a54851515613927565b6040840151518514806140f7575b806140e9575b806140db575b613a779061393c565b613a8960208501515185515114613952565b613aa063ffffffff431663ffffffff841610613968565b613aa86103c0565b5f81525f602082015292613aba61390e565b613ac387612389565b6020820152613ad187612389565b8152613adb61390e565b92613aea602088015151612389565b8452613afa602088015151612389565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561033557613b63915f916140ac575b50613b5e368b876107a1565b6146de565b985f965b60208901518051891015613cc257602088613bb7610d6e8c613baf8f96868e613b9461339e8680956123c8565b613ba184848401516123c8565b5282613c8f575b01516123c8565b5195516123c8565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610335576123348a613c648f613c5d8f8460208f92613c5493613c4c8460019e613c6a9e5f91613c72575b508f8060c01b031692516123c8565b5201516123c8565b51938d516123c8565b5116614709565b9061473a565b970196613b67565b613c899150863d8111610f3857610f25818361038d565b5f613c3d565b613cbd613c9f84848401516123c8565b51613cb684840151613cb087613993565b906123c8565b51106139a1565b613ba8565b50909597949650613cd79198939299506147f7565b915f905b808210613d2f5750505092613d16613d11613d0a613d2995856114c19860806060602099015192015192612281565b9190613a0e565b613a24565b015160405192839160208301958661305f565b51902090565b9184959693613d6c610df5610de78587613d66879f989e9b61339e846040613d589301516123c8565b67ffffffffffffffff191690565b96612ab3565b602087613d80610d6e8d60a08d01516123c8565b604051631a2f32ab60e21b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa92831561033557613e1c93613e07925f9161407e575b5067ffffffffffffffff199182169116146139d8565b613e158960408901516123c8565b51906142c1565b90613e2e610df5610de78a868d612ab3565b602086613e42610d6e8c60c08c01516123c8565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa8015610335576116308a60208e613ba8613ed895613ee3975f91614061575b50613eca85858501516123c8565b906001600160601b03169052565b613eca8a8d516123c8565b5f965f5b60208801515181101561404d57898b613f23613f04848a516123c8565b51613f16610df5610de7868c87612ab3565b60ff161c60019081161490565b613f32575b5050600101613ee7565b8888613fb3613f73610d6e879f8f978060e08f849c6020613f62610df5610de7839f98613f6a96610ea99a612ab3565b9a01516123c8565b519a01516123c8565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830195909552919093166064840152829081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103355761401c8e6001948e86955f92614027575b50614016613eca9293519361401161163084876123c8565b6139ee565b926123c8565b01989050898b613f28565b613eca92506140466140169160203d81116128895761287a818361038d565b9250613ff9565b509594976001919993929497500190613cdb565b6140789150843d81116128895761287a818361038d565b5f613ebc565b61409f915060203d81116140a5575b614097818361038d565b8101906139b7565b5f613df1565b503d61408d565b6140ce915060203d6020116140d4575b6140c6818361038d565b81019061397e565b5f613b52565b503d6140bc565b5060e0840151518514613a6e565b5060c0840151518514613a68565b5060a0840151518514613a62565b1561410c57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b614171906148d4565b60ce80546001600160a01b03199081166001600160a01b039384161790915560cf805482169383169390931790925560d1805483169382169390931790925560d0805482169383169390931790925560cd80549092169216919091179055565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6040519061424482610352565b5f6020838281520152565b60405190610180614260818461038d565b368337565b6040519061427460208361038d565b6020368337565b9190604090606061428a614237565b948592602085519261429c858561038d565b8436853780518452015160208301528482015260076107cf195a01fa156142bf57565bfe5b6020929160806040926142d2614237565b958693818651936142e3868661038d565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa80156142bf571561431457565b63d4b68fd760e01b5f5260045ffd5b60405161432f81610352565b604090815161433e838261038d565b8236823781526020825191614353848461038d565b8336843701528051614365828261038d565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed60208201528151906143bb838361038d565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208301526144108351938461038d565b8252602082015290565b5f516020614a5f5f395f51905f5290614431614237565b505f919006602060c0835b614531575f935f516020614a5f5f395f51905f5260038186818180090908604051614467858261038d565b8436823784818560405161447b828261038d565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614a5f5f395f51905f5260a082015260056107cf195a01fa80156142bf576144e590614a48565b5191614531575f516020614a5f5f395f51905f528280091461451c57505f516020614a5f5f395f51905f5260015f9408929361443c565b929350506145286103c0565b92835282015290565b61226d565b61453e614237565b5060405161454b81610352565b600181526002602082015290565b9060018201809211612a7957565b9060028201809211612a7957565b9060038201809211612a7957565b9060048201809211612a7957565b9060058201809211612a7957565b90600c8110156122685760051b0190565b939290916145be60406103ed565b94855260208501526145d060406103ed565b91825260208201526145e061424f565b925f5b6002811061460d575050506020610180926145fc614265565b93849160086201d4c0fa9151151590565b80614619600192612d8d565b6146238285612257565b515161462f828961459f565b52602061463c8386612257565b51015161465161464b83614559565b8961459f565b5261465c8286612257565b51515161466b61464b83614567565b526146816146798387612257565b515160200190565b5161468e61464b83614575565b52602061469b8387612257565b510151516146ab61464b83614583565b526146d76146d16146ca60206146c1868a612257565b51015160200190565b5192614591565b8861459f565b52016145e3565b9060016146ec60ff936149d0565b928392161b11156146fa5790565b63ca95733360e01b5f5260045ffd5b805f915b614715575090565b5f198101818111612a795761ffff9116911661ffff8114612a7957600101908061470d565b90614743614237565b5061ffff8116906102008210156147e857600182146147e3576147646103c0565b5f81525f602082015292906001905f925b61ffff831685101561478957505050505090565b600161ffff831660ff86161c8116146147c3575b60016147b96147ae8360ff946142c1565b9460011b61fffe1690565b9401169291614775565b9460016147b96147ae6147d88960ff956142c1565b98935050505061479d565b505090565b637fc4ea7d60e11b5f5260045ffd5b6147ff614237565b50805190811580614870575b1561482c57505060405161482060408261038d565b5f81525f602082015290565b60205f516020614a5f5f395f51905f52910151065f516020614a5f5f395f51905f52035f516020614a5f5f395f51905f528111612a79576040519161441083610352565b5060208101511561480b565b6033546001600160a01b0316330361489057565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff61492882614709565b1661493281610786565b90614940604051928361038d565b80825261494f601f1991610786565b013660208301375f5f5b82518210806149af575b156149a8576001811b8416614981575b61497c90612ad4565b614959565b90600161497c9160ff60f81b8460f81b165f1a61499e828761250e565b5301919050614973565b5050905090565b506101008110614963565b156149c157565b631019106960e31b5f5260045ffd5b90610100825111614a3957815115614a3457602082015160019060f81c81901b5b8351821015614a2f57600190614a1a614a10610df56126d4868961250e565b60ff600191161b90565b90614a268183116149ba565b179101906149f1565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614a4f57565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212200f1c7f3d025b2713afffec7bd7b2670675509f54e75c92f2f9e7f71729622d8b64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@\x80`@R4a\x01\xD1W``\x81aMh\x808\x03\x80\x91a\0 \x82\x85a\x02|V[\x839\x81\x01\x03\x12a\x01\xD1W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x01\xD1W` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x84\x82\x03a\x01\xD1W`@\x01Q\x93c\xFF\xFF\xFF\xFF\x85\x16\x85\x03a\x01\xD1W\x15a\x02mW`\x80R`\xA0R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x01\xDDW_\x91a\x02*W[P`\xC0R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x01\xDDW_\x91a\x01\xE8W[P`\xE0R`\xC0Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x01\xDDW_\x91a\x01\x97W[Pa\x01\0Ra\x01 R`@QaJ\xB4\x90\x81a\x02\xB4\x829`\x80Q\x81\x81\x81a\x02\xAA\x01R\x81\x81a\x10\xE8\x01R\x81\x81a\x1C\xF7\x01Ra!;\x01R`\xA0Q\x81\x81\x81a\x1B$\x01R\x81\x81a4\xE8\x01R\x81\x81a;\x17\x01Ra;\xED\x01R`\xC0Q\x81\x81\x81a\x19\x01\x01R\x81\x81a>{\x01Ra?\xB7\x01R`\xE0Q\x81\x81\x81a\x18\xBD\x01R\x81\x81a4\t\x01Ra=\xB9\x01Ra\x01\0Q\x81a 7\x01Ra\x01 Q\x81\x81\x81a\x05\x8E\x01Ra\x157\x01R\xF3[\x90P` \x81=` \x11a\x01\xD5W[\x81a\x01\xB2` \x93\x83a\x02|V[\x81\x01\x03\x12a\x01\xD1WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xD1W_a\0\xFAV[_\x80\xFD[=\x91Pa\x01\xA5V[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x02\"W[\x81a\x02\x03` \x93\x83a\x02|V[\x81\x01\x03\x12a\x01\xD1WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xD1W_a\0\xC2V[=\x91Pa\x01\xF6V[\x90P` \x81=` \x11a\x02eW[\x81a\x02E` \x93\x83a\x02|V[\x81\x01\x03\x12a\x01\xD1WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xD1W`\x04a\0\x96V[=\x91Pa\x028V[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\x9FW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02zW\x80c\x17\x1F\x1D[\x14a\x02uW\x80c\x1A\xD41\x89\x14a\x01\xD0W\x80c$Z{\xFC\x14a\x02pW\x80c,\xB2#\xD5\x14a\x02kW\x80c-\x89\xF6\xFC\x14a\x02fW\x80c1\xB3k\xD9\x14a\x02aW\x80c5c\xB0\xD1\x14a\x02\\W\x80c9\x98\xFD\xD3\x14a\x02WW\x80cM+W\xFE\x14a\x02RW\x80cOs\x9Ft\x14a\x02MW\x80cY\\jg\x14a\x02HW\x80cZ-\x7F\x02\x14a\x02CW\x80cZ\xC8j\xB7\x14a\x02>W\x80c[\xAE\xC9\xA0\x14a\x029W\x80c\\\x15Vb\x14a\x024W\x80c\\\x97Z\xBB\x14a\x02/W\x80c]\xEC\xC3\xF5\x14a\x02*W\x80c]\xF4YF\x14a\x02%W\x80ch0H5\x14a\x02 W\x80ckS.\x9E\x14a\x02\x1BW\x80ck\x92x~\x14a\x02\x16W\x80cm\x14\xA9\x87\x14a\x02\x11W\x80cn\xFBF6\x14a\x02\x0CW\x80cqP\x18\xA6\x14a\x02\x07W\x80cr\xD1\x8E\x8D\x14a\x01\xF8W\x80cz\xFA\x1E\xED\x14a\x02\x02W\x80c\x88o\x11\x95\x14a\x01\xFDW\x80c\x8B\0\xCE|\x14a\x01\xF8W\x80c\x8D\xA5\xCB[\x14a\x01\xF3W\x80c\x9B)\x0E\x98\x14a\x01\xEEW\x80c\xCA\x8A\xA7\xC7\x14a\x01\xE9W\x80c\xCC*\x9A[\x14a\x01\xE4W\x80c\xCE\xFD\xC1\xD4\x14a\x01\xDFW\x80c\xDF\\\xF7#\x14a\x01\xDAW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xD5W\x80c\xF5\xC9\x89\x9D\x14a\x01\xD0W\x80c\xF6<[\xAB\x14a\x01\xCBWc\xFA\xBC\x1C\xBC\x14a\x01\xC6W_\x80\xFD[a!\x12V[a \xF7V[a\x05rV[a fV[a \"V[a\x1E\xDEV[a\x1D\x9EV[a\x1DvV[a\x1DNV[a\x1D&V[a\x1C\x97V[a\x1C\xE2V[a\x1C\xBAV[a\x1C<V[a\x1B\x8FV[a\x1B\x0FV[a\x19\xA1V[a\x190V[a\x18\xECV[a\x18\xA8V[a\x18jV[a\x18MV[a\x16\xF6V[a\x14\x0CV[a\x11]V[a\x110V[a\x10\xBDV[a\x0B\xDAV[a\t\xBFV[a\x08\xF8V[a\x08~V[a\x06\xD4V[a\x06,V[a\x05\xF3V[a\x05\xB2V[a\x05\0V[4a\x03:W` 6`\x03\x19\x01\x12a\x03:W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x035Wa\x03\x04\x92a\x02\xF0\x91_\x91a\x03\x06W[Pa\"\x17V[a\x02\xFF`fT\x82\x81\x16\x14a\"-V[aB\x05V[\0[a\x03(\x91P` =` \x11a\x03.W[a\x03 \x81\x83a\x03\x8DV[\x81\x01\x90a!\xF4V[_a\x02\xEAV[P=a\x03\x16V[a\"\x0CV[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03mW`@RV[a\x03>V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03mW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03mW`@RV[`@Q\x90a\x03\xBEa\x01\0\x83a\x03\x8DV[V[`@Q\x90a\x03\xBE`@\x83a\x03\x8DV[`@Q\x90a\x03\xBE``\x83a\x03\x8DV[`@Q\x90a\x03\xBE`\xA0\x83a\x03\x8DV[\x90a\x03\xBE`@Q\x92\x83a\x03\x8DV[`@\x90`\xE3\x19\x01\x12a\x03:W`@Q\x90a\x04\x14\x82a\x03RV[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03:W`@Qa\x04<\x81a\x03RV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03:W`@Q\x91a\x04g`@\x84a\x03\x8DV[\x82\x90`@\x81\x01\x92\x83\x11a\x03:W\x90[\x82\x82\x10a\x04\x83WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04vV[\x90`\x80`c\x19\x83\x01\x12a\x03:W`@Qa\x04\xAC\x81a\x03RV[` a\x04\xC7\x82\x94a\x04\xBE\x81`da\x04LV[\x84R`\xA4a\x04LV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03:W` a\x04\xC7`@Q\x92a\x04\xEA\x84a\x03RV[`@\x84\x96a\x04\xF8\x83\x82a\x04LV[\x86R\x01a\x04LV[4a\x03:Wa\x01 6`\x03\x19\x01\x12a\x03:W`\x045`@6`#\x19\x01\x12a\x03:Wa\x05X`@\x91\x82Qa\x052\x81a\x03RV[`$5\x81R`D5` \x82\x01Ra\x05H6a\x04\x93V[\x90a\x05R6a\x03\xFBV[\x92a\"\x81V[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03:WV[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03:WV[5\x90a\x03\xBE\x82a\x05\xDAV[4a\x03:W` 6`\x03\x19\x01\x12a\x03:Wc\xFF\xFF\xFF\xFF`\x045a\x06\x15\x81a\x05\xDAV[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03:W` 6`\x03\x19\x01\x12a\x03:Wc\xFF\xFF\xFF\xFF`\x045a\x06N\x81a\x05\xDAV[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03:WV[`\x01`\x01`@\x1B\x03\x81\x11a\x03mW`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xAAWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\x9DV[\x90` a\x06\xD1\x92\x81\x81R\x01\x90a\x06\x8DV[\x90V[4a\x03:W`@6`\x03\x19\x01\x12a\x03:W`\x045a\x06\xF1\x81a\x06eV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W6`#\x83\x01\x12\x15a\x03:W\x81`\x04\x015\x91a\x07\x1D\x83a\x06vV[\x92a\x07+`@Q\x94\x85a\x03\x8DV[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03:W`$\x01\x90[\x82\x82\x10a\x07lWa\x07ha\x07\\\x86\x86a#\xEBV[`@Q\x91\x82\x91\x82a\x06\xC0V[\x03\x90\xF3[` \x80\x91\x835a\x07{\x81a\x06eV[\x81R\x01\x91\x01\x90a\x07HV[`\x01`\x01`@\x1B\x03\x81\x11a\x03mW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xAD\x82a\x07\x86V[\x91a\x07\xBB`@Q\x93\x84a\x03\x8DV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03:W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x08\x02WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x08BWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x07\xF3V[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08#V[4a\x03:W``6`\x03\x19\x01\x12a\x03:W`\x045a\x08\x9B\x81a\x06eV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03:W6`#\x82\x01\x12\x15a\x03:Wa\x07h\x91a\x08\xD2a\x08\xE4\x926\x90`$\x81`\x04\x015\x91\x01a\x07\xA1V[`D5\x91a\x08\xDF\x83a\x05\xDAV[a&)V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xD7V[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x815a\t7\x81a\x06vV[\x92a\tE`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a\tmWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\t`V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\t\xA0WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\t\x93V[4a\x03:W`@6`\x03\x19\x01\x12a\x03:W`\x045a\t\xDC\x81a\x06eV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\t\xFB\x906\x90`\x04\x01a\t V[a\n\x05\x81Qa#\x89V[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\n\xB2W\x80` a\n*a\nJ\x93\x86a#\xC8V[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x035W`\x01\x92a\n~\x91_\x91a\n\x84W[Pa\no\x83\x88a#\xC8V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\n\x11V[a\n\xA5\x91P` =\x81\x11a\n\xABW[a\n\x9D\x81\x83a\x03\x8DV[\x81\x01\x90a$\xB0V[_a\ndV[P=a\n\x93V[`@Q\x80a\x07h\x86\x82a\t}V[\x91\x81`\x1F\x84\x01\x12\x15a\x03:W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03:W` \x83\x81\x86\x01\x95\x01\x01\x11a\x03:WV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\nWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\xFDV[\x90` \x82R``a\x0Bta\x0B_a\x0BI\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\n\xEDV[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\n\xEDV[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\n\xEDV[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0B\xADWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0B\xCB`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\n\xEDV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0B\x9EV[4a\x03:W`\x806`\x03\x19\x01\x12a\x03:W`\x045a\x0B\xF7\x81a\x06eV[`$5\x90a\x0C\x04\x82a\x05\xDAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x0C#\x906\x90`\x04\x01a\n\xC0V[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x0CC\x906\x90`\x04\x01a\t V[\x92a\x0CLa)SV[P`@Qch0H5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91` \x82`\x04\x81\x86Z\xFA\x91\x82\x15a\x035W_\x92a\x10\x9CW[Pa\x0C\x87a)SV[\x91`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a\x0C\xA6\x8A\x8C`\x04\x84\x01a)\xFAV[\x03\x81\x88Z\xFA\x90\x81\x15a\x035W_\x91a\x10\x82W[P\x83R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90_\x81\x80a\x0C\xE9\x89\x86\x8D`\x04\x85\x01a*7V[\x03\x81\x86Z\xFA\x90\x81\x15a\x035W_\x91a\x10hW[P`@\x84\x01Ra\r\x0B\x85a$\xC5V[\x94``\x84\x01\x95\x86R_\x96[`\xFF\x88\x16\x94\x82\x86\x10\x15a\x0F\xA8W_\x96a\rF\x87a\r3\x85Qa#\x89V[\x8BQ\x90a\r@\x83\x83a#\xC8V[Ra#\xC8V[P_[\x83Q\x81\x10\x15a\x0F?Wa\r\\\x81\x85a#\xC8V[Q` \x8Da\rxa\rn\x85\x88Qa#\xC8V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81\x86Z\xFA\x90\x81\x15a\x035W`\x01a\x0E\x11\x81\x93\x8C\x8A\x8Ca\x0E\x1E\x96_\x94a\x0E\xFFW[Pa\r\xFB\x92a\r\xF5\x92a\r\xE7\x92a\r\xE2\x89\x80`\xC0\x1B\x03\x88\x16\x15\x15a*\x9DV[a*\xB3V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a\x0E,W[`\x01\x01a\rIV[\x97\x8B` a\x0E:\x8B\x87a#\xC8V[Qa\x0ELa\r\xF5a\r\xE7\x8D\x8B\x8Da*\xB3V[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81\x8BZ\xFA\x91\x82\x15a\x035Wa\x0E\xBA\x8C`\x01\x94\x83\x8Da\x0E\xBF\x96_\x93a\x0E\xC7W[Pa\x0E\xA9\x90a\x0E\xAF\x93\x94Qa#\xC8V[Qa#\xC8V[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a*\xD4V[\x98\x90Pa\x0E$V[a\x0E\xAF\x93P\x90a\x0E\xF0a\x0E\xA9\x92` =\x81\x11a\x0E\xF8W[a\x0E\xE8\x81\x83a\x03\x8DV[\x81\x01\x90a*\xBFV[\x93P\x90a\x0E\x99V[P=a\x0E\xDEV[a\r\xE7\x91\x94P\x92a\r\xF5\x92a\x0F-a\r\xFB\x95` =\x81\x11a\x0F8W[a\x0F%\x81\x83a\x03\x8DV[\x81\x01\x90a*~V[\x95\x92P\x92P\x92a\r\xC3V[P=a\x0F\x1BV[P\x96\x98\x91\x98\x95\x90\x95a\x0FP\x81a#\x89V[\x90_[\x8A\x82\x82\x10a\x0F\x80Wa\x0Fz\x95\x94\x92Pa\x0Ft\x93\x91PQ\x90a\r@\x83\x83a#\xC8V[Pa*hV[\x96a\r\x16V[\x90a\x0F\xA2a\x0F\x98a\rn\x83a\x0E\xA9\x89`\x01\x97Qa#\xC8V[a\x0E\xAF\x83\x87a#\xC8V[\x01a\x0FSV[`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x84\x91\x90\x8B\x85` \x83`\x04\x81\x8EZ\xFA\x90\x81\x15a\x035Wa\x0F\xF5\x95_\x94\x85\x93a\x10GW[P`@Qc5IR\xA3`\xE2\x1B\x81R\x96\x87\x94\x85\x93\x84\x93`\x04\x85\x01a*\xE2V[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x035Wa\x07h\x92_\x91a\x10%W[P` \x82\x01R`@Q\x91\x82\x91\x82a\x0B&V[a\x10A\x91P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[\x81\x01\x90a)wV[\x83a\x10\x13V[a\x10a\x91\x93P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x91\x87a\x0F\xD7V[a\x10|\x91P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[_a\x0C\xFCV[a\x10\x96\x91P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[_a\x0C\xB9V[a\x10\xB6\x91\x92P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x90_a\x0C~V[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x035Wa\x11(\x91_\x91a\x03\x06WPa\"\x17V[a\x03\x04aA\xD1V[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x03:WV[4a\x03:W` 6`\x03\x19\x01\x12a\x03:W` `\x01`\xFF`\x045a\x11\x80\x81a\x11RV[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[\x90\x81`\x80\x91\x03\x12a\x03:W\x90V[`@\x90`#\x19\x01\x12a\x03:W`$\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x815a\x11\xC6\x81a\x06vV[\x92a\x11\xD4`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a\x11\xFCWPPP\x90V[` \x80\x91\x835a\x12\x0B\x81a\x05\xDAV[\x81R\x01\x91\x01\x90a\x11\xEFV[\x81`\x1F\x82\x01\x12\x15a\x03:W\x805a\x12,\x81a\x06vV[\x92a\x12:`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03:W` \x01\x91[\x83\x83\x10a\x12dWPPPP\x90V[` `@\x91a\x12s\x84\x86a\x04$V[\x81R\x01\x92\x01\x91a\x12VV[\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x815a\x12\x95\x81a\x06vV[\x92a\x12\xA3`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03:W` \x82\x01\x90[\x83\x82\x10a\x12\xCFWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03:W` \x91a\x12\xF1\x87\x84\x80\x94\x88\x01\x01a\x11\xAFV[\x81R\x01\x91\x01\x90a\x12\xC0V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03:Wa\x13\x12a\x03\xAEV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13/\x91\x84\x01a\x11\xAFV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13P\x91\x84\x01a\x12\x16V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13t\x91\x84\x01a\x12\x16V[`@\x85\x01Ra\x13\x86\x81``\x84\x01a\x04\xCCV[``\x85\x01Ra\x13\x98\x81`\xE0\x84\x01a\x04$V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13\xBD\x91\x84\x01a\x11\xAFV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13\xE2\x91\x84\x01a\x11\xAFV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x14\x05\x92\x01a\x12~V[`\xE0\x83\x01RV[4a\x03:W`\x806`\x03\x19\x01\x12a\x03:W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x14<\x906\x90`\x04\x01a\x11\x90V[a\x14E6a\x11\x9EV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x14e\x906\x90`\x04\x01a\x12\xFCV[`\xCET\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x16xWa\x14\x88` \x83\x94\x93\x01a+\x07V[\x91a\x15\x8Ba\x14\x99`@\x86\x01\x86a+\x11V[\x92\x90\x94a\x14\xF9a\x14\xAB``\x89\x01a+\x07V[\x97`@Qa\x14\xCF\x81a\x14\xC1` \x82\x01\x94\x85a+CV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\x8DV[Q\x90 a\x14\xF2a\x14\xDE\x88a+\x07V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14a+\xCAV[a\x15#a\x15\x1Ca\x15\x08\x87a+\x07V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15a,<V[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x15ma\x15ea\x15\\\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a,\xCDV[c\xFF\xFF\xFF\xFF\x16\x90V[\x89\x11\x15a,\xE7V[`@Q` \x81\x01\x90a\x15\x83\x81a\x14\xC1\x8B\x85a-gV[Q\x90 a::V[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x16\x16W\x7F4\x9C\x1E\xE6\x0EN\x89r\xEE\x9D\xBAd,\x17tT=\\A6\x87\x9B\x7FL\xAA\xF0K\xF8\x1AHz*\x86\x86\x86a\x15\xD6a\x15\xCAa\x03\xC0V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x15\xF2\x81a\x14\xC1\x86\x86\x86a.JV[Q\x90 a\x16\x01a\x15\x08\x83a+\x07V[Ua\x16\x11`@Q\x92\x83\x92\x83a.JV[\x03\x90\xA1\0[\x80a\x16ra\x16Na\x16Ia\x16=a\x160`\x01\x96\x88Qa#\xC8V[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[a-wV[a\x16ka\x16=\x8Ba\x16fa\x160\x87` \x8B\x01Qa#\xC8V[a-\xB6V[\x11\x15a-\xD9V[\x01a\x15\x94V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x16\xE0WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x16\xD3V[4a\x03:W``6`\x03\x19\x01\x12a\x03:W`\x045a\x17\x13\x81a\x06eV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x172\x906\x90`\x04\x01a\t V[`D5\x91a\x17?\x83a\x05\xDAV[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x17g\x86\x88`\x04\x84\x01a)\xFAV[\x03\x81\x84Z\xFA\x91\x82\x15a\x035W_\x92a\x181W[Pa\x17\x85\x83Qa#\x89V[\x93_[\x84Q\x81\x10\x15a\x18#Wa\x17\x9B\x81\x86a#\xC8V[Q\x90` \x83a\x17\xADa\rn\x84\x89a#\xC8V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x035W`\x01\x92_\x91a\x18\x05W[P\x82\x80`\xC0\x1B\x03\x16a\x17\xFE\x82\x89a#\xC8V[R\x01a\x17\x88V[a\x18\x1D\x91P` =\x81\x11a\x0F8Wa\x0F%\x81\x83a\x03\x8DV[_a\x17\xECV[`@Q\x80a\x07h\x88\x82a\x16\xBDV[a\x18F\x91\x92P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[\x90_a\x17zV[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` `fT`@Q\x90\x81R\xF3[4a\x03:W` 6`\x03\x19\x01\x12a\x03:Wc\xFF\xFF\xFF\xFF`\x045a\x18\x8C\x81a\x05\xDAV[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03:W`\xC06`\x03\x19\x01\x12a\x03:W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x19`\x906\x90`\x04\x01a\x11\x90V[a\x19i6a\x11\x9EV[\x90`@6`c\x19\x01\x12a\x03:W`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x03\x04\x92a\x19\x9B`d\x926\x90`\x04\x01a\x12\x16V[\x92a2\xA5V[4a\x03:W``6`\x03\x19\x01\x12a\x03:W`$5`\x045a\x19\xC1\x82a\x05\xDAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x19\xE0\x906\x90`\x04\x01a\n\xC0V[`\xCFT\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1A\xC0Wa\x03\x04\x93a\x1A\xAA\x93a\x1A*a\x1A1\x93a\x1A\x0Ea8\xA5V[\x95\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\x07\xA1V[`@\x82\x01R`@Q` \x81\x01\x90a\x1AL\x81a\x14\xC1\x85\x85a8\xC9V[Q\x90 a\x1Aaa\x14\xDE`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\x16\x95\xB8\xD0n\xC8\0\xB4a^t\\\xFB[\xD0\x0C\x1F(ua]B\x92\\;Z\xFAT;\xB2LH`@Q\x80a\x1A\xA2c\xFF\xFF\xFF\xFF\x86\x16\x94\x82a8\xC9V[\x03\x90\xA2a,\x9DV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x1BpWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1BcV[4a\x03:W`\x806`\x03\x19\x01\x12a\x03:W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x1B\xC2\x906\x90`\x04\x01a\n\xC0V[\x90\x91`D5a\x1B\xD0\x81a\x05\xDAV[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03:Wa\x1C2\x94a\x1B\xF7a\x1B\xFD\x956\x90`\x04\x01a\x12\xFCV[\x93a::V[`@Q\x92\x83\x92`@\x84R` a\x1C\x1E\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x1BSV[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x1BSV[\x90` \x83\x01R\x03\x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:Wa\x1CTaH|V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03:W`\xC06`\x03\x19\x01\x12a\x03:W`\x045a\x1D\xBB\x81a\x06eV[a\x1E;`$5a\x1D\xCA\x81a\x06eV[`D5a\x1D\xD6\x81a\x06eV[`d5a\x1D\xE2\x81a\x06eV[`\x845\x91a\x1D\xEF\x83a\x06eV[`\xA45\x93a\x1D\xFC\x85a\x06eV[_T\x96a\x1E!`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa\x1E\xB9W[\x81\x15a\x1E\x99W[PaA\x05V[\x87a\x1E2`\x01`\xFF\x19_T\x16\x17_UV[a\x1E\x82WaAhV[a\x1EAW\0[a\x1EOa\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x16\x11V[a\x1E\x94a\x01\0a\xFF\0\x19_T\x16\x17_UV[aAhV[0;\x15\x91P\x81a\x1E\xABW[P_a\x1E\x1BV[`\xFF\x16`\x01\x14\x90P_a\x1E\xA4V[`\x01`\xFF\x82\x16\x10\x91Pa\x1E\x14V[`@\x90a\x06\xD1\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xD7V[4a\x03:W``6`\x03\x19\x01\x12a\x03:W`\x045a\x1E\xFB\x81a\x06eV[`$5`D5a\x1F\n\x81a\x05\xDAV[a\x1FKa\x1F\x15a#gV[\x92\x80a\x1F \x85a#\xBBV[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a)\xFAV[\x03\x81\x87Z\xFA\x93\x84\x15a\x035W\x83a\x1Fua\x15\\a\rna\x1F\xAA\x98` \x97_\x91a \x08W[Pa#\xBBV[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x035Wa\x1F\xD9\x92_\x91a\x1F\xE9W[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1F\xD3\x84aI\x1CV[\x90a&)V[\x90a\x07h`@Q\x92\x83\x92\x83a\x1E\xC7V[a \x02\x91P` =` \x11a\x0F8Wa\x0F%\x81\x83a\x03\x8DV[_a\x1F\xBFV[a \x1C\x91P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[_a\x1FoV[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03:W` 6`\x03\x19\x01\x12a\x03:W`\x045a \x83\x81a\x06eV[a \x8BaH|V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a \xA3Wa\x03\x04\x90aH\xD4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` `@Q`d\x81R\xF3[4a\x03:W` 6`\x03\x19\x01\x12a\x03:W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x035W_\x91a!\xD5W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a!\xC6Wa!\x94`fT\x19\x82\x19\x81\x16\x14a\"-V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a!\xEE\x91P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[_a!sV[\x90\x81` \x91\x03\x12a\x03:WQ\x80\x15\x15\x81\x03a\x03:W\x90V[`@Q=_\x82>=\x90\xFD[\x15a\"\x1EWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a\"4WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\"hW`\x05\x1B\x01\x90V[a\"CV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a#]a#:a#c\x95a#4a#-\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra#\x04\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\x8DV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aB{V[\x90aB\xC1V[\x92a#4a#Oa#IaC#V[\x94aD\x1AV[\x91a#XaE6V[aB{V[\x91aE\xB0V[\x90\x91V[`@\x80Q\x90\x91\x90a#x\x83\x82a\x03\x8DV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a#\x93\x82a\x06vV[a#\xA0`@Q\x91\x82a\x03\x8DV[\x82\x81R\x80\x92a#\xB1`\x1F\x19\x91a\x06vV[\x01\x90` 6\x91\x017V[\x80Q\x15a\"hW` \x01\x90V[\x80Q\x82\x10\x15a\"hW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03:WQ\x90V[\x91\x90\x91a#\xF8\x83Qa#\x89V[\x92_[\x81Q\x81\x10\x15a$\xABW\x80` a$$a$\x17a$M\x94\x86a#\xC8V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x035W`\x01\x92_\x91a$}W[Pa$v\x82\x88a#\xC8V[R\x01a#\xFBV[a$\x9E\x91P` =\x81\x11a$\xA4W[a$\x96\x81\x83a\x03\x8DV[\x81\x01\x90a#\xDCV[_a$kV[P=a$\x8CV[PPPV[\x90\x81` \x91\x03\x12a\x03:WQa\x06\xD1\x81a\x06eV[\x90a$\xCF\x82a\x06vV[a$\xDC`@Q\x91\x82a\x03\x8DV[\x82\x81R` \x81\x93a$\xEF`\x1F\x19\x91a\x06vV[\x01\x91\x01_[\x82\x81\x10a%\0WPPPV[``\x82\x82\x01R` \x01a$\xF4V[\x90\x81Q\x81\x10\x15a\"hW\x01` \x01\x90V[` \x81\x83\x03\x12a\x03:W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x81Qa%R\x81a\x06vV[\x92a%``@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a%\x88WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a%{V[\x90a%\xA2\x82a\x06vV[a%\xAF`@Q\x91\x82a\x03\x8DV[\x82\x81R\x80\x92a%\xC0`\x1F\x19\x91a\x06vV[\x01_[\x81\x81\x10a%\xCFWPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03mW` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a%\xC3V[\x90\x81` \x91\x03\x12a\x03:WQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03:W\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x035W_\x95a)2W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x035W`\x04\x96_\x93a)\x10W[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x035W_\x96a(\xEFW[Pa&\xB7\x85\x93\x92\x95Qa$\xC5V[\x94_\x93[\x80Q\x85\x10\x15a(\xE5Wa&\xE2a\r\xF5a&\xD4\x87\x84a%\x0EV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x035W_\x94a(\xC1W[Pa'2\x84Qa%\x98V[a'<\x88\x8Ba#\xC8V[Ra'G\x87\x8Aa#\xC8V[P_[\x84Q\x81\x10\x15a(\xB0W\x80` a'ca'\x85\x93\x88a#\xC8V[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x035W_\x92a(\x90W[Pa'\xAB\x81\x87a#\xC8V[Q\x8A` \x8Aa'\xBA\x85\x8Ba#\xC8V[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x035Wa(M\x8C\x8Fa(H`\x01\x98a(Y\x97\x89\x97_\x92a(`W[Pa(3a($a\x03\xCFV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a#\xC8V[Q\x90a\r@\x83\x83a#\xC8V[P\x01a'JV[a(\x82\x91\x92P` =\x81\x11a(\x89W[a(z\x81\x83a\x03\x8DV[\x81\x01\x90a&\nV[\x90_a(\x18V[P=a(pV[a(\xA9\x91\x92P` =\x81\x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x90_a'\xA0V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa&\xBBV[a(\xDE\x91\x94P=\x80_\x83>a(\xD6\x81\x83a\x03\x8DV[\x81\x01\x90a%\x1FV[\x92_a''V[PPP\x93PPP\x90V[a)\t\x91\x96P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x94_a&\xA9V[` \x91\x93Pa)+\x90\x82=\x84\x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x92\x90a&\x84V[a)L\x91\x95P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x93_a&[V[`@Q\x90a)`\x82a\x03rV[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03:W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x81Qa)\xAA\x81a\x06vV[\x92a)\xB8`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a)\xE0WPPP\x90V[` \x80\x91\x83Qa)\xEF\x81a\x05\xDAV[\x81R\x01\x91\x01\x90a)\xD3V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xD1\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\x8DV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xD1\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a*\x17V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a*yW`\x01\x01\x90V[a*TV[\x90\x81` \x91\x03\x12a\x03:WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03:W\x90V[\x15a*\xA4WV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\"hW\x01\x90V[\x90\x81` \x91\x03\x12a\x03:WQa\x06\xD1\x81a\x05\xDAV[_\x19\x81\x14a*yW`\x01\x01\x90V[\x91a+\0` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a*\x17V[\x94\x16\x91\x01RV[5a\x06\xD1\x81a\x05\xDAV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03:W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W` \x01\x91\x816\x03\x83\x13a\x03:WV[` \x81R\x815` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x015a+a\x81a\x05\xDAV[\x16`@\x82\x01R`@\x82\x015`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03:W\x82\x01\x90` \x825\x92\x01`\x01`\x01`@\x1B\x03\x83\x11a\x03:W\x826\x03\x81\x13a\x03:Wa+\xBF``a+\xB8`\x80\x93a\x06\xD1\x96\x85\x84\x88\x01R`\xA0\x87\x01\x91a*\x17V[\x95\x01a\x05\xE8V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15a+\xD1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a,CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a*yWV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a*yWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a*yWV[\x15a,\xEEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a-]\x81a\x05\xDAV[\x16\x84R\x015\x91\x01RV[`@\x81\x01\x92\x91a\x03\xBE\x91\x90a-IV[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a*yWV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a*yWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a*yWV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a*yWV[\x15a-\xE0WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91a.`\x84`\x80\x81\x01\x97a-IV[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[\x15a.{WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x92\x91` ``\x91a.\xE0\x84`\x80\x81\x01\x97a-IV[c\xFF\xFF\xFF\xFF\x815a.\xF0\x81a\x05\xDAV[\x16`@\x85\x01R\x015\x91\x01RV[\x15a/\x04WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a/vWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a/\xF4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a0\x89WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a0|V[\x15a0\xA6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[` \x81\x83\x03\x12a\x03:W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x81Qa1]\x81a\x06vV[\x92a1k`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a1\x93WPPP\x90V[` \x80\x91\x83Qa1\xA2\x81a\x06eV[\x81R\x01\x91\x01\x90a1\x86V[`@Q\x90a1\xBC`@\x83a\x03\x8DV[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10a2\x86WPPP`\x80a2qa\x06\xD1\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x06\x8DV[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra1\xDDV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a2HV[\x90\x92\x91`\x01a2\xB3\x85a+\x07V[\x94` a3o\x855a2\xDEa2\xD6\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a.tV[a3\x19a2\xF9\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T`@Q\x85\x81\x01\x90a3\x10\x81a\x14\xC1\x8C\x8B\x86a.\xCAV[Q\x90 \x14a.\xFDV[a3Da3>a37\x8Bc\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a/oV[a3ia3[a\x15\\a3V\x89a+\x07V[a,\xB5V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a/\xEDV[\x80a-\xA3V[\x91\x015\x14\x14a8tWa3\x82\x83Qa#\x89V[\x93_[\x84Q\x81\x10\x15a3\xC2W\x80a3\xB1a3\x9E`\x01\x93\x88a#\xC8V[Q\x80Q_R` \x01Q` R`@_ \x90V[a3\xBB\x82\x89a#\xC8V[R\x01a3\x85V[P\x90\x92\x93\x91\x94a3\xFC` \x87\x01\x94` a3\xDB\x87a+\x07V[`@Qa3\xF0\x81a\x14\xC1\x8A\x86\x83\x01\x95\x86a0_V[Q\x90 \x91\x015\x14a0\x9FV[a4\x06\x85Qa#\x89V[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a4\xB6W\x80` a4Ma4m\x93\x89a#\xC8V[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x035W`\x01\x92a4\x92\x91_\x91a4\x98W[Pa\no\x83\x8Da#\xC8V[\x01a44V[a4\xB0\x91P` =\x81\x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[_a4\x87V[P\x92\x96\x95P\x92P\x92a5\x15a4\xDEa4\xE6`@\x86\x01\x94a4\xD6\x86\x88a+\x11V[\x93\x90\x91a+\x07V[\x926\x91a\x07\xA1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a&)V[\x94_\x91[\x86Q\x83\x10\x15a8\x16W_\x97\x96\x97\x95[a52\x84\x8Aa#\xC8V[QQ\x87\x10\x15a8\x06W\x97a5s\x98` \x80a5Q\x8Aa\x0E\xA9\x89\x87a#\xC8V[Q\x01Q`@Q\x80\x9C\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x99\x8A\x15a\x035W_\x9Aa7\xE6W[P`\x01\x98_[\x85Q\x81\x10\x15a7\xD8Wa5\xB1a5\xA5a$\x17\x83\x89a#\xC8V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x14a5\xC8W`\x01\x01a5\x8CV[P\x99\x90\x97\x91\x98P`\x01_[\x15\x15\x14a5\xE8W[P`\x01\x01\x95\x97\x96\x97a5(V[\x98\x96\x97\x86\x98_\x87\x95\x93\x98a6M`\xFFa6%a\r\xF5a\r\xE7\x8Ca6\x1Fa6\xA2\x9Fa6\x19`\xCDT`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a+\x11V[\x90a*\xB3V[a6?a60a\x03\xC0V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD1Ta6d\x90a5\xA5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x98\x89\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x96\x87\x15a\x035W_\x97a7\xB4W[Pa6\xBF\x87Qa#\x89V[\x98_[\x8AQ\x81\x10\x15a6\xE8W\x80g\x01cEx]\x8A\0\0a6\xE1`\x01\x93\x8Ea#\xC8V[R\x01a6\xC2V[P\x9A\x92\x94\x96a7&`\xFFa7\ra\r\xF5a\r\xE7\x8F\x9D\x97\x9F\x96\x9E\x96a6\x1F\x8E\x8E\x92a+\x11V[a7\x18a($a\x03\xDEV[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01Ra78a1\xADV[`\x80\x83\x01R`\xD0Ta7T\x90a5\xA5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03:W`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90a7\x80\x90`\x04\x83\x01a2\x01V[\x03\x92Z\xF1\x91\x82\x15a\x035W`\x01\x92a7\x9AW[P\x90a5\xDBV[\x80a7\xA8_a7\xAE\x93a\x03\x8DV[\x80a\x05hV[_a7\x93V[a7\xD1\x91\x97P=\x80_\x83>a7\xC9\x81\x83a\x03\x8DV[\x81\x01\x90a1*V[\x95_a6\xB4V[P\x99\x90\x97\x91\x98`\x01\x90a5\xD3V[a7\xFF\x91\x9AP` =\x81\x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x98_a5\x86V[\x96\x97\x96\x95P`\x01\x90\x92\x01\x91a5\x19V[PPPPP\x91\x90Pa8Fa89\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPPc\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[`@Q\x90a8\xB2\x82a\x03rV[_``\x83\x82\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81R\x81Q` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`\x80c\xFF\xFF\xFF\xFF``a9\x04`@\x86\x01Q\x84\x83\x87\x01R`\xA0\x86\x01\x90a1\xDDV[\x94\x01Q\x16\x91\x01R\x90V[`@Q\x90a9\x1B\x82a\x03RV[``` \x83\x82\x81R\x01RV[\x15a9.WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a9CWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a9YWV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a9oWV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03:WQa\x06\xD1\x81a\x11RV[_\x19\x81\x01\x91\x90\x82\x11a*yWV[\x15a9\xA8WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03:WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03:W\x90V[\x15a9\xDFWV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a*yWV[\x15a:\x15WV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a:+WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[\x94\x93\x92\x90\x91\x93a:Ha9\x0EV[Pa:T\x85\x15\x15a9'V[`@\x84\x01QQ\x85\x14\x80a@\xF7W[\x80a@\xE9W[\x80a@\xDBW[a:w\x90a9<V[a:\x89` \x85\x01QQ\x85QQ\x14a9RV[a:\xA0c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a9hV[a:\xA8a\x03\xC0V[_\x81R_` \x82\x01R\x92a:\xBAa9\x0EV[a:\xC3\x87a#\x89V[` \x82\x01Ra:\xD1\x87a#\x89V[\x81Ra:\xDBa9\x0EV[\x92a:\xEA` \x88\x01QQa#\x89V[\x84Ra:\xFA` \x88\x01QQa#\x89V[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x035Wa;c\x91_\x91a@\xACW[Pa;^6\x8B\x87a\x07\xA1V[aF\xDEV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a<\xC2W` \x88a;\xB7a\rn\x8Ca;\xAF\x8F\x96\x86\x8Ea;\x94a3\x9E\x86\x80\x95a#\xC8V[a;\xA1\x84\x84\x84\x01Qa#\xC8V[R\x82a<\x8FW[\x01Qa#\xC8V[Q\x95Qa#\xC8V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x035Wa#4\x8Aa<d\x8Fa<]\x8F\x84` \x8F\x92a<T\x93a<L\x84`\x01\x9Ea<j\x9E_\x91a<rW[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa#\xC8V[R\x01Qa#\xC8V[Q\x93\x8DQa#\xC8V[Q\x16aG\tV[\x90aG:V[\x97\x01\x96a;gV[a<\x89\x91P\x86=\x81\x11a\x0F8Wa\x0F%\x81\x83a\x03\x8DV[_a<=V[a<\xBDa<\x9F\x84\x84\x84\x01Qa#\xC8V[Qa<\xB6\x84\x84\x01Qa<\xB0\x87a9\x93V[\x90a#\xC8V[Q\x10a9\xA1V[a;\xA8V[P\x90\x95\x97\x94\x96Pa<\xD7\x91\x98\x93\x92\x99PaG\xF7V[\x91_\x90[\x80\x82\x10a=/WPPP\x92a=\x16a=\x11a=\na=)\x95\x85a\x14\xC1\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\"\x81V[\x91\x90a:\x0EV[a:$V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a0_V[Q\x90 \x90V[\x91\x84\x95\x96\x93a=la\r\xF5a\r\xE7\x85\x87a=f\x87\x9F\x98\x9E\x9Ba3\x9E\x84`@a=X\x93\x01Qa#\xC8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x96a*\xB3V[` \x87a=\x80a\rn\x8D`\xA0\x8D\x01Qa#\xC8V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x92\x83\x15a\x035Wa>\x1C\x93a>\x07\x92_\x91a@~W[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a9\xD8V[a>\x15\x89`@\x89\x01Qa#\xC8V[Q\x90aB\xC1V[\x90a>.a\r\xF5a\r\xE7\x8A\x86\x8Da*\xB3V[` \x86a>Ba\rn\x8C`\xC0\x8C\x01Qa#\xC8V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x035Wa\x160\x8A` \x8Ea;\xA8a>\xD8\x95a>\xE3\x97_\x91a@aW[Pa>\xCA\x85\x85\x85\x01Qa#\xC8V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a>\xCA\x8A\x8DQa#\xC8V[_\x96_[` \x88\x01QQ\x81\x10\x15a@MW\x89\x8Ba?#a?\x04\x84\x8AQa#\xC8V[Qa?\x16a\r\xF5a\r\xE7\x86\x8C\x87a*\xB3V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a?2W[PP`\x01\x01a>\xE7V[\x88\x88a?\xB3a?sa\rn\x87\x9F\x8F\x97\x80`\xE0\x8F\x84\x9C` a?ba\r\xF5a\r\xE7\x83\x9F\x98a?j\x96a\x0E\xA9\x9Aa*\xB3V[\x9A\x01Qa#\xC8V[Q\x9A\x01Qa#\xC8V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x95\x90\x95R\x91\x90\x93\x16`d\x84\x01R\x82\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x035Wa@\x1C\x8E`\x01\x94\x8E\x86\x95_\x92a@'W[Pa@\x16a>\xCA\x92\x93Q\x93a@\x11a\x160\x84\x87a#\xC8V[a9\xEEV[\x92a#\xC8V[\x01\x98\x90P\x89\x8Ba?(V[a>\xCA\x92Pa@Fa@\x16\x91` =\x81\x11a(\x89Wa(z\x81\x83a\x03\x8DV[\x92Pa?\xF9V[P\x95\x94\x97`\x01\x91\x99\x93\x92\x94\x97P\x01\x90a<\xDBV[a@x\x91P\x84=\x81\x11a(\x89Wa(z\x81\x83a\x03\x8DV[_a>\xBCV[a@\x9F\x91P` =\x81\x11a@\xA5W[a@\x97\x81\x83a\x03\x8DV[\x81\x01\x90a9\xB7V[_a=\xF1V[P=a@\x8DV[a@\xCE\x91P` =` \x11a@\xD4W[a@\xC6\x81\x83a\x03\x8DV[\x81\x01\x90a9~V[_a;RV[P=a@\xBCV[P`\xE0\x84\x01QQ\x85\x14a:nV[P`\xC0\x84\x01QQ\x85\x14a:hV[P`\xA0\x84\x01QQ\x85\x14a:bV[\x15aA\x0CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aAq\x90aH\xD4V[`\xCE\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xCD\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aBD\x82a\x03RV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aB`\x81\x84a\x03\x8DV[6\x837V[`@Q\x90aBt` \x83a\x03\x8DV[` 6\x837V[\x91\x90`@\x90``aB\x8AaB7V[\x94\x85\x92` \x85Q\x92aB\x9C\x85\x85a\x03\x8DV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aB\xBFWV[\xFE[` \x92\x91`\x80`@\x92aB\xD2aB7V[\x95\x86\x93\x81\x86Q\x93aB\xE3\x86\x86a\x03\x8DV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aB\xBFW\x15aC\x14WV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaC/\x81a\x03RV[`@\x90\x81QaC>\x83\x82a\x03\x8DV[\x826\x827\x81R` \x82Q\x91aCS\x84\x84a\x03\x8DV[\x836\x847\x01R\x80QaCe\x82\x82a\x03\x8DV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aC\xBB\x83\x83a\x03\x8DV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaD\x10\x83Q\x93\x84a\x03\x8DV[\x82R` \x82\x01R\x90V[_Q` aJ__9_Q\x90_R\x90aD1aB7V[P_\x91\x90\x06` `\xC0\x83[aE1W_\x93_Q` aJ__9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaDg\x85\x82a\x03\x8DV[\x846\x827\x84\x81\x85`@QaD{\x82\x82a\x03\x8DV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aJ__9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aB\xBFWaD\xE5\x90aJHV[Q\x91aE1W_Q` aJ__9_Q\x90_R\x82\x80\t\x14aE\x1CWP_Q` aJ__9_Q\x90_R`\x01_\x94\x08\x92\x93aD<V[\x92\x93PPaE(a\x03\xC0V[\x92\x83R\x82\x01R\x90V[a\"mV[aE>aB7V[P`@QaEK\x81a\x03RV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x01\x82\x01\x80\x92\x11a*yWV[\x90`\x02\x82\x01\x80\x92\x11a*yWV[\x90`\x03\x82\x01\x80\x92\x11a*yWV[\x90`\x04\x82\x01\x80\x92\x11a*yWV[\x90`\x05\x82\x01\x80\x92\x11a*yWV[\x90`\x0C\x81\x10\x15a\"hW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aE\xBE`@a\x03\xEDV[\x94\x85R` \x85\x01RaE\xD0`@a\x03\xEDV[\x91\x82R` \x82\x01RaE\xE0aBOV[\x92_[`\x02\x81\x10aF\rWPPP` a\x01\x80\x92aE\xFCaBeV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aF\x19`\x01\x92a-\x8DV[aF#\x82\x85a\"WV[QQaF/\x82\x89aE\x9FV[R` aF<\x83\x86a\"WV[Q\x01QaFQaFK\x83aEYV[\x89aE\x9FV[RaF\\\x82\x86a\"WV[QQQaFkaFK\x83aEgV[RaF\x81aFy\x83\x87a\"WV[QQ` \x01\x90V[QaF\x8EaFK\x83aEuV[R` aF\x9B\x83\x87a\"WV[Q\x01QQaF\xABaFK\x83aE\x83V[RaF\xD7aF\xD1aF\xCA` aF\xC1\x86\x8Aa\"WV[Q\x01Q` \x01\x90V[Q\x92aE\x91V[\x88aE\x9FV[R\x01aE\xE3V[\x90`\x01aF\xEC`\xFF\x93aI\xD0V[\x92\x83\x92\x16\x1B\x11\x15aF\xFAW\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aG\x15WP\x90V[_\x19\x81\x01\x81\x81\x11a*yWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a*yW`\x01\x01\x90\x80aG\rV[\x90aGCaB7V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aG\xE8W`\x01\x82\x14aG\xE3WaGda\x03\xC0V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aG\x89WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aG\xC3W[`\x01aG\xB9aG\xAE\x83`\xFF\x94aB\xC1V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aGuV[\x94`\x01aG\xB9aG\xAEaG\xD8\x89`\xFF\x95aB\xC1V[\x98\x93PPPPaG\x9DV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aG\xFFaB7V[P\x80Q\x90\x81\x15\x80aHpW[\x15aH,WPP`@QaH `@\x82a\x03\x8DV[_\x81R_` \x82\x01R\x90V[` _Q` aJ__9_Q\x90_R\x91\x01Q\x06_Q` aJ__9_Q\x90_R\x03_Q` aJ__9_Q\x90_R\x81\x11a*yW`@Q\x91aD\x10\x83a\x03RV[P` \x81\x01Q\x15aH\x0BV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aH\x90WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaI(\x82aG\tV[\x16aI2\x81a\x07\x86V[\x90aI@`@Q\x92\x83a\x03\x8DV[\x80\x82RaIO`\x1F\x19\x91a\x07\x86V[\x016` \x83\x017__[\x82Q\x82\x10\x80aI\xAFW[\x15aI\xA8W`\x01\x81\x1B\x84\x16aI\x81W[aI|\x90a*\xD4V[aIYV[\x90`\x01aI|\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaI\x9E\x82\x87a%\x0EV[S\x01\x91\x90PaIsV[PP\x90P\x90V[Pa\x01\0\x81\x10aIcV[\x15aI\xC1WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aJ9W\x81Q\x15aJ4W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aJ/W`\x01\x90aJ\x1AaJ\x10a\r\xF5a&\xD4\x86\x89a%\x0EV[`\xFF`\x01\x91\x16\x1B\x90V[\x90aJ&\x81\x83\x11aI\xBAV[\x17\x91\x01\x90aI\xF1V[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aJOWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x0F\x1C\x7F=\x02['\x13\xAF\xFF\xEC{\xD7\xB2g\x06uP\x9FT\xE7\\\x92\xF2\xF9\xE7\xF7\x17)b-\x8BdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063136439dd1461027a578063171f1d5b146102755780631ad43189146101d0578063245a7bfc146102705780632cb223d51461026b5780632d89f6fc1461026657806331b36bd9146102615780633563b0d11461025c5780633998fdd3146102575780634d2b57fe146102525780634f739f741461024d578063595c6a67146102485780635a2d7f02146102435780635ac86ab71461023e5780635baec9a0146102395780635c155662146102345780635c975abb1461022f5780635decc3f51461022a5780635df459461461022557806368304835146102205780636b532e9e1461021b5780636b92787e146102165780636d14a987146102115780636efb46361461020c578063715018a61461020757806372d18e8d146101f85780637afa1eed14610202578063886f1195146101fd5780638b00ce7c146101f85780638da5cb5b146101f35780639b290e98146101ee578063ca8aa7c7146101e9578063cc2a9a5b146101e4578063cefdc1d4146101df578063df5cf723146101da578063f2fde38b146101d5578063f5c9899d146101d0578063f63c5bab146101cb5763fabc1cbc146101c6575f80fd5b612112565b6120f7565b610572565b612066565b612022565b611ede565b611d9e565b611d76565b611d4e565b611d26565b611c97565b611ce2565b611cba565b611c3c565b611b8f565b611b0f565b6119a1565b611930565b6118ec565b6118a8565b61186a565b61184d565b6116f6565b61140c565b61115d565b611130565b6110bd565b610bda565b6109bf565b6108f8565b61087e565b6106d4565b61062c565b6105f3565b6105b2565b610500565b3461033a57602036600319011261033a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561033557610304926102f0915f91610306575b50612217565b6102ff6066548281161461222d565b614205565b005b610328915060203d60201161032e575b610320818361038d565b8101906121f4565b5f6102ea565b503d610316565b61220c565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761036d57604052565b61033e565b608081019081106001600160401b0382111761036d57604052565b90601f801991011681019081106001600160401b0382111761036d57604052565b604051906103be6101008361038d565b565b604051906103be60408361038d565b604051906103be60608361038d565b604051906103be60a08361038d565b906103be604051928361038d565b60409060e319011261033a576040519061041482610352565b60e4358252610104356020830152565b919082604091031261033a5760405161043c81610352565b6020808294803584520135910152565b9080601f8301121561033a576040519161046760408461038d565b82906040810192831161033a57905b8282106104835750505090565b8135815260209182019101610476565b90608060631983011261033a576040516104ac81610352565b60206104c782946104be81606461044c565b845260a461044c565b910152565b919060808382031261033a5760206104c7604051926104ea84610352565b604084966104f8838261044c565b86520161044c565b3461033a5761012036600319011261033a57600435604036602319011261033a57610558604091825161053281610352565b6024358152604435602082015261054836610493565b90610552366103fb565b92612281565b8251911515825215156020820152f35b5f91031261033a57565b3461033a575f36600319011261033a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461033a575f36600319011261033a5760ce546040516001600160a01b039091168152602090f35b63ffffffff81160361033a57565b35906103be826105da565b3461033a57602036600319011261033a5763ffffffff600435610615816105da565b165f5260cb602052602060405f2054604051908152f35b3461033a57602036600319011261033a5763ffffffff60043561064e816105da565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361033a57565b6001600160401b03811161036d5760051b60200190565b90602080835192838152019201905f5b8181106106aa5750505090565b825184526020938401939092019160010161069d565b9060206106d192818152019061068d565b90565b3461033a57604036600319011261033a576004356106f181610665565b602435906001600160401b03821161033a573660238301121561033a5781600401359161071d83610676565b9261072b604051948561038d565b8084526024602085019160051b8301019136831161033a57602401905b82821061076c5761076861075c86866123eb565b604051918291826106c0565b0390f35b60208091833561077b81610665565b815201910190610748565b6001600160401b03811161036d57601f01601f191660200190565b9291926107ad82610786565b916107bb604051938461038d565b82948184528183011161033a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b83831061080257505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b80821061084257505050602080600192970193019301919392906107f3565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610823565b3461033a57606036600319011261033a5760043561089b81610665565b6024356001600160401b03811161033a573660238201121561033a57610768916108d26108e49236906024816004013591016107a1565b604435916108df836105da565b612629565b6040519182916020835260208301906107d7565b3461033a575f36600319011261033a5760cd546040516001600160a01b039091168152602090f35b9080601f8301121561033a57813561093781610676565b92610945604051948561038d565b81845260208085019260051b82010192831161033a57602001905b82821061096d5750505090565b8135815260209182019101610960565b60206040818301928281528451809452019201905f5b8181106109a05750505090565b82516001600160a01b0316845260209384019390920191600101610993565b3461033a57604036600319011261033a576004356109dc81610665565b6024356001600160401b03811161033a576109fb903690600401610920565b610a058151612389565b916001600160a01b03165f5b8251811015610ab257806020610a2a610a4a93866123c8565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561033557600192610a7e915f91610a84575b50610a6f83886123c8565b6001600160a01b039091169052565b01610a11565b610aa5915060203d8111610aab575b610a9d818361038d565b8101906124b0565b5f610a64565b503d610a93565b60405180610768868261097d565b9181601f8401121561033a578235916001600160401b03831161033a576020838186019501011161033a57565b90602080835192838152019201905f5b818110610b0a5750505090565b825163ffffffff16845260209384019390920191600101610afd565b90602082526060610b74610b5f610b4984516080602088015260a0870190610aed565b6020850151868203601f19016040880152610aed565b6040840151858203601f190184870152610aed565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610bad57505050505090565b9091929394602080610bcb600193601f198682030187528951610aed565b97019301930191939290610b9e565b3461033a57608036600319011261033a57600435610bf781610665565b60243590610c04826105da565b6044356001600160401b03811161033a57610c23903690600401610ac0565b906064356001600160401b03811161033a57610c43903690600401610920565b92610c4c612953565b50604051636830483560e01b81526001600160a01b039190911691602082600481865afa918215610335575f9261109c575b50610c87612953565b916040516361c8a12f60e11b81525f8180610ca68a8c600484016129fa565b0381885afa908115610335575f91611082575b5083526040516340e03a8160e11b81526001600160a01b039190911691905f8180610ce989868d60048501612a37565b0381865afa908115610335575f91611068575b506040840152610d0b856124c5565b94606084019586525f965b60ff88169482861015610fa8575f96610d4687610d338551612389565b8b5190610d4083836123c8565b526123c8565b505f5b8351811015610f3f57610d5c81856123c8565b5160208d610d78610d6e8588516123c8565b5163ffffffff1690565b6040516304ec635160e01b8152600481019490945263ffffffff918216602485015216604483015281606481865afa908115610335576001610e1181938c8a8c610e1e965f94610eff575b50610dfb92610df592610de792610de2898060c01b0388161515612a9d565b612ab3565b356001600160f81b03191690565b60f81c90565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14610e2c575b600101610d49565b978b6020610e3a8b876123c8565b51610e4c610df5610de78d8b8d612ab3565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff929092166044830152816064818b5afa91821561033557610eba8c600194838d610ebf965f93610ec7575b50610ea990610eaf9394516123c8565b516123c8565b9063ffffffff169052565b612ad4565b989050610e24565b610eaf935090610ef0610ea99260203d8111610ef8575b610ee8818361038d565b810190612abf565b935090610e99565b503d610ede565b610de791945092610df592610f2d610dfb9560203d8111610f38575b610f25818361038d565b810190612a7e565b959250925092610dc3565b503d610f1b565b5096989198959095610f5081612389565b905f5b8a828210610f8057610f7a95949250610f749391505190610d4083836123c8565b50612a68565b96610d16565b90610fa2610f98610d6e83610ea989600197516123c8565b610eaf83876123c8565b01610f53565b604051632efa2ca360e11b81528491908b856020836004818e5afa90811561033557610ff5955f948593611047575b5060405163354952a360e21b81529687948593849360048501612ae2565b03916001600160a01b03165afa801561033557610768925f91611025575b50602082015260405191829182610b26565b61104191503d805f833e611039818361038d565b810190612977565b83611013565b61106191935060203d602011610aab57610a9d818361038d565b9187610fd7565b61107c91503d805f833e611039818361038d565b5f610cfc565b61109691503d805f833e611039818361038d565b5f610cb9565b6110b691925060203d602011610aab57610a9d818361038d565b905f610c7e565b3461033a575f36600319011261033a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561033557611128915f916103065750612217565b6103046141d1565b3461033a575f36600319011261033a57602060405167016345785d8a00008152f35b60ff81160361033a57565b3461033a57602036600319011261033a576020600160ff60043561118081611152565b161b806066541614604051908152f35b9081608091031261033a5790565b604090602319011261033a57602490565b9080601f8301121561033a5781356111c681610676565b926111d4604051948561038d565b81845260208085019260051b82010192831161033a57602001905b8282106111fc5750505090565b60208091833561120b816105da565b8152019101906111ef565b81601f8201121561033a57803561122c81610676565b9261123a604051948561038d565b81845260208085019260061b8401019281841161033a57602001915b838310611264575050505090565b60206040916112738486610424565b815201920191611256565b9080601f8301121561033a57813561129581610676565b926112a3604051948561038d565b81845260208085019260051b8201019183831161033a5760208201905b8382106112cf57505050505090565b81356001600160401b03811161033a576020916112f1878480948801016111af565b8152019101906112c0565b9190916101808184031261033a576113126103ae565b9281356001600160401b03811161033a578161132f9184016111af565b845260208201356001600160401b03811161033a5781611350918401611216565b602085015260408201356001600160401b03811161033a5781611374918401611216565b604085015261138681606084016104cc565b60608501526113988160e08401610424565b60808501526101208201356001600160401b03811161033a57816113bd9184016111af565b60a08501526101408201356001600160401b03811161033a57816113e29184016111af565b60c08501526101608201356001600160401b03811161033a57611405920161127e565b60e0830152565b3461033a57608036600319011261033a576004356001600160401b03811161033a5761143c903690600401611190565b6114453661119e565b906064356001600160401b03811161033a576114659036906004016112fc565b60ce549092906001600160a01b0316330361167857611488602083949301612b07565b9161158b6114996040860186612b11565b9290946114f96114ab60608901612b07565b976040516114cf816114c1602082019485612b43565b03601f19810183528261038d565b5190206114f26114de88612b07565b63ffffffff165f5260ca60205260405f2090565b5414612bca565b61152361151c61150887612b07565b63ffffffff165f5260cb60205260405f2090565b5415612c3c565b8363ffffffff43169661156d61156561155c7f000000000000000000000000000000000000000000000000000000000000000086612ccd565b63ffffffff1690565b891115612ce7565b6040516020810190611583816114c18b85612d67565b519020613a3a565b919060ff5f9616955b828110611616577f349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a8686866115d66115ca6103c0565b63ffffffff9094168452565b602083015260405160208101906115f2816114c1868686612e4a565b51902061160161150883612b07565b5561161160405192839283612e4a565b0390a1005b8061167261164e61164961163d61163060019688516123c8565b516001600160601b031690565b6001600160601b031690565b612d77565b61166b61163d8b6116666116308760208b01516123c8565b612db6565b1115612dd9565b01611594565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b60206040818301928281528451809452019201905f5b8181106116e05750505090565b82518452602093840193909201916001016116d3565b3461033a57606036600319011261033a5760043561171381610665565b6024356001600160401b03811161033a57611732903690600401610920565b6044359161173f836105da565b6040516361c8a12f60e11b8152906001600160a01b03165f82806117678688600484016129fa565b0381845afa918215610335575f92611831575b506117858351612389565b935f5b84518110156118235761179b81866123c8565b51906020836117ad610d6e84896123c8565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610335576001925f91611805575b50828060c01b03166117fe82896123c8565b5201611788565b61181d915060203d8111610f3857610f25818361038d565b5f6117ec565b6040518061076888826116bd565b6118469192503d805f833e611039818361038d565b905f61177a565b3461033a575f36600319011261033a576020606654604051908152f35b3461033a57602036600319011261033a5763ffffffff60043561188c816105da565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033a5760c036600319011261033a576004356001600160401b03811161033a57611960903690600401611190565b6119693661119e565b90604036606319011261033a5760a4356001600160401b03811161033a576103049261199b6064923690600401611216565b926132a5565b3461033a57606036600319011261033a576024356004356119c1826105da565b6044356001600160401b03811161033a576119e0903690600401610ac0565b60cf5491939092916001600160a01b03163303611ac05761030493611aaa93611a2a611a3193611a0e6138a5565b9586524363ffffffff16602087015263ffffffff166060860152565b36916107a1565b60408201526040516020810190611a4c816114c185856138c9565b519020611a616114de60c95463ffffffff1690565b5560c95463ffffffff16907f1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c4860405180611aa263ffffffff861694826138c9565b0390a2612c9d565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b90602080835192838152019201905f5b818110611b705750505090565b82516001600160601b0316845260209384019390920191600101611b63565b3461033a57608036600319011261033a576004356024356001600160401b03811161033a57611bc2903690600401610ac0565b9091604435611bd0816105da565b606435926001600160401b03841161033a57611c3294611bf7611bfd9536906004016112fc565b93613a3a565b604051928392604084526020611c1e82516040808801526080870190611b53565b910151848203603f19016060860152611b53565b9060208301520390f35b3461033a575f36600319011261033a57611c5461487c565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461033a575f36600319011261033a57602063ffffffff60c95416604051908152f35b3461033a575f36600319011261033a5760cf546040516001600160a01b039091168152602090f35b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033a575f36600319011261033a576033546040516001600160a01b039091168152602090f35b3461033a575f36600319011261033a5760d0546040516001600160a01b039091168152602090f35b3461033a575f36600319011261033a5760d1546040516001600160a01b039091168152602090f35b3461033a5760c036600319011261033a57600435611dbb81610665565b611e3b602435611dca81610665565b604435611dd681610665565b606435611de281610665565b60843591611def83610665565b60a43593611dfc85610665565b5f5496611e2160ff60088a901c16158099819a611eb9575b8115611e99575b50614105565b87611e32600160ff195f5416175f55565b611e8257614168565b611e4157005b611e4f61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498908060208101611611565b611e9461010061ff00195f5416175f55565b614168565b303b15915081611eab575b505f611e1b565b60ff1660011490505f611ea4565b600160ff8216109150611e14565b6040906106d19392815281602082015201906107d7565b3461033a57606036600319011261033a57600435611efb81610665565b602435604435611f0a816105da565b611f4b611f15612367565b9280611f20856123bb565b526040516361c8a12f60e11b81526001600160a01b0386169490925f918491829187600484016129fa565b0381875afa9384156103355783611f7561155c610d6e611faa986020975f91612008575b506123bb565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa801561033557611fd9925f91611fe9575b506001600160c01b031692611fd38461491c565b90612629565b9061076860405192839283611ec7565b612002915060203d602011610f3857610f25818361038d565b5f611fbf565b61201c91503d805f833e611039818361038d565b5f611f6f565b3461033a575f36600319011261033a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461033a57602036600319011261033a5760043561208381610665565b61208b61487c565b6001600160a01b038116156120a357610304906148d4565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461033a575f36600319011261033a57602060405160648152f35b3461033a57602036600319011261033a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610335575f916121d5575b506001600160a01b031633036121c65761219460665419821981161461222d565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b6121ee915060203d602011610aab57610a9d818361038d565b5f612173565b9081602091031261033a5751801515810361033a5790565b6040513d5f823e3d90fd5b1561221e57565b631d77d47760e21b5f5260045ffd5b1561223457565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b9060028110156122685760051b0190565b612243565b634e487b7160e01b5f52601260045260245ffd5b61235d61233a6123639561233461232d85875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e084015261010083015261230481610120840103601f19810183528261038d565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b809661427b565b906142c1565b9261233461234f612349614323565b9461441a565b91612358614536565b61427b565b916145b0565b9091565b60408051909190612378838261038d565b6001815291601f1901366020840137565b9061239382610676565b6123a0604051918261038d565b82815280926123b1601f1991610676565b0190602036910137565b8051156122685760200190565b80518210156122685760209160051b010190565b9081602091031261033a575190565b9190916123f88351612389565b925f5b81518110156124ab5780602061242461241761244d94866123c8565b516001600160a01b031690565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610335576001925f9161247d575b5061247682886123c8565b52016123fb565b61249e915060203d81116124a4575b612496818361038d565b8101906123dc565b5f61246b565b503d61248c565b505050565b9081602091031261033a57516106d181610665565b906124cf82610676565b6124dc604051918261038d565b828152602081936124ef601f1991610676565b0191015f5b82811061250057505050565b6060828201526020016124f4565b908151811015612268570160200190565b60208183031261033a578051906001600160401b03821161033a57019080601f8301121561033a57815161255281610676565b92612560604051948561038d565b81845260208085019260051b82010192831161033a57602001905b8282106125885750505090565b815181526020918201910161257b565b906125a282610676565b6125af604051918261038d565b82815280926125c0601f1991610676565b015f5b8181106125cf57505050565b6040519060608201918083106001600160401b0384111761036d576020926040525f81525f838201525f6040820152828286010152016125c3565b9081602091031261033a57516001600160601b038116810361033a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610335575f95612932575b50604051634f4c91e160e11b815294602086600481855afa918215610335576004965f93612910575b5060209060405197888092632efa2ca360e11b82525afa958615610335575f966128ef575b506126b785939295516124c5565b945f935b80518510156128e5576126e2610df56126d4878461250e565b516001600160f81b03191690565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610335575f946128c1575b506127328451612598565b61273c888b6123c8565b52612747878a6123c8565b505f5b84518110156128b05780602061276361278593886123c8565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610335575f92612890575b506127ab81876123c8565b518a60208a6127ba858b6123c8565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa9384156103355761284d8c8f6128486001986128599789975f92612860575b506128336128246103cf565b6001600160a01b039098168852565b60208701526001600160601b03166040860152565b6123c8565b5190610d4083836123c8565b500161274a565b61288291925060203d8111612889575b61287a818361038d565b81019061260a565b905f612818565b503d612870565b6128a991925060203d8111610aab57610a9d818361038d565b905f6127a0565b5060019096019590945091506126bb565b6128de9194503d805f833e6128d6818361038d565b81019061251f565b925f612727565b5050509350505090565b61290991965060203d602011610aab57610a9d818361038d565b945f6126a9565b602091935061292b90823d8411610aab57610a9d818361038d565b9290612684565b61294c91955060203d602011610aab57610a9d818361038d565b935f61265b565b6040519061296082610372565b606080838181528160208201528160408201520152565b60208183031261033a578051906001600160401b03821161033a57019080601f8301121561033a5781516129aa81610676565b926129b8604051948561038d565b81845260208085019260051b82010192831161033a57602001905b8282106129e05750505090565b6020809183516129ef816105da565b8152019101906129d3565b60409063ffffffff6106d19493168152816020820152019061068d565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106d195931681528160208201520191612a17565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff8114612a795760010190565b612a54565b9081602091031261033a57516001600160c01b038116810361033a5790565b15612aa457565b6325ec6c1f60e01b5f5260045ffd5b90821015612268570190565b9081602091031261033a57516106d1816105da565b5f198114612a795760010190565b91612b0060209263ffffffff92969596604086526040860191612a17565b9416910152565b356106d1816105da565b903590601e198136030182121561033a57018035906001600160401b03821161033a5760200191813603831361033a57565b602081528135602082015263ffffffff6020830135612b61816105da565b1660408201526040820135601e198336030181121561033a578201906020823592016001600160401b03831161033a57823603811361033a57612bbf6060612bb86080936106d196858488015260a0870191612a17565b95016105e8565b63ffffffff16910152565b15612bd157565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15612c4357565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b63ffffffff60019116019063ffffffff8211612a7957565b63ffffffff60649116019063ffffffff8211612a7957565b9063ffffffff8091169116019063ffffffff8211612a7957565b15612cee57565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6020809163ffffffff8135612d5d816105da565b1684520135910152565b6040810192916103be9190612d49565b90606482029180830460641490151715612a7957565b90600682029180830460061490151715612a7957565b81810292918115918404141715612a7957565b906001600160601b03809116911602906001600160601b038216918203612a7957565b15612de057565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091612e60846080810197612d49565b63ffffffff81511660408501520151910152565b15612e7b57565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b9092916020606091612ee0846080810197612d49565b63ffffffff8135612ef0816105da565b1660408501520135910152565b15612f0457565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15612f7657565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b15612ff457565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106130895750505090565b825184526020938401939092019160010161307c565b156130a657565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b60208183031261033a578051906001600160401b03821161033a57019080601f8301121561033a57815161315d81610676565b9261316b604051948561038d565b81845260208085019260051b82010192831161033a57602001905b8282106131935750505090565b6020809183516131a281610665565b815201910190613186565b604051906131bc60408361038d565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b8181106132865750505060806132716106d194956060850151601f19858303018486015261068d565b9201519060a0601f19828503019101526131dd565b82516001600160a01b0316875260209687019690920191600101613248565b90929160016132b385612b07565b94602061336f85356132de6132d68a63ffffffff165f5260cb60205260405f2090565b541515612e74565b6133196132f98a63ffffffff165f5260cb60205260405f2090565b5460405185810190613310816114c18c8b86612eca565b51902014612efd565b61334461333e6133378b63ffffffff165f5260cc60205260405f2090565b5460ff1690565b15612f6f565b61336961335b61155c61335689612b07565b612cb5565b63ffffffff43161115612fed565b80612da3565b9101351414613874576133828351612389565b935f5b84518110156133c257806133b161339e600193886123c8565b5180515f526020015160205260405f2090565b6133bb82896123c8565b5201613385565b5090929391946133fc602087019460206133db87612b07565b6040516133f0816114c18a868301958661305f565b5190209101351461309f565b6134068551612389565b957f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316945f5b87518110156134b65780602061344d61346d93896123c8565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818b5afa91821561033557600192613492915f91613498575b50610a6f838d6123c8565b01613434565b6134b0915060203d8111610aab57610a9d818361038d565b5f613487565b50929695509250926135156134de6134e660408601946134d68688612b11565b939091612b07565b9236916107a1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612629565b945f915b8651831015613816575f979697955b613532848a6123c8565b51518710156138065797613573986020806135518a610ea989876123c8565b510151604051809c81926308f6629d60e31b8352600483019190602083019252565b0381865afa998a15610335575f9a6137e6575b506001985f5b85518110156137d8576135b16135a561241783896123c8565b6001600160a01b031690565b6001600160a01b038d16146135c85760010161358c565b5099909791985060015f5b1515146135e8575b5060010195979697613528565b98969786985f8795939861364d60ff613625610df5610de78c61361f6136a29f61361960cd5460018060a01b031690565b98612b11565b90612ab3565b61363f6136306103c0565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d154613664906135a5906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529098899190829081906044820190565b03915afa968715610335575f976137b4575b506136bf8751612389565b985f5b8a518110156136e8578067016345785d8a00006136e16001938e6123c8565b52016136c2565b509a92949661372660ff61370d610df5610de78f9d979f969e9661361f8e8e92612b11565b6137186128246103de565b1663ffffffff166020860152565b604084015260608301526137386131ad565b608083015260d054613754906135a5906001600160a01b031681565b803b1561033a57604051636a669b4160e01b8152925f9184918290849082906137809060048301613201565b03925af19182156103355760019261379a575b50906135db565b806137a85f6137ae9361038d565b80610568565b5f613793565b6137d19197503d805f833e6137c9818361038d565b81019061312a565b955f6136b4565b5099909791986001906135d3565b6137ff919a5060203d8111610aab57610a9d818361038d565b985f613586565b9697969550600190920191613519565b50505050509190506138466138398263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3565b50505063ffffffff3391167ffd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb055f80a3565b604051906138b282610372565b5f6060838281528260208201528160408201520152565b602081528151602082015263ffffffff6020830151166040820152608063ffffffff60606139046040860151848387015260a08601906131dd565b9401511691015290565b6040519061391b82610352565b60606020838281520152565b1561392e57565b62f8202d60e51b5f5260045ffd5b1561394357565b6343714afd60e01b5f5260045ffd5b1561395957565b635f832f4160e01b5f5260045ffd5b1561396f57565b634b874f4560e01b5f5260045ffd5b9081602091031261033a57516106d181611152565b5f19810191908211612a7957565b156139a857565b633fdc650560e21b5f5260045ffd5b9081602091031261033a575167ffffffffffffffff198116810361033a5790565b156139df57565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b038211612a7957565b15613a1557565b6367988d3360e01b5f5260045ffd5b15613a2b57565b63ab1b236b60e01b5f5260045ffd5b949392909193613a4861390e565b50613a54851515613927565b6040840151518514806140f7575b806140e9575b806140db575b613a779061393c565b613a8960208501515185515114613952565b613aa063ffffffff431663ffffffff841610613968565b613aa86103c0565b5f81525f602082015292613aba61390e565b613ac387612389565b6020820152613ad187612389565b8152613adb61390e565b92613aea602088015151612389565b8452613afa602088015151612389565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561033557613b63915f916140ac575b50613b5e368b876107a1565b6146de565b985f965b60208901518051891015613cc257602088613bb7610d6e8c613baf8f96868e613b9461339e8680956123c8565b613ba184848401516123c8565b5282613c8f575b01516123c8565b5195516123c8565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa918215610335576123348a613c648f613c5d8f8460208f92613c5493613c4c8460019e613c6a9e5f91613c72575b508f8060c01b031692516123c8565b5201516123c8565b51938d516123c8565b5116614709565b9061473a565b970196613b67565b613c899150863d8111610f3857610f25818361038d565b5f613c3d565b613cbd613c9f84848401516123c8565b51613cb684840151613cb087613993565b906123c8565b51106139a1565b613ba8565b50909597949650613cd79198939299506147f7565b915f905b808210613d2f5750505092613d16613d11613d0a613d2995856114c19860806060602099015192015192612281565b9190613a0e565b613a24565b015160405192839160208301958661305f565b51902090565b9184959693613d6c610df5610de78587613d66879f989e9b61339e846040613d589301516123c8565b67ffffffffffffffff191690565b96612ab3565b602087613d80610d6e8d60a08d01516123c8565b604051631a2f32ab60e21b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa92831561033557613e1c93613e07925f9161407e575b5067ffffffffffffffff199182169116146139d8565b613e158960408901516123c8565b51906142c1565b90613e2e610df5610de78a868d612ab3565b602086613e42610d6e8c60c08c01516123c8565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa8015610335576116308a60208e613ba8613ed895613ee3975f91614061575b50613eca85858501516123c8565b906001600160601b03169052565b613eca8a8d516123c8565b5f965f5b60208801515181101561404d57898b613f23613f04848a516123c8565b51613f16610df5610de7868c87612ab3565b60ff161c60019081161490565b613f32575b5050600101613ee7565b8888613fb3613f73610d6e879f8f978060e08f849c6020613f62610df5610de7839f98613f6a96610ea99a612ab3565b9a01516123c8565b519a01516123c8565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830195909552919093166064840152829081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103355761401c8e6001948e86955f92614027575b50614016613eca9293519361401161163084876123c8565b6139ee565b926123c8565b01989050898b613f28565b613eca92506140466140169160203d81116128895761287a818361038d565b9250613ff9565b509594976001919993929497500190613cdb565b6140789150843d81116128895761287a818361038d565b5f613ebc565b61409f915060203d81116140a5575b614097818361038d565b8101906139b7565b5f613df1565b503d61408d565b6140ce915060203d6020116140d4575b6140c6818361038d565b81019061397e565b5f613b52565b503d6140bc565b5060e0840151518514613a6e565b5060c0840151518514613a68565b5060a0840151518514613a62565b1561410c57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b614171906148d4565b60ce80546001600160a01b03199081166001600160a01b039384161790915560cf805482169383169390931790925560d1805483169382169390931790925560d0805482169383169390931790925560cd80549092169216919091179055565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b6040519061424482610352565b5f6020838281520152565b60405190610180614260818461038d565b368337565b6040519061427460208361038d565b6020368337565b9190604090606061428a614237565b948592602085519261429c858561038d565b8436853780518452015160208301528482015260076107cf195a01fa156142bf57565bfe5b6020929160806040926142d2614237565b958693818651936142e3868661038d565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa80156142bf571561431457565b63d4b68fd760e01b5f5260045ffd5b60405161432f81610352565b604090815161433e838261038d565b8236823781526020825191614353848461038d565b8336843701528051614365828261038d565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed60208201528151906143bb838361038d565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208301526144108351938461038d565b8252602082015290565b5f516020614a5f5f395f51905f5290614431614237565b505f919006602060c0835b614531575f935f516020614a5f5f395f51905f5260038186818180090908604051614467858261038d565b8436823784818560405161447b828261038d565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614a5f5f395f51905f5260a082015260056107cf195a01fa80156142bf576144e590614a48565b5191614531575f516020614a5f5f395f51905f528280091461451c57505f516020614a5f5f395f51905f5260015f9408929361443c565b929350506145286103c0565b92835282015290565b61226d565b61453e614237565b5060405161454b81610352565b600181526002602082015290565b9060018201809211612a7957565b9060028201809211612a7957565b9060038201809211612a7957565b9060048201809211612a7957565b9060058201809211612a7957565b90600c8110156122685760051b0190565b939290916145be60406103ed565b94855260208501526145d060406103ed565b91825260208201526145e061424f565b925f5b6002811061460d575050506020610180926145fc614265565b93849160086201d4c0fa9151151590565b80614619600192612d8d565b6146238285612257565b515161462f828961459f565b52602061463c8386612257565b51015161465161464b83614559565b8961459f565b5261465c8286612257565b51515161466b61464b83614567565b526146816146798387612257565b515160200190565b5161468e61464b83614575565b52602061469b8387612257565b510151516146ab61464b83614583565b526146d76146d16146ca60206146c1868a612257565b51015160200190565b5192614591565b8861459f565b52016145e3565b9060016146ec60ff936149d0565b928392161b11156146fa5790565b63ca95733360e01b5f5260045ffd5b805f915b614715575090565b5f198101818111612a795761ffff9116911661ffff8114612a7957600101908061470d565b90614743614237565b5061ffff8116906102008210156147e857600182146147e3576147646103c0565b5f81525f602082015292906001905f925b61ffff831685101561478957505050505090565b600161ffff831660ff86161c8116146147c3575b60016147b96147ae8360ff946142c1565b9460011b61fffe1690565b9401169291614775565b9460016147b96147ae6147d88960ff956142c1565b98935050505061479d565b505090565b637fc4ea7d60e11b5f5260045ffd5b6147ff614237565b50805190811580614870575b1561482c57505060405161482060408261038d565b5f81525f602082015290565b60205f516020614a5f5f395f51905f52910151065f516020614a5f5f395f51905f52035f516020614a5f5f395f51905f528111612a79576040519161441083610352565b5060208101511561480b565b6033546001600160a01b0316330361489057565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff61492882614709565b1661493281610786565b90614940604051928361038d565b80825261494f601f1991610786565b013660208301375f5f5b82518210806149af575b156149a8576001811b8416614981575b61497c90612ad4565b614959565b90600161497c9160ff60f81b8460f81b165f1a61499e828761250e565b5301919050614973565b5050905090565b506101008110614963565b156149c157565b631019106960e31b5f5260045ffd5b90610100825111614a3957815115614a3457602082015160019060f81c81901b5b8351821015614a2f57600190614a1a614a10610df56126d4868961250e565b60ff600191161b90565b90614a268183116149ba565b179101906149f1565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614a4f57565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212200f1c7f3d025b2713afffec7bd7b2670675509f54e75c92f2f9e7f71729622d8b64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02zW\x80c\x17\x1F\x1D[\x14a\x02uW\x80c\x1A\xD41\x89\x14a\x01\xD0W\x80c$Z{\xFC\x14a\x02pW\x80c,\xB2#\xD5\x14a\x02kW\x80c-\x89\xF6\xFC\x14a\x02fW\x80c1\xB3k\xD9\x14a\x02aW\x80c5c\xB0\xD1\x14a\x02\\W\x80c9\x98\xFD\xD3\x14a\x02WW\x80cM+W\xFE\x14a\x02RW\x80cOs\x9Ft\x14a\x02MW\x80cY\\jg\x14a\x02HW\x80cZ-\x7F\x02\x14a\x02CW\x80cZ\xC8j\xB7\x14a\x02>W\x80c[\xAE\xC9\xA0\x14a\x029W\x80c\\\x15Vb\x14a\x024W\x80c\\\x97Z\xBB\x14a\x02/W\x80c]\xEC\xC3\xF5\x14a\x02*W\x80c]\xF4YF\x14a\x02%W\x80ch0H5\x14a\x02 W\x80ckS.\x9E\x14a\x02\x1BW\x80ck\x92x~\x14a\x02\x16W\x80cm\x14\xA9\x87\x14a\x02\x11W\x80cn\xFBF6\x14a\x02\x0CW\x80cqP\x18\xA6\x14a\x02\x07W\x80cr\xD1\x8E\x8D\x14a\x01\xF8W\x80cz\xFA\x1E\xED\x14a\x02\x02W\x80c\x88o\x11\x95\x14a\x01\xFDW\x80c\x8B\0\xCE|\x14a\x01\xF8W\x80c\x8D\xA5\xCB[\x14a\x01\xF3W\x80c\x9B)\x0E\x98\x14a\x01\xEEW\x80c\xCA\x8A\xA7\xC7\x14a\x01\xE9W\x80c\xCC*\x9A[\x14a\x01\xE4W\x80c\xCE\xFD\xC1\xD4\x14a\x01\xDFW\x80c\xDF\\\xF7#\x14a\x01\xDAW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xD5W\x80c\xF5\xC9\x89\x9D\x14a\x01\xD0W\x80c\xF6<[\xAB\x14a\x01\xCBWc\xFA\xBC\x1C\xBC\x14a\x01\xC6W_\x80\xFD[a!\x12V[a \xF7V[a\x05rV[a fV[a \"V[a\x1E\xDEV[a\x1D\x9EV[a\x1DvV[a\x1DNV[a\x1D&V[a\x1C\x97V[a\x1C\xE2V[a\x1C\xBAV[a\x1C<V[a\x1B\x8FV[a\x1B\x0FV[a\x19\xA1V[a\x190V[a\x18\xECV[a\x18\xA8V[a\x18jV[a\x18MV[a\x16\xF6V[a\x14\x0CV[a\x11]V[a\x110V[a\x10\xBDV[a\x0B\xDAV[a\t\xBFV[a\x08\xF8V[a\x08~V[a\x06\xD4V[a\x06,V[a\x05\xF3V[a\x05\xB2V[a\x05\0V[4a\x03:W` 6`\x03\x19\x01\x12a\x03:W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x035Wa\x03\x04\x92a\x02\xF0\x91_\x91a\x03\x06W[Pa\"\x17V[a\x02\xFF`fT\x82\x81\x16\x14a\"-V[aB\x05V[\0[a\x03(\x91P` =` \x11a\x03.W[a\x03 \x81\x83a\x03\x8DV[\x81\x01\x90a!\xF4V[_a\x02\xEAV[P=a\x03\x16V[a\"\x0CV[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03mW`@RV[a\x03>V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03mW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03mW`@RV[`@Q\x90a\x03\xBEa\x01\0\x83a\x03\x8DV[V[`@Q\x90a\x03\xBE`@\x83a\x03\x8DV[`@Q\x90a\x03\xBE``\x83a\x03\x8DV[`@Q\x90a\x03\xBE`\xA0\x83a\x03\x8DV[\x90a\x03\xBE`@Q\x92\x83a\x03\x8DV[`@\x90`\xE3\x19\x01\x12a\x03:W`@Q\x90a\x04\x14\x82a\x03RV[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03:W`@Qa\x04<\x81a\x03RV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03:W`@Q\x91a\x04g`@\x84a\x03\x8DV[\x82\x90`@\x81\x01\x92\x83\x11a\x03:W\x90[\x82\x82\x10a\x04\x83WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04vV[\x90`\x80`c\x19\x83\x01\x12a\x03:W`@Qa\x04\xAC\x81a\x03RV[` a\x04\xC7\x82\x94a\x04\xBE\x81`da\x04LV[\x84R`\xA4a\x04LV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03:W` a\x04\xC7`@Q\x92a\x04\xEA\x84a\x03RV[`@\x84\x96a\x04\xF8\x83\x82a\x04LV[\x86R\x01a\x04LV[4a\x03:Wa\x01 6`\x03\x19\x01\x12a\x03:W`\x045`@6`#\x19\x01\x12a\x03:Wa\x05X`@\x91\x82Qa\x052\x81a\x03RV[`$5\x81R`D5` \x82\x01Ra\x05H6a\x04\x93V[\x90a\x05R6a\x03\xFBV[\x92a\"\x81V[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03:WV[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03:WV[5\x90a\x03\xBE\x82a\x05\xDAV[4a\x03:W` 6`\x03\x19\x01\x12a\x03:Wc\xFF\xFF\xFF\xFF`\x045a\x06\x15\x81a\x05\xDAV[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03:W` 6`\x03\x19\x01\x12a\x03:Wc\xFF\xFF\xFF\xFF`\x045a\x06N\x81a\x05\xDAV[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03:WV[`\x01`\x01`@\x1B\x03\x81\x11a\x03mW`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xAAWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\x9DV[\x90` a\x06\xD1\x92\x81\x81R\x01\x90a\x06\x8DV[\x90V[4a\x03:W`@6`\x03\x19\x01\x12a\x03:W`\x045a\x06\xF1\x81a\x06eV[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W6`#\x83\x01\x12\x15a\x03:W\x81`\x04\x015\x91a\x07\x1D\x83a\x06vV[\x92a\x07+`@Q\x94\x85a\x03\x8DV[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03:W`$\x01\x90[\x82\x82\x10a\x07lWa\x07ha\x07\\\x86\x86a#\xEBV[`@Q\x91\x82\x91\x82a\x06\xC0V[\x03\x90\xF3[` \x80\x91\x835a\x07{\x81a\x06eV[\x81R\x01\x91\x01\x90a\x07HV[`\x01`\x01`@\x1B\x03\x81\x11a\x03mW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xAD\x82a\x07\x86V[\x91a\x07\xBB`@Q\x93\x84a\x03\x8DV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03:W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x08\x02WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x08BWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x07\xF3V[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08#V[4a\x03:W``6`\x03\x19\x01\x12a\x03:W`\x045a\x08\x9B\x81a\x06eV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03:W6`#\x82\x01\x12\x15a\x03:Wa\x07h\x91a\x08\xD2a\x08\xE4\x926\x90`$\x81`\x04\x015\x91\x01a\x07\xA1V[`D5\x91a\x08\xDF\x83a\x05\xDAV[a&)V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xD7V[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x815a\t7\x81a\x06vV[\x92a\tE`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a\tmWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\t`V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\t\xA0WPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\t\x93V[4a\x03:W`@6`\x03\x19\x01\x12a\x03:W`\x045a\t\xDC\x81a\x06eV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\t\xFB\x906\x90`\x04\x01a\t V[a\n\x05\x81Qa#\x89V[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\n\xB2W\x80` a\n*a\nJ\x93\x86a#\xC8V[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x035W`\x01\x92a\n~\x91_\x91a\n\x84W[Pa\no\x83\x88a#\xC8V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\n\x11V[a\n\xA5\x91P` =\x81\x11a\n\xABW[a\n\x9D\x81\x83a\x03\x8DV[\x81\x01\x90a$\xB0V[_a\ndV[P=a\n\x93V[`@Q\x80a\x07h\x86\x82a\t}V[\x91\x81`\x1F\x84\x01\x12\x15a\x03:W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03:W` \x83\x81\x86\x01\x95\x01\x01\x11a\x03:WV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\nWPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n\xFDV[\x90` \x82R``a\x0Bta\x0B_a\x0BI\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\n\xEDV[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\n\xEDV[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\n\xEDV[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0B\xADWPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0B\xCB`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\n\xEDV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0B\x9EV[4a\x03:W`\x806`\x03\x19\x01\x12a\x03:W`\x045a\x0B\xF7\x81a\x06eV[`$5\x90a\x0C\x04\x82a\x05\xDAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x0C#\x906\x90`\x04\x01a\n\xC0V[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x0CC\x906\x90`\x04\x01a\t V[\x92a\x0CLa)SV[P`@Qch0H5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91` \x82`\x04\x81\x86Z\xFA\x91\x82\x15a\x035W_\x92a\x10\x9CW[Pa\x0C\x87a)SV[\x91`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a\x0C\xA6\x8A\x8C`\x04\x84\x01a)\xFAV[\x03\x81\x88Z\xFA\x90\x81\x15a\x035W_\x91a\x10\x82W[P\x83R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90_\x81\x80a\x0C\xE9\x89\x86\x8D`\x04\x85\x01a*7V[\x03\x81\x86Z\xFA\x90\x81\x15a\x035W_\x91a\x10hW[P`@\x84\x01Ra\r\x0B\x85a$\xC5V[\x94``\x84\x01\x95\x86R_\x96[`\xFF\x88\x16\x94\x82\x86\x10\x15a\x0F\xA8W_\x96a\rF\x87a\r3\x85Qa#\x89V[\x8BQ\x90a\r@\x83\x83a#\xC8V[Ra#\xC8V[P_[\x83Q\x81\x10\x15a\x0F?Wa\r\\\x81\x85a#\xC8V[Q` \x8Da\rxa\rn\x85\x88Qa#\xC8V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81\x86Z\xFA\x90\x81\x15a\x035W`\x01a\x0E\x11\x81\x93\x8C\x8A\x8Ca\x0E\x1E\x96_\x94a\x0E\xFFW[Pa\r\xFB\x92a\r\xF5\x92a\r\xE7\x92a\r\xE2\x89\x80`\xC0\x1B\x03\x88\x16\x15\x15a*\x9DV[a*\xB3V[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a\x0E,W[`\x01\x01a\rIV[\x97\x8B` a\x0E:\x8B\x87a#\xC8V[Qa\x0ELa\r\xF5a\r\xE7\x8D\x8B\x8Da*\xB3V[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81\x8BZ\xFA\x91\x82\x15a\x035Wa\x0E\xBA\x8C`\x01\x94\x83\x8Da\x0E\xBF\x96_\x93a\x0E\xC7W[Pa\x0E\xA9\x90a\x0E\xAF\x93\x94Qa#\xC8V[Qa#\xC8V[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a*\xD4V[\x98\x90Pa\x0E$V[a\x0E\xAF\x93P\x90a\x0E\xF0a\x0E\xA9\x92` =\x81\x11a\x0E\xF8W[a\x0E\xE8\x81\x83a\x03\x8DV[\x81\x01\x90a*\xBFV[\x93P\x90a\x0E\x99V[P=a\x0E\xDEV[a\r\xE7\x91\x94P\x92a\r\xF5\x92a\x0F-a\r\xFB\x95` =\x81\x11a\x0F8W[a\x0F%\x81\x83a\x03\x8DV[\x81\x01\x90a*~V[\x95\x92P\x92P\x92a\r\xC3V[P=a\x0F\x1BV[P\x96\x98\x91\x98\x95\x90\x95a\x0FP\x81a#\x89V[\x90_[\x8A\x82\x82\x10a\x0F\x80Wa\x0Fz\x95\x94\x92Pa\x0Ft\x93\x91PQ\x90a\r@\x83\x83a#\xC8V[Pa*hV[\x96a\r\x16V[\x90a\x0F\xA2a\x0F\x98a\rn\x83a\x0E\xA9\x89`\x01\x97Qa#\xC8V[a\x0E\xAF\x83\x87a#\xC8V[\x01a\x0FSV[`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x84\x91\x90\x8B\x85` \x83`\x04\x81\x8EZ\xFA\x90\x81\x15a\x035Wa\x0F\xF5\x95_\x94\x85\x93a\x10GW[P`@Qc5IR\xA3`\xE2\x1B\x81R\x96\x87\x94\x85\x93\x84\x93`\x04\x85\x01a*\xE2V[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x035Wa\x07h\x92_\x91a\x10%W[P` \x82\x01R`@Q\x91\x82\x91\x82a\x0B&V[a\x10A\x91P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[\x81\x01\x90a)wV[\x83a\x10\x13V[a\x10a\x91\x93P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x91\x87a\x0F\xD7V[a\x10|\x91P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[_a\x0C\xFCV[a\x10\x96\x91P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[_a\x0C\xB9V[a\x10\xB6\x91\x92P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x90_a\x0C~V[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x035Wa\x11(\x91_\x91a\x03\x06WPa\"\x17V[a\x03\x04aA\xD1V[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x03:WV[4a\x03:W` 6`\x03\x19\x01\x12a\x03:W` `\x01`\xFF`\x045a\x11\x80\x81a\x11RV[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[\x90\x81`\x80\x91\x03\x12a\x03:W\x90V[`@\x90`#\x19\x01\x12a\x03:W`$\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x815a\x11\xC6\x81a\x06vV[\x92a\x11\xD4`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a\x11\xFCWPPP\x90V[` \x80\x91\x835a\x12\x0B\x81a\x05\xDAV[\x81R\x01\x91\x01\x90a\x11\xEFV[\x81`\x1F\x82\x01\x12\x15a\x03:W\x805a\x12,\x81a\x06vV[\x92a\x12:`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03:W` \x01\x91[\x83\x83\x10a\x12dWPPPP\x90V[` `@\x91a\x12s\x84\x86a\x04$V[\x81R\x01\x92\x01\x91a\x12VV[\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x815a\x12\x95\x81a\x06vV[\x92a\x12\xA3`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03:W` \x82\x01\x90[\x83\x82\x10a\x12\xCFWPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03:W` \x91a\x12\xF1\x87\x84\x80\x94\x88\x01\x01a\x11\xAFV[\x81R\x01\x91\x01\x90a\x12\xC0V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03:Wa\x13\x12a\x03\xAEV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13/\x91\x84\x01a\x11\xAFV[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13P\x91\x84\x01a\x12\x16V[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13t\x91\x84\x01a\x12\x16V[`@\x85\x01Ra\x13\x86\x81``\x84\x01a\x04\xCCV[``\x85\x01Ra\x13\x98\x81`\xE0\x84\x01a\x04$V[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13\xBD\x91\x84\x01a\x11\xAFV[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:W\x81a\x13\xE2\x91\x84\x01a\x11\xAFV[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x14\x05\x92\x01a\x12~V[`\xE0\x83\x01RV[4a\x03:W`\x806`\x03\x19\x01\x12a\x03:W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x14<\x906\x90`\x04\x01a\x11\x90V[a\x14E6a\x11\x9EV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x14e\x906\x90`\x04\x01a\x12\xFCV[`\xCET\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x16xWa\x14\x88` \x83\x94\x93\x01a+\x07V[\x91a\x15\x8Ba\x14\x99`@\x86\x01\x86a+\x11V[\x92\x90\x94a\x14\xF9a\x14\xAB``\x89\x01a+\x07V[\x97`@Qa\x14\xCF\x81a\x14\xC1` \x82\x01\x94\x85a+CV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\x8DV[Q\x90 a\x14\xF2a\x14\xDE\x88a+\x07V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14a+\xCAV[a\x15#a\x15\x1Ca\x15\x08\x87a+\x07V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15a,<V[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x15ma\x15ea\x15\\\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a,\xCDV[c\xFF\xFF\xFF\xFF\x16\x90V[\x89\x11\x15a,\xE7V[`@Q` \x81\x01\x90a\x15\x83\x81a\x14\xC1\x8B\x85a-gV[Q\x90 a::V[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x16\x16W\x7F4\x9C\x1E\xE6\x0EN\x89r\xEE\x9D\xBAd,\x17tT=\\A6\x87\x9B\x7FL\xAA\xF0K\xF8\x1AHz*\x86\x86\x86a\x15\xD6a\x15\xCAa\x03\xC0V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x15\xF2\x81a\x14\xC1\x86\x86\x86a.JV[Q\x90 a\x16\x01a\x15\x08\x83a+\x07V[Ua\x16\x11`@Q\x92\x83\x92\x83a.JV[\x03\x90\xA1\0[\x80a\x16ra\x16Na\x16Ia\x16=a\x160`\x01\x96\x88Qa#\xC8V[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[a-wV[a\x16ka\x16=\x8Ba\x16fa\x160\x87` \x8B\x01Qa#\xC8V[a-\xB6V[\x11\x15a-\xD9V[\x01a\x15\x94V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x16\xE0WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x16\xD3V[4a\x03:W``6`\x03\x19\x01\x12a\x03:W`\x045a\x17\x13\x81a\x06eV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x172\x906\x90`\x04\x01a\t V[`D5\x91a\x17?\x83a\x05\xDAV[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x17g\x86\x88`\x04\x84\x01a)\xFAV[\x03\x81\x84Z\xFA\x91\x82\x15a\x035W_\x92a\x181W[Pa\x17\x85\x83Qa#\x89V[\x93_[\x84Q\x81\x10\x15a\x18#Wa\x17\x9B\x81\x86a#\xC8V[Q\x90` \x83a\x17\xADa\rn\x84\x89a#\xC8V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x035W`\x01\x92_\x91a\x18\x05W[P\x82\x80`\xC0\x1B\x03\x16a\x17\xFE\x82\x89a#\xC8V[R\x01a\x17\x88V[a\x18\x1D\x91P` =\x81\x11a\x0F8Wa\x0F%\x81\x83a\x03\x8DV[_a\x17\xECV[`@Q\x80a\x07h\x88\x82a\x16\xBDV[a\x18F\x91\x92P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[\x90_a\x17zV[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` `fT`@Q\x90\x81R\xF3[4a\x03:W` 6`\x03\x19\x01\x12a\x03:Wc\xFF\xFF\xFF\xFF`\x045a\x18\x8C\x81a\x05\xDAV[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03:W`\xC06`\x03\x19\x01\x12a\x03:W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x19`\x906\x90`\x04\x01a\x11\x90V[a\x19i6a\x11\x9EV[\x90`@6`c\x19\x01\x12a\x03:W`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x03\x04\x92a\x19\x9B`d\x926\x90`\x04\x01a\x12\x16V[\x92a2\xA5V[4a\x03:W``6`\x03\x19\x01\x12a\x03:W`$5`\x045a\x19\xC1\x82a\x05\xDAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x19\xE0\x906\x90`\x04\x01a\n\xC0V[`\xCFT\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1A\xC0Wa\x03\x04\x93a\x1A\xAA\x93a\x1A*a\x1A1\x93a\x1A\x0Ea8\xA5V[\x95\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\x07\xA1V[`@\x82\x01R`@Q` \x81\x01\x90a\x1AL\x81a\x14\xC1\x85\x85a8\xC9V[Q\x90 a\x1Aaa\x14\xDE`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\x16\x95\xB8\xD0n\xC8\0\xB4a^t\\\xFB[\xD0\x0C\x1F(ua]B\x92\\;Z\xFAT;\xB2LH`@Q\x80a\x1A\xA2c\xFF\xFF\xFF\xFF\x86\x16\x94\x82a8\xC9V[\x03\x90\xA2a,\x9DV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x1BpWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1BcV[4a\x03:W`\x806`\x03\x19\x01\x12a\x03:W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03:Wa\x1B\xC2\x906\x90`\x04\x01a\n\xC0V[\x90\x91`D5a\x1B\xD0\x81a\x05\xDAV[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03:Wa\x1C2\x94a\x1B\xF7a\x1B\xFD\x956\x90`\x04\x01a\x12\xFCV[\x93a::V[`@Q\x92\x83\x92`@\x84R` a\x1C\x1E\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x1BSV[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x1BSV[\x90` \x83\x01R\x03\x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:Wa\x1CTaH|V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03:W`\xC06`\x03\x19\x01\x12a\x03:W`\x045a\x1D\xBB\x81a\x06eV[a\x1E;`$5a\x1D\xCA\x81a\x06eV[`D5a\x1D\xD6\x81a\x06eV[`d5a\x1D\xE2\x81a\x06eV[`\x845\x91a\x1D\xEF\x83a\x06eV[`\xA45\x93a\x1D\xFC\x85a\x06eV[_T\x96a\x1E!`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa\x1E\xB9W[\x81\x15a\x1E\x99W[PaA\x05V[\x87a\x1E2`\x01`\xFF\x19_T\x16\x17_UV[a\x1E\x82WaAhV[a\x1EAW\0[a\x1EOa\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x16\x11V[a\x1E\x94a\x01\0a\xFF\0\x19_T\x16\x17_UV[aAhV[0;\x15\x91P\x81a\x1E\xABW[P_a\x1E\x1BV[`\xFF\x16`\x01\x14\x90P_a\x1E\xA4V[`\x01`\xFF\x82\x16\x10\x91Pa\x1E\x14V[`@\x90a\x06\xD1\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xD7V[4a\x03:W``6`\x03\x19\x01\x12a\x03:W`\x045a\x1E\xFB\x81a\x06eV[`$5`D5a\x1F\n\x81a\x05\xDAV[a\x1FKa\x1F\x15a#gV[\x92\x80a\x1F \x85a#\xBBV[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a)\xFAV[\x03\x81\x87Z\xFA\x93\x84\x15a\x035W\x83a\x1Fua\x15\\a\rna\x1F\xAA\x98` \x97_\x91a \x08W[Pa#\xBBV[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x035Wa\x1F\xD9\x92_\x91a\x1F\xE9W[P`\x01`\x01`\xC0\x1B\x03\x16\x92a\x1F\xD3\x84aI\x1CV[\x90a&)V[\x90a\x07h`@Q\x92\x83\x92\x83a\x1E\xC7V[a \x02\x91P` =` \x11a\x0F8Wa\x0F%\x81\x83a\x03\x8DV[_a\x1F\xBFV[a \x1C\x91P=\x80_\x83>a\x109\x81\x83a\x03\x8DV[_a\x1FoV[4a\x03:W_6`\x03\x19\x01\x12a\x03:W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03:W` 6`\x03\x19\x01\x12a\x03:W`\x045a \x83\x81a\x06eV[a \x8BaH|V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a \xA3Wa\x03\x04\x90aH\xD4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03:W_6`\x03\x19\x01\x12a\x03:W` `@Q`d\x81R\xF3[4a\x03:W` 6`\x03\x19\x01\x12a\x03:W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x035W_\x91a!\xD5W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a!\xC6Wa!\x94`fT\x19\x82\x19\x81\x16\x14a\"-V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a!\xEE\x91P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[_a!sV[\x90\x81` \x91\x03\x12a\x03:WQ\x80\x15\x15\x81\x03a\x03:W\x90V[`@Q=_\x82>=\x90\xFD[\x15a\"\x1EWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a\"4WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a\"hW`\x05\x1B\x01\x90V[a\"CV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a#]a#:a#c\x95a#4a#-\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra#\x04\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\x8DV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aB{V[\x90aB\xC1V[\x92a#4a#Oa#IaC#V[\x94aD\x1AV[\x91a#XaE6V[aB{V[\x91aE\xB0V[\x90\x91V[`@\x80Q\x90\x91\x90a#x\x83\x82a\x03\x8DV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a#\x93\x82a\x06vV[a#\xA0`@Q\x91\x82a\x03\x8DV[\x82\x81R\x80\x92a#\xB1`\x1F\x19\x91a\x06vV[\x01\x90` 6\x91\x017V[\x80Q\x15a\"hW` \x01\x90V[\x80Q\x82\x10\x15a\"hW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03:WQ\x90V[\x91\x90\x91a#\xF8\x83Qa#\x89V[\x92_[\x81Q\x81\x10\x15a$\xABW\x80` a$$a$\x17a$M\x94\x86a#\xC8V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x035W`\x01\x92_\x91a$}W[Pa$v\x82\x88a#\xC8V[R\x01a#\xFBV[a$\x9E\x91P` =\x81\x11a$\xA4W[a$\x96\x81\x83a\x03\x8DV[\x81\x01\x90a#\xDCV[_a$kV[P=a$\x8CV[PPPV[\x90\x81` \x91\x03\x12a\x03:WQa\x06\xD1\x81a\x06eV[\x90a$\xCF\x82a\x06vV[a$\xDC`@Q\x91\x82a\x03\x8DV[\x82\x81R` \x81\x93a$\xEF`\x1F\x19\x91a\x06vV[\x01\x91\x01_[\x82\x81\x10a%\0WPPPV[``\x82\x82\x01R` \x01a$\xF4V[\x90\x81Q\x81\x10\x15a\"hW\x01` \x01\x90V[` \x81\x83\x03\x12a\x03:W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x81Qa%R\x81a\x06vV[\x92a%``@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a%\x88WPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a%{V[\x90a%\xA2\x82a\x06vV[a%\xAF`@Q\x91\x82a\x03\x8DV[\x82\x81R\x80\x92a%\xC0`\x1F\x19\x91a\x06vV[\x01_[\x81\x81\x10a%\xCFWPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03mW` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a%\xC3V[\x90\x81` \x91\x03\x12a\x03:WQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03:W\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x035W_\x95a)2W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x035W`\x04\x96_\x93a)\x10W[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x035W_\x96a(\xEFW[Pa&\xB7\x85\x93\x92\x95Qa$\xC5V[\x94_\x93[\x80Q\x85\x10\x15a(\xE5Wa&\xE2a\r\xF5a&\xD4\x87\x84a%\x0EV[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x035W_\x94a(\xC1W[Pa'2\x84Qa%\x98V[a'<\x88\x8Ba#\xC8V[Ra'G\x87\x8Aa#\xC8V[P_[\x84Q\x81\x10\x15a(\xB0W\x80` a'ca'\x85\x93\x88a#\xC8V[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x035W_\x92a(\x90W[Pa'\xAB\x81\x87a#\xC8V[Q\x8A` \x8Aa'\xBA\x85\x8Ba#\xC8V[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x035Wa(M\x8C\x8Fa(H`\x01\x98a(Y\x97\x89\x97_\x92a(`W[Pa(3a($a\x03\xCFV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a#\xC8V[Q\x90a\r@\x83\x83a#\xC8V[P\x01a'JV[a(\x82\x91\x92P` =\x81\x11a(\x89W[a(z\x81\x83a\x03\x8DV[\x81\x01\x90a&\nV[\x90_a(\x18V[P=a(pV[a(\xA9\x91\x92P` =\x81\x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x90_a'\xA0V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa&\xBBV[a(\xDE\x91\x94P=\x80_\x83>a(\xD6\x81\x83a\x03\x8DV[\x81\x01\x90a%\x1FV[\x92_a''V[PPP\x93PPP\x90V[a)\t\x91\x96P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x94_a&\xA9V[` \x91\x93Pa)+\x90\x82=\x84\x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x92\x90a&\x84V[a)L\x91\x95P` =` \x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x93_a&[V[`@Q\x90a)`\x82a\x03rV[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03:W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x81Qa)\xAA\x81a\x06vV[\x92a)\xB8`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a)\xE0WPPP\x90V[` \x80\x91\x83Qa)\xEF\x81a\x05\xDAV[\x81R\x01\x91\x01\x90a)\xD3V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xD1\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\x8DV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xD1\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a*\x17V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a*yW`\x01\x01\x90V[a*TV[\x90\x81` \x91\x03\x12a\x03:WQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03:W\x90V[\x15a*\xA4WV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a\"hW\x01\x90V[\x90\x81` \x91\x03\x12a\x03:WQa\x06\xD1\x81a\x05\xDAV[_\x19\x81\x14a*yW`\x01\x01\x90V[\x91a+\0` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a*\x17V[\x94\x16\x91\x01RV[5a\x06\xD1\x81a\x05\xDAV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03:W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W` \x01\x91\x816\x03\x83\x13a\x03:WV[` \x81R\x815` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x015a+a\x81a\x05\xDAV[\x16`@\x82\x01R`@\x82\x015`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03:W\x82\x01\x90` \x825\x92\x01`\x01`\x01`@\x1B\x03\x83\x11a\x03:W\x826\x03\x81\x13a\x03:Wa+\xBF``a+\xB8`\x80\x93a\x06\xD1\x96\x85\x84\x88\x01R`\xA0\x87\x01\x91a*\x17V[\x95\x01a\x05\xE8V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15a+\xD1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a,CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a*yWV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a*yWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a*yWV[\x15a,\xEEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a-]\x81a\x05\xDAV[\x16\x84R\x015\x91\x01RV[`@\x81\x01\x92\x91a\x03\xBE\x91\x90a-IV[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a*yWV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a*yWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a*yWV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a*yWV[\x15a-\xE0WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91a.`\x84`\x80\x81\x01\x97a-IV[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[\x15a.{WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x92\x91` ``\x91a.\xE0\x84`\x80\x81\x01\x97a-IV[c\xFF\xFF\xFF\xFF\x815a.\xF0\x81a\x05\xDAV[\x16`@\x85\x01R\x015\x91\x01RV[\x15a/\x04WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a/vWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x15a/\xF4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a0\x89WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a0|V[\x15a0\xA6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[` \x81\x83\x03\x12a\x03:W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03:W\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03:W\x81Qa1]\x81a\x06vV[\x92a1k`@Q\x94\x85a\x03\x8DV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03:W` \x01\x90[\x82\x82\x10a1\x93WPPP\x90V[` \x80\x91\x83Qa1\xA2\x81a\x06eV[\x81R\x01\x91\x01\x90a1\x86V[`@Q\x90a1\xBC`@\x83a\x03\x8DV[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10a2\x86WPPP`\x80a2qa\x06\xD1\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x06\x8DV[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra1\xDDV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a2HV[\x90\x92\x91`\x01a2\xB3\x85a+\x07V[\x94` a3o\x855a2\xDEa2\xD6\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a.tV[a3\x19a2\xF9\x8Ac\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T`@Q\x85\x81\x01\x90a3\x10\x81a\x14\xC1\x8C\x8B\x86a.\xCAV[Q\x90 \x14a.\xFDV[a3Da3>a37\x8Bc\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a/oV[a3ia3[a\x15\\a3V\x89a+\x07V[a,\xB5V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a/\xEDV[\x80a-\xA3V[\x91\x015\x14\x14a8tWa3\x82\x83Qa#\x89V[\x93_[\x84Q\x81\x10\x15a3\xC2W\x80a3\xB1a3\x9E`\x01\x93\x88a#\xC8V[Q\x80Q_R` \x01Q` R`@_ \x90V[a3\xBB\x82\x89a#\xC8V[R\x01a3\x85V[P\x90\x92\x93\x91\x94a3\xFC` \x87\x01\x94` a3\xDB\x87a+\x07V[`@Qa3\xF0\x81a\x14\xC1\x8A\x86\x83\x01\x95\x86a0_V[Q\x90 \x91\x015\x14a0\x9FV[a4\x06\x85Qa#\x89V[\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94_[\x87Q\x81\x10\x15a4\xB6W\x80` a4Ma4m\x93\x89a#\xC8V[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8BZ\xFA\x91\x82\x15a\x035W`\x01\x92a4\x92\x91_\x91a4\x98W[Pa\no\x83\x8Da#\xC8V[\x01a44V[a4\xB0\x91P` =\x81\x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[_a4\x87V[P\x92\x96\x95P\x92P\x92a5\x15a4\xDEa4\xE6`@\x86\x01\x94a4\xD6\x86\x88a+\x11V[\x93\x90\x91a+\x07V[\x926\x91a\x07\xA1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a&)V[\x94_\x91[\x86Q\x83\x10\x15a8\x16W_\x97\x96\x97\x95[a52\x84\x8Aa#\xC8V[QQ\x87\x10\x15a8\x06W\x97a5s\x98` \x80a5Q\x8Aa\x0E\xA9\x89\x87a#\xC8V[Q\x01Q`@Q\x80\x9C\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x99\x8A\x15a\x035W_\x9Aa7\xE6W[P`\x01\x98_[\x85Q\x81\x10\x15a7\xD8Wa5\xB1a5\xA5a$\x17\x83\x89a#\xC8V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x14a5\xC8W`\x01\x01a5\x8CV[P\x99\x90\x97\x91\x98P`\x01_[\x15\x15\x14a5\xE8W[P`\x01\x01\x95\x97\x96\x97a5(V[\x98\x96\x97\x86\x98_\x87\x95\x93\x98a6M`\xFFa6%a\r\xF5a\r\xE7\x8Ca6\x1Fa6\xA2\x9Fa6\x19`\xCDT`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a+\x11V[\x90a*\xB3V[a6?a60a\x03\xC0V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD1Ta6d\x90a5\xA5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x98\x89\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x96\x87\x15a\x035W_\x97a7\xB4W[Pa6\xBF\x87Qa#\x89V[\x98_[\x8AQ\x81\x10\x15a6\xE8W\x80g\x01cEx]\x8A\0\0a6\xE1`\x01\x93\x8Ea#\xC8V[R\x01a6\xC2V[P\x9A\x92\x94\x96a7&`\xFFa7\ra\r\xF5a\r\xE7\x8F\x9D\x97\x9F\x96\x9E\x96a6\x1F\x8E\x8E\x92a+\x11V[a7\x18a($a\x03\xDEV[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01Ra78a1\xADV[`\x80\x83\x01R`\xD0Ta7T\x90a5\xA5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03:W`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90a7\x80\x90`\x04\x83\x01a2\x01V[\x03\x92Z\xF1\x91\x82\x15a\x035W`\x01\x92a7\x9AW[P\x90a5\xDBV[\x80a7\xA8_a7\xAE\x93a\x03\x8DV[\x80a\x05hV[_a7\x93V[a7\xD1\x91\x97P=\x80_\x83>a7\xC9\x81\x83a\x03\x8DV[\x81\x01\x90a1*V[\x95_a6\xB4V[P\x99\x90\x97\x91\x98`\x01\x90a5\xD3V[a7\xFF\x91\x9AP` =\x81\x11a\n\xABWa\n\x9D\x81\x83a\x03\x8DV[\x98_a5\x86V[\x96\x97\x96\x95P`\x01\x90\x92\x01\x91a5\x19V[PPPPP\x91\x90Pa8Fa89\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3V[PPPc\xFF\xFF\xFF\xFF3\x91\x16\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05_\x80\xA3V[`@Q\x90a8\xB2\x82a\x03rV[_``\x83\x82\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81R\x81Q` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`\x80c\xFF\xFF\xFF\xFF``a9\x04`@\x86\x01Q\x84\x83\x87\x01R`\xA0\x86\x01\x90a1\xDDV[\x94\x01Q\x16\x91\x01R\x90V[`@Q\x90a9\x1B\x82a\x03RV[``` \x83\x82\x81R\x01RV[\x15a9.WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a9CWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a9YWV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a9oWV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03:WQa\x06\xD1\x81a\x11RV[_\x19\x81\x01\x91\x90\x82\x11a*yWV[\x15a9\xA8WV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03:WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03:W\x90V[\x15a9\xDFWV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a*yWV[\x15a:\x15WV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a:+WV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[\x94\x93\x92\x90\x91\x93a:Ha9\x0EV[Pa:T\x85\x15\x15a9'V[`@\x84\x01QQ\x85\x14\x80a@\xF7W[\x80a@\xE9W[\x80a@\xDBW[a:w\x90a9<V[a:\x89` \x85\x01QQ\x85QQ\x14a9RV[a:\xA0c\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a9hV[a:\xA8a\x03\xC0V[_\x81R_` \x82\x01R\x92a:\xBAa9\x0EV[a:\xC3\x87a#\x89V[` \x82\x01Ra:\xD1\x87a#\x89V[\x81Ra:\xDBa9\x0EV[\x92a:\xEA` \x88\x01QQa#\x89V[\x84Ra:\xFA` \x88\x01QQa#\x89V[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x035Wa;c\x91_\x91a@\xACW[Pa;^6\x8B\x87a\x07\xA1V[aF\xDEV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a<\xC2W` \x88a;\xB7a\rn\x8Ca;\xAF\x8F\x96\x86\x8Ea;\x94a3\x9E\x86\x80\x95a#\xC8V[a;\xA1\x84\x84\x84\x01Qa#\xC8V[R\x82a<\x8FW[\x01Qa#\xC8V[Q\x95Qa#\xC8V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x035Wa#4\x8Aa<d\x8Fa<]\x8F\x84` \x8F\x92a<T\x93a<L\x84`\x01\x9Ea<j\x9E_\x91a<rW[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa#\xC8V[R\x01Qa#\xC8V[Q\x93\x8DQa#\xC8V[Q\x16aG\tV[\x90aG:V[\x97\x01\x96a;gV[a<\x89\x91P\x86=\x81\x11a\x0F8Wa\x0F%\x81\x83a\x03\x8DV[_a<=V[a<\xBDa<\x9F\x84\x84\x84\x01Qa#\xC8V[Qa<\xB6\x84\x84\x01Qa<\xB0\x87a9\x93V[\x90a#\xC8V[Q\x10a9\xA1V[a;\xA8V[P\x90\x95\x97\x94\x96Pa<\xD7\x91\x98\x93\x92\x99PaG\xF7V[\x91_\x90[\x80\x82\x10a=/WPPP\x92a=\x16a=\x11a=\na=)\x95\x85a\x14\xC1\x98`\x80``` \x99\x01Q\x92\x01Q\x92a\"\x81V[\x91\x90a:\x0EV[a:$V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a0_V[Q\x90 \x90V[\x91\x84\x95\x96\x93a=la\r\xF5a\r\xE7\x85\x87a=f\x87\x9F\x98\x9E\x9Ba3\x9E\x84`@a=X\x93\x01Qa#\xC8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x96a*\xB3V[` \x87a=\x80a\rn\x8D`\xA0\x8D\x01Qa#\xC8V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x92\x83\x15a\x035Wa>\x1C\x93a>\x07\x92_\x91a@~W[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a9\xD8V[a>\x15\x89`@\x89\x01Qa#\xC8V[Q\x90aB\xC1V[\x90a>.a\r\xF5a\r\xE7\x8A\x86\x8Da*\xB3V[` \x86a>Ba\rn\x8C`\xC0\x8C\x01Qa#\xC8V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x035Wa\x160\x8A` \x8Ea;\xA8a>\xD8\x95a>\xE3\x97_\x91a@aW[Pa>\xCA\x85\x85\x85\x01Qa#\xC8V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a>\xCA\x8A\x8DQa#\xC8V[_\x96_[` \x88\x01QQ\x81\x10\x15a@MW\x89\x8Ba?#a?\x04\x84\x8AQa#\xC8V[Qa?\x16a\r\xF5a\r\xE7\x86\x8C\x87a*\xB3V[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a?2W[PP`\x01\x01a>\xE7V[\x88\x88a?\xB3a?sa\rn\x87\x9F\x8F\x97\x80`\xE0\x8F\x84\x9C` a?ba\r\xF5a\r\xE7\x83\x9F\x98a?j\x96a\x0E\xA9\x9Aa*\xB3V[\x9A\x01Qa#\xC8V[Q\x9A\x01Qa#\xC8V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x95\x90\x95R\x91\x90\x93\x16`d\x84\x01R\x82\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x035Wa@\x1C\x8E`\x01\x94\x8E\x86\x95_\x92a@'W[Pa@\x16a>\xCA\x92\x93Q\x93a@\x11a\x160\x84\x87a#\xC8V[a9\xEEV[\x92a#\xC8V[\x01\x98\x90P\x89\x8Ba?(V[a>\xCA\x92Pa@Fa@\x16\x91` =\x81\x11a(\x89Wa(z\x81\x83a\x03\x8DV[\x92Pa?\xF9V[P\x95\x94\x97`\x01\x91\x99\x93\x92\x94\x97P\x01\x90a<\xDBV[a@x\x91P\x84=\x81\x11a(\x89Wa(z\x81\x83a\x03\x8DV[_a>\xBCV[a@\x9F\x91P` =\x81\x11a@\xA5W[a@\x97\x81\x83a\x03\x8DV[\x81\x01\x90a9\xB7V[_a=\xF1V[P=a@\x8DV[a@\xCE\x91P` =` \x11a@\xD4W[a@\xC6\x81\x83a\x03\x8DV[\x81\x01\x90a9~V[_a;RV[P=a@\xBCV[P`\xE0\x84\x01QQ\x85\x14a:nV[P`\xC0\x84\x01QQ\x85\x14a:hV[P`\xA0\x84\x01QQ\x85\x14a:bV[\x15aA\x0CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aAq\x90aH\xD4V[`\xCE\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xCD\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aBD\x82a\x03RV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aB`\x81\x84a\x03\x8DV[6\x837V[`@Q\x90aBt` \x83a\x03\x8DV[` 6\x837V[\x91\x90`@\x90``aB\x8AaB7V[\x94\x85\x92` \x85Q\x92aB\x9C\x85\x85a\x03\x8DV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aB\xBFWV[\xFE[` \x92\x91`\x80`@\x92aB\xD2aB7V[\x95\x86\x93\x81\x86Q\x93aB\xE3\x86\x86a\x03\x8DV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aB\xBFW\x15aC\x14WV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaC/\x81a\x03RV[`@\x90\x81QaC>\x83\x82a\x03\x8DV[\x826\x827\x81R` \x82Q\x91aCS\x84\x84a\x03\x8DV[\x836\x847\x01R\x80QaCe\x82\x82a\x03\x8DV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aC\xBB\x83\x83a\x03\x8DV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaD\x10\x83Q\x93\x84a\x03\x8DV[\x82R` \x82\x01R\x90V[_Q` aJ__9_Q\x90_R\x90aD1aB7V[P_\x91\x90\x06` `\xC0\x83[aE1W_\x93_Q` aJ__9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaDg\x85\x82a\x03\x8DV[\x846\x827\x84\x81\x85`@QaD{\x82\x82a\x03\x8DV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aJ__9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aB\xBFWaD\xE5\x90aJHV[Q\x91aE1W_Q` aJ__9_Q\x90_R\x82\x80\t\x14aE\x1CWP_Q` aJ__9_Q\x90_R`\x01_\x94\x08\x92\x93aD<V[\x92\x93PPaE(a\x03\xC0V[\x92\x83R\x82\x01R\x90V[a\"mV[aE>aB7V[P`@QaEK\x81a\x03RV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x01\x82\x01\x80\x92\x11a*yWV[\x90`\x02\x82\x01\x80\x92\x11a*yWV[\x90`\x03\x82\x01\x80\x92\x11a*yWV[\x90`\x04\x82\x01\x80\x92\x11a*yWV[\x90`\x05\x82\x01\x80\x92\x11a*yWV[\x90`\x0C\x81\x10\x15a\"hW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aE\xBE`@a\x03\xEDV[\x94\x85R` \x85\x01RaE\xD0`@a\x03\xEDV[\x91\x82R` \x82\x01RaE\xE0aBOV[\x92_[`\x02\x81\x10aF\rWPPP` a\x01\x80\x92aE\xFCaBeV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aF\x19`\x01\x92a-\x8DV[aF#\x82\x85a\"WV[QQaF/\x82\x89aE\x9FV[R` aF<\x83\x86a\"WV[Q\x01QaFQaFK\x83aEYV[\x89aE\x9FV[RaF\\\x82\x86a\"WV[QQQaFkaFK\x83aEgV[RaF\x81aFy\x83\x87a\"WV[QQ` \x01\x90V[QaF\x8EaFK\x83aEuV[R` aF\x9B\x83\x87a\"WV[Q\x01QQaF\xABaFK\x83aE\x83V[RaF\xD7aF\xD1aF\xCA` aF\xC1\x86\x8Aa\"WV[Q\x01Q` \x01\x90V[Q\x92aE\x91V[\x88aE\x9FV[R\x01aE\xE3V[\x90`\x01aF\xEC`\xFF\x93aI\xD0V[\x92\x83\x92\x16\x1B\x11\x15aF\xFAW\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aG\x15WP\x90V[_\x19\x81\x01\x81\x81\x11a*yWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a*yW`\x01\x01\x90\x80aG\rV[\x90aGCaB7V[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aG\xE8W`\x01\x82\x14aG\xE3WaGda\x03\xC0V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aG\x89WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aG\xC3W[`\x01aG\xB9aG\xAE\x83`\xFF\x94aB\xC1V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aGuV[\x94`\x01aG\xB9aG\xAEaG\xD8\x89`\xFF\x95aB\xC1V[\x98\x93PPPPaG\x9DV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aG\xFFaB7V[P\x80Q\x90\x81\x15\x80aHpW[\x15aH,WPP`@QaH `@\x82a\x03\x8DV[_\x81R_` \x82\x01R\x90V[` _Q` aJ__9_Q\x90_R\x91\x01Q\x06_Q` aJ__9_Q\x90_R\x03_Q` aJ__9_Q\x90_R\x81\x11a*yW`@Q\x91aD\x10\x83a\x03RV[P` \x81\x01Q\x15aH\x0BV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aH\x90WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaI(\x82aG\tV[\x16aI2\x81a\x07\x86V[\x90aI@`@Q\x92\x83a\x03\x8DV[\x80\x82RaIO`\x1F\x19\x91a\x07\x86V[\x016` \x83\x017__[\x82Q\x82\x10\x80aI\xAFW[\x15aI\xA8W`\x01\x81\x1B\x84\x16aI\x81W[aI|\x90a*\xD4V[aIYV[\x90`\x01aI|\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaI\x9E\x82\x87a%\x0EV[S\x01\x91\x90PaIsV[PP\x90P\x90V[Pa\x01\0\x81\x10aIcV[\x15aI\xC1WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aJ9W\x81Q\x15aJ4W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aJ/W`\x01\x90aJ\x1AaJ\x10a\r\xF5a&\xD4\x86\x89a%\x0EV[`\xFF`\x01\x91\x16\x1B\x90V[\x90aJ&\x81\x83\x11aI\xBAV[\x17\x91\x01\x90aI\xF1V[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aJOWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x0F\x1C\x7F=\x02['\x13\xAF\xFF\xEC{\xD7\xB2g\x06uP\x9FT\xE7\\\x92\xF2\xF9\xE7\xF7\x17)b-\x8BdsolcC\0\x08\x1B\x003",
    );
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BitmapValueTooLarge()` and selector `0xca957333`.
```solidity
error BitmapValueTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapValueTooLarge {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BitmapValueTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapValueTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapValueTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapValueTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapValueTooLarge()";
            const SELECTOR: [u8; 4] = [202u8, 149u8, 115u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BytesArrayLengthTooLong()` and selector `0xfb4a9c8e`.
```solidity
error BytesArrayLengthTooLong();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayLengthTooLong {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BytesArrayLengthTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayLengthTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayLengthTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayLengthTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayLengthTooLong()";
            const SELECTOR: [u8; 4] = [251u8, 74u8, 156u8, 142u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BytesArrayNotOrdered()` and selector `0x80c88348`.
```solidity
error BytesArrayNotOrdered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayNotOrdered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BytesArrayNotOrdered> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayNotOrdered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayNotOrdered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayNotOrdered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayNotOrdered()";
            const SELECTOR: [u8; 4] = [128u8, 200u8, 131u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CurrentlyPaused()` and selector `0x840a48d5`.
```solidity
error CurrentlyPaused();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CurrentlyPaused {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CurrentlyPaused> for UnderlyingRustTuple<'_> {
            fn from(value: CurrentlyPaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CurrentlyPaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrentlyPaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrentlyPaused()";
            const SELECTOR: [u8; 4] = [132u8, 10u8, 72u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECAddFailed()` and selector `0xd4b68fd7`.
```solidity
error ECAddFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECAddFailed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECAddFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECAddFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECAddFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECAddFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECAddFailed()";
            const SELECTOR: [u8; 4] = [212u8, 182u8, 143u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECMulFailed()` and selector `0x4633be32`.
```solidity
error ECMulFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECMulFailed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECMulFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECMulFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECMulFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECMulFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECMulFailed()";
            const SELECTOR: [u8; 4] = [70u8, 51u8, 190u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExpModFailed()` and selector `0xd51edae3`.
```solidity
error ExpModFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpModFailed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpModFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ExpModFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpModFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpModFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpModFailed()";
            const SELECTOR: [u8; 4] = [213u8, 30u8, 218u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputAddressZero()` and selector `0x73632176`.
```solidity
error InputAddressZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputAddressZero {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputAddressZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputAddressZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputAddressZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputAddressZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputAddressZero()";
            const SELECTOR: [u8; 4] = [115u8, 99u8, 33u8, 118u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputArrayLengthMismatch()` and selector `0x43714afd`.
```solidity
error InputArrayLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthMismatch {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputArrayLengthMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InputArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [67u8, 113u8, 74u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputEmptyQuorumNumbers()` and selector `0x1f0405a0`.
```solidity
error InputEmptyQuorumNumbers();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputEmptyQuorumNumbers {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputEmptyQuorumNumbers> for UnderlyingRustTuple<'_> {
            fn from(value: InputEmptyQuorumNumbers) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputEmptyQuorumNumbers {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputEmptyQuorumNumbers {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputEmptyQuorumNumbers()";
            const SELECTOR: [u8; 4] = [31u8, 4u8, 5u8, 160u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputNonSignerLengthMismatch()` and selector `0x5f832f41`.
```solidity
error InputNonSignerLengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputNonSignerLengthMismatch {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputNonSignerLengthMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: InputNonSignerLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InputNonSignerLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputNonSignerLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputNonSignerLengthMismatch()";
            const SELECTOR: [u8; 4] = [95u8, 131u8, 47u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBLSPairingKey()` and selector `0x67988d33`.
```solidity
error InvalidBLSPairingKey();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSPairingKey {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBLSPairingKey> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSPairingKey) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSPairingKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSPairingKey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBLSPairingKey()";
            const SELECTOR: [u8; 4] = [103u8, 152u8, 141u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBLSSignature()` and selector `0xab1b236b`.
```solidity
error InvalidBLSSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBLSSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBLSSignature()";
            const SELECTOR: [u8; 4] = [171u8, 27u8, 35u8, 107u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidNewPausedStatus()` and selector `0xc61dca5d`.
```solidity
error InvalidNewPausedStatus();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNewPausedStatus {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidNewPausedStatus> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNewPausedStatus) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNewPausedStatus {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNewPausedStatus {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNewPausedStatus()";
            const SELECTOR: [u8; 4] = [198u8, 29u8, 202u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidQuorumApkHash()` and selector `0xe1310aed`.
```solidity
error InvalidQuorumApkHash();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorumApkHash {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidQuorumApkHash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorumApkHash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorumApkHash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorumApkHash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorumApkHash()";
            const SELECTOR: [u8; 4] = [225u8, 49u8, 10u8, 237u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidReferenceBlocknumber()` and selector `0x4b874f45`.
```solidity
error InvalidReferenceBlocknumber();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlocknumber {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidReferenceBlocknumber>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlocknumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidReferenceBlocknumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlocknumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlocknumber()";
            const SELECTOR: [u8; 4] = [75u8, 135u8, 79u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NonSignerPubkeysNotSorted()` and selector `0xff719414`.
```solidity
error NonSignerPubkeysNotSorted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerPubkeysNotSorted {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NonSignerPubkeysNotSorted>
        for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerPubkeysNotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NonSignerPubkeysNotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonSignerPubkeysNotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonSignerPubkeysNotSorted()";
            const SELECTOR: [u8; 4] = [255u8, 113u8, 148u8, 20u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyPauser()` and selector `0x75df51dc`.
```solidity
error OnlyPauser();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyPauser {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyPauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyPauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyPauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyPauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyPauser()";
            const SELECTOR: [u8; 4] = [117u8, 223u8, 81u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyRegistryCoordinatorOwner()` and selector `0xe0e1e762`.
```solidity
error OnlyRegistryCoordinatorOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinatorOwner {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyRegistryCoordinatorOwner>
        for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinatorOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OnlyRegistryCoordinatorOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinatorOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRegistryCoordinatorOwner()";
            const SELECTOR: [u8; 4] = [224u8, 225u8, 231u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyUnpauser()` and selector `0x794821ff`.
```solidity
error OnlyUnpauser();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyUnpauser {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyUnpauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyUnpauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyUnpauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyUnpauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyUnpauser()";
            const SELECTOR: [u8; 4] = [121u8, 72u8, 33u8, 255u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
```solidity
error OperatorNotRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ScalarTooLarge()` and selector `0xff89d4fa`.
```solidity
error ScalarTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ScalarTooLarge {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ScalarTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: ScalarTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ScalarTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ScalarTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ScalarTooLarge()";
            const SELECTOR: [u8; 4] = [255u8, 137u8, 212u8, 250u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))` and selector `0x1695b8d06ec800b4615e745cfb5bd00c1f2875615d42925c3b5afa543bb24c48`.
```solidity
event NewTaskCreated(uint32 indexed taskIndex, IIncredibleSquaringTaskManager.Task task);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewTaskCreated {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewTaskCreated {
            type DataTuple<'a> = (IIncredibleSquaringTaskManager::Task,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                22u8, 149u8, 184u8, 208u8, 110u8, 200u8, 0u8, 180u8, 97u8, 94u8, 116u8,
                92u8, 251u8, 91u8, 208u8, 12u8, 31u8, 40u8, 117u8, 97u8, 93u8, 66u8,
                146u8, 92u8, 59u8, 90u8, 250u8, 84u8, 59u8, 178u8, 76u8, 72u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    task: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewTaskCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewTaskCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewTaskCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Paused(address,uint256)` and selector `0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d`.
```solidity
event Paused(address indexed account, uint256 newPausedStatus);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8,
                188u8, 152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8,
                11u8, 182u8, 224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TaskChallengedSuccessfully(uint32,address)` and selector `0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec`.
```solidity
event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskChallengedSuccessfully {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TaskChallengedSuccessfully {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedSuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                194u8, 13u8, 27u8, 176u8, 241u8, 98u8, 54u8, 128u8, 48u8, 107u8, 131u8,
                212u8, 255u8, 75u8, 185u8, 154u8, 43u8, 235u8, 157u8, 134u8, 217u8,
                120u8, 50u8, 243u8, 202u8, 64u8, 253u8, 19u8, 162u8, 157u8, 241u8, 236u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    challenger: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.taskIndex.clone(),
                    self.challenger.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.challenger,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskChallengedSuccessfully {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskChallengedSuccessfully> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &TaskChallengedSuccessfully,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TaskChallengedUnsuccessfully(uint32,address)` and selector `0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05`.
```solidity
event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskChallengedUnsuccessfully {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TaskChallengedUnsuccessfully {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedUnsuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8,
                68u8, 105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8,
                165u8, 27u8, 75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    challenger: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.taskIndex.clone(),
                    self.challenger.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.challenger,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskChallengedUnsuccessfully {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskChallengedUnsuccessfully> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &TaskChallengedUnsuccessfully,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TaskCompleted(uint32)` and selector `0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430`.
```solidity
event TaskCompleted(uint32 indexed taskIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskCompleted {
        #[allow(missing_docs)]
        pub taskIndex: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TaskCompleted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "TaskCompleted(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                154u8, 20u8, 79u8, 34u8, 138u8, 147u8, 27u8, 157u8, 13u8, 22u8, 150u8,
                251u8, 205u8, 175u8, 49u8, 11u8, 36u8, 181u8, 210u8, 210u8, 30u8, 121u8,
                157u8, 182u8, 35u8, 252u8, 152u8, 106u8, 15u8, 84u8, 116u8, 48u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { taskIndex: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskCompleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TaskResponded((uint32,uint256),(uint32,bytes32))` and selector `0x349c1ee60e4e8972ee9dba642c1774543d5c4136879b7f4caaf04bf81a487a2a`.
```solidity
event TaskResponded(IIncredibleSquaringTaskManager.TaskResponse taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata taskResponseMetadata);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskResponded {
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TaskResponded {
            type DataTuple<'a> = (
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TaskResponded((uint32,uint256),(uint32,bytes32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                52u8, 156u8, 30u8, 230u8, 14u8, 78u8, 137u8, 114u8, 238u8, 157u8, 186u8,
                100u8, 44u8, 23u8, 116u8, 84u8, 61u8, 92u8, 65u8, 54u8, 135u8, 155u8,
                127u8, 76u8, 170u8, 240u8, 75u8, 248u8, 26u8, 72u8, 122u8, 42u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskResponse: data.0,
                    taskResponseMetadata: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponseMetadata,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskResponded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskResponded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskResponded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Unpaused(address,uint256)` and selector `0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c`.
```solidity
event Unpaused(address indexed account, uint256 newPausedStatus);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8,
                2u8, 188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8,
                228u8, 152u8, 107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _slashingRegistryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _taskResponseWindowBlock: u32,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._slashingRegistryCoordinator,
                        value._pauserRegistry,
                        value._taskResponseWindowBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _slashingRegistryCoordinator: tuple.0,
                        _pauserRegistry: tuple.1,
                        _taskResponseWindowBlock: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slashingRegistryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._taskResponseWindowBlock,
                    ),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TASK_CHALLENGE_WINDOW_BLOCK()` and selector `0xf63c5bab`.
```solidity
function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`TASK_CHALLENGE_WINDOW_BLOCK()`](TASK_CHALLENGE_WINDOW_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TASK_CHALLENGE_WINDOW_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TASK_CHALLENGE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_CHALLENGE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = TASK_CHALLENGE_WINDOW_BLOCKReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TASK_CHALLENGE_WINDOW_BLOCK()";
            const SELECTOR: [u8; 4] = [246u8, 60u8, 91u8, 171u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TASK_RESPONSE_WINDOW_BLOCK()` and selector `0x1ad43189`.
```solidity
function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`TASK_RESPONSE_WINDOW_BLOCK()`](TASK_RESPONSE_WINDOW_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TASK_RESPONSE_WINDOW_BLOCKCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TASK_RESPONSE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_RESPONSE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = TASK_RESPONSE_WINDOW_BLOCKReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TASK_RESPONSE_WINDOW_BLOCK()";
            const SELECTOR: [u8; 4] = [26u8, 212u8, 49u8, 137u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `WADS_TO_SLASH()` and selector `0x5a2d7f02`.
```solidity
function WADS_TO_SLASH() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WADS_TO_SLASHCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`WADS_TO_SLASH()`](WADS_TO_SLASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WADS_TO_SLASHReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<WADS_TO_SLASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: WADS_TO_SLASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WADS_TO_SLASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<WADS_TO_SLASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: WADS_TO_SLASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WADS_TO_SLASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for WADS_TO_SLASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = WADS_TO_SLASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WADS_TO_SLASH()";
            const SELECTOR: [u8; 4] = [90u8, 45u8, 127u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `aggregator()` and selector `0x245a7bfc`.
```solidity
function aggregator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`aggregator()`](aggregatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<aggregatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: aggregatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<aggregatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: aggregatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for aggregatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = aggregatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "aggregator()";
            const SELECTOR: [u8; 4] = [36u8, 90u8, 123u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allTaskHashes(uint32)` and selector `0x2d89f6fc`.
```solidity
function allTaskHashes(uint32) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`allTaskHashes(uint32)`](allTaskHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allTaskHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allTaskHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskHashesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskHashesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskHashes(uint32)";
            const SELECTOR: [u8; 4] = [45u8, 137u8, 246u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allTaskResponses(uint32)` and selector `0x2cb223d5`.
```solidity
function allTaskResponses(uint32) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`allTaskResponses(uint32)`](allTaskResponsesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allTaskResponsesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allTaskResponsesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allTaskResponsesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allTaskResponsesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskResponsesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskResponsesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskResponses(uint32)";
            const SELECTOR: [u8; 4] = [44u8, 178u8, 35u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
```solidity
function allocationManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allocationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `blsApkRegistry()` and selector `0x5df45946`.
```solidity
function blsApkRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`blsApkRegistry()`](blsApkRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blsApkRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<blsApkRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blsApkRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blsApkRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = blsApkRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blsApkRegistry()";
            const SELECTOR: [u8; 4] = [93u8, 244u8, 89u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`.
```solidity
function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
        #[allow(missing_docs)]
        pub params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {
        #[allow(missing_docs)]
        pub _0: <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                u32,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkSignaturesCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesCall) -> Self {
                    (
                        value.msgHash,
                        value.quorumNumbers,
                        value.referenceBlockNumber,
                        value.params,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        quorumNumbers: tuple.1,
                        referenceBlockNumber: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkSignaturesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkSignaturesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkSignaturesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = (
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [110u8, 251u8, 70u8, 54u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `createNewTask(uint256,uint32,bytes)` and selector `0x6b92787e`.
```solidity
function createNewTask(uint256 numberToBeSquared, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskCall {
        #[allow(missing_docs)]
        pub numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub quorumThresholdPercentage: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`createNewTask(uint256,uint32,bytes)`](createNewTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u32,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createNewTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskCall) -> Self {
                    (
                        value.numberToBeSquared,
                        value.quorumThresholdPercentage,
                        value.quorumNumbers,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        numberToBeSquared: tuple.0,
                        quorumThresholdPercentage: tuple.1,
                        quorumNumbers: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createNewTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createNewTaskCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createNewTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createNewTask(uint256,uint32,bytes)";
            const SELECTOR: [u8; 4] = [107u8, 146u8, 120u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.numberToBeSquared),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
```solidity
function delegation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`delegation()`](delegationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegation()";
            const SELECTOR: [u8; 4] = [223u8, 92u8, 247u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `generator()` and selector `0x7afa1eed`.
```solidity
function generator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`generator()`](generatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<generatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: generatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for generatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<generatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: generatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for generatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for generatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = generatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "generator()";
            const SELECTOR: [u8; 4] = [122u8, 250u8, 30u8, 237u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getBatchOperatorFromId(address,bytes32[])` and selector `0x4d2b57fe`.
```solidity
function getBatchOperatorFromId(address registryCoordinator, bytes32[] memory operatorIds) external view returns (address[] memory operators);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getBatchOperatorFromId(address,bytes32[])`](getBatchOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdReturn {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBatchOperatorFromIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdCall) -> Self {
                    (value.registryCoordinator, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchOperatorFromIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorIds: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBatchOperatorFromIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdReturn) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorFromIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchOperatorFromIdReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchOperatorFromId(address,bytes32[])";
            const SELECTOR: [u8; 4] = [77u8, 43u8, 87u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getBatchOperatorId(address,address[])` and selector `0x31b36bd9`.
```solidity
function getBatchOperatorId(address registryCoordinator, address[] memory operators) external view returns (bytes32[] memory operatorIds);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getBatchOperatorId(address,address[])`](getBatchOperatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdReturn {
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBatchOperatorIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdCall) -> Self {
                    (value.registryCoordinator, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchOperatorIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operators: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBatchOperatorIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdReturn) -> Self {
                    (value.operatorIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchOperatorIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorIds: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchOperatorIdReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchOperatorId(address,address[])";
            const SELECTOR: [u8; 4] = [49u8, 179u8, 107u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`.
```solidity
function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes memory quorumNumbers, bytes32[] memory nonSignerOperatorIds) external view returns (OperatorStateRetriever.CheckSignaturesIndices memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub nonSignerOperatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCheckSignaturesIndices(address,uint32,bytes,bytes32[])`](getCheckSignaturesIndicesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesReturn {
        #[allow(missing_docs)]
        pub _0: <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCheckSignaturesIndicesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesCall) -> Self {
                    (
                        value.registryCoordinator,
                        value.referenceBlockNumber,
                        value.quorumNumbers,
                        value.nonSignerOperatorIds,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCheckSignaturesIndicesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        referenceBlockNumber: tuple.1,
                        quorumNumbers: tuple.2,
                        nonSignerOperatorIds: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                OperatorStateRetriever::CheckSignaturesIndices,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCheckSignaturesIndicesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCheckSignaturesIndicesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCheckSignaturesIndicesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCheckSignaturesIndicesReturn;
            type ReturnTuple<'a> = (OperatorStateRetriever::CheckSignaturesIndices,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCheckSignaturesIndices(address,uint32,bytes,bytes32[])";
            const SELECTOR: [u8; 4] = [79u8, 115u8, 159u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerOperatorIds),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`.
```solidity
function getOperatorState(address registryCoordinator, bytes memory quorumNumbers, uint32 blockNumber) external view returns (OperatorStateRetriever.Operator[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_0Call {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorState(address,bytes,uint32)`](getOperatorState_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_0Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
            >,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
                u32,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorState_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Call) -> Self {
                    (value.registryCoordinator, value.quorumNumbers, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorState_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        quorumNumbers: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
                    >,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorState_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorState_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(address,bytes,uint32)";
            const SELECTOR: [u8; 4] = [53u8, 99u8, 176u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`.
```solidity
function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) external view returns (uint256, OperatorStateRetriever.Operator[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_1Call {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorState(address,bytes32,uint32)`](getOperatorState_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_1Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
            >,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                u32,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorState_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Call) -> Self {
                    (value.registryCoordinator, value.operatorId, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorState_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorId: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
                    >,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorState_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Return) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorState_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(address,bytes32,uint32)";
            const SELECTOR: [u8; 4] = [206u8, 253u8, 193u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)` and selector `0x5c155662`.
```solidity
function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] memory operatorIds, uint32 blockNumber) external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)`](getQuorumBitmapsAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                u32,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberCall) -> Self {
                    (value.registryCoordinator, value.operatorIds, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapsAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorIds: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumBitmapsAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapsAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapsAtBlockNumberReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)";
            const SELECTOR: [u8; 4] = [92u8, 21u8, 86u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTaskResponseWindowBlock()` and selector `0xf5c9899d`.
```solidity
function getTaskResponseWindowBlock() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTaskResponseWindowBlock()`](getTaskResponseWindowBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTaskResponseWindowBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTaskResponseWindowBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTaskResponseWindowBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTaskResponseWindowBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTaskResponseWindowBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTaskResponseWindowBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTaskResponseWindowBlock()";
            const SELECTOR: [u8; 4] = [245u8, 201u8, 137u8, 157u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(address,address,address,address,address,address)` and selector `0xcc2a9a5b`.
```solidity
function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _aggregator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _generator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _slasher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _serviceManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address,address,address,address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.initialOwner,
                        value._aggregator,
                        value._generator,
                        value._allocationManager,
                        value._slasher,
                        value._serviceManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        _aggregator: tuple.1,
                        _generator: tuple.2,
                        _allocationManager: tuple.3,
                        _slasher: tuple.4,
                        _serviceManager: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address,address,address,address,address)";
            const SELECTOR: [u8; 4] = [204u8, 42u8, 154u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initialOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._aggregator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._generator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slasher,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._serviceManager,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `instantSlasher()` and selector `0x9b290e98`.
```solidity
function instantSlasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`instantSlasher()`](instantSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<instantSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for instantSlasherCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<instantSlasherReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for instantSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for instantSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = instantSlasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "instantSlasher()";
            const SELECTOR: [u8; 4] = [155u8, 41u8, 14u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `latestTaskNum()` and selector `0x8b00ce7c`.
```solidity
function latestTaskNum() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`latestTaskNum()`](latestTaskNumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestTaskNumCall> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestTaskNumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestTaskNumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = latestTaskNumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestTaskNum()";
            const SELECTOR: [u8; 4] = [139u8, 0u8, 206u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pause(uint256)` and selector `0x136439dd`.
```solidity
function pause(uint256 newPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pause(uint256)`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPausedStatus: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause(uint256)";
            const SELECTOR: [u8; 4] = [19u8, 100u8, 57u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pauseAll()` and selector `0x595c6a67`.
```solidity
function pauseAll() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllCall {}
    ///Container type for the return parameters of the [`pauseAll()`](pauseAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseAllCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauseAll()";
            const SELECTOR: [u8; 4] = [89u8, 92u8, 106u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused(uint8)` and selector `0x5ac86ab7`.
```solidity
function paused(uint8 index) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Call {
        #[allow(missing_docs)]
        pub index: u8,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Call) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused(uint8)";
            const SELECTOR: [u8; 4] = [90u8, 200u8, 106u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Call {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](paused_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pauserRegistry()` and selector `0x886f1195`.
```solidity
function pauserRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`pauserRegistry()`](pauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauserRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauserRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauserRegistry()";
            const SELECTOR: [u8; 4] = [136u8, 111u8, 17u8, 149u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])` and selector `0x6b532e9e`.
```solidity
function raiseAndResolveChallenge(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IIncredibleSquaringTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeCall {
        #[allow(missing_docs)]
        pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])`](raiseAndResolveChallengeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<raiseAndResolveChallengeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeCall) -> Self {
                    (
                        value.task,
                        value.taskResponse,
                        value.taskResponseMetadata,
                        value.pubkeysOfNonSigningOperators,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for raiseAndResolveChallengeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        taskResponse: tuple.1,
                        taskResponseMetadata: tuple.2,
                        pubkeysOfNonSigningOperators: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<raiseAndResolveChallengeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for raiseAndResolveChallengeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for raiseAndResolveChallengeCall {
            type Parameters<'a> = (
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IIncredibleSquaringTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = raiseAndResolveChallengeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32,bytes32),(uint256,uint256)[])";
            const SELECTOR: [u8; 4] = [107u8, 83u8, 46u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponseMetadata,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeysOfNonSigningOperators,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
```solidity
function registryCoordinator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`registryCoordinator()`](registryCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registryCoordinator()";
            const SELECTOR: [u8; 4] = [109u8, 20u8, 169u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x5baec9a0`.
```solidity
function respondToTask(IIncredibleSquaringTaskManager.Task memory task, IIncredibleSquaringTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskCall {
        #[allow(missing_docs)]
        pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](respondToTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<respondToTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskCall) -> Self {
                    (value.task, value.taskResponse, value.nonSignerStakesAndSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        taskResponse: tuple.1,
                        nonSignerStakesAndSignature: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<respondToTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respondToTaskCall {
            type Parameters<'a> = (
                IIncredibleSquaringTaskManager::Task,
                IIncredibleSquaringTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = respondToTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respondToTask((uint256,uint32,bytes,uint32),(uint32,uint256),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [91u8, 174u8, 201u8, 160u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IIncredibleSquaringTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IIncredibleSquaringTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerStakesAndSignature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `serviceManager()` and selector `0x3998fdd3`.
```solidity
function serviceManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`serviceManager()`](serviceManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<serviceManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<serviceManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for serviceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "serviceManager()";
            const SELECTOR: [u8; 4] = [57u8, 152u8, 253u8, 211u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
```solidity
function stakeRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `taskNumber()` and selector `0x72d18e8d`.
```solidity
function taskNumber() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`taskNumber()`](taskNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<taskNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: taskNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<taskNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: taskNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskNumber()";
            const SELECTOR: [u8; 4] = [114u8, 209u8, 142u8, 141u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `taskSuccesfullyChallenged(uint32)` and selector `0x5decc3f5`.
```solidity
function taskSuccesfullyChallenged(uint32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`taskSuccesfullyChallenged(uint32)`](taskSuccesfullyChallengedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<taskSuccesfullyChallengedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for taskSuccesfullyChallengedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<taskSuccesfullyChallengedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for taskSuccesfullyChallengedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskSuccesfullyChallengedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskSuccesfullyChallengedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskSuccesfullyChallenged(uint32)";
            const SELECTOR: [u8; 4] = [93u8, 236u8, 195u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`.
```solidity
function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`](trySignatureAndApkVerificationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationReturn {
        #[allow(missing_docs)]
        pub pairingSuccessful: bool,
        #[allow(missing_docs)]
        pub siganatureIsValid: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<trySignatureAndApkVerificationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationCall) -> Self {
                    (value.msgHash, value.apk, value.apkG2, value.sigma)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for trySignatureAndApkVerificationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        apk: tuple.1,
                        apkG2: tuple.2,
                        sigma: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<trySignatureAndApkVerificationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationReturn) -> Self {
                    (value.pairingSuccessful, value.siganatureIsValid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for trySignatureAndApkVerificationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pairingSuccessful: tuple.0,
                        siganatureIsValid: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for trySignatureAndApkVerificationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = trySignatureAndApkVerificationReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))";
            const SELECTOR: [u8; 4] = [23u8, 31u8, 29u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.apk),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
```solidity
function unpause(uint256 newPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`unpause(uint256)`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPausedStatus: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause(uint256)";
            const SELECTOR: [u8; 4] = [250u8, 188u8, 28u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`IncredibleSquaringTaskManager`](self) function calls.
    #[derive()]
    pub enum IncredibleSquaringTaskManagerCalls {
        #[allow(missing_docs)]
        TASK_CHALLENGE_WINDOW_BLOCK(TASK_CHALLENGE_WINDOW_BLOCKCall),
        #[allow(missing_docs)]
        TASK_RESPONSE_WINDOW_BLOCK(TASK_RESPONSE_WINDOW_BLOCKCall),
        #[allow(missing_docs)]
        WADS_TO_SLASH(WADS_TO_SLASHCall),
        #[allow(missing_docs)]
        aggregator(aggregatorCall),
        #[allow(missing_docs)]
        allTaskHashes(allTaskHashesCall),
        #[allow(missing_docs)]
        allTaskResponses(allTaskResponsesCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        blsApkRegistry(blsApkRegistryCall),
        #[allow(missing_docs)]
        checkSignatures(checkSignaturesCall),
        #[allow(missing_docs)]
        createNewTask(createNewTaskCall),
        #[allow(missing_docs)]
        delegation(delegationCall),
        #[allow(missing_docs)]
        generator(generatorCall),
        #[allow(missing_docs)]
        getBatchOperatorFromId(getBatchOperatorFromIdCall),
        #[allow(missing_docs)]
        getBatchOperatorId(getBatchOperatorIdCall),
        #[allow(missing_docs)]
        getCheckSignaturesIndices(getCheckSignaturesIndicesCall),
        #[allow(missing_docs)]
        getOperatorState_0(getOperatorState_0Call),
        #[allow(missing_docs)]
        getOperatorState_1(getOperatorState_1Call),
        #[allow(missing_docs)]
        getQuorumBitmapsAtBlockNumber(getQuorumBitmapsAtBlockNumberCall),
        #[allow(missing_docs)]
        getTaskResponseWindowBlock(getTaskResponseWindowBlockCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        instantSlasher(instantSlasherCall),
        #[allow(missing_docs)]
        latestTaskNum(latestTaskNumCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        pauseAll(pauseAllCall),
        #[allow(missing_docs)]
        paused_0(paused_0Call),
        #[allow(missing_docs)]
        paused_1(paused_1Call),
        #[allow(missing_docs)]
        pauserRegistry(pauserRegistryCall),
        #[allow(missing_docs)]
        raiseAndResolveChallenge(raiseAndResolveChallengeCall),
        #[allow(missing_docs)]
        registryCoordinator(registryCoordinatorCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        respondToTask(respondToTaskCall),
        #[allow(missing_docs)]
        serviceManager(serviceManagerCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
        #[allow(missing_docs)]
        taskNumber(taskNumberCall),
        #[allow(missing_docs)]
        taskSuccesfullyChallenged(taskSuccesfullyChallengedCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        trySignatureAndApkVerification(trySignatureAndApkVerificationCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
    }
    #[automatically_derived]
    impl IncredibleSquaringTaskManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [19u8, 100u8, 57u8, 221u8],
            [23u8, 31u8, 29u8, 91u8],
            [26u8, 212u8, 49u8, 137u8],
            [36u8, 90u8, 123u8, 252u8],
            [44u8, 178u8, 35u8, 213u8],
            [45u8, 137u8, 246u8, 252u8],
            [49u8, 179u8, 107u8, 217u8],
            [53u8, 99u8, 176u8, 209u8],
            [57u8, 152u8, 253u8, 211u8],
            [77u8, 43u8, 87u8, 254u8],
            [79u8, 115u8, 159u8, 116u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 45u8, 127u8, 2u8],
            [90u8, 200u8, 106u8, 183u8],
            [91u8, 174u8, 201u8, 160u8],
            [92u8, 21u8, 86u8, 98u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 236u8, 195u8, 245u8],
            [93u8, 244u8, 89u8, 70u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 83u8, 46u8, 158u8],
            [107u8, 146u8, 120u8, 126u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [113u8, 80u8, 24u8, 166u8],
            [114u8, 209u8, 142u8, 141u8],
            [122u8, 250u8, 30u8, 237u8],
            [136u8, 111u8, 17u8, 149u8],
            [139u8, 0u8, 206u8, 124u8],
            [141u8, 165u8, 203u8, 91u8],
            [155u8, 41u8, 14u8, 152u8],
            [202u8, 138u8, 167u8, 199u8],
            [204u8, 42u8, 154u8, 91u8],
            [206u8, 253u8, 193u8, 212u8],
            [223u8, 92u8, 247u8, 35u8],
            [242u8, 253u8, 227u8, 139u8],
            [245u8, 201u8, 137u8, 157u8],
            [246u8, 60u8, 91u8, 171u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IncredibleSquaringTaskManagerCalls {
        const NAME: &'static str = "IncredibleSquaringTaskManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 39usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(_) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(_) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::WADS_TO_SLASH(_) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::aggregator(_) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allTaskHashes(_) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allTaskResponses(_) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createNewTask(_) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::generator(_) => {
                    <generatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBatchOperatorFromId(_) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBatchOperatorId(_) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCheckSignaturesIndices(_) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState_0(_) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState_1(_) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapsAtBlockNumber(_) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTaskResponseWindowBlock(_) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::instantSlasher(_) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestTaskNum(_) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::raiseAndResolveChallenge(_) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respondToTask(_) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::taskNumber(_) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::taskSuccesfullyChallenged(_) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trySignatureAndApkVerification(_) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls>] = &[
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn TASK_RESPONSE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::TASK_RESPONSE_WINDOW_BLOCK,
                            )
                    }
                    TASK_RESPONSE_WINDOW_BLOCK
                },
                {
                    fn aggregator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <aggregatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::aggregator)
                    }
                    aggregator
                },
                {
                    fn allTaskResponses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::allTaskResponses)
                    }
                    allTaskResponses
                },
                {
                    fn allTaskHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <allTaskHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::allTaskHashes)
                    }
                    allTaskHashes
                },
                {
                    fn getBatchOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::getBatchOperatorId)
                    }
                    getBatchOperatorId
                },
                {
                    fn getOperatorState_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::getOperatorState_0)
                    }
                    getOperatorState_0
                },
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn getBatchOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::getBatchOperatorFromId,
                            )
                    }
                    getBatchOperatorFromId
                },
                {
                    fn getCheckSignaturesIndices(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::getCheckSignaturesIndices,
                            )
                    }
                    getCheckSignaturesIndices
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn WADS_TO_SLASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::WADS_TO_SLASH)
                    }
                    WADS_TO_SLASH
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn respondToTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <respondToTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::respondToTask)
                    }
                    respondToTask
                },
                {
                    fn getQuorumBitmapsAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::getQuorumBitmapsAtBlockNumber,
                            )
                    }
                    getQuorumBitmapsAtBlockNumber
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn taskSuccesfullyChallenged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::taskSuccesfullyChallenged,
                            )
                    }
                    taskSuccesfullyChallenged
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn raiseAndResolveChallenge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::raiseAndResolveChallenge,
                            )
                    }
                    raiseAndResolveChallenge
                },
                {
                    fn createNewTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <createNewTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::createNewTask)
                    }
                    createNewTask
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn taskNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <taskNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::taskNumber)
                    }
                    taskNumber
                },
                {
                    fn generator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <generatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::generator)
                    }
                    generator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn latestTaskNum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <latestTaskNumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::latestTaskNum)
                    }
                    latestTaskNum
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn instantSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <instantSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::instantSlasher)
                    }
                    instantSlasher
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperatorState_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::getOperatorState_1)
                    }
                    getOperatorState_1
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getTaskResponseWindowBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::getTaskResponseWindowBlock,
                            )
                    }
                    getTaskResponseWindowBlock
                },
                {
                    fn TASK_CHALLENGE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerCalls::TASK_CHALLENGE_WINDOW_BLOCK,
                            )
                    }
                    TASK_CHALLENGE_WINDOW_BLOCK
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerCalls::unpause)
                    }
                    unpause
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(inner) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WADS_TO_SLASH(inner) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(inner) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WADS_TO_SLASH(inner) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IncredibleSquaringTaskManager`](self) custom errors.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum IncredibleSquaringTaskManagerErrors {
        #[allow(missing_docs)]
        BitmapValueTooLarge(BitmapValueTooLarge),
        #[allow(missing_docs)]
        BytesArrayLengthTooLong(BytesArrayLengthTooLong),
        #[allow(missing_docs)]
        BytesArrayNotOrdered(BytesArrayNotOrdered),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        ECAddFailed(ECAddFailed),
        #[allow(missing_docs)]
        ECMulFailed(ECMulFailed),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InputEmptyQuorumNumbers(InputEmptyQuorumNumbers),
        #[allow(missing_docs)]
        InputNonSignerLengthMismatch(InputNonSignerLengthMismatch),
        #[allow(missing_docs)]
        InvalidBLSPairingKey(InvalidBLSPairingKey),
        #[allow(missing_docs)]
        InvalidBLSSignature(InvalidBLSSignature),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidQuorumApkHash(InvalidQuorumApkHash),
        #[allow(missing_docs)]
        InvalidReferenceBlocknumber(InvalidReferenceBlocknumber),
        #[allow(missing_docs)]
        NonSignerPubkeysNotSorted(NonSignerPubkeysNotSorted),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyRegistryCoordinatorOwner(OnlyRegistryCoordinatorOwner),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        OperatorNotRegistered(OperatorNotRegistered),
        #[allow(missing_docs)]
        ScalarTooLarge(ScalarTooLarge),
    }
    #[automatically_derived]
    impl IncredibleSquaringTaskManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [31u8, 4u8, 5u8, 160u8],
            [37u8, 236u8, 108u8, 31u8],
            [67u8, 113u8, 74u8, 253u8],
            [70u8, 51u8, 190u8, 50u8],
            [75u8, 135u8, 79u8, 69u8],
            [95u8, 131u8, 47u8, 65u8],
            [103u8, 152u8, 141u8, 51u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [128u8, 200u8, 131u8, 72u8],
            [132u8, 10u8, 72u8, 213u8],
            [171u8, 27u8, 35u8, 107u8],
            [198u8, 29u8, 202u8, 93u8],
            [202u8, 149u8, 115u8, 51u8],
            [212u8, 182u8, 143u8, 215u8],
            [213u8, 30u8, 218u8, 227u8],
            [224u8, 225u8, 231u8, 98u8],
            [225u8, 49u8, 10u8, 237u8],
            [251u8, 74u8, 156u8, 142u8],
            [255u8, 113u8, 148u8, 20u8],
            [255u8, 137u8, 212u8, 250u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IncredibleSquaringTaskManagerErrors {
        const NAME: &'static str = "IncredibleSquaringTaskManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BitmapValueTooLarge(_) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayLengthTooLong(_) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayNotOrdered(_) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECAddFailed(_) => {
                    <ECAddFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECMulFailed(_) => {
                    <ECMulFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpModFailed(_) => {
                    <ExpModFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputEmptyQuorumNumbers(_) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputNonSignerLengthMismatch(_) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBLSPairingKey(_) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBLSSignature(_) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidQuorumApkHash(_) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReferenceBlocknumber(_) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonSignerPubkeysNotSorted(_) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRegistryCoordinatorOwner(_) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ScalarTooLarge(_) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors>] = &[
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::InputEmptyQuorumNumbers,
                            )
                    }
                    InputEmptyQuorumNumbers
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::OperatorNotRegistered,
                            )
                    }
                    OperatorNotRegistered
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::InputArrayLengthMismatch,
                            )
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidReferenceBlocknumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::InvalidReferenceBlocknumber,
                            )
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::InputNonSignerLengthMismatch,
                            )
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::InvalidBLSPairingKey,
                            )
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::BytesArrayNotOrdered,
                            )
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidBLSSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InvalidBLSSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::InvalidBLSSignature,
                            )
                    }
                    InvalidBLSSignature
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::InvalidNewPausedStatus,
                            )
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::BitmapValueTooLarge,
                            )
                    }
                    BitmapValueTooLarge
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn OnlyRegistryCoordinatorOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::OnlyRegistryCoordinatorOwner,
                            )
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::InvalidQuorumApkHash,
                            )
                    }
                    InvalidQuorumApkHash
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::BytesArrayLengthTooLong,
                            )
                    }
                    BytesArrayLengthTooLong
                },
                {
                    fn NonSignerPubkeysNotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                IncredibleSquaringTaskManagerErrors::NonSignerPubkeysNotSorted,
                            )
                    }
                    NonSignerPubkeysNotSorted
                },
                {
                    fn ScalarTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IncredibleSquaringTaskManagerErrors> {
                        <ScalarTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IncredibleSquaringTaskManagerErrors::ScalarTooLarge)
                    }
                    ScalarTooLarge
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IncredibleSquaringTaskManager`](self) events.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum IncredibleSquaringTaskManagerEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NewTaskCreated(NewTaskCreated),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        TaskChallengedSuccessfully(TaskChallengedSuccessfully),
        #[allow(missing_docs)]
        TaskChallengedUnsuccessfully(TaskChallengedUnsuccessfully),
        #[allow(missing_docs)]
        TaskCompleted(TaskCompleted),
        #[allow(missing_docs)]
        TaskResponded(TaskResponded),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl IncredibleSquaringTaskManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                22u8, 149u8, 184u8, 208u8, 110u8, 200u8, 0u8, 180u8, 97u8, 94u8, 116u8,
                92u8, 251u8, 91u8, 208u8, 12u8, 31u8, 40u8, 117u8, 97u8, 93u8, 66u8,
                146u8, 92u8, 59u8, 90u8, 250u8, 84u8, 59u8, 178u8, 76u8, 72u8,
            ],
            [
                52u8, 156u8, 30u8, 230u8, 14u8, 78u8, 137u8, 114u8, 238u8, 157u8, 186u8,
                100u8, 44u8, 23u8, 116u8, 84u8, 61u8, 92u8, 65u8, 54u8, 135u8, 155u8,
                127u8, 76u8, 170u8, 240u8, 75u8, 248u8, 26u8, 72u8, 122u8, 42u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8,
                2u8, 188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8,
                228u8, 152u8, 107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8,
                19u8, 56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8,
                146u8, 20u8, 96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                154u8, 20u8, 79u8, 34u8, 138u8, 147u8, 27u8, 157u8, 13u8, 22u8, 150u8,
                251u8, 205u8, 175u8, 49u8, 11u8, 36u8, 181u8, 210u8, 210u8, 30u8, 121u8,
                157u8, 182u8, 35u8, 252u8, 152u8, 106u8, 15u8, 84u8, 116u8, 48u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8,
                188u8, 152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8,
                11u8, 182u8, 224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                194u8, 13u8, 27u8, 176u8, 241u8, 98u8, 54u8, 128u8, 48u8, 107u8, 131u8,
                212u8, 255u8, 75u8, 185u8, 154u8, 43u8, 235u8, 157u8, 134u8, 217u8,
                120u8, 50u8, 243u8, 202u8, 64u8, 253u8, 19u8, 162u8, 157u8, 241u8, 236u8,
            ],
            [
                253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8,
                68u8, 105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8,
                165u8, 27u8, 75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IncredibleSquaringTaskManagerEvents {
        const NAME: &'static str = "IncredibleSquaringTaskManagerEvents";
        const COUNT: usize = 9usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(<NewTaskCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTaskCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NewTaskCreated)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Paused)
                }
                Some(
                    <TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TaskChallengedSuccessfully)
                }
                Some(
                    <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TaskChallengedUnsuccessfully)
                }
                Some(<TaskCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TaskCompleted)
                }
                Some(<TaskResponded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskResponded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TaskResponded)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Unpaused)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for IncredibleSquaringTaskManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskChallengedSuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskChallengedUnsuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskChallengedSuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskChallengedUnsuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IncredibleSquaringTaskManager`](self) contract instance.

See the [wrapper's documentation](`IncredibleSquaringTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IncredibleSquaringTaskManagerInstance<T, P, N> {
        IncredibleSquaringTaskManagerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IncredibleSquaringTaskManagerInstance<T, P, N>>,
    > {
        IncredibleSquaringTaskManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _slashingRegistryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
        )
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        IncredibleSquaringTaskManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _slashingRegistryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
        )
    }
    /**A [`IncredibleSquaringTaskManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IncredibleSquaringTaskManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IncredibleSquaringTaskManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IncredibleSquaringTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IncredibleSquaringTaskManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IncredibleSquaringTaskManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IncredibleSquaringTaskManager`](self) contract instance.

See the [wrapper's documentation](`IncredibleSquaringTaskManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
        ) -> alloy_contract::Result<IncredibleSquaringTaskManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _slashingRegistryCoordinator,
                _pauserRegistry,
                _taskResponseWindowBlock,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _slashingRegistryCoordinator,
                            _pauserRegistry,
                            _taskResponseWindowBlock,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IncredibleSquaringTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> IncredibleSquaringTaskManagerInstance<T, P, N> {
            IncredibleSquaringTaskManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IncredibleSquaringTaskManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`TASK_CHALLENGE_WINDOW_BLOCK`] function.
        pub fn TASK_CHALLENGE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TASK_CHALLENGE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_CHALLENGE_WINDOW_BLOCKCall {})
        }
        ///Creates a new call builder for the [`TASK_RESPONSE_WINDOW_BLOCK`] function.
        pub fn TASK_RESPONSE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TASK_RESPONSE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_RESPONSE_WINDOW_BLOCKCall {})
        }
        ///Creates a new call builder for the [`WADS_TO_SLASH`] function.
        pub fn WADS_TO_SLASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, WADS_TO_SLASHCall, N> {
            self.call_builder(&WADS_TO_SLASHCall {})
        }
        ///Creates a new call builder for the [`aggregator`] function.
        pub fn aggregator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, aggregatorCall, N> {
            self.call_builder(&aggregatorCall {})
        }
        ///Creates a new call builder for the [`allTaskHashes`] function.
        pub fn allTaskHashes(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskHashesCall, N> {
            self.call_builder(&allTaskHashesCall { _0 })
        }
        ///Creates a new call builder for the [`allTaskResponses`] function.
        pub fn allTaskResponses(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskResponsesCall, N> {
            self.call_builder(&allTaskResponsesCall { _0 })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`blsApkRegistry`] function.
        pub fn blsApkRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blsApkRegistryCall, N> {
            self.call_builder(&blsApkRegistryCall {})
        }
        ///Creates a new call builder for the [`checkSignatures`] function.
        pub fn checkSignatures(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
            referenceBlockNumber: u32,
            params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkSignaturesCall, N> {
            self.call_builder(
                &checkSignaturesCall {
                    msgHash,
                    quorumNumbers,
                    referenceBlockNumber,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`createNewTask`] function.
        pub fn createNewTask(
            &self,
            numberToBeSquared: alloy::sol_types::private::primitives::aliases::U256,
            quorumThresholdPercentage: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, createNewTaskCall, N> {
            self.call_builder(
                &createNewTaskCall {
                    numberToBeSquared,
                    quorumThresholdPercentage,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`generator`] function.
        pub fn generator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, generatorCall, N> {
            self.call_builder(&generatorCall {})
        }
        ///Creates a new call builder for the [`getBatchOperatorFromId`] function.
        pub fn getBatchOperatorFromId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorFromIdCall, N> {
            self.call_builder(
                &getBatchOperatorFromIdCall {
                    registryCoordinator,
                    operatorIds,
                },
            )
        }
        ///Creates a new call builder for the [`getBatchOperatorId`] function.
        pub fn getBatchOperatorId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorIdCall, N> {
            self.call_builder(
                &getBatchOperatorIdCall {
                    registryCoordinator,
                    operators,
                },
            )
        }
        ///Creates a new call builder for the [`getCheckSignaturesIndices`] function.
        pub fn getCheckSignaturesIndices(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            referenceBlockNumber: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
            nonSignerOperatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCheckSignaturesIndicesCall, N> {
            self.call_builder(
                &getCheckSignaturesIndicesCall {
                    registryCoordinator,
                    referenceBlockNumber,
                    quorumNumbers,
                    nonSignerOperatorIds,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorState_0`] function.
        pub fn getOperatorState_0(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_0Call, N> {
            self.call_builder(
                &getOperatorState_0Call {
                    registryCoordinator,
                    quorumNumbers,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorState_1`] function.
        pub fn getOperatorState_1(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_1Call, N> {
            self.call_builder(
                &getOperatorState_1Call {
                    registryCoordinator,
                    operatorId,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getQuorumBitmapsAtBlockNumber`] function.
        pub fn getQuorumBitmapsAtBlockNumber(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getQuorumBitmapsAtBlockNumberCall,
            N,
        > {
            self.call_builder(
                &getQuorumBitmapsAtBlockNumberCall {
                    registryCoordinator,
                    operatorIds,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getTaskResponseWindowBlock`] function.
        pub fn getTaskResponseWindowBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTaskResponseWindowBlockCall, N> {
            self.call_builder(&getTaskResponseWindowBlockCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _aggregator: alloy::sol_types::private::Address,
            _generator: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
            _serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    _aggregator,
                    _generator,
                    _allocationManager,
                    _slasher,
                    _serviceManager,
                },
            )
        }
        ///Creates a new call builder for the [`instantSlasher`] function.
        pub fn instantSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, instantSlasherCall, N> {
            self.call_builder(&instantSlasherCall {})
        }
        ///Creates a new call builder for the [`latestTaskNum`] function.
        pub fn latestTaskNum(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, latestTaskNumCall, N> {
            self.call_builder(&latestTaskNumCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`pauseAll`] function.
        pub fn pauseAll(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
            self.call_builder(&pauseAllCall {})
        }
        ///Creates a new call builder for the [`paused_0`] function.
        pub fn paused_0(
            &self,
            index: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_0Call, N> {
            self.call_builder(&paused_0Call { index })
        }
        ///Creates a new call builder for the [`paused_1`] function.
        pub fn paused_1(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
            self.call_builder(&paused_1Call {})
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall {})
        }
        ///Creates a new call builder for the [`raiseAndResolveChallenge`] function.
        pub fn raiseAndResolveChallenge(
            &self,
            task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            taskResponseMetadata: <IIncredibleSquaringTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
            pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, raiseAndResolveChallengeCall, N> {
            self.call_builder(
                &raiseAndResolveChallengeCall {
                    task,
                    taskResponse,
                    taskResponseMetadata,
                    pubkeysOfNonSigningOperators,
                },
            )
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`respondToTask`] function.
        pub fn respondToTask(
            &self,
            task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IIncredibleSquaringTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, respondToTaskCall, N> {
            self.call_builder(
                &respondToTaskCall {
                    task,
                    taskResponse,
                    nonSignerStakesAndSignature,
                },
            )
        }
        ///Creates a new call builder for the [`serviceManager`] function.
        pub fn serviceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceManagerCall, N> {
            self.call_builder(&serviceManagerCall {})
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`taskNumber`] function.
        pub fn taskNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, taskNumberCall, N> {
            self.call_builder(&taskNumberCall {})
        }
        ///Creates a new call builder for the [`taskSuccesfullyChallenged`] function.
        pub fn taskSuccesfullyChallenged(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, taskSuccesfullyChallengedCall, N> {
            self.call_builder(
                &taskSuccesfullyChallengedCall {
                    _0,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`trySignatureAndApkVerification`] function.
        pub fn trySignatureAndApkVerification(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            trySignatureAndApkVerificationCall,
            N,
        > {
            self.call_builder(
                &trySignatureAndApkVerificationCall {
                    msgHash,
                    apk,
                    apkG2,
                    sigma,
                },
            )
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IncredibleSquaringTaskManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NewTaskCreated`] event.
        pub fn NewTaskCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NewTaskCreated, N> {
            self.event_filter::<NewTaskCreated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`TaskChallengedSuccessfully`] event.
        pub fn TaskChallengedSuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskChallengedSuccessfully, N> {
            self.event_filter::<TaskChallengedSuccessfully>()
        }
        ///Creates a new event filter for the [`TaskChallengedUnsuccessfully`] event.
        pub fn TaskChallengedUnsuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskChallengedUnsuccessfully, N> {
            self.event_filter::<TaskChallengedUnsuccessfully>()
        }
        ///Creates a new event filter for the [`TaskCompleted`] event.
        pub fn TaskCompleted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskCompleted, N> {
            self.event_filter::<TaskCompleted>()
        }
        ///Creates a new event filter for the [`TaskResponded`] event.
        pub fn TaskResponded_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskResponded, N> {
            self.event_filter::<TaskResponded>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
