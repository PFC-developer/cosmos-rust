// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for ContractConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_count != 0 {
            len += 1;
        }
        if !self.signers.is_empty() {
            len += 1;
        }
        if !self.transmitters.is_empty() {
            len += 1;
        }
        if self.f != 0 {
            len += 1;
        }
        if !self.onchain_config.is_empty() {
            len += 1;
        }
        if self.offchain_config_version != 0 {
            len += 1;
        }
        if !self.offchain_config.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.ContractConfig", len)?;
        if self.config_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "configCount",
                ToString::to_string(&self.config_count).as_str(),
            )?;
        }
        if !self.signers.is_empty() {
            struct_ser.serialize_field("signers", &self.signers)?;
        }
        if !self.transmitters.is_empty() {
            struct_ser.serialize_field("transmitters", &self.transmitters)?;
        }
        if self.f != 0 {
            struct_ser.serialize_field("f", &self.f)?;
        }
        if !self.onchain_config.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "onchainConfig",
                pbjson::private::base64::encode(&self.onchain_config).as_str(),
            )?;
        }
        if self.offchain_config_version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "offchainConfigVersion",
                ToString::to_string(&self.offchain_config_version).as_str(),
            )?;
        }
        if !self.offchain_config.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "offchainConfig",
                pbjson::private::base64::encode(&self.offchain_config).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ContractConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_count",
            "configCount",
            "signers",
            "transmitters",
            "f",
            "onchain_config",
            "onchainConfig",
            "offchain_config_version",
            "offchainConfigVersion",
            "offchain_config",
            "offchainConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigCount,
            Signers,
            Transmitters,
            F,
            OnchainConfig,
            OffchainConfigVersion,
            OffchainConfig,
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
                            "configCount" | "config_count" => Ok(GeneratedField::ConfigCount),
                            "signers" => Ok(GeneratedField::Signers),
                            "transmitters" => Ok(GeneratedField::Transmitters),
                            "f" => Ok(GeneratedField::F),
                            "onchainConfig" | "onchain_config" => Ok(GeneratedField::OnchainConfig),
                            "offchainConfigVersion" | "offchain_config_version" => {
                                Ok(GeneratedField::OffchainConfigVersion)
                            }
                            "offchainConfig" | "offchain_config" => {
                                Ok(GeneratedField::OffchainConfig)
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
            type Value = ContractConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.ContractConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContractConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut config_count__ = None;
                let mut signers__ = None;
                let mut transmitters__ = None;
                let mut f__ = None;
                let mut onchain_config__ = None;
                let mut offchain_config_version__ = None;
                let mut offchain_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConfigCount => {
                            if config_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configCount"));
                            }
                            config_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Signers => {
                            if signers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signers"));
                            }
                            signers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitters => {
                            if transmitters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitters"));
                            }
                            transmitters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::F => {
                            if f__.is_some() {
                                return Err(serde::de::Error::duplicate_field("f"));
                            }
                            f__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OnchainConfig => {
                            if onchain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onchainConfig"));
                            }
                            onchain_config__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OffchainConfigVersion => {
                            if offchain_config_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "offchainConfigVersion",
                                ));
                            }
                            offchain_config_version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OffchainConfig => {
                            if offchain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offchainConfig"));
                            }
                            offchain_config__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ContractConfig {
                    config_count: config_count__.unwrap_or_default(),
                    signers: signers__.unwrap_or_default(),
                    transmitters: transmitters__.unwrap_or_default(),
                    f: f__.unwrap_or_default(),
                    onchain_config: onchain_config__.unwrap_or_default(),
                    offchain_config_version: offchain_config_version__.unwrap_or_default(),
                    offchain_config: offchain_config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.ContractConfig",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Count {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.ocr.v1beta1.Count", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Count {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "count"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Count,
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
                            "address" => Ok(GeneratedField::Address),
                            "count" => Ok(GeneratedField::Count),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Count;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.Count")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Count, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Count {
                    address: address__.unwrap_or_default(),
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.ocr.v1beta1.Count", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EpochAndRound {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch != 0 {
            len += 1;
        }
        if self.round != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.EpochAndRound", len)?;
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if self.round != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("round", ToString::to_string(&self.round).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EpochAndRound {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["epoch", "round"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Epoch,
            Round,
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
                            "epoch" => Ok(GeneratedField::Epoch),
                            "round" => Ok(GeneratedField::Round),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EpochAndRound;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.EpochAndRound")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EpochAndRound, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut epoch__ = None;
                let mut round__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Round => {
                            if round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("round"));
                            }
                            round__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EpochAndRound {
                    epoch: epoch__.unwrap_or_default(),
                    round: round__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.EpochAndRound",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventAnswerUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.current.is_empty() {
            len += 1;
        }
        if !self.round_id.is_empty() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.EventAnswerUpdated", len)?;
        if !self.current.is_empty() {
            struct_ser.serialize_field("current", &self.current)?;
        }
        if !self.round_id.is_empty() {
            struct_ser.serialize_field("roundId", &self.round_id)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventAnswerUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["current", "round_id", "roundId", "updated_at", "updatedAt"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Current,
            RoundId,
            UpdatedAt,
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
                            "current" => Ok(GeneratedField::Current),
                            "roundId" | "round_id" => Ok(GeneratedField::RoundId),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventAnswerUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.EventAnswerUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventAnswerUpdated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut current__ = None;
                let mut round_id__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Current => {
                            if current__.is_some() {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            current__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoundId => {
                            if round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundId"));
                            }
                            round_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventAnswerUpdated {
                    current: current__.unwrap_or_default(),
                    round_id: round_id__.unwrap_or_default(),
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.EventAnswerUpdated",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventConfigSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.config_digest.is_empty() {
            len += 1;
        }
        if self.previous_config_block_number != 0 {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.config_info.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.EventConfigSet", len)?;
        if !self.config_digest.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "configDigest",
                pbjson::private::base64::encode(&self.config_digest).as_str(),
            )?;
        }
        if self.previous_config_block_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "previousConfigBlockNumber",
                ToString::to_string(&self.previous_config_block_number).as_str(),
            )?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.config_info.as_ref() {
            struct_ser.serialize_field("configInfo", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventConfigSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_digest",
            "configDigest",
            "previous_config_block_number",
            "previousConfigBlockNumber",
            "config",
            "config_info",
            "configInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigDigest,
            PreviousConfigBlockNumber,
            Config,
            ConfigInfo,
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
                            "configDigest" | "config_digest" => Ok(GeneratedField::ConfigDigest),
                            "previousConfigBlockNumber" | "previous_config_block_number" => {
                                Ok(GeneratedField::PreviousConfigBlockNumber)
                            }
                            "config" => Ok(GeneratedField::Config),
                            "configInfo" | "config_info" => Ok(GeneratedField::ConfigInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventConfigSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.EventConfigSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventConfigSet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut config_digest__ = None;
                let mut previous_config_block_number__ = None;
                let mut config__ = None;
                let mut config_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConfigDigest => {
                            if config_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDigest"));
                            }
                            config_digest__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PreviousConfigBlockNumber => {
                            if previous_config_block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "previousConfigBlockNumber",
                                ));
                            }
                            previous_config_block_number__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::ConfigInfo => {
                            if config_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configInfo"));
                            }
                            config_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventConfigSet {
                    config_digest: config_digest__.unwrap_or_default(),
                    previous_config_block_number: previous_config_block_number__
                        .unwrap_or_default(),
                    config: config__,
                    config_info: config_info__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.EventConfigSet",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventNewRound {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.round_id.is_empty() {
            len += 1;
        }
        if !self.started_by.is_empty() {
            len += 1;
        }
        if self.started_at.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.EventNewRound", len)?;
        if !self.round_id.is_empty() {
            struct_ser.serialize_field("roundId", &self.round_id)?;
        }
        if !self.started_by.is_empty() {
            struct_ser.serialize_field("startedBy", &self.started_by)?;
        }
        if let Some(v) = self.started_at.as_ref() {
            struct_ser.serialize_field("startedAt", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventNewRound {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "round_id",
            "roundId",
            "started_by",
            "startedBy",
            "started_at",
            "startedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoundId,
            StartedBy,
            StartedAt,
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
                            "roundId" | "round_id" => Ok(GeneratedField::RoundId),
                            "startedBy" | "started_by" => Ok(GeneratedField::StartedBy),
                            "startedAt" | "started_at" => Ok(GeneratedField::StartedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventNewRound;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.EventNewRound")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventNewRound, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut round_id__ = None;
                let mut started_by__ = None;
                let mut started_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoundId => {
                            if round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roundId"));
                            }
                            round_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartedBy => {
                            if started_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startedBy"));
                            }
                            started_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartedAt => {
                            if started_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startedAt"));
                            }
                            started_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventNewRound {
                    round_id: round_id__.unwrap_or_default(),
                    started_by: started_by__.unwrap_or_default(),
                    started_at: started_at__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.EventNewRound",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventNewTransmission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.aggregator_round_id != 0 {
            len += 1;
        }
        if !self.answer.is_empty() {
            len += 1;
        }
        if !self.transmitter.is_empty() {
            len += 1;
        }
        if self.observations_timestamp != 0 {
            len += 1;
        }
        if !self.observations.is_empty() {
            len += 1;
        }
        if !self.observers.is_empty() {
            len += 1;
        }
        if !self.config_digest.is_empty() {
            len += 1;
        }
        if self.epoch_and_round.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.EventNewTransmission", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if self.aggregator_round_id != 0 {
            struct_ser.serialize_field("aggregatorRoundId", &self.aggregator_round_id)?;
        }
        if !self.answer.is_empty() {
            struct_ser.serialize_field("answer", &self.answer)?;
        }
        if !self.transmitter.is_empty() {
            struct_ser.serialize_field("transmitter", &self.transmitter)?;
        }
        if self.observations_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "observationsTimestamp",
                ToString::to_string(&self.observations_timestamp).as_str(),
            )?;
        }
        if !self.observations.is_empty() {
            struct_ser.serialize_field("observations", &self.observations)?;
        }
        if !self.observers.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "observers",
                pbjson::private::base64::encode(&self.observers).as_str(),
            )?;
        }
        if !self.config_digest.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "configDigest",
                pbjson::private::base64::encode(&self.config_digest).as_str(),
            )?;
        }
        if let Some(v) = self.epoch_and_round.as_ref() {
            struct_ser.serialize_field("epochAndRound", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventNewTransmission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_id",
            "feedId",
            "aggregator_round_id",
            "aggregatorRoundId",
            "answer",
            "transmitter",
            "observations_timestamp",
            "observationsTimestamp",
            "observations",
            "observers",
            "config_digest",
            "configDigest",
            "epoch_and_round",
            "epochAndRound",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            AggregatorRoundId,
            Answer,
            Transmitter,
            ObservationsTimestamp,
            Observations,
            Observers,
            ConfigDigest,
            EpochAndRound,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "aggregatorRoundId" | "aggregator_round_id" => {
                                Ok(GeneratedField::AggregatorRoundId)
                            }
                            "answer" => Ok(GeneratedField::Answer),
                            "transmitter" => Ok(GeneratedField::Transmitter),
                            "observationsTimestamp" | "observations_timestamp" => {
                                Ok(GeneratedField::ObservationsTimestamp)
                            }
                            "observations" => Ok(GeneratedField::Observations),
                            "observers" => Ok(GeneratedField::Observers),
                            "configDigest" | "config_digest" => Ok(GeneratedField::ConfigDigest),
                            "epochAndRound" | "epoch_and_round" => {
                                Ok(GeneratedField::EpochAndRound)
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
            type Value = EventNewTransmission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.EventNewTransmission")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventNewTransmission, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut aggregator_round_id__ = None;
                let mut answer__ = None;
                let mut transmitter__ = None;
                let mut observations_timestamp__ = None;
                let mut observations__ = None;
                let mut observers__ = None;
                let mut config_digest__ = None;
                let mut epoch_and_round__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AggregatorRoundId => {
                            if aggregator_round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregatorRoundId"));
                            }
                            aggregator_round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Answer => {
                            if answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answer"));
                            }
                            answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitter => {
                            if transmitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitter"));
                            }
                            transmitter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ObservationsTimestamp => {
                            if observations_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "observationsTimestamp",
                                ));
                            }
                            observations_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Observations => {
                            if observations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("observations"));
                            }
                            observations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Observers => {
                            if observers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("observers"));
                            }
                            observers__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ConfigDigest => {
                            if config_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDigest"));
                            }
                            config_digest__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EpochAndRound => {
                            if epoch_and_round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochAndRound"));
                            }
                            epoch_and_round__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventNewTransmission {
                    feed_id: feed_id__.unwrap_or_default(),
                    aggregator_round_id: aggregator_round_id__.unwrap_or_default(),
                    answer: answer__.unwrap_or_default(),
                    transmitter: transmitter__.unwrap_or_default(),
                    observations_timestamp: observations_timestamp__.unwrap_or_default(),
                    observations: observations__.unwrap_or_default(),
                    observers: observers__.unwrap_or_default(),
                    config_digest: config_digest__.unwrap_or_default(),
                    epoch_and_round: epoch_and_round__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.EventNewTransmission",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventOraclePaid {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transmitter_addr.is_empty() {
            len += 1;
        }
        if !self.payee_addr.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.EventOraclePaid", len)?;
        if !self.transmitter_addr.is_empty() {
            struct_ser.serialize_field("transmitterAddr", &self.transmitter_addr)?;
        }
        if !self.payee_addr.is_empty() {
            struct_ser.serialize_field("payeeAddr", &self.payee_addr)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventOraclePaid {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transmitter_addr",
            "transmitterAddr",
            "payee_addr",
            "payeeAddr",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransmitterAddr,
            PayeeAddr,
            Amount,
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
                            "transmitterAddr" | "transmitter_addr" => {
                                Ok(GeneratedField::TransmitterAddr)
                            }
                            "payeeAddr" | "payee_addr" => Ok(GeneratedField::PayeeAddr),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventOraclePaid;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.EventOraclePaid")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventOraclePaid, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut transmitter_addr__ = None;
                let mut payee_addr__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransmitterAddr => {
                            if transmitter_addr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitterAddr"));
                            }
                            transmitter_addr__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PayeeAddr => {
                            if payee_addr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payeeAddr"));
                            }
                            payee_addr__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventOraclePaid {
                    transmitter_addr: transmitter_addr__.unwrap_or_default(),
                    payee_addr: payee_addr__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.EventOraclePaid",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventTransmitted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.config_digest.is_empty() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.EventTransmitted", len)?;
        if !self.config_digest.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "configDigest",
                pbjson::private::base64::encode(&self.config_digest).as_str(),
            )?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventTransmitted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["config_digest", "configDigest", "epoch"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigDigest,
            Epoch,
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
                            "configDigest" | "config_digest" => Ok(GeneratedField::ConfigDigest),
                            "epoch" => Ok(GeneratedField::Epoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventTransmitted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.EventTransmitted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventTransmitted, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut config_digest__ = None;
                let mut epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConfigDigest => {
                            if config_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDigest"));
                            }
                            config_digest__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EventTransmitted {
                    config_digest: config_digest__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.EventTransmitted",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeedConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signers.is_empty() {
            len += 1;
        }
        if !self.transmitters.is_empty() {
            len += 1;
        }
        if self.f != 0 {
            len += 1;
        }
        if !self.onchain_config.is_empty() {
            len += 1;
        }
        if self.offchain_config_version != 0 {
            len += 1;
        }
        if !self.offchain_config.is_empty() {
            len += 1;
        }
        if self.module_params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.FeedConfig", len)?;
        if !self.signers.is_empty() {
            struct_ser.serialize_field("signers", &self.signers)?;
        }
        if !self.transmitters.is_empty() {
            struct_ser.serialize_field("transmitters", &self.transmitters)?;
        }
        if self.f != 0 {
            struct_ser.serialize_field("f", &self.f)?;
        }
        if !self.onchain_config.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "onchainConfig",
                pbjson::private::base64::encode(&self.onchain_config).as_str(),
            )?;
        }
        if self.offchain_config_version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "offchainConfigVersion",
                ToString::to_string(&self.offchain_config_version).as_str(),
            )?;
        }
        if !self.offchain_config.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "offchainConfig",
                pbjson::private::base64::encode(&self.offchain_config).as_str(),
            )?;
        }
        if let Some(v) = self.module_params.as_ref() {
            struct_ser.serialize_field("moduleParams", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeedConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signers",
            "transmitters",
            "f",
            "onchain_config",
            "onchainConfig",
            "offchain_config_version",
            "offchainConfigVersion",
            "offchain_config",
            "offchainConfig",
            "module_params",
            "moduleParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signers,
            Transmitters,
            F,
            OnchainConfig,
            OffchainConfigVersion,
            OffchainConfig,
            ModuleParams,
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
                            "signers" => Ok(GeneratedField::Signers),
                            "transmitters" => Ok(GeneratedField::Transmitters),
                            "f" => Ok(GeneratedField::F),
                            "onchainConfig" | "onchain_config" => Ok(GeneratedField::OnchainConfig),
                            "offchainConfigVersion" | "offchain_config_version" => {
                                Ok(GeneratedField::OffchainConfigVersion)
                            }
                            "offchainConfig" | "offchain_config" => {
                                Ok(GeneratedField::OffchainConfig)
                            }
                            "moduleParams" | "module_params" => Ok(GeneratedField::ModuleParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeedConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.FeedConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signers__ = None;
                let mut transmitters__ = None;
                let mut f__ = None;
                let mut onchain_config__ = None;
                let mut offchain_config_version__ = None;
                let mut offchain_config__ = None;
                let mut module_params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signers => {
                            if signers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signers"));
                            }
                            signers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitters => {
                            if transmitters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitters"));
                            }
                            transmitters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::F => {
                            if f__.is_some() {
                                return Err(serde::de::Error::duplicate_field("f"));
                            }
                            f__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OnchainConfig => {
                            if onchain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onchainConfig"));
                            }
                            onchain_config__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OffchainConfigVersion => {
                            if offchain_config_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "offchainConfigVersion",
                                ));
                            }
                            offchain_config_version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OffchainConfig => {
                            if offchain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offchainConfig"));
                            }
                            offchain_config__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ModuleParams => {
                            if module_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moduleParams"));
                            }
                            module_params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FeedConfig {
                    signers: signers__.unwrap_or_default(),
                    transmitters: transmitters__.unwrap_or_default(),
                    f: f__.unwrap_or_default(),
                    onchain_config: onchain_config__.unwrap_or_default(),
                    offchain_config_version: offchain_config_version__.unwrap_or_default(),
                    offchain_config: offchain_config__.unwrap_or_default(),
                    module_params: module_params__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.FeedConfig",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeedConfigInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.latest_config_digest.is_empty() {
            len += 1;
        }
        if self.f != 0 {
            len += 1;
        }
        if self.n != 0 {
            len += 1;
        }
        if self.config_count != 0 {
            len += 1;
        }
        if self.latest_config_block_number != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.FeedConfigInfo", len)?;
        if !self.latest_config_digest.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "latestConfigDigest",
                pbjson::private::base64::encode(&self.latest_config_digest).as_str(),
            )?;
        }
        if self.f != 0 {
            struct_ser.serialize_field("f", &self.f)?;
        }
        if self.n != 0 {
            struct_ser.serialize_field("n", &self.n)?;
        }
        if self.config_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "configCount",
                ToString::to_string(&self.config_count).as_str(),
            )?;
        }
        if self.latest_config_block_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "latestConfigBlockNumber",
                ToString::to_string(&self.latest_config_block_number).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeedConfigInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "latest_config_digest",
            "latestConfigDigest",
            "f",
            "n",
            "config_count",
            "configCount",
            "latest_config_block_number",
            "latestConfigBlockNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LatestConfigDigest,
            F,
            N,
            ConfigCount,
            LatestConfigBlockNumber,
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
                            "latestConfigDigest" | "latest_config_digest" => {
                                Ok(GeneratedField::LatestConfigDigest)
                            }
                            "f" => Ok(GeneratedField::F),
                            "n" => Ok(GeneratedField::N),
                            "configCount" | "config_count" => Ok(GeneratedField::ConfigCount),
                            "latestConfigBlockNumber" | "latest_config_block_number" => {
                                Ok(GeneratedField::LatestConfigBlockNumber)
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
            type Value = FeedConfigInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.FeedConfigInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedConfigInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut latest_config_digest__ = None;
                let mut f__ = None;
                let mut n__ = None;
                let mut config_count__ = None;
                let mut latest_config_block_number__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LatestConfigDigest => {
                            if latest_config_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "latestConfigDigest",
                                ));
                            }
                            latest_config_digest__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::F => {
                            if f__.is_some() {
                                return Err(serde::de::Error::duplicate_field("f"));
                            }
                            f__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::N => {
                            if n__.is_some() {
                                return Err(serde::de::Error::duplicate_field("n"));
                            }
                            n__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ConfigCount => {
                            if config_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configCount"));
                            }
                            config_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LatestConfigBlockNumber => {
                            if latest_config_block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "latestConfigBlockNumber",
                                ));
                            }
                            latest_config_block_number__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(FeedConfigInfo {
                    latest_config_digest: latest_config_digest__.unwrap_or_default(),
                    f: f__.unwrap_or_default(),
                    n: n__.unwrap_or_default(),
                    config_count: config_count__.unwrap_or_default(),
                    latest_config_block_number: latest_config_block_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.FeedConfigInfo",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeedCounts {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if !self.counts.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.FeedCounts", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if !self.counts.is_empty() {
            struct_ser.serialize_field("counts", &self.counts)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeedCounts {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId", "counts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            Counts,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "counts" => Ok(GeneratedField::Counts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeedCounts;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.FeedCounts")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedCounts, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut counts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Counts => {
                            if counts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counts"));
                            }
                            counts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FeedCounts {
                    feed_id: feed_id__.unwrap_or_default(),
                    counts: counts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.FeedCounts",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeedEpochAndRound {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.epoch_and_round.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.FeedEpochAndRound", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if let Some(v) = self.epoch_and_round.as_ref() {
            struct_ser.serialize_field("epochAndRound", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeedEpochAndRound {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId", "epoch_and_round", "epochAndRound"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            EpochAndRound,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "epochAndRound" | "epoch_and_round" => {
                                Ok(GeneratedField::EpochAndRound)
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
            type Value = FeedEpochAndRound;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.FeedEpochAndRound")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedEpochAndRound, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut epoch_and_round__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EpochAndRound => {
                            if epoch_and_round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochAndRound"));
                            }
                            epoch_and_round__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FeedEpochAndRound {
                    feed_id: feed_id__.unwrap_or_default(),
                    epoch_and_round: epoch_and_round__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.FeedEpochAndRound",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeedLatestAggregatorRoundIDs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.aggregator_round_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.ocr.v1beta1.FeedLatestAggregatorRoundIDs", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if self.aggregator_round_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "aggregatorRoundId",
                ToString::to_string(&self.aggregator_round_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeedLatestAggregatorRoundIDs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_id",
            "feedId",
            "aggregator_round_id",
            "aggregatorRoundId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            AggregatorRoundId,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "aggregatorRoundId" | "aggregator_round_id" => {
                                Ok(GeneratedField::AggregatorRoundId)
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
            type Value = FeedLatestAggregatorRoundIDs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.FeedLatestAggregatorRoundIDs")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<FeedLatestAggregatorRoundIDs, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut aggregator_round_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AggregatorRoundId => {
                            if aggregator_round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregatorRoundId"));
                            }
                            aggregator_round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(FeedLatestAggregatorRoundIDs {
                    feed_id: feed_id__.unwrap_or_default(),
                    aggregator_round_id: aggregator_round_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.FeedLatestAggregatorRoundIDs",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeedProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.f != 0 {
            len += 1;
        }
        if !self.onchain_config.is_empty() {
            len += 1;
        }
        if self.offchain_config_version != 0 {
            len += 1;
        }
        if !self.offchain_config.is_empty() {
            len += 1;
        }
        if !self.min_answer.is_empty() {
            len += 1;
        }
        if !self.max_answer.is_empty() {
            len += 1;
        }
        if !self.link_per_observation.is_empty() {
            len += 1;
        }
        if !self.link_per_transmission.is_empty() {
            len += 1;
        }
        if self.unique_reports {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.FeedProperties", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if self.f != 0 {
            struct_ser.serialize_field("f", &self.f)?;
        }
        if !self.onchain_config.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "onchainConfig",
                pbjson::private::base64::encode(&self.onchain_config).as_str(),
            )?;
        }
        if self.offchain_config_version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "offchainConfigVersion",
                ToString::to_string(&self.offchain_config_version).as_str(),
            )?;
        }
        if !self.offchain_config.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "offchainConfig",
                pbjson::private::base64::encode(&self.offchain_config).as_str(),
            )?;
        }
        if !self.min_answer.is_empty() {
            struct_ser.serialize_field("minAnswer", &self.min_answer)?;
        }
        if !self.max_answer.is_empty() {
            struct_ser.serialize_field("maxAnswer", &self.max_answer)?;
        }
        if !self.link_per_observation.is_empty() {
            struct_ser.serialize_field("linkPerObservation", &self.link_per_observation)?;
        }
        if !self.link_per_transmission.is_empty() {
            struct_ser.serialize_field("linkPerTransmission", &self.link_per_transmission)?;
        }
        if self.unique_reports {
            struct_ser.serialize_field("uniqueReports", &self.unique_reports)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeedProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_id",
            "feedId",
            "f",
            "onchain_config",
            "onchainConfig",
            "offchain_config_version",
            "offchainConfigVersion",
            "offchain_config",
            "offchainConfig",
            "min_answer",
            "minAnswer",
            "max_answer",
            "maxAnswer",
            "link_per_observation",
            "linkPerObservation",
            "link_per_transmission",
            "linkPerTransmission",
            "unique_reports",
            "uniqueReports",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            F,
            OnchainConfig,
            OffchainConfigVersion,
            OffchainConfig,
            MinAnswer,
            MaxAnswer,
            LinkPerObservation,
            LinkPerTransmission,
            UniqueReports,
            Description,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "f" => Ok(GeneratedField::F),
                            "onchainConfig" | "onchain_config" => Ok(GeneratedField::OnchainConfig),
                            "offchainConfigVersion" | "offchain_config_version" => {
                                Ok(GeneratedField::OffchainConfigVersion)
                            }
                            "offchainConfig" | "offchain_config" => {
                                Ok(GeneratedField::OffchainConfig)
                            }
                            "minAnswer" | "min_answer" => Ok(GeneratedField::MinAnswer),
                            "maxAnswer" | "max_answer" => Ok(GeneratedField::MaxAnswer),
                            "linkPerObservation" | "link_per_observation" => {
                                Ok(GeneratedField::LinkPerObservation)
                            }
                            "linkPerTransmission" | "link_per_transmission" => {
                                Ok(GeneratedField::LinkPerTransmission)
                            }
                            "uniqueReports" | "unique_reports" => Ok(GeneratedField::UniqueReports),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeedProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.FeedProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedProperties, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut f__ = None;
                let mut onchain_config__ = None;
                let mut offchain_config_version__ = None;
                let mut offchain_config__ = None;
                let mut min_answer__ = None;
                let mut max_answer__ = None;
                let mut link_per_observation__ = None;
                let mut link_per_transmission__ = None;
                let mut unique_reports__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::F => {
                            if f__.is_some() {
                                return Err(serde::de::Error::duplicate_field("f"));
                            }
                            f__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OnchainConfig => {
                            if onchain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onchainConfig"));
                            }
                            onchain_config__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OffchainConfigVersion => {
                            if offchain_config_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "offchainConfigVersion",
                                ));
                            }
                            offchain_config_version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OffchainConfig => {
                            if offchain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offchainConfig"));
                            }
                            offchain_config__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinAnswer => {
                            if min_answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minAnswer"));
                            }
                            min_answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxAnswer => {
                            if max_answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAnswer"));
                            }
                            max_answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkPerObservation => {
                            if link_per_observation__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "linkPerObservation",
                                ));
                            }
                            link_per_observation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkPerTransmission => {
                            if link_per_transmission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "linkPerTransmission",
                                ));
                            }
                            link_per_transmission__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UniqueReports => {
                            if unique_reports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uniqueReports"));
                            }
                            unique_reports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FeedProperties {
                    feed_id: feed_id__.unwrap_or_default(),
                    f: f__.unwrap_or_default(),
                    onchain_config: onchain_config__.unwrap_or_default(),
                    offchain_config_version: offchain_config_version__.unwrap_or_default(),
                    offchain_config: offchain_config__.unwrap_or_default(),
                    min_answer: min_answer__.unwrap_or_default(),
                    max_answer: max_answer__.unwrap_or_default(),
                    link_per_observation: link_per_observation__.unwrap_or_default(),
                    link_per_transmission: link_per_transmission__.unwrap_or_default(),
                    unique_reports: unique_reports__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.FeedProperties",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeedTransmission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.transmission.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.FeedTransmission", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if let Some(v) = self.transmission.as_ref() {
            struct_ser.serialize_field("transmission", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeedTransmission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId", "transmission"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            Transmission,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "transmission" => Ok(GeneratedField::Transmission),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeedTransmission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.FeedTransmission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeedTransmission, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut transmission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmission => {
                            if transmission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmission"));
                            }
                            transmission__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FeedTransmission {
                    feed_id: feed_id__.unwrap_or_default(),
                    transmission: transmission__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.FeedTransmission",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GasReimbursements {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reimbursements.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.GasReimbursements", len)?;
        if !self.reimbursements.is_empty() {
            struct_ser.serialize_field("reimbursements", &self.reimbursements)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GasReimbursements {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reimbursements"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reimbursements,
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
                            "reimbursements" => Ok(GeneratedField::Reimbursements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GasReimbursements;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.GasReimbursements")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GasReimbursements, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reimbursements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reimbursements => {
                            if reimbursements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reimbursements"));
                            }
                            reimbursements__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GasReimbursements {
                    reimbursements: reimbursements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.GasReimbursements",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        if !self.feed_configs.is_empty() {
            len += 1;
        }
        if !self.latest_epoch_and_rounds.is_empty() {
            len += 1;
        }
        if !self.feed_transmissions.is_empty() {
            len += 1;
        }
        if !self.latest_aggregator_round_ids.is_empty() {
            len += 1;
        }
        if !self.reward_pools.is_empty() {
            len += 1;
        }
        if !self.feed_observation_counts.is_empty() {
            len += 1;
        }
        if !self.feed_transmission_counts.is_empty() {
            len += 1;
        }
        if !self.pending_payeeships.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.feed_configs.is_empty() {
            struct_ser.serialize_field("feedConfigs", &self.feed_configs)?;
        }
        if !self.latest_epoch_and_rounds.is_empty() {
            struct_ser.serialize_field("latestEpochAndRounds", &self.latest_epoch_and_rounds)?;
        }
        if !self.feed_transmissions.is_empty() {
            struct_ser.serialize_field("feedTransmissions", &self.feed_transmissions)?;
        }
        if !self.latest_aggregator_round_ids.is_empty() {
            struct_ser.serialize_field(
                "latestAggregatorRoundIds",
                &self.latest_aggregator_round_ids,
            )?;
        }
        if !self.reward_pools.is_empty() {
            struct_ser.serialize_field("rewardPools", &self.reward_pools)?;
        }
        if !self.feed_observation_counts.is_empty() {
            struct_ser.serialize_field("feedObservationCounts", &self.feed_observation_counts)?;
        }
        if !self.feed_transmission_counts.is_empty() {
            struct_ser.serialize_field("feedTransmissionCounts", &self.feed_transmission_counts)?;
        }
        if !self.pending_payeeships.is_empty() {
            struct_ser.serialize_field("pendingPayeeships", &self.pending_payeeships)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
            "feed_configs",
            "feedConfigs",
            "latest_epoch_and_rounds",
            "latestEpochAndRounds",
            "feed_transmissions",
            "feedTransmissions",
            "latest_aggregator_round_ids",
            "latestAggregatorRoundIds",
            "reward_pools",
            "rewardPools",
            "feed_observation_counts",
            "feedObservationCounts",
            "feed_transmission_counts",
            "feedTransmissionCounts",
            "pending_payeeships",
            "pendingPayeeships",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            FeedConfigs,
            LatestEpochAndRounds,
            FeedTransmissions,
            LatestAggregatorRoundIds,
            RewardPools,
            FeedObservationCounts,
            FeedTransmissionCounts,
            PendingPayeeships,
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
                            "params" => Ok(GeneratedField::Params),
                            "feedConfigs" | "feed_configs" => Ok(GeneratedField::FeedConfigs),
                            "latestEpochAndRounds" | "latest_epoch_and_rounds" => {
                                Ok(GeneratedField::LatestEpochAndRounds)
                            }
                            "feedTransmissions" | "feed_transmissions" => {
                                Ok(GeneratedField::FeedTransmissions)
                            }
                            "latestAggregatorRoundIds" | "latest_aggregator_round_ids" => {
                                Ok(GeneratedField::LatestAggregatorRoundIds)
                            }
                            "rewardPools" | "reward_pools" => Ok(GeneratedField::RewardPools),
                            "feedObservationCounts" | "feed_observation_counts" => {
                                Ok(GeneratedField::FeedObservationCounts)
                            }
                            "feedTransmissionCounts" | "feed_transmission_counts" => {
                                Ok(GeneratedField::FeedTransmissionCounts)
                            }
                            "pendingPayeeships" | "pending_payeeships" => {
                                Ok(GeneratedField::PendingPayeeships)
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
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut feed_configs__ = None;
                let mut latest_epoch_and_rounds__ = None;
                let mut feed_transmissions__ = None;
                let mut latest_aggregator_round_ids__ = None;
                let mut reward_pools__ = None;
                let mut feed_observation_counts__ = None;
                let mut feed_transmission_counts__ = None;
                let mut pending_payeeships__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::FeedConfigs => {
                            if feed_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedConfigs"));
                            }
                            feed_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LatestEpochAndRounds => {
                            if latest_epoch_and_rounds__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "latestEpochAndRounds",
                                ));
                            }
                            latest_epoch_and_rounds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedTransmissions => {
                            if feed_transmissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedTransmissions"));
                            }
                            feed_transmissions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LatestAggregatorRoundIds => {
                            if latest_aggregator_round_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "latestAggregatorRoundIds",
                                ));
                            }
                            latest_aggregator_round_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RewardPools => {
                            if reward_pools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardPools"));
                            }
                            reward_pools__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedObservationCounts => {
                            if feed_observation_counts__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "feedObservationCounts",
                                ));
                            }
                            feed_observation_counts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedTransmissionCounts => {
                            if feed_transmission_counts__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "feedTransmissionCounts",
                                ));
                            }
                            feed_transmission_counts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PendingPayeeships => {
                            if pending_payeeships__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingPayeeships"));
                            }
                            pending_payeeships__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    feed_configs: feed_configs__.unwrap_or_default(),
                    latest_epoch_and_rounds: latest_epoch_and_rounds__.unwrap_or_default(),
                    feed_transmissions: feed_transmissions__.unwrap_or_default(),
                    latest_aggregator_round_ids: latest_aggregator_round_ids__.unwrap_or_default(),
                    reward_pools: reward_pools__.unwrap_or_default(),
                    feed_observation_counts: feed_observation_counts__.unwrap_or_default(),
                    feed_transmission_counts: feed_transmission_counts__.unwrap_or_default(),
                    pending_payeeships: pending_payeeships__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ModuleParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if !self.min_answer.is_empty() {
            len += 1;
        }
        if !self.max_answer.is_empty() {
            len += 1;
        }
        if !self.link_per_observation.is_empty() {
            len += 1;
        }
        if !self.link_per_transmission.is_empty() {
            len += 1;
        }
        if !self.link_denom.is_empty() {
            len += 1;
        }
        if self.unique_reports {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.feed_admin.is_empty() {
            len += 1;
        }
        if !self.billing_admin.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.ModuleParams", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if !self.min_answer.is_empty() {
            struct_ser.serialize_field("minAnswer", &self.min_answer)?;
        }
        if !self.max_answer.is_empty() {
            struct_ser.serialize_field("maxAnswer", &self.max_answer)?;
        }
        if !self.link_per_observation.is_empty() {
            struct_ser.serialize_field("linkPerObservation", &self.link_per_observation)?;
        }
        if !self.link_per_transmission.is_empty() {
            struct_ser.serialize_field("linkPerTransmission", &self.link_per_transmission)?;
        }
        if !self.link_denom.is_empty() {
            struct_ser.serialize_field("linkDenom", &self.link_denom)?;
        }
        if self.unique_reports {
            struct_ser.serialize_field("uniqueReports", &self.unique_reports)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.feed_admin.is_empty() {
            struct_ser.serialize_field("feedAdmin", &self.feed_admin)?;
        }
        if !self.billing_admin.is_empty() {
            struct_ser.serialize_field("billingAdmin", &self.billing_admin)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ModuleParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_id",
            "feedId",
            "min_answer",
            "minAnswer",
            "max_answer",
            "maxAnswer",
            "link_per_observation",
            "linkPerObservation",
            "link_per_transmission",
            "linkPerTransmission",
            "link_denom",
            "linkDenom",
            "unique_reports",
            "uniqueReports",
            "description",
            "feed_admin",
            "feedAdmin",
            "billing_admin",
            "billingAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            MinAnswer,
            MaxAnswer,
            LinkPerObservation,
            LinkPerTransmission,
            LinkDenom,
            UniqueReports,
            Description,
            FeedAdmin,
            BillingAdmin,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "minAnswer" | "min_answer" => Ok(GeneratedField::MinAnswer),
                            "maxAnswer" | "max_answer" => Ok(GeneratedField::MaxAnswer),
                            "linkPerObservation" | "link_per_observation" => {
                                Ok(GeneratedField::LinkPerObservation)
                            }
                            "linkPerTransmission" | "link_per_transmission" => {
                                Ok(GeneratedField::LinkPerTransmission)
                            }
                            "linkDenom" | "link_denom" => Ok(GeneratedField::LinkDenom),
                            "uniqueReports" | "unique_reports" => Ok(GeneratedField::UniqueReports),
                            "description" => Ok(GeneratedField::Description),
                            "feedAdmin" | "feed_admin" => Ok(GeneratedField::FeedAdmin),
                            "billingAdmin" | "billing_admin" => Ok(GeneratedField::BillingAdmin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModuleParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.ModuleParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModuleParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut min_answer__ = None;
                let mut max_answer__ = None;
                let mut link_per_observation__ = None;
                let mut link_per_transmission__ = None;
                let mut link_denom__ = None;
                let mut unique_reports__ = None;
                let mut description__ = None;
                let mut feed_admin__ = None;
                let mut billing_admin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MinAnswer => {
                            if min_answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minAnswer"));
                            }
                            min_answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxAnswer => {
                            if max_answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAnswer"));
                            }
                            max_answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkPerObservation => {
                            if link_per_observation__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "linkPerObservation",
                                ));
                            }
                            link_per_observation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkPerTransmission => {
                            if link_per_transmission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "linkPerTransmission",
                                ));
                            }
                            link_per_transmission__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkDenom => {
                            if link_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkDenom"));
                            }
                            link_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UniqueReports => {
                            if unique_reports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uniqueReports"));
                            }
                            unique_reports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedAdmin => {
                            if feed_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedAdmin"));
                            }
                            feed_admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BillingAdmin => {
                            if billing_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("billingAdmin"));
                            }
                            billing_admin__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ModuleParams {
                    feed_id: feed_id__.unwrap_or_default(),
                    min_answer: min_answer__.unwrap_or_default(),
                    max_answer: max_answer__.unwrap_or_default(),
                    link_per_observation: link_per_observation__.unwrap_or_default(),
                    link_per_transmission: link_per_transmission__.unwrap_or_default(),
                    link_denom: link_denom__.unwrap_or_default(),
                    unique_reports: unique_reports__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    feed_admin: feed_admin__.unwrap_or_default(),
                    billing_admin: billing_admin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.ModuleParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgAcceptPayeeship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.payee.is_empty() {
            len += 1;
        }
        if !self.transmitter.is_empty() {
            len += 1;
        }
        if !self.feed_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgAcceptPayeeship", len)?;
        if !self.payee.is_empty() {
            struct_ser.serialize_field("payee", &self.payee)?;
        }
        if !self.transmitter.is_empty() {
            struct_ser.serialize_field("transmitter", &self.transmitter)?;
        }
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgAcceptPayeeship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["payee", "transmitter", "feed_id", "feedId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Payee,
            Transmitter,
            FeedId,
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
                            "payee" => Ok(GeneratedField::Payee),
                            "transmitter" => Ok(GeneratedField::Transmitter),
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAcceptPayeeship;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgAcceptPayeeship")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgAcceptPayeeship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut payee__ = None;
                let mut transmitter__ = None;
                let mut feed_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Payee => {
                            if payee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payee"));
                            }
                            payee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitter => {
                            if transmitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitter"));
                            }
                            transmitter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgAcceptPayeeship {
                    payee: payee__.unwrap_or_default(),
                    transmitter: transmitter__.unwrap_or_default(),
                    feed_id: feed_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgAcceptPayeeship",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgAcceptPayeeshipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgAcceptPayeeshipResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgAcceptPayeeshipResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAcceptPayeeshipResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgAcceptPayeeshipResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgAcceptPayeeshipResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAcceptPayeeshipResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgAcceptPayeeshipResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreateFeed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgCreateFeed", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreateFeed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Config,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateFeed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgCreateFeed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreateFeed, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgCreateFeed {
                    sender: sender__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgCreateFeed",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreateFeedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgCreateFeedResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreateFeedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateFeedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgCreateFeedResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgCreateFeedResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreateFeedResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgCreateFeedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgFundFeedRewardPool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgFundFeedRewardPool", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgFundFeedRewardPool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "feed_id", "feedId", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            FeedId,
            Amount,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgFundFeedRewardPool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgFundFeedRewardPool")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgFundFeedRewardPool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut feed_id__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgFundFeedRewardPool {
                    sender: sender__.unwrap_or_default(),
                    feed_id: feed_id__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgFundFeedRewardPool",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgFundFeedRewardPoolResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.ocr.v1beta1.MsgFundFeedRewardPoolResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgFundFeedRewardPoolResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgFundFeedRewardPoolResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgFundFeedRewardPoolResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgFundFeedRewardPoolResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgFundFeedRewardPoolResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgFundFeedRewardPoolResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSetPayees {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if !self.transmitters.is_empty() {
            len += 1;
        }
        if !self.payees.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgSetPayees", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if !self.transmitters.is_empty() {
            struct_ser.serialize_field("transmitters", &self.transmitters)?;
        }
        if !self.payees.is_empty() {
            struct_ser.serialize_field("payees", &self.payees)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSetPayees {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "feed_id", "feedId", "transmitters", "payees"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            FeedId,
            Transmitters,
            Payees,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "transmitters" => Ok(GeneratedField::Transmitters),
                            "payees" => Ok(GeneratedField::Payees),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetPayees;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgSetPayees")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSetPayees, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut feed_id__ = None;
                let mut transmitters__ = None;
                let mut payees__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitters => {
                            if transmitters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitters"));
                            }
                            transmitters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Payees => {
                            if payees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payees"));
                            }
                            payees__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetPayees {
                    sender: sender__.unwrap_or_default(),
                    feed_id: feed_id__.unwrap_or_default(),
                    transmitters: transmitters__.unwrap_or_default(),
                    payees: payees__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgSetPayees",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSetPayeesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgSetPayeesResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSetPayeesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetPayeesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgSetPayeesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetPayeesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetPayeesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgSetPayeesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgTransferPayeeship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.transmitter.is_empty() {
            len += 1;
        }
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if !self.proposed.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgTransferPayeeship", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.transmitter.is_empty() {
            struct_ser.serialize_field("transmitter", &self.transmitter)?;
        }
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if !self.proposed.is_empty() {
            struct_ser.serialize_field("proposed", &self.proposed)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgTransferPayeeship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "transmitter", "feed_id", "feedId", "proposed"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Transmitter,
            FeedId,
            Proposed,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "transmitter" => Ok(GeneratedField::Transmitter),
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "proposed" => Ok(GeneratedField::Proposed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTransferPayeeship;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgTransferPayeeship")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgTransferPayeeship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut transmitter__ = None;
                let mut feed_id__ = None;
                let mut proposed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitter => {
                            if transmitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitter"));
                            }
                            transmitter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Proposed => {
                            if proposed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposed"));
                            }
                            proposed__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgTransferPayeeship {
                    sender: sender__.unwrap_or_default(),
                    transmitter: transmitter__.unwrap_or_default(),
                    feed_id: feed_id__.unwrap_or_default(),
                    proposed: proposed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgTransferPayeeship",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgTransferPayeeshipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.ocr.v1beta1.MsgTransferPayeeshipResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgTransferPayeeshipResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTransferPayeeshipResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgTransferPayeeshipResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgTransferPayeeshipResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgTransferPayeeshipResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgTransferPayeeshipResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgTransmit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transmitter.is_empty() {
            len += 1;
        }
        if !self.config_digest.is_empty() {
            len += 1;
        }
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        if self.round != 0 {
            len += 1;
        }
        if !self.extra_hash.is_empty() {
            len += 1;
        }
        if self.report.is_some() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgTransmit", len)?;
        if !self.transmitter.is_empty() {
            struct_ser.serialize_field("transmitter", &self.transmitter)?;
        }
        if !self.config_digest.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "configDigest",
                pbjson::private::base64::encode(&self.config_digest).as_str(),
            )?;
        }
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if self.round != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("round", ToString::to_string(&self.round).as_str())?;
        }
        if !self.extra_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "extraHash",
                pbjson::private::base64::encode(&self.extra_hash).as_str(),
            )?;
        }
        if let Some(v) = self.report.as_ref() {
            struct_ser.serialize_field("report", v)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field(
                "signatures",
                &self
                    .signatures
                    .iter()
                    .map(pbjson::private::base64::encode)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgTransmit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transmitter",
            "config_digest",
            "configDigest",
            "feed_id",
            "feedId",
            "epoch",
            "round",
            "extra_hash",
            "extraHash",
            "report",
            "signatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transmitter,
            ConfigDigest,
            FeedId,
            Epoch,
            Round,
            ExtraHash,
            Report,
            Signatures,
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
                            "transmitter" => Ok(GeneratedField::Transmitter),
                            "configDigest" | "config_digest" => Ok(GeneratedField::ConfigDigest),
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "epoch" => Ok(GeneratedField::Epoch),
                            "round" => Ok(GeneratedField::Round),
                            "extraHash" | "extra_hash" => Ok(GeneratedField::ExtraHash),
                            "report" => Ok(GeneratedField::Report),
                            "signatures" => Ok(GeneratedField::Signatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTransmit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgTransmit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgTransmit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut transmitter__ = None;
                let mut config_digest__ = None;
                let mut feed_id__ = None;
                let mut epoch__ = None;
                let mut round__ = None;
                let mut extra_hash__ = None;
                let mut report__ = None;
                let mut signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transmitter => {
                            if transmitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitter"));
                            }
                            transmitter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigDigest => {
                            if config_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDigest"));
                            }
                            config_digest__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Round => {
                            if round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("round"));
                            }
                            round__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ExtraHash => {
                            if extra_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extraHash"));
                            }
                            extra_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Report => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("report"));
                            }
                            report__ = map_.next_value()?;
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(
                                map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(MsgTransmit {
                    transmitter: transmitter__.unwrap_or_default(),
                    config_digest: config_digest__.unwrap_or_default(),
                    feed_id: feed_id__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                    round: round__.unwrap_or_default(),
                    extra_hash: extra_hash__.unwrap_or_default(),
                    report: report__,
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgTransmit",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgTransmitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgTransmitResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgTransmitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTransmitResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgTransmitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgTransmitResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgTransmitResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgTransmitResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateFeed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if !self.signers.is_empty() {
            len += 1;
        }
        if !self.transmitters.is_empty() {
            len += 1;
        }
        if !self.link_per_observation.is_empty() {
            len += 1;
        }
        if !self.link_per_transmission.is_empty() {
            len += 1;
        }
        if !self.link_denom.is_empty() {
            len += 1;
        }
        if !self.feed_admin.is_empty() {
            len += 1;
        }
        if !self.billing_admin.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgUpdateFeed", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if !self.signers.is_empty() {
            struct_ser.serialize_field("signers", &self.signers)?;
        }
        if !self.transmitters.is_empty() {
            struct_ser.serialize_field("transmitters", &self.transmitters)?;
        }
        if !self.link_per_observation.is_empty() {
            struct_ser.serialize_field("linkPerObservation", &self.link_per_observation)?;
        }
        if !self.link_per_transmission.is_empty() {
            struct_ser.serialize_field("linkPerTransmission", &self.link_per_transmission)?;
        }
        if !self.link_denom.is_empty() {
            struct_ser.serialize_field("linkDenom", &self.link_denom)?;
        }
        if !self.feed_admin.is_empty() {
            struct_ser.serialize_field("feedAdmin", &self.feed_admin)?;
        }
        if !self.billing_admin.is_empty() {
            struct_ser.serialize_field("billingAdmin", &self.billing_admin)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateFeed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "feed_id",
            "feedId",
            "signers",
            "transmitters",
            "link_per_observation",
            "linkPerObservation",
            "link_per_transmission",
            "linkPerTransmission",
            "link_denom",
            "linkDenom",
            "feed_admin",
            "feedAdmin",
            "billing_admin",
            "billingAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            FeedId,
            Signers,
            Transmitters,
            LinkPerObservation,
            LinkPerTransmission,
            LinkDenom,
            FeedAdmin,
            BillingAdmin,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "signers" => Ok(GeneratedField::Signers),
                            "transmitters" => Ok(GeneratedField::Transmitters),
                            "linkPerObservation" | "link_per_observation" => {
                                Ok(GeneratedField::LinkPerObservation)
                            }
                            "linkPerTransmission" | "link_per_transmission" => {
                                Ok(GeneratedField::LinkPerTransmission)
                            }
                            "linkDenom" | "link_denom" => Ok(GeneratedField::LinkDenom),
                            "feedAdmin" | "feed_admin" => Ok(GeneratedField::FeedAdmin),
                            "billingAdmin" | "billing_admin" => Ok(GeneratedField::BillingAdmin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateFeed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgUpdateFeed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateFeed, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut feed_id__ = None;
                let mut signers__ = None;
                let mut transmitters__ = None;
                let mut link_per_observation__ = None;
                let mut link_per_transmission__ = None;
                let mut link_denom__ = None;
                let mut feed_admin__ = None;
                let mut billing_admin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signers => {
                            if signers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signers"));
                            }
                            signers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitters => {
                            if transmitters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitters"));
                            }
                            transmitters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkPerObservation => {
                            if link_per_observation__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "linkPerObservation",
                                ));
                            }
                            link_per_observation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkPerTransmission => {
                            if link_per_transmission__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "linkPerTransmission",
                                ));
                            }
                            link_per_transmission__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkDenom => {
                            if link_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkDenom"));
                            }
                            link_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedAdmin => {
                            if feed_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedAdmin"));
                            }
                            feed_admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BillingAdmin => {
                            if billing_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("billingAdmin"));
                            }
                            billing_admin__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateFeed {
                    sender: sender__.unwrap_or_default(),
                    feed_id: feed_id__.unwrap_or_default(),
                    signers: signers__.unwrap_or_default(),
                    transmitters: transmitters__.unwrap_or_default(),
                    link_per_observation: link_per_observation__.unwrap_or_default(),
                    link_per_transmission: link_per_transmission__.unwrap_or_default(),
                    link_denom: link_denom__.unwrap_or_default(),
                    feed_admin: feed_admin__.unwrap_or_default(),
                    billing_admin: billing_admin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgUpdateFeed",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateFeedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgUpdateFeedResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateFeedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateFeedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgUpdateFeedResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateFeedResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateFeedResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgUpdateFeedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgUpdateParams", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Params,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgUpdateParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgUpdateParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgUpdateParamsResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgUpdateParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgUpdateParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgWithdrawFeedRewardPool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.MsgWithdrawFeedRewardPool", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgWithdrawFeedRewardPool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "feed_id", "feedId", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            FeedId,
            Amount,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgWithdrawFeedRewardPool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.MsgWithdrawFeedRewardPool")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgWithdrawFeedRewardPool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut feed_id__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgWithdrawFeedRewardPool {
                    sender: sender__.unwrap_or_default(),
                    feed_id: feed_id__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgWithdrawFeedRewardPool",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgWithdrawFeedRewardPoolResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.ocr.v1beta1.MsgWithdrawFeedRewardPoolResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgWithdrawFeedRewardPoolResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgWithdrawFeedRewardPoolResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.ocr.v1beta1.MsgWithdrawFeedRewardPoolResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgWithdrawFeedRewardPoolResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgWithdrawFeedRewardPoolResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.MsgWithdrawFeedRewardPoolResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OracleObservationsCounts {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.counts.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.OracleObservationsCounts", len)?;
        if !self.counts.is_empty() {
            struct_ser.serialize_field("counts", &self.counts)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OracleObservationsCounts {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["counts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Counts,
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
                            "counts" => Ok(GeneratedField::Counts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OracleObservationsCounts;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.OracleObservationsCounts")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<OracleObservationsCounts, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut counts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Counts => {
                            if counts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counts"));
                            }
                            counts__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(OracleObservationsCounts {
                    counts: counts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.OracleObservationsCounts",
            FIELDS,
            GeneratedVisitor,
        )
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
        if !self.link_denom.is_empty() {
            len += 1;
        }
        if self.payout_block_interval != 0 {
            len += 1;
        }
        if !self.module_admin.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.ocr.v1beta1.Params", len)?;
        if !self.link_denom.is_empty() {
            struct_ser.serialize_field("linkDenom", &self.link_denom)?;
        }
        if self.payout_block_interval != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "payoutBlockInterval",
                ToString::to_string(&self.payout_block_interval).as_str(),
            )?;
        }
        if !self.module_admin.is_empty() {
            struct_ser.serialize_field("moduleAdmin", &self.module_admin)?;
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
            "link_denom",
            "linkDenom",
            "payout_block_interval",
            "payoutBlockInterval",
            "module_admin",
            "moduleAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LinkDenom,
            PayoutBlockInterval,
            ModuleAdmin,
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
                            "linkDenom" | "link_denom" => Ok(GeneratedField::LinkDenom),
                            "payoutBlockInterval" | "payout_block_interval" => {
                                Ok(GeneratedField::PayoutBlockInterval)
                            }
                            "moduleAdmin" | "module_admin" => Ok(GeneratedField::ModuleAdmin),
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
                formatter.write_str("struct injective.ocr.v1beta1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut link_denom__ = None;
                let mut payout_block_interval__ = None;
                let mut module_admin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LinkDenom => {
                            if link_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkDenom"));
                            }
                            link_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PayoutBlockInterval => {
                            if payout_block_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "payoutBlockInterval",
                                ));
                            }
                            payout_block_interval__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ModuleAdmin => {
                            if module_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moduleAdmin"));
                            }
                            module_admin__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    link_denom: link_denom__.unwrap_or_default(),
                    payout_block_interval: payout_block_interval__.unwrap_or_default(),
                    module_admin: module_admin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.ocr.v1beta1.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Payee {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transmitter_addr.is_empty() {
            len += 1;
        }
        if !self.payment_addr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.ocr.v1beta1.Payee", len)?;
        if !self.transmitter_addr.is_empty() {
            struct_ser.serialize_field("transmitterAddr", &self.transmitter_addr)?;
        }
        if !self.payment_addr.is_empty() {
            struct_ser.serialize_field("paymentAddr", &self.payment_addr)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Payee {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transmitter_addr",
            "transmitterAddr",
            "payment_addr",
            "paymentAddr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransmitterAddr,
            PaymentAddr,
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
                            "transmitterAddr" | "transmitter_addr" => {
                                Ok(GeneratedField::TransmitterAddr)
                            }
                            "paymentAddr" | "payment_addr" => Ok(GeneratedField::PaymentAddr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Payee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.Payee")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Payee, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut transmitter_addr__ = None;
                let mut payment_addr__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransmitterAddr => {
                            if transmitter_addr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitterAddr"));
                            }
                            transmitter_addr__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PaymentAddr => {
                            if payment_addr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentAddr"));
                            }
                            payment_addr__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Payee {
                    transmitter_addr: transmitter_addr__.unwrap_or_default(),
                    payment_addr: payment_addr__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.ocr.v1beta1.Payee", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PendingPayeeship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if !self.transmitter.is_empty() {
            len += 1;
        }
        if !self.proposed_payee.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.PendingPayeeship", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if !self.transmitter.is_empty() {
            struct_ser.serialize_field("transmitter", &self.transmitter)?;
        }
        if !self.proposed_payee.is_empty() {
            struct_ser.serialize_field("proposedPayee", &self.proposed_payee)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PendingPayeeship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_id",
            "feedId",
            "transmitter",
            "proposed_payee",
            "proposedPayee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            Transmitter,
            ProposedPayee,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "transmitter" => Ok(GeneratedField::Transmitter),
                            "proposedPayee" | "proposed_payee" => Ok(GeneratedField::ProposedPayee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PendingPayeeship;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.PendingPayeeship")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PendingPayeeship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut transmitter__ = None;
                let mut proposed_payee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitter => {
                            if transmitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitter"));
                            }
                            transmitter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProposedPayee => {
                            if proposed_payee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposedPayee"));
                            }
                            proposed_payee__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PendingPayeeship {
                    feed_id: feed_id__.unwrap_or_default(),
                    transmitter: transmitter__.unwrap_or_default(),
                    proposed_payee: proposed_payee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.PendingPayeeship",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryFeedConfigInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryFeedConfigInfoRequest", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryFeedConfigInfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryFeedConfigInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryFeedConfigInfoRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryFeedConfigInfoRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryFeedConfigInfoRequest {
                    feed_id: feed_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryFeedConfigInfoRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryFeedConfigInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.feed_config_info.is_some() {
            len += 1;
        }
        if self.epoch_and_round.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.ocr.v1beta1.QueryFeedConfigInfoResponse", len)?;
        if let Some(v) = self.feed_config_info.as_ref() {
            struct_ser.serialize_field("feedConfigInfo", v)?;
        }
        if let Some(v) = self.epoch_and_round.as_ref() {
            struct_ser.serialize_field("epochAndRound", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryFeedConfigInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_config_info",
            "feedConfigInfo",
            "epoch_and_round",
            "epochAndRound",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedConfigInfo,
            EpochAndRound,
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
                            "feedConfigInfo" | "feed_config_info" => {
                                Ok(GeneratedField::FeedConfigInfo)
                            }
                            "epochAndRound" | "epoch_and_round" => {
                                Ok(GeneratedField::EpochAndRound)
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
            type Value = QueryFeedConfigInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryFeedConfigInfoResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryFeedConfigInfoResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_config_info__ = None;
                let mut epoch_and_round__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedConfigInfo => {
                            if feed_config_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedConfigInfo"));
                            }
                            feed_config_info__ = map_.next_value()?;
                        }
                        GeneratedField::EpochAndRound => {
                            if epoch_and_round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochAndRound"));
                            }
                            epoch_and_round__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryFeedConfigInfoResponse {
                    feed_config_info: feed_config_info__,
                    epoch_and_round: epoch_and_round__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryFeedConfigInfoResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryFeedConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryFeedConfigRequest", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryFeedConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryFeedConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryFeedConfigRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryFeedConfigRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryFeedConfigRequest {
                    feed_id: feed_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryFeedConfigRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryFeedConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.feed_config_info.is_some() {
            len += 1;
        }
        if self.feed_config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryFeedConfigResponse", len)?;
        if let Some(v) = self.feed_config_info.as_ref() {
            struct_ser.serialize_field("feedConfigInfo", v)?;
        }
        if let Some(v) = self.feed_config.as_ref() {
            struct_ser.serialize_field("feedConfig", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryFeedConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_config_info",
            "feedConfigInfo",
            "feed_config",
            "feedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedConfigInfo,
            FeedConfig,
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
                            "feedConfigInfo" | "feed_config_info" => {
                                Ok(GeneratedField::FeedConfigInfo)
                            }
                            "feedConfig" | "feed_config" => Ok(GeneratedField::FeedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryFeedConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryFeedConfigResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryFeedConfigResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_config_info__ = None;
                let mut feed_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedConfigInfo => {
                            if feed_config_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedConfigInfo"));
                            }
                            feed_config_info__ = map_.next_value()?;
                        }
                        GeneratedField::FeedConfig => {
                            if feed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedConfig"));
                            }
                            feed_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryFeedConfigResponse {
                    feed_config_info: feed_config_info__,
                    feed_config: feed_config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryFeedConfigResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLatestRoundRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryLatestRoundRequest", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLatestRoundRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLatestRoundRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryLatestRoundRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLatestRoundRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLatestRoundRequest {
                    feed_id: feed_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryLatestRoundRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLatestRoundResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.latest_round_id != 0 {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryLatestRoundResponse", len)?;
        if self.latest_round_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "latestRoundId",
                ToString::to_string(&self.latest_round_id).as_str(),
            )?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLatestRoundResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["latest_round_id", "latestRoundId", "data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LatestRoundId,
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
                            "latestRoundId" | "latest_round_id" => {
                                Ok(GeneratedField::LatestRoundId)
                            }
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
            type Value = QueryLatestRoundResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryLatestRoundResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLatestRoundResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut latest_round_id__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LatestRoundId => {
                            if latest_round_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latestRoundId"));
                            }
                            latest_round_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLatestRoundResponse {
                    latest_round_id: latest_round_id__.unwrap_or_default(),
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryLatestRoundResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLatestTransmissionDetailsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.ocr.v1beta1.QueryLatestTransmissionDetailsRequest",
            len,
        )?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLatestTransmissionDetailsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLatestTransmissionDetailsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.ocr.v1beta1.QueryLatestTransmissionDetailsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLatestTransmissionDetailsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLatestTransmissionDetailsRequest {
                    feed_id: feed_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryLatestTransmissionDetailsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLatestTransmissionDetailsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.config_digest.is_empty() {
            len += 1;
        }
        if self.epoch_and_round.is_some() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.ocr.v1beta1.QueryLatestTransmissionDetailsResponse",
            len,
        )?;
        if !self.config_digest.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "configDigest",
                pbjson::private::base64::encode(&self.config_digest).as_str(),
            )?;
        }
        if let Some(v) = self.epoch_and_round.as_ref() {
            struct_ser.serialize_field("epochAndRound", v)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLatestTransmissionDetailsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_digest",
            "configDigest",
            "epoch_and_round",
            "epochAndRound",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigDigest,
            EpochAndRound,
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
                            "configDigest" | "config_digest" => Ok(GeneratedField::ConfigDigest),
                            "epochAndRound" | "epoch_and_round" => {
                                Ok(GeneratedField::EpochAndRound)
                            }
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
            type Value = QueryLatestTransmissionDetailsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.ocr.v1beta1.QueryLatestTransmissionDetailsResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLatestTransmissionDetailsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut config_digest__ = None;
                let mut epoch_and_round__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConfigDigest => {
                            if config_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDigest"));
                            }
                            config_digest__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EpochAndRound => {
                            if epoch_and_round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epochAndRound"));
                            }
                            epoch_and_round__ = map_.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLatestTransmissionDetailsResponse {
                    config_digest: config_digest__.unwrap_or_default(),
                    epoch_and_round: epoch_and_round__,
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryLatestTransmissionDetailsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryModuleStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryModuleStateRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryModuleStateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryModuleStateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryModuleStateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryModuleStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryModuleStateRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryModuleStateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryModuleStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryModuleStateResponse", len)?;
        if let Some(v) = self.state.as_ref() {
            struct_ser.serialize_field("state", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryModuleStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["state"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
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
                            "state" => Ok(GeneratedField::State),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryModuleStateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryModuleStateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryModuleStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryModuleStateResponse { state: state__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryModuleStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOwedAmountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transmitter.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryOwedAmountRequest", len)?;
        if !self.transmitter.is_empty() {
            struct_ser.serialize_field("transmitter", &self.transmitter)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOwedAmountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["transmitter"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transmitter,
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
                            "transmitter" => Ok(GeneratedField::Transmitter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOwedAmountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryOwedAmountRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOwedAmountRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut transmitter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transmitter => {
                            if transmitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitter"));
                            }
                            transmitter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOwedAmountRequest {
                    transmitter: transmitter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryOwedAmountRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOwedAmountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryOwedAmountResponse", len)?;
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOwedAmountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
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
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOwedAmountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryOwedAmountResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOwedAmountResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryOwedAmountResponse { amount: amount__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryOwedAmountResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
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
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Report {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.observations_timestamp != 0 {
            len += 1;
        }
        if !self.observers.is_empty() {
            len += 1;
        }
        if !self.observations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.ocr.v1beta1.Report", len)?;
        if self.observations_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "observationsTimestamp",
                ToString::to_string(&self.observations_timestamp).as_str(),
            )?;
        }
        if !self.observers.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "observers",
                pbjson::private::base64::encode(&self.observers).as_str(),
            )?;
        }
        if !self.observations.is_empty() {
            struct_ser.serialize_field("observations", &self.observations)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Report {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "observations_timestamp",
            "observationsTimestamp",
            "observers",
            "observations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObservationsTimestamp,
            Observers,
            Observations,
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
                            "observationsTimestamp" | "observations_timestamp" => {
                                Ok(GeneratedField::ObservationsTimestamp)
                            }
                            "observers" => Ok(GeneratedField::Observers),
                            "observations" => Ok(GeneratedField::Observations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Report;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.Report")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Report, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut observations_timestamp__ = None;
                let mut observers__ = None;
                let mut observations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ObservationsTimestamp => {
                            if observations_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "observationsTimestamp",
                                ));
                            }
                            observations_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Observers => {
                            if observers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("observers"));
                            }
                            observers__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Observations => {
                            if observations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("observations"));
                            }
                            observations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Report {
                    observations_timestamp: observations_timestamp__.unwrap_or_default(),
                    observers: observers__.unwrap_or_default(),
                    observations: observations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.ocr.v1beta1.Report", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ReportToSign {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.config_digest.is_empty() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        if self.round != 0 {
            len += 1;
        }
        if !self.extra_hash.is_empty() {
            len += 1;
        }
        if !self.report.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.ReportToSign", len)?;
        if !self.config_digest.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "configDigest",
                pbjson::private::base64::encode(&self.config_digest).as_str(),
            )?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if self.round != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("round", ToString::to_string(&self.round).as_str())?;
        }
        if !self.extra_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "extraHash",
                pbjson::private::base64::encode(&self.extra_hash).as_str(),
            )?;
        }
        if !self.report.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "report",
                pbjson::private::base64::encode(&self.report).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ReportToSign {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_digest",
            "configDigest",
            "epoch",
            "round",
            "extra_hash",
            "extraHash",
            "report",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigDigest,
            Epoch,
            Round,
            ExtraHash,
            Report,
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
                            "configDigest" | "config_digest" => Ok(GeneratedField::ConfigDigest),
                            "epoch" => Ok(GeneratedField::Epoch),
                            "round" => Ok(GeneratedField::Round),
                            "extraHash" | "extra_hash" => Ok(GeneratedField::ExtraHash),
                            "report" => Ok(GeneratedField::Report),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportToSign;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.ReportToSign")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportToSign, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut config_digest__ = None;
                let mut epoch__ = None;
                let mut round__ = None;
                let mut extra_hash__ = None;
                let mut report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConfigDigest => {
                            if config_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configDigest"));
                            }
                            config_digest__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Round => {
                            if round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("round"));
                            }
                            round__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ExtraHash => {
                            if extra_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extraHash"));
                            }
                            extra_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Report => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("report"));
                            }
                            report__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ReportToSign {
                    config_digest: config_digest__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                    round: round__.unwrap_or_default(),
                    extra_hash: extra_hash__.unwrap_or_default(),
                    report: report__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.ReportToSign",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RewardPool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feed_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.RewardPool", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RewardPool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            Amount,
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
                            "feedId" | "feed_id" => Ok(GeneratedField::FeedId),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RewardPool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.RewardPool")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RewardPool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RewardPool {
                    feed_id: feed_id__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.RewardPool",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetBatchConfigProposal {
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
        if !self.signers.is_empty() {
            len += 1;
        }
        if !self.transmitters.is_empty() {
            len += 1;
        }
        if !self.link_denom.is_empty() {
            len += 1;
        }
        if !self.feed_properties.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.SetBatchConfigProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.signers.is_empty() {
            struct_ser.serialize_field("signers", &self.signers)?;
        }
        if !self.transmitters.is_empty() {
            struct_ser.serialize_field("transmitters", &self.transmitters)?;
        }
        if !self.link_denom.is_empty() {
            struct_ser.serialize_field("linkDenom", &self.link_denom)?;
        }
        if !self.feed_properties.is_empty() {
            struct_ser.serialize_field("feedProperties", &self.feed_properties)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetBatchConfigProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "signers",
            "transmitters",
            "link_denom",
            "linkDenom",
            "feed_properties",
            "feedProperties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Signers,
            Transmitters,
            LinkDenom,
            FeedProperties,
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
                            "signers" => Ok(GeneratedField::Signers),
                            "transmitters" => Ok(GeneratedField::Transmitters),
                            "linkDenom" | "link_denom" => Ok(GeneratedField::LinkDenom),
                            "feedProperties" | "feed_properties" => {
                                Ok(GeneratedField::FeedProperties)
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
            type Value = SetBatchConfigProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.SetBatchConfigProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SetBatchConfigProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut signers__ = None;
                let mut transmitters__ = None;
                let mut link_denom__ = None;
                let mut feed_properties__ = None;
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
                        GeneratedField::Signers => {
                            if signers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signers"));
                            }
                            signers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transmitters => {
                            if transmitters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitters"));
                            }
                            transmitters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkDenom => {
                            if link_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkDenom"));
                            }
                            link_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeedProperties => {
                            if feed_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedProperties"));
                            }
                            feed_properties__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetBatchConfigProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    signers: signers__.unwrap_or_default(),
                    transmitters: transmitters__.unwrap_or_default(),
                    link_denom: link_denom__.unwrap_or_default(),
                    feed_properties: feed_properties__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.SetBatchConfigProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetConfigProposal {
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
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.SetConfigProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetConfigProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Config,
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
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetConfigProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.SetConfigProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetConfigProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut config__ = None;
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
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetConfigProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.SetConfigProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Transmission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.answer.is_empty() {
            len += 1;
        }
        if self.observations_timestamp != 0 {
            len += 1;
        }
        if self.transmission_timestamp != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.ocr.v1beta1.Transmission", len)?;
        if !self.answer.is_empty() {
            struct_ser.serialize_field("answer", &self.answer)?;
        }
        if self.observations_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "observationsTimestamp",
                ToString::to_string(&self.observations_timestamp).as_str(),
            )?;
        }
        if self.transmission_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "transmissionTimestamp",
                ToString::to_string(&self.transmission_timestamp).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Transmission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "answer",
            "observations_timestamp",
            "observationsTimestamp",
            "transmission_timestamp",
            "transmissionTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Answer,
            ObservationsTimestamp,
            TransmissionTimestamp,
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
                            "answer" => Ok(GeneratedField::Answer),
                            "observationsTimestamp" | "observations_timestamp" => {
                                Ok(GeneratedField::ObservationsTimestamp)
                            }
                            "transmissionTimestamp" | "transmission_timestamp" => {
                                Ok(GeneratedField::TransmissionTimestamp)
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
            type Value = Transmission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.ocr.v1beta1.Transmission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Transmission, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut answer__ = None;
                let mut observations_timestamp__ = None;
                let mut transmission_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Answer => {
                            if answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answer"));
                            }
                            answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ObservationsTimestamp => {
                            if observations_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "observationsTimestamp",
                                ));
                            }
                            observations_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TransmissionTimestamp => {
                            if transmission_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "transmissionTimestamp",
                                ));
                            }
                            transmission_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Transmission {
                    answer: answer__.unwrap_or_default(),
                    observations_timestamp: observations_timestamp__.unwrap_or_default(),
                    transmission_timestamp: transmission_timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.ocr.v1beta1.Transmission",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
