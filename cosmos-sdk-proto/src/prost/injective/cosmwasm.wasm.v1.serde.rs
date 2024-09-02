// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for AbsoluteTxPosition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_height != 0 {
            len += 1;
        }
        if self.tx_index != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AbsoluteTxPosition", len)?;
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        if self.tx_index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("txIndex", ToString::to_string(&self.tx_index).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AbsoluteTxPosition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["block_height", "blockHeight", "tx_index", "txIndex"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockHeight,
            TxIndex,
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
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "txIndex" | "tx_index" => Ok(GeneratedField::TxIndex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AbsoluteTxPosition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AbsoluteTxPosition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AbsoluteTxPosition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut block_height__ = None;
                let mut tx_index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TxIndex => {
                            if tx_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txIndex"));
                            }
                            tx_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(AbsoluteTxPosition {
                    block_height: block_height__.unwrap_or_default(),
                    tx_index: tx_index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AbsoluteTxPosition",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AccessConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.permission != 0 {
            len += 1;
        }
        if !self.addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.AccessConfig", len)?;
        if self.permission != 0 {
            let v = AccessType::try_from(self.permission).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.permission))
            })?;
            struct_ser.serialize_field("permission", &v)?;
        }
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AccessConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["permission", "addresses"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permission,
            Addresses,
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
                            "permission" => Ok(GeneratedField::Permission),
                            "addresses" => Ok(GeneratedField::Addresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AccessConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccessConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut permission__ = None;
                let mut addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(map_.next_value::<AccessType>()? as i32);
                        }
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AccessConfig {
                    permission: permission__.unwrap_or_default(),
                    addresses: addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.AccessConfig", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AccessConfigUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AccessConfigUpdate", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AccessConfigUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_id",
            "codeId",
            "instantiate_permission",
            "instantiatePermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            InstantiatePermission,
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessConfigUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AccessConfigUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccessConfigUpdate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut instantiate_permission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AccessConfigUpdate {
                    code_id: code_id__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AccessConfigUpdate",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AccessType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACCESS_TYPE_UNSPECIFIED",
            Self::Nobody => "ACCESS_TYPE_NOBODY",
            Self::Everybody => "ACCESS_TYPE_EVERYBODY",
            Self::AnyOfAddresses => "ACCESS_TYPE_ANY_OF_ADDRESSES",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AccessType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACCESS_TYPE_UNSPECIFIED",
            "ACCESS_TYPE_NOBODY",
            "ACCESS_TYPE_EVERYBODY",
            "ACCESS_TYPE_ANY_OF_ADDRESSES",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ACCESS_TYPE_UNSPECIFIED" => Ok(AccessType::Unspecified),
                    "ACCESS_TYPE_NOBODY" => Ok(AccessType::Nobody),
                    "ACCESS_TYPE_EVERYBODY" => Ok(AccessType::Everybody),
                    "ACCESS_TYPE_ANY_OF_ADDRESSES" => Ok(AccessType::AnyOfAddresses),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AccessTypeParam {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.AccessTypeParam", len)?;
        if self.value != 0 {
            let v = AccessType::try_from(self.value).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.value))
            })?;
            struct_ser.serialize_field("value", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AccessTypeParam {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessTypeParam;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.AccessTypeParam")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccessTypeParam, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value::<AccessType>()? as i32);
                        }
                    }
                }
                Ok(AccessTypeParam {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.AccessTypeParam",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ClearAdminProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.ClearAdminProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ClearAdminProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "contract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Contract,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "contract" => Ok(GeneratedField::Contract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClearAdminProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ClearAdminProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClearAdminProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClearAdminProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.ClearAdminProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CodeInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code_hash.is_empty() {
            len += 1;
        }
        if !self.creator.is_empty() {
            len += 1;
        }
        if self.instantiate_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.CodeInfo", len)?;
        if !self.code_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "codeHash",
                pbjson::private::base64::encode(&self.code_hash).as_str(),
            )?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        if let Some(v) = self.instantiate_config.as_ref() {
            struct_ser.serialize_field("instantiateConfig", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CodeInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_hash",
            "codeHash",
            "creator",
            "instantiate_config",
            "instantiateConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeHash,
            Creator,
            InstantiateConfig,
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
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            "creator" => Ok(GeneratedField::Creator),
                            "instantiateConfig" | "instantiate_config" => {
                                Ok(GeneratedField::InstantiateConfig)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CodeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.CodeInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CodeInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_hash__ = None;
                let mut creator__ = None;
                let mut instantiate_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InstantiateConfig => {
                            if instantiate_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instantiateConfig"));
                            }
                            instantiate_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CodeInfo {
                    code_hash: code_hash__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                    instantiate_config: instantiate_config__,
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.CodeInfo", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ContractCodeHistoryEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation != 0 {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if self.updated.is_some() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.ContractCodeHistoryEntry", len)?;
        if self.operation != 0 {
            let v = ContractCodeHistoryOperationType::try_from(self.operation).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.operation))
            })?;
            struct_ser.serialize_field("operation", &v)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if let Some(v) = self.updated.as_ref() {
            struct_ser.serialize_field("updated", v)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ContractCodeHistoryEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["operation", "code_id", "codeId", "updated", "msg"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
            CodeId,
            Updated,
            Msg,
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
                            "operation" => Ok(GeneratedField::Operation),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "updated" => Ok(GeneratedField::Updated),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractCodeHistoryEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ContractCodeHistoryEntry")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ContractCodeHistoryEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut operation__ = None;
                let mut code_id__ = None;
                let mut updated__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ =
                                Some(map_.next_value::<ContractCodeHistoryOperationType>()? as i32);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Updated => {
                            if updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated"));
                            }
                            updated__ = map_.next_value()?;
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ContractCodeHistoryEntry {
                    operation: operation__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    updated: updated__,
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.ContractCodeHistoryEntry",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ContractCodeHistoryOperationType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED",
            Self::Init => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT",
            Self::Migrate => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE",
            Self::Genesis => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ContractCodeHistoryOperationType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED",
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT",
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE",
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractCodeHistoryOperationType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED" => {
                        Ok(ContractCodeHistoryOperationType::Unspecified)
                    }
                    "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT" => {
                        Ok(ContractCodeHistoryOperationType::Init)
                    }
                    "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE" => {
                        Ok(ContractCodeHistoryOperationType::Migrate)
                    }
                    "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS" => {
                        Ok(ContractCodeHistoryOperationType::Genesis)
                    }
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ContractInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_id != 0 {
            len += 1;
        }
        if !self.creator.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if self.created.is_some() {
            len += 1;
        }
        if !self.ibc_port_id.is_empty() {
            len += 1;
        }
        if self.extension.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.ContractInfo", len)?;
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if let Some(v) = self.created.as_ref() {
            struct_ser.serialize_field("created", v)?;
        }
        if !self.ibc_port_id.is_empty() {
            struct_ser.serialize_field("ibcPortId", &self.ibc_port_id)?;
        }
        if let Some(v) = self.extension.as_ref() {
            struct_ser.serialize_field("extension", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ContractInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_id",
            "codeId",
            "creator",
            "admin",
            "label",
            "created",
            "ibc_port_id",
            "ibcPortId",
            "extension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeId,
            Creator,
            Admin,
            Label,
            Created,
            IbcPortId,
            Extension,
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
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "creator" => Ok(GeneratedField::Creator),
                            "admin" => Ok(GeneratedField::Admin),
                            "label" => Ok(GeneratedField::Label),
                            "created" => Ok(GeneratedField::Created),
                            "ibcPortId" | "ibc_port_id" => Ok(GeneratedField::IbcPortId),
                            "extension" => Ok(GeneratedField::Extension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ContractInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContractInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_id__ = None;
                let mut creator__ = None;
                let mut admin__ = None;
                let mut label__ = None;
                let mut created__ = None;
                let mut ibc_port_id__ = None;
                let mut extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Created => {
                            if created__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            created__ = map_.next_value()?;
                        }
                        GeneratedField::IbcPortId => {
                            if ibc_port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcPortId"));
                            }
                            ibc_port_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Extension => {
                            if extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            extension__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ContractInfo {
                    code_id: code_id__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    created: created__,
                    ibc_port_id: ibc_port_id__.unwrap_or_default(),
                    extension: extension__,
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.ContractInfo", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ExecuteContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.ExecuteContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ExecuteContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "contract",
            "msg",
            "funds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            Contract,
            Msg,
            Funds,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "contract" => Ok(GeneratedField::Contract),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.ExecuteContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ExecuteContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut contract__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExecuteContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.ExecuteContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InstantiateContract2Proposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        if !self.salt.is_empty() {
            len += 1;
        }
        if self.fix_msg {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.InstantiateContract2Proposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        if !self.salt.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("salt", pbjson::private::base64::encode(&self.salt).as_str())?;
        }
        if self.fix_msg {
            struct_ser.serialize_field("fixMsg", &self.fix_msg)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InstantiateContract2Proposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "admin",
            "code_id",
            "codeId",
            "label",
            "msg",
            "funds",
            "salt",
            "fix_msg",
            "fixMsg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            Admin,
            CodeId,
            Label,
            Msg,
            Funds,
            Salt,
            FixMsg,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "admin" => Ok(GeneratedField::Admin),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            "salt" => Ok(GeneratedField::Salt),
                            "fixMsg" | "fix_msg" => Ok(GeneratedField::FixMsg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstantiateContract2Proposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.InstantiateContract2Proposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<InstantiateContract2Proposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut admin__ = None;
                let mut code_id__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                let mut salt__ = None;
                let mut fix_msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Salt => {
                            if salt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salt"));
                            }
                            salt__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FixMsg => {
                            if fix_msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixMsg"));
                            }
                            fix_msg__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InstantiateContract2Proposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                    salt: salt__.unwrap_or_default(),
                    fix_msg: fix_msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.InstantiateContract2Proposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InstantiateContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.InstantiateContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InstantiateContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "admin",
            "code_id",
            "codeId",
            "label",
            "msg",
            "funds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            Admin,
            CodeId,
            Label,
            Msg,
            Funds,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "admin" => Ok(GeneratedField::Admin),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstantiateContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.InstantiateContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<InstantiateContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut admin__ = None;
                let mut code_id__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InstantiateContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.InstantiateContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MigrateContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.MigrateContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MigrateContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "contract",
            "code_id",
            "codeId",
            "msg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Contract,
            CodeId,
            Msg,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "contract" => Ok(GeneratedField::Contract),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MigrateContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.MigrateContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MigrateContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contract__ = None;
                let mut code_id__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeId => {
                            if code_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeId"));
                            }
                            code_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MigrateContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.MigrateContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Model {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.Model", len)?;
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "value",
                pbjson::private::base64::encode(&self.value).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Model {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "value"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Model;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.Model")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Model, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Model {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.Model", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code_upload_access.is_some() {
            len += 1;
        }
        if self.instantiate_default_permission != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cosmwasm.wasm.v1.Params", len)?;
        if let Some(v) = self.code_upload_access.as_ref() {
            struct_ser.serialize_field("codeUploadAccess", v)?;
        }
        if self.instantiate_default_permission != 0 {
            let v = AccessType::try_from(self.instantiate_default_permission).map_err(|_| {
                serde::ser::Error::custom(format!(
                    "Invalid variant {}",
                    self.instantiate_default_permission
                ))
            })?;
            struct_ser.serialize_field("instantiateDefaultPermission", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code_upload_access",
            "codeUploadAccess",
            "instantiate_default_permission",
            "instantiateDefaultPermission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodeUploadAccess,
            InstantiateDefaultPermission,
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
                            "codeUploadAccess" | "code_upload_access" => {
                                Ok(GeneratedField::CodeUploadAccess)
                            }
                            "instantiateDefaultPermission" | "instantiate_default_permission" => {
                                Ok(GeneratedField::InstantiateDefaultPermission)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code_upload_access__ = None;
                let mut instantiate_default_permission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CodeUploadAccess => {
                            if code_upload_access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeUploadAccess"));
                            }
                            code_upload_access__ = map_.next_value()?;
                        }
                        GeneratedField::InstantiateDefaultPermission => {
                            if instantiate_default_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiateDefaultPermission",
                                ));
                            }
                            instantiate_default_permission__ =
                                Some(map_.next_value::<AccessType>()? as i32);
                        }
                    }
                }
                Ok(Params {
                    code_upload_access: code_upload_access__,
                    instantiate_default_permission: instantiate_default_permission__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cosmwasm.wasm.v1.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PinCodesProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.code_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.PinCodesProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.code_ids.is_empty() {
            struct_ser.serialize_field(
                "codeIds",
                &self
                    .code_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PinCodesProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "code_ids", "codeIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CodeIds,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "codeIds" | "code_ids" => Ok(GeneratedField::CodeIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PinCodesProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.PinCodesProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PinCodesProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut code_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeIds => {
                            if code_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeIds"));
                            }
                            code_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(PinCodesProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    code_ids: code_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.PinCodesProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StoreAndInstantiateContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.wasm_byte_code.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        if self.unpin_code {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.builder.is_empty() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmwasm.wasm.v1.StoreAndInstantiateContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.wasm_byte_code.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "wasmByteCode",
                pbjson::private::base64::encode(&self.wasm_byte_code).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        if self.unpin_code {
            struct_ser.serialize_field("unpinCode", &self.unpin_code)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.builder.is_empty() {
            struct_ser.serialize_field("builder", &self.builder)?;
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
impl<'de> serde::Deserialize<'de> for StoreAndInstantiateContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "wasm_byte_code",
            "wasmByteCode",
            "instantiate_permission",
            "instantiatePermission",
            "unpin_code",
            "unpinCode",
            "admin",
            "label",
            "msg",
            "funds",
            "source",
            "builder",
            "code_hash",
            "codeHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            WasmByteCode,
            InstantiatePermission,
            UnpinCode,
            Admin,
            Label,
            Msg,
            Funds,
            Source,
            Builder,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "wasmByteCode" | "wasm_byte_code" => Ok(GeneratedField::WasmByteCode),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
                            }
                            "unpinCode" | "unpin_code" => Ok(GeneratedField::UnpinCode),
                            "admin" => Ok(GeneratedField::Admin),
                            "label" => Ok(GeneratedField::Label),
                            "msg" => Ok(GeneratedField::Msg),
                            "funds" => Ok(GeneratedField::Funds),
                            "source" => Ok(GeneratedField::Source),
                            "builder" => Ok(GeneratedField::Builder),
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
            type Value = StoreAndInstantiateContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.StoreAndInstantiateContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<StoreAndInstantiateContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut wasm_byte_code__ = None;
                let mut instantiate_permission__ = None;
                let mut unpin_code__ = None;
                let mut admin__ = None;
                let mut label__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                let mut source__ = None;
                let mut builder__ = None;
                let mut code_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WasmByteCode => {
                            if wasm_byte_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasmByteCode"));
                            }
                            wasm_byte_code__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                        GeneratedField::UnpinCode => {
                            if unpin_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unpinCode"));
                            }
                            unpin_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Builder => {
                            if builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("builder"));
                            }
                            builder__ = Some(map_.next_value()?);
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
                Ok(StoreAndInstantiateContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    wasm_byte_code: wasm_byte_code__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                    unpin_code: unpin_code__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    builder: builder__.unwrap_or_default(),
                    code_hash: code_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.StoreAndInstantiateContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StoreCodeProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_as.is_empty() {
            len += 1;
        }
        if !self.wasm_byte_code.is_empty() {
            len += 1;
        }
        if self.instantiate_permission.is_some() {
            len += 1;
        }
        if self.unpin_code {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.builder.is_empty() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.StoreCodeProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_as.is_empty() {
            struct_ser.serialize_field("runAs", &self.run_as)?;
        }
        if !self.wasm_byte_code.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "wasmByteCode",
                pbjson::private::base64::encode(&self.wasm_byte_code).as_str(),
            )?;
        }
        if let Some(v) = self.instantiate_permission.as_ref() {
            struct_ser.serialize_field("instantiatePermission", v)?;
        }
        if self.unpin_code {
            struct_ser.serialize_field("unpinCode", &self.unpin_code)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.builder.is_empty() {
            struct_ser.serialize_field("builder", &self.builder)?;
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
impl<'de> serde::Deserialize<'de> for StoreCodeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "run_as",
            "runAs",
            "wasm_byte_code",
            "wasmByteCode",
            "instantiate_permission",
            "instantiatePermission",
            "unpin_code",
            "unpinCode",
            "source",
            "builder",
            "code_hash",
            "codeHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            RunAs,
            WasmByteCode,
            InstantiatePermission,
            UnpinCode,
            Source,
            Builder,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "runAs" | "run_as" => Ok(GeneratedField::RunAs),
                            "wasmByteCode" | "wasm_byte_code" => Ok(GeneratedField::WasmByteCode),
                            "instantiatePermission" | "instantiate_permission" => {
                                Ok(GeneratedField::InstantiatePermission)
                            }
                            "unpinCode" | "unpin_code" => Ok(GeneratedField::UnpinCode),
                            "source" => Ok(GeneratedField::Source),
                            "builder" => Ok(GeneratedField::Builder),
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
            type Value = StoreCodeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.StoreCodeProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StoreCodeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut run_as__ = None;
                let mut wasm_byte_code__ = None;
                let mut instantiate_permission__ = None;
                let mut unpin_code__ = None;
                let mut source__ = None;
                let mut builder__ = None;
                let mut code_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunAs => {
                            if run_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runAs"));
                            }
                            run_as__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WasmByteCode => {
                            if wasm_byte_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasmByteCode"));
                            }
                            wasm_byte_code__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InstantiatePermission => {
                            if instantiate_permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatePermission",
                                ));
                            }
                            instantiate_permission__ = map_.next_value()?;
                        }
                        GeneratedField::UnpinCode => {
                            if unpin_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unpinCode"));
                            }
                            unpin_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Builder => {
                            if builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("builder"));
                            }
                            builder__ = Some(map_.next_value()?);
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
                Ok(StoreCodeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_as: run_as__.unwrap_or_default(),
                    wasm_byte_code: wasm_byte_code__.unwrap_or_default(),
                    instantiate_permission: instantiate_permission__,
                    unpin_code: unpin_code__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    builder: builder__.unwrap_or_default(),
                    code_hash: code_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.StoreCodeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SudoContractProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.SudoContractProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.msg.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("msg", pbjson::private::base64::encode(&self.msg).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SudoContractProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "contract", "msg"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Contract,
            Msg,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "contract" => Ok(GeneratedField::Contract),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SudoContractProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.SudoContractProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SudoContractProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contract__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SudoContractProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.SudoContractProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for UnpinCodesProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.code_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.UnpinCodesProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.code_ids.is_empty() {
            struct_ser.serialize_field(
                "codeIds",
                &self
                    .code_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UnpinCodesProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "code_ids", "codeIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CodeIds,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "codeIds" | "code_ids" => Ok(GeneratedField::CodeIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnpinCodesProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.UnpinCodesProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnpinCodesProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut code_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeIds => {
                            if code_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeIds"));
                            }
                            code_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(UnpinCodesProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    code_ids: code_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.UnpinCodesProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for UpdateAdminProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.new_admin.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.UpdateAdminProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.new_admin.is_empty() {
            struct_ser.serialize_field("newAdmin", &self.new_admin)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UpdateAdminProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "new_admin", "newAdmin", "contract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            NewAdmin,
            Contract,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "newAdmin" | "new_admin" => Ok(GeneratedField::NewAdmin),
                            "contract" => Ok(GeneratedField::Contract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAdminProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.UpdateAdminProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAdminProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut new_admin__ = None;
                let mut contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAdmin => {
                            if new_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAdmin"));
                            }
                            new_admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateAdminProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    new_admin: new_admin__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.UpdateAdminProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for UpdateInstantiateConfigProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.access_config_updates.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmwasm.wasm.v1.UpdateInstantiateConfigProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.access_config_updates.is_empty() {
            struct_ser.serialize_field("accessConfigUpdates", &self.access_config_updates)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UpdateInstantiateConfigProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "access_config_updates",
            "accessConfigUpdates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            AccessConfigUpdates,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "accessConfigUpdates" | "access_config_updates" => {
                                Ok(GeneratedField::AccessConfigUpdates)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateInstantiateConfigProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmwasm.wasm.v1.UpdateInstantiateConfigProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<UpdateInstantiateConfigProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut access_config_updates__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessConfigUpdates => {
                            if access_config_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "accessConfigUpdates",
                                ));
                            }
                            access_config_updates__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateInstantiateConfigProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    access_config_updates: access_config_updates__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmwasm.wasm.v1.UpdateInstantiateConfigProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
