// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for EthAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_account.is_some() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.types.v1beta1.EthAccount", len)?;
        if let Some(v) = self.base_account.as_ref() {
            struct_ser.serialize_field("baseAccount", v)?;
        }
        if !self.code_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "codeHash",
                pbjson::private::base64::encode(&self.code_hash).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EthAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["base_account", "baseAccount", "code_hash", "codeHash"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseAccount,
            CodeHash,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "baseAccount" | "base_account" => Ok(GeneratedField::BaseAccount),
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EthAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.types.v1beta1.EthAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EthAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut base_account__ = None;
                let mut code_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseAccount => {
                            if base_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseAccount"));
                            }
                            base_account__ = map_.next_value()?;
                        }
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EthAccount {
                    base_account: base_account__,
                    code_hash: code_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.types.v1beta1.EthAccount",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ExtensionOptionsWeb3Tx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.typed_data_chain_id != 0 {
            len += 1;
        }
        if !self.fee_payer.is_empty() {
            len += 1;
        }
        if !self.fee_payer_sig.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.types.v1beta1.ExtensionOptionsWeb3Tx", len)?;
        if self.typed_data_chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "typedDataChainID",
                ToString::to_string(&self.typed_data_chain_id).as_str(),
            )?;
        }
        if !self.fee_payer.is_empty() {
            struct_ser.serialize_field("feePayer", &self.fee_payer)?;
        }
        if !self.fee_payer_sig.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "feePayerSig",
                pbjson::private::base64::encode(&self.fee_payer_sig).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ExtensionOptionsWeb3Tx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["typedDataChainID", "feePayer", "feePayerSig"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypedDataChainId,
            FeePayer,
            FeePayerSig,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "typedDataChainID" => Ok(GeneratedField::TypedDataChainId),
                            "feePayer" => Ok(GeneratedField::FeePayer),
                            "feePayerSig" => Ok(GeneratedField::FeePayerSig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtensionOptionsWeb3Tx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.types.v1beta1.ExtensionOptionsWeb3Tx")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ExtensionOptionsWeb3Tx, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut typed_data_chain_id__ = None;
                let mut fee_payer__ = None;
                let mut fee_payer_sig__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypedDataChainId => {
                            if typed_data_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedDataChainID"));
                            }
                            typed_data_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FeePayer => {
                            if fee_payer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feePayer"));
                            }
                            fee_payer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeePayerSig => {
                            if fee_payer_sig__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feePayerSig"));
                            }
                            fee_payer_sig__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ExtensionOptionsWeb3Tx {
                    typed_data_chain_id: typed_data_chain_id__.unwrap_or_default(),
                    fee_payer: fee_payer__.unwrap_or_default(),
                    fee_payer_sig: fee_payer_sig__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.types.v1beta1.ExtensionOptionsWeb3Tx",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for TxResponseData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.messages.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.types.v1beta1.TxResponseData", len)?;
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TxResponseData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["messages"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Messages,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "messages" => Ok(GeneratedField::Messages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxResponseData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.types.v1beta1.TxResponseData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxResponseData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TxResponseData {
                    messages: messages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.types.v1beta1.TxResponseData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for TxResponseGenericMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.types.v1beta1.TxResponseGenericMessage", len)?;
        if !self.header.is_empty() {
            struct_ser.serialize_field("header", &self.header)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TxResponseGenericMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["header", "data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Data,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "header" => Ok(GeneratedField::Header),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxResponseGenericMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.types.v1beta1.TxResponseGenericMessage")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<TxResponseGenericMessage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(TxResponseGenericMessage {
                    header: header__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.types.v1beta1.TxResponseGenericMessage",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
