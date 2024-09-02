// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for AssetPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.signed_prices.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.AssetPair", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.signed_prices.is_empty() {
            struct_ser.serialize_field("signedPrices", &self.signed_prices)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AssetPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["asset_id", "assetId", "signed_prices", "signedPrices"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            SignedPrices,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "signedPrices" | "signed_prices" => Ok(GeneratedField::SignedPrices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.AssetPair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssetPair, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut signed_prices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SignedPrices => {
                            if signed_prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signedPrices"));
                            }
                            signed_prices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AssetPair {
                    asset_id: asset_id__.unwrap_or_default(),
                    signed_prices: signed_prices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.AssetPair",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AuthorizeBandOracleRequestProposal {
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
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.AuthorizeBandOracleRequestProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AuthorizeBandOracleRequestProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "request"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Request,
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
                            "request" => Ok(GeneratedField::Request),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizeBandOracleRequestProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.AuthorizeBandOracleRequestProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<AuthorizeBandOracleRequestProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut request__ = None;
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
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AuthorizeBandOracleRequestProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.AuthorizeBandOracleRequestProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BandIbcParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.band_ibc_enabled {
            len += 1;
        }
        if self.ibc_request_interval != 0 {
            len += 1;
        }
        if !self.ibc_source_channel.is_empty() {
            len += 1;
        }
        if !self.ibc_version.is_empty() {
            len += 1;
        }
        if !self.ibc_port_id.is_empty() {
            len += 1;
        }
        if !self.legacy_oracle_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.BandIBCParams", len)?;
        if self.band_ibc_enabled {
            struct_ser.serialize_field("bandIbcEnabled", &self.band_ibc_enabled)?;
        }
        if self.ibc_request_interval != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "ibcRequestInterval",
                ToString::to_string(&self.ibc_request_interval).as_str(),
            )?;
        }
        if !self.ibc_source_channel.is_empty() {
            struct_ser.serialize_field("ibcSourceChannel", &self.ibc_source_channel)?;
        }
        if !self.ibc_version.is_empty() {
            struct_ser.serialize_field("ibcVersion", &self.ibc_version)?;
        }
        if !self.ibc_port_id.is_empty() {
            struct_ser.serialize_field("ibcPortId", &self.ibc_port_id)?;
        }
        if !self.legacy_oracle_ids.is_empty() {
            struct_ser.serialize_field(
                "legacyOracleIds",
                &self
                    .legacy_oracle_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BandIbcParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "band_ibc_enabled",
            "bandIbcEnabled",
            "ibc_request_interval",
            "ibcRequestInterval",
            "ibc_source_channel",
            "ibcSourceChannel",
            "ibc_version",
            "ibcVersion",
            "ibc_port_id",
            "ibcPortId",
            "legacy_oracle_ids",
            "legacyOracleIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BandIbcEnabled,
            IbcRequestInterval,
            IbcSourceChannel,
            IbcVersion,
            IbcPortId,
            LegacyOracleIds,
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
                            "bandIbcEnabled" | "band_ibc_enabled" => {
                                Ok(GeneratedField::BandIbcEnabled)
                            }
                            "ibcRequestInterval" | "ibc_request_interval" => {
                                Ok(GeneratedField::IbcRequestInterval)
                            }
                            "ibcSourceChannel" | "ibc_source_channel" => {
                                Ok(GeneratedField::IbcSourceChannel)
                            }
                            "ibcVersion" | "ibc_version" => Ok(GeneratedField::IbcVersion),
                            "ibcPortId" | "ibc_port_id" => Ok(GeneratedField::IbcPortId),
                            "legacyOracleIds" | "legacy_oracle_ids" => {
                                Ok(GeneratedField::LegacyOracleIds)
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
            type Value = BandIbcParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.BandIBCParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BandIbcParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut band_ibc_enabled__ = None;
                let mut ibc_request_interval__ = None;
                let mut ibc_source_channel__ = None;
                let mut ibc_version__ = None;
                let mut ibc_port_id__ = None;
                let mut legacy_oracle_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BandIbcEnabled => {
                            if band_ibc_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandIbcEnabled"));
                            }
                            band_ibc_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IbcRequestInterval => {
                            if ibc_request_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ibcRequestInterval",
                                ));
                            }
                            ibc_request_interval__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::IbcSourceChannel => {
                            if ibc_source_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcSourceChannel"));
                            }
                            ibc_source_channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IbcVersion => {
                            if ibc_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcVersion"));
                            }
                            ibc_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IbcPortId => {
                            if ibc_port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibcPortId"));
                            }
                            ibc_port_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LegacyOracleIds => {
                            if legacy_oracle_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("legacyOracleIds"));
                            }
                            legacy_oracle_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(BandIbcParams {
                    band_ibc_enabled: band_ibc_enabled__.unwrap_or_default(),
                    ibc_request_interval: ibc_request_interval__.unwrap_or_default(),
                    ibc_source_channel: ibc_source_channel__.unwrap_or_default(),
                    ibc_version: ibc_version__.unwrap_or_default(),
                    ibc_port_id: ibc_port_id__.unwrap_or_default(),
                    legacy_oracle_ids: legacy_oracle_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.BandIBCParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BandOracleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_id != 0 {
            len += 1;
        }
        if self.oracle_script_id != 0 {
            len += 1;
        }
        if !self.symbols.is_empty() {
            len += 1;
        }
        if self.ask_count != 0 {
            len += 1;
        }
        if self.min_count != 0 {
            len += 1;
        }
        if !self.fee_limit.is_empty() {
            len += 1;
        }
        if self.prepare_gas != 0 {
            len += 1;
        }
        if self.execute_gas != 0 {
            len += 1;
        }
        if self.min_source_count != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.BandOracleRequest", len)?;
        if self.request_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("requestId", ToString::to_string(&self.request_id).as_str())?;
        }
        if self.oracle_script_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "oracleScriptId",
                ToString::to_string(&self.oracle_script_id).as_str(),
            )?;
        }
        if !self.symbols.is_empty() {
            struct_ser.serialize_field("symbols", &self.symbols)?;
        }
        if self.ask_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("askCount", ToString::to_string(&self.ask_count).as_str())?;
        }
        if self.min_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("minCount", ToString::to_string(&self.min_count).as_str())?;
        }
        if !self.fee_limit.is_empty() {
            struct_ser.serialize_field("feeLimit", &self.fee_limit)?;
        }
        if self.prepare_gas != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "prepareGas",
                ToString::to_string(&self.prepare_gas).as_str(),
            )?;
        }
        if self.execute_gas != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "executeGas",
                ToString::to_string(&self.execute_gas).as_str(),
            )?;
        }
        if self.min_source_count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "minSourceCount",
                ToString::to_string(&self.min_source_count).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BandOracleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
            "oracle_script_id",
            "oracleScriptId",
            "symbols",
            "ask_count",
            "askCount",
            "min_count",
            "minCount",
            "fee_limit",
            "feeLimit",
            "prepare_gas",
            "prepareGas",
            "execute_gas",
            "executeGas",
            "min_source_count",
            "minSourceCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            OracleScriptId,
            Symbols,
            AskCount,
            MinCount,
            FeeLimit,
            PrepareGas,
            ExecuteGas,
            MinSourceCount,
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
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "oracleScriptId" | "oracle_script_id" => {
                                Ok(GeneratedField::OracleScriptId)
                            }
                            "symbols" => Ok(GeneratedField::Symbols),
                            "askCount" | "ask_count" => Ok(GeneratedField::AskCount),
                            "minCount" | "min_count" => Ok(GeneratedField::MinCount),
                            "feeLimit" | "fee_limit" => Ok(GeneratedField::FeeLimit),
                            "prepareGas" | "prepare_gas" => Ok(GeneratedField::PrepareGas),
                            "executeGas" | "execute_gas" => Ok(GeneratedField::ExecuteGas),
                            "minSourceCount" | "min_source_count" => {
                                Ok(GeneratedField::MinSourceCount)
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
            type Value = BandOracleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.BandOracleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BandOracleRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                let mut oracle_script_id__ = None;
                let mut symbols__ = None;
                let mut ask_count__ = None;
                let mut min_count__ = None;
                let mut fee_limit__ = None;
                let mut prepare_gas__ = None;
                let mut execute_gas__ = None;
                let mut min_source_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OracleScriptId => {
                            if oracle_script_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleScriptId"));
                            }
                            oracle_script_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Symbols => {
                            if symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbols"));
                            }
                            symbols__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AskCount => {
                            if ask_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("askCount"));
                            }
                            ask_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinCount => {
                            if min_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minCount"));
                            }
                            min_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FeeLimit => {
                            if fee_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeLimit"));
                            }
                            fee_limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrepareGas => {
                            if prepare_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prepareGas"));
                            }
                            prepare_gas__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ExecuteGas => {
                            if execute_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executeGas"));
                            }
                            execute_gas__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinSourceCount => {
                            if min_source_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minSourceCount"));
                            }
                            min_source_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(BandOracleRequest {
                    request_id: request_id__.unwrap_or_default(),
                    oracle_script_id: oracle_script_id__.unwrap_or_default(),
                    symbols: symbols__.unwrap_or_default(),
                    ask_count: ask_count__.unwrap_or_default(),
                    min_count: min_count__.unwrap_or_default(),
                    fee_limit: fee_limit__.unwrap_or_default(),
                    prepare_gas: prepare_gas__.unwrap_or_default(),
                    execute_gas: execute_gas__.unwrap_or_default(),
                    min_source_count: min_source_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.BandOracleRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BandPriceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symbol.is_empty() {
            len += 1;
        }
        if !self.rate.is_empty() {
            len += 1;
        }
        if self.resolve_time != 0 {
            len += 1;
        }
        if self.request_id != 0 {
            len += 1;
        }
        if self.price_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.BandPriceState", len)?;
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if !self.rate.is_empty() {
            struct_ser.serialize_field("rate", &self.rate)?;
        }
        if self.resolve_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "resolveTime",
                ToString::to_string(&self.resolve_time).as_str(),
            )?;
        }
        if self.request_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("requestID", ToString::to_string(&self.request_id).as_str())?;
        }
        if let Some(v) = self.price_state.as_ref() {
            struct_ser.serialize_field("priceState", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BandPriceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "symbol",
            "rate",
            "resolve_time",
            "resolveTime",
            "request_ID",
            "requestID",
            "price_state",
            "priceState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Symbol,
            Rate,
            ResolveTime,
            RequestId,
            PriceState,
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
                            "symbol" => Ok(GeneratedField::Symbol),
                            "rate" => Ok(GeneratedField::Rate),
                            "resolveTime" | "resolve_time" => Ok(GeneratedField::ResolveTime),
                            "requestID" | "request_ID" => Ok(GeneratedField::RequestId),
                            "priceState" | "price_state" => Ok(GeneratedField::PriceState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BandPriceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.BandPriceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BandPriceState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut symbol__ = None;
                let mut rate__ = None;
                let mut resolve_time__ = None;
                let mut request_id__ = None;
                let mut price_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rate => {
                            if rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rate"));
                            }
                            rate__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResolveTime => {
                            if resolve_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolveTime"));
                            }
                            resolve_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestID"));
                            }
                            request_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PriceState => {
                            if price_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceState"));
                            }
                            price_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BandPriceState {
                    symbol: symbol__.unwrap_or_default(),
                    rate: rate__.unwrap_or_default(),
                    resolve_time: resolve_time__.unwrap_or_default(),
                    request_id: request_id__.unwrap_or_default(),
                    price_state: price_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.BandPriceState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CalldataRecord {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.client_id != 0 {
            len += 1;
        }
        if !self.calldata.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.CalldataRecord", len)?;
        if self.client_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("clientId", ToString::to_string(&self.client_id).as_str())?;
        }
        if !self.calldata.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "calldata",
                pbjson::private::base64::encode(&self.calldata).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CalldataRecord {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["client_id", "clientId", "calldata"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Calldata,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "calldata" => Ok(GeneratedField::Calldata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalldataRecord;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.CalldataRecord")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalldataRecord, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut calldata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Calldata => {
                            if calldata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calldata"));
                            }
                            calldata__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(CalldataRecord {
                    client_id: client_id__.unwrap_or_default(),
                    calldata: calldata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.CalldataRecord",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ChainlinkPriceState {
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
        if !self.answer.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        if self.price_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.ChainlinkPriceState", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if !self.answer.is_empty() {
            struct_ser.serialize_field("answer", &self.answer)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if let Some(v) = self.price_state.as_ref() {
            struct_ser.serialize_field("priceState", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ChainlinkPriceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feed_id",
            "feedId",
            "answer",
            "timestamp",
            "price_state",
            "priceState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            Answer,
            Timestamp,
            PriceState,
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
                            "answer" => Ok(GeneratedField::Answer),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "priceState" | "price_state" => Ok(GeneratedField::PriceState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainlinkPriceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.ChainlinkPriceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChainlinkPriceState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut answer__ = None;
                let mut timestamp__ = None;
                let mut price_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Answer => {
                            if answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answer"));
                            }
                            answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PriceState => {
                            if price_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceState"));
                            }
                            price_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ChainlinkPriceState {
                    feed_id: feed_id__.unwrap_or_default(),
                    answer: answer__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                    price_state: price_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.ChainlinkPriceState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CoinbasePriceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if self.value != 0 {
            len += 1;
        }
        if self.price_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.CoinbasePriceState", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.value != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        if let Some(v) = self.price_state.as_ref() {
            struct_ser.serialize_field("priceState", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CoinbasePriceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "timestamp",
            "key",
            "value",
            "price_state",
            "priceState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Timestamp,
            Key,
            Value,
            PriceState,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            "priceState" | "price_state" => Ok(GeneratedField::PriceState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CoinbasePriceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.CoinbasePriceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CoinbasePriceState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut timestamp__ = None;
                let mut key__ = None;
                let mut value__ = None;
                let mut price_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PriceState => {
                            if price_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceState"));
                            }
                            price_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CoinbasePriceState {
                    kind: kind__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    price_state: price_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.CoinbasePriceState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EnableBandIbcProposal {
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
        if self.band_ibc_params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.EnableBandIBCProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.band_ibc_params.as_ref() {
            struct_ser.serialize_field("bandIbcParams", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EnableBandIbcProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "band_ibc_params", "bandIbcParams"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            BandIbcParams,
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
                            "bandIbcParams" | "band_ibc_params" => {
                                Ok(GeneratedField::BandIbcParams)
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
            type Value = EnableBandIbcProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.EnableBandIBCProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EnableBandIbcProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut band_ibc_params__ = None;
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
                        GeneratedField::BandIbcParams => {
                            if band_ibc_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandIbcParams"));
                            }
                            band_ibc_params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EnableBandIbcProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    band_ibc_params: band_ibc_params__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.EnableBandIBCProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventBandIbcAckError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ack_error.is_empty() {
            len += 1;
        }
        if self.client_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.EventBandIBCAckError", len)?;
        if !self.ack_error.is_empty() {
            struct_ser.serialize_field("ackError", &self.ack_error)?;
        }
        if self.client_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("clientId", ToString::to_string(&self.client_id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventBandIbcAckError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ack_error", "ackError", "client_id", "clientId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AckError,
            ClientId,
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
                            "ackError" | "ack_error" => Ok(GeneratedField::AckError),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventBandIbcAckError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.EventBandIBCAckError")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventBandIbcAckError, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ack_error__ = None;
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AckError => {
                            if ack_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ackError"));
                            }
                            ack_error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EventBandIbcAckError {
                    ack_error: ack_error__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.EventBandIBCAckError",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventBandIbcAckSuccess {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ack_result.is_empty() {
            len += 1;
        }
        if self.client_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.EventBandIBCAckSuccess", len)?;
        if !self.ack_result.is_empty() {
            struct_ser.serialize_field("ackResult", &self.ack_result)?;
        }
        if self.client_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("clientId", ToString::to_string(&self.client_id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventBandIbcAckSuccess {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ack_result", "ackResult", "client_id", "clientId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AckResult,
            ClientId,
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
                            "ackResult" | "ack_result" => Ok(GeneratedField::AckResult),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventBandIbcAckSuccess;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.EventBandIBCAckSuccess")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventBandIbcAckSuccess, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ack_result__ = None;
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AckResult => {
                            if ack_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ackResult"));
                            }
                            ack_result__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EventBandIbcAckSuccess {
                    ack_result: ack_result__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.EventBandIBCAckSuccess",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventBandIbcResponseTimeout {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.client_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.EventBandIBCResponseTimeout", len)?;
        if self.client_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("clientId", ToString::to_string(&self.client_id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventBandIbcResponseTimeout {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["client_id", "clientId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventBandIbcResponseTimeout;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.EventBandIBCResponseTimeout")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventBandIbcResponseTimeout, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EventBandIbcResponseTimeout {
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.EventBandIBCResponseTimeout",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventSetPythPrices {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prices.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.EventSetPythPrices", len)?;
        if !self.prices.is_empty() {
            struct_ser.serialize_field("prices", &self.prices)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventSetPythPrices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["prices"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prices,
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
                            "prices" => Ok(GeneratedField::Prices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventSetPythPrices;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.EventSetPythPrices")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventSetPythPrices, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut prices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Prices => {
                            if prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prices"));
                            }
                            prices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventSetPythPrices {
                    prices: prices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.EventSetPythPrices",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventSetStorkPrices {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prices.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.EventSetStorkPrices", len)?;
        if !self.prices.is_empty() {
            struct_ser.serialize_field("prices", &self.prices)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventSetStorkPrices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["prices"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prices,
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
                            "prices" => Ok(GeneratedField::Prices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventSetStorkPrices;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.EventSetStorkPrices")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventSetStorkPrices, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut prices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Prices => {
                            if prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prices"));
                            }
                            prices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventSetStorkPrices {
                    prices: prices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.EventSetStorkPrices",
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
        if !self.band_relayers.is_empty() {
            len += 1;
        }
        if !self.band_price_states.is_empty() {
            len += 1;
        }
        if !self.price_feed_price_states.is_empty() {
            len += 1;
        }
        if !self.coinbase_price_states.is_empty() {
            len += 1;
        }
        if !self.band_ibc_price_states.is_empty() {
            len += 1;
        }
        if !self.band_ibc_oracle_requests.is_empty() {
            len += 1;
        }
        if self.band_ibc_params.is_some() {
            len += 1;
        }
        if self.band_ibc_latest_client_id != 0 {
            len += 1;
        }
        if !self.calldata_records.is_empty() {
            len += 1;
        }
        if self.band_ibc_latest_request_id != 0 {
            len += 1;
        }
        if !self.chainlink_price_states.is_empty() {
            len += 1;
        }
        if !self.historical_price_records.is_empty() {
            len += 1;
        }
        if !self.provider_states.is_empty() {
            len += 1;
        }
        if !self.pyth_price_states.is_empty() {
            len += 1;
        }
        if !self.stork_price_states.is_empty() {
            len += 1;
        }
        if !self.stork_publishers.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.band_relayers.is_empty() {
            struct_ser.serialize_field("bandRelayers", &self.band_relayers)?;
        }
        if !self.band_price_states.is_empty() {
            struct_ser.serialize_field("bandPriceStates", &self.band_price_states)?;
        }
        if !self.price_feed_price_states.is_empty() {
            struct_ser.serialize_field("priceFeedPriceStates", &self.price_feed_price_states)?;
        }
        if !self.coinbase_price_states.is_empty() {
            struct_ser.serialize_field("coinbasePriceStates", &self.coinbase_price_states)?;
        }
        if !self.band_ibc_price_states.is_empty() {
            struct_ser.serialize_field("bandIbcPriceStates", &self.band_ibc_price_states)?;
        }
        if !self.band_ibc_oracle_requests.is_empty() {
            struct_ser.serialize_field("bandIbcOracleRequests", &self.band_ibc_oracle_requests)?;
        }
        if let Some(v) = self.band_ibc_params.as_ref() {
            struct_ser.serialize_field("bandIbcParams", v)?;
        }
        if self.band_ibc_latest_client_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "bandIbcLatestClientId",
                ToString::to_string(&self.band_ibc_latest_client_id).as_str(),
            )?;
        }
        if !self.calldata_records.is_empty() {
            struct_ser.serialize_field("calldataRecords", &self.calldata_records)?;
        }
        if self.band_ibc_latest_request_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "bandIbcLatestRequestId",
                ToString::to_string(&self.band_ibc_latest_request_id).as_str(),
            )?;
        }
        if !self.chainlink_price_states.is_empty() {
            struct_ser.serialize_field("chainlinkPriceStates", &self.chainlink_price_states)?;
        }
        if !self.historical_price_records.is_empty() {
            struct_ser.serialize_field("historicalPriceRecords", &self.historical_price_records)?;
        }
        if !self.provider_states.is_empty() {
            struct_ser.serialize_field("providerStates", &self.provider_states)?;
        }
        if !self.pyth_price_states.is_empty() {
            struct_ser.serialize_field("pythPriceStates", &self.pyth_price_states)?;
        }
        if !self.stork_price_states.is_empty() {
            struct_ser.serialize_field("storkPriceStates", &self.stork_price_states)?;
        }
        if !self.stork_publishers.is_empty() {
            struct_ser.serialize_field("storkPublishers", &self.stork_publishers)?;
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
            "band_relayers",
            "bandRelayers",
            "band_price_states",
            "bandPriceStates",
            "price_feed_price_states",
            "priceFeedPriceStates",
            "coinbase_price_states",
            "coinbasePriceStates",
            "band_ibc_price_states",
            "bandIbcPriceStates",
            "band_ibc_oracle_requests",
            "bandIbcOracleRequests",
            "band_ibc_params",
            "bandIbcParams",
            "band_ibc_latest_client_id",
            "bandIbcLatestClientId",
            "calldata_records",
            "calldataRecords",
            "band_ibc_latest_request_id",
            "bandIbcLatestRequestId",
            "chainlink_price_states",
            "chainlinkPriceStates",
            "historical_price_records",
            "historicalPriceRecords",
            "provider_states",
            "providerStates",
            "pyth_price_states",
            "pythPriceStates",
            "stork_price_states",
            "storkPriceStates",
            "stork_publishers",
            "storkPublishers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            BandRelayers,
            BandPriceStates,
            PriceFeedPriceStates,
            CoinbasePriceStates,
            BandIbcPriceStates,
            BandIbcOracleRequests,
            BandIbcParams,
            BandIbcLatestClientId,
            CalldataRecords,
            BandIbcLatestRequestId,
            ChainlinkPriceStates,
            HistoricalPriceRecords,
            ProviderStates,
            PythPriceStates,
            StorkPriceStates,
            StorkPublishers,
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
                            "bandRelayers" | "band_relayers" => Ok(GeneratedField::BandRelayers),
                            "bandPriceStates" | "band_price_states" => {
                                Ok(GeneratedField::BandPriceStates)
                            }
                            "priceFeedPriceStates" | "price_feed_price_states" => {
                                Ok(GeneratedField::PriceFeedPriceStates)
                            }
                            "coinbasePriceStates" | "coinbase_price_states" => {
                                Ok(GeneratedField::CoinbasePriceStates)
                            }
                            "bandIbcPriceStates" | "band_ibc_price_states" => {
                                Ok(GeneratedField::BandIbcPriceStates)
                            }
                            "bandIbcOracleRequests" | "band_ibc_oracle_requests" => {
                                Ok(GeneratedField::BandIbcOracleRequests)
                            }
                            "bandIbcParams" | "band_ibc_params" => {
                                Ok(GeneratedField::BandIbcParams)
                            }
                            "bandIbcLatestClientId" | "band_ibc_latest_client_id" => {
                                Ok(GeneratedField::BandIbcLatestClientId)
                            }
                            "calldataRecords" | "calldata_records" => {
                                Ok(GeneratedField::CalldataRecords)
                            }
                            "bandIbcLatestRequestId" | "band_ibc_latest_request_id" => {
                                Ok(GeneratedField::BandIbcLatestRequestId)
                            }
                            "chainlinkPriceStates" | "chainlink_price_states" => {
                                Ok(GeneratedField::ChainlinkPriceStates)
                            }
                            "historicalPriceRecords" | "historical_price_records" => {
                                Ok(GeneratedField::HistoricalPriceRecords)
                            }
                            "providerStates" | "provider_states" => {
                                Ok(GeneratedField::ProviderStates)
                            }
                            "pythPriceStates" | "pyth_price_states" => {
                                Ok(GeneratedField::PythPriceStates)
                            }
                            "storkPriceStates" | "stork_price_states" => {
                                Ok(GeneratedField::StorkPriceStates)
                            }
                            "storkPublishers" | "stork_publishers" => {
                                Ok(GeneratedField::StorkPublishers)
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
                formatter.write_str("struct injective.oracle.v1beta1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut band_relayers__ = None;
                let mut band_price_states__ = None;
                let mut price_feed_price_states__ = None;
                let mut coinbase_price_states__ = None;
                let mut band_ibc_price_states__ = None;
                let mut band_ibc_oracle_requests__ = None;
                let mut band_ibc_params__ = None;
                let mut band_ibc_latest_client_id__ = None;
                let mut calldata_records__ = None;
                let mut band_ibc_latest_request_id__ = None;
                let mut chainlink_price_states__ = None;
                let mut historical_price_records__ = None;
                let mut provider_states__ = None;
                let mut pyth_price_states__ = None;
                let mut stork_price_states__ = None;
                let mut stork_publishers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::BandRelayers => {
                            if band_relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandRelayers"));
                            }
                            band_relayers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BandPriceStates => {
                            if band_price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandPriceStates"));
                            }
                            band_price_states__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PriceFeedPriceStates => {
                            if price_feed_price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "priceFeedPriceStates",
                                ));
                            }
                            price_feed_price_states__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CoinbasePriceStates => {
                            if coinbase_price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "coinbasePriceStates",
                                ));
                            }
                            coinbase_price_states__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BandIbcPriceStates => {
                            if band_ibc_price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bandIbcPriceStates",
                                ));
                            }
                            band_ibc_price_states__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BandIbcOracleRequests => {
                            if band_ibc_oracle_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bandIbcOracleRequests",
                                ));
                            }
                            band_ibc_oracle_requests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BandIbcParams => {
                            if band_ibc_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandIbcParams"));
                            }
                            band_ibc_params__ = map_.next_value()?;
                        }
                        GeneratedField::BandIbcLatestClientId => {
                            if band_ibc_latest_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bandIbcLatestClientId",
                                ));
                            }
                            band_ibc_latest_client_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CalldataRecords => {
                            if calldata_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calldataRecords"));
                            }
                            calldata_records__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BandIbcLatestRequestId => {
                            if band_ibc_latest_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bandIbcLatestRequestId",
                                ));
                            }
                            band_ibc_latest_request_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ChainlinkPriceStates => {
                            if chainlink_price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "chainlinkPriceStates",
                                ));
                            }
                            chainlink_price_states__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HistoricalPriceRecords => {
                            if historical_price_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "historicalPriceRecords",
                                ));
                            }
                            historical_price_records__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProviderStates => {
                            if provider_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerStates"));
                            }
                            provider_states__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PythPriceStates => {
                            if pyth_price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pythPriceStates"));
                            }
                            pyth_price_states__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorkPriceStates => {
                            if stork_price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storkPriceStates"));
                            }
                            stork_price_states__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorkPublishers => {
                            if stork_publishers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storkPublishers"));
                            }
                            stork_publishers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    band_relayers: band_relayers__.unwrap_or_default(),
                    band_price_states: band_price_states__.unwrap_or_default(),
                    price_feed_price_states: price_feed_price_states__.unwrap_or_default(),
                    coinbase_price_states: coinbase_price_states__.unwrap_or_default(),
                    band_ibc_price_states: band_ibc_price_states__.unwrap_or_default(),
                    band_ibc_oracle_requests: band_ibc_oracle_requests__.unwrap_or_default(),
                    band_ibc_params: band_ibc_params__,
                    band_ibc_latest_client_id: band_ibc_latest_client_id__.unwrap_or_default(),
                    calldata_records: calldata_records__.unwrap_or_default(),
                    band_ibc_latest_request_id: band_ibc_latest_request_id__.unwrap_or_default(),
                    chainlink_price_states: chainlink_price_states__.unwrap_or_default(),
                    historical_price_records: historical_price_records__.unwrap_or_default(),
                    provider_states: provider_states__.unwrap_or_default(),
                    pyth_price_states: pyth_price_states__.unwrap_or_default(),
                    stork_price_states: stork_price_states__.unwrap_or_default(),
                    stork_publishers: stork_publishers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GrantBandOraclePrivilegeProposal {
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
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.GrantBandOraclePrivilegeProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GrantBandOraclePrivilegeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Relayers,
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
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrantBandOraclePrivilegeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.GrantBandOraclePrivilegeProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<GrantBandOraclePrivilegeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut relayers__ = None;
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
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GrantBandOraclePrivilegeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.GrantBandOraclePrivilegeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GrantPriceFeederPrivilegeProposal {
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
        if !self.base.is_empty() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.GrantPriceFeederPrivilegeProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.base.is_empty() {
            struct_ser.serialize_field("base", &self.base)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GrantPriceFeederPrivilegeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "base", "quote", "relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Base,
            Quote,
            Relayers,
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
                            "base" => Ok(GeneratedField::Base),
                            "quote" => Ok(GeneratedField::Quote),
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrantPriceFeederPrivilegeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.GrantPriceFeederPrivilegeProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<GrantPriceFeederPrivilegeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut base__ = None;
                let mut quote__ = None;
                let mut relayers__ = None;
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
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GrantPriceFeederPrivilegeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    base: base__.unwrap_or_default(),
                    quote: quote__.unwrap_or_default(),
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.GrantPriceFeederPrivilegeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GrantProviderPrivilegeProposal {
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
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.GrantProviderPrivilegeProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GrantProviderPrivilegeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "provider", "relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Provider,
            Relayers,
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
                            "provider" => Ok(GeneratedField::Provider),
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrantProviderPrivilegeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.GrantProviderPrivilegeProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<GrantProviderPrivilegeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut provider__ = None;
                let mut relayers__ = None;
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
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GrantProviderPrivilegeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    provider: provider__.unwrap_or_default(),
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.GrantProviderPrivilegeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GrantStorkPublisherPrivilegeProposal {
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
        if !self.stork_publishers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.GrantStorkPublisherPrivilegeProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.stork_publishers.is_empty() {
            struct_ser.serialize_field("storkPublishers", &self.stork_publishers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GrantStorkPublisherPrivilegeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "stork_publishers",
            "storkPublishers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            StorkPublishers,
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
                            "storkPublishers" | "stork_publishers" => {
                                Ok(GeneratedField::StorkPublishers)
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
            type Value = GrantStorkPublisherPrivilegeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.oracle.v1beta1.GrantStorkPublisherPrivilegeProposal",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<GrantStorkPublisherPrivilegeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut stork_publishers__ = None;
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
                        GeneratedField::StorkPublishers => {
                            if stork_publishers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storkPublishers"));
                            }
                            stork_publishers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GrantStorkPublisherPrivilegeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    stork_publishers: stork_publishers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.GrantStorkPublisherPrivilegeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LastPriceTimestamps {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.last_price_timestamps.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.LastPriceTimestamps", len)?;
        if !self.last_price_timestamps.is_empty() {
            struct_ser.serialize_field("lastPriceTimestamps", &self.last_price_timestamps)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LastPriceTimestamps {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["last_price_timestamps", "lastPriceTimestamps"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LastPriceTimestamps,
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
                            "lastPriceTimestamps" | "last_price_timestamps" => {
                                Ok(GeneratedField::LastPriceTimestamps)
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
            type Value = LastPriceTimestamps;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.LastPriceTimestamps")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LastPriceTimestamps, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut last_price_timestamps__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LastPriceTimestamps => {
                            if last_price_timestamps__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "lastPriceTimestamps",
                                ));
                            }
                            last_price_timestamps__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LastPriceTimestamps {
                    last_price_timestamps: last_price_timestamps__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.LastPriceTimestamps",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MetadataStatistics {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group_count != 0 {
            len += 1;
        }
        if self.records_sample_size != 0 {
            len += 1;
        }
        if !self.mean.is_empty() {
            len += 1;
        }
        if !self.twap.is_empty() {
            len += 1;
        }
        if self.first_timestamp != 0 {
            len += 1;
        }
        if self.last_timestamp != 0 {
            len += 1;
        }
        if !self.min_price.is_empty() {
            len += 1;
        }
        if !self.max_price.is_empty() {
            len += 1;
        }
        if !self.median_price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.MetadataStatistics", len)?;
        if self.group_count != 0 {
            struct_ser.serialize_field("groupCount", &self.group_count)?;
        }
        if self.records_sample_size != 0 {
            struct_ser.serialize_field("recordsSampleSize", &self.records_sample_size)?;
        }
        if !self.mean.is_empty() {
            struct_ser.serialize_field("mean", &self.mean)?;
        }
        if !self.twap.is_empty() {
            struct_ser.serialize_field("twap", &self.twap)?;
        }
        if self.first_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "firstTimestamp",
                ToString::to_string(&self.first_timestamp).as_str(),
            )?;
        }
        if self.last_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "lastTimestamp",
                ToString::to_string(&self.last_timestamp).as_str(),
            )?;
        }
        if !self.min_price.is_empty() {
            struct_ser.serialize_field("minPrice", &self.min_price)?;
        }
        if !self.max_price.is_empty() {
            struct_ser.serialize_field("maxPrice", &self.max_price)?;
        }
        if !self.median_price.is_empty() {
            struct_ser.serialize_field("medianPrice", &self.median_price)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MetadataStatistics {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group_count",
            "groupCount",
            "records_sample_size",
            "recordsSampleSize",
            "mean",
            "twap",
            "first_timestamp",
            "firstTimestamp",
            "last_timestamp",
            "lastTimestamp",
            "min_price",
            "minPrice",
            "max_price",
            "maxPrice",
            "median_price",
            "medianPrice",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GroupCount,
            RecordsSampleSize,
            Mean,
            Twap,
            FirstTimestamp,
            LastTimestamp,
            MinPrice,
            MaxPrice,
            MedianPrice,
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
                            "groupCount" | "group_count" => Ok(GeneratedField::GroupCount),
                            "recordsSampleSize" | "records_sample_size" => {
                                Ok(GeneratedField::RecordsSampleSize)
                            }
                            "mean" => Ok(GeneratedField::Mean),
                            "twap" => Ok(GeneratedField::Twap),
                            "firstTimestamp" | "first_timestamp" => {
                                Ok(GeneratedField::FirstTimestamp)
                            }
                            "lastTimestamp" | "last_timestamp" => Ok(GeneratedField::LastTimestamp),
                            "minPrice" | "min_price" => Ok(GeneratedField::MinPrice),
                            "maxPrice" | "max_price" => Ok(GeneratedField::MaxPrice),
                            "medianPrice" | "median_price" => Ok(GeneratedField::MedianPrice),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetadataStatistics;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MetadataStatistics")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetadataStatistics, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut group_count__ = None;
                let mut records_sample_size__ = None;
                let mut mean__ = None;
                let mut twap__ = None;
                let mut first_timestamp__ = None;
                let mut last_timestamp__ = None;
                let mut min_price__ = None;
                let mut max_price__ = None;
                let mut median_price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GroupCount => {
                            if group_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupCount"));
                            }
                            group_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RecordsSampleSize => {
                            if records_sample_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordsSampleSize"));
                            }
                            records_sample_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Mean => {
                            if mean__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mean"));
                            }
                            mean__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Twap => {
                            if twap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("twap"));
                            }
                            twap__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstTimestamp => {
                            if first_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstTimestamp"));
                            }
                            first_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LastTimestamp => {
                            if last_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastTimestamp"));
                            }
                            last_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinPrice => {
                            if min_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPrice"));
                            }
                            min_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxPrice => {
                            if max_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPrice"));
                            }
                            max_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MedianPrice => {
                            if median_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("medianPrice"));
                            }
                            median_price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MetadataStatistics {
                    group_count: group_count__.unwrap_or_default(),
                    records_sample_size: records_sample_size__.unwrap_or_default(),
                    mean: mean__.unwrap_or_default(),
                    twap: twap__.unwrap_or_default(),
                    first_timestamp: first_timestamp__.unwrap_or_default(),
                    last_timestamp: last_timestamp__.unwrap_or_default(),
                    min_price: min_price__.unwrap_or_default(),
                    max_price: max_price__.unwrap_or_default(),
                    median_price: median_price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MetadataStatistics",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayBandRates {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relayer.is_empty() {
            len += 1;
        }
        if !self.symbols.is_empty() {
            len += 1;
        }
        if !self.rates.is_empty() {
            len += 1;
        }
        if !self.resolve_times.is_empty() {
            len += 1;
        }
        if !self.request_i_ds.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.MsgRelayBandRates", len)?;
        if !self.relayer.is_empty() {
            struct_ser.serialize_field("relayer", &self.relayer)?;
        }
        if !self.symbols.is_empty() {
            struct_ser.serialize_field("symbols", &self.symbols)?;
        }
        if !self.rates.is_empty() {
            struct_ser.serialize_field(
                "rates",
                &self
                    .rates
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        if !self.resolve_times.is_empty() {
            struct_ser.serialize_field(
                "resolveTimes",
                &self
                    .resolve_times
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        if !self.request_i_ds.is_empty() {
            struct_ser.serialize_field(
                "requestIDs",
                &self
                    .request_i_ds
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayBandRates {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relayer",
            "symbols",
            "rates",
            "resolve_times",
            "resolveTimes",
            "requestIDs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayer,
            Symbols,
            Rates,
            ResolveTimes,
            RequestIDs,
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
                            "relayer" => Ok(GeneratedField::Relayer),
                            "symbols" => Ok(GeneratedField::Symbols),
                            "rates" => Ok(GeneratedField::Rates),
                            "resolveTimes" | "resolve_times" => Ok(GeneratedField::ResolveTimes),
                            "requestIDs" => Ok(GeneratedField::RequestIDs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRelayBandRates;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayBandRates")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRelayBandRates, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut relayer__ = None;
                let mut symbols__ = None;
                let mut rates__ = None;
                let mut resolve_times__ = None;
                let mut request_i_ds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relayer => {
                            if relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayer"));
                            }
                            relayer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbols => {
                            if symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbols"));
                            }
                            symbols__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rates => {
                            if rates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rates"));
                            }
                            rates__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::ResolveTimes => {
                            if resolve_times__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolveTimes"));
                            }
                            resolve_times__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::RequestIDs => {
                            if request_i_ds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestIDs"));
                            }
                            request_i_ds__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(MsgRelayBandRates {
                    relayer: relayer__.unwrap_or_default(),
                    symbols: symbols__.unwrap_or_default(),
                    rates: rates__.unwrap_or_default(),
                    resolve_times: resolve_times__.unwrap_or_default(),
                    request_i_ds: request_i_ds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayBandRates",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayBandRatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.MsgRelayBandRatesResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayBandRatesResponse {
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
            type Value = MsgRelayBandRatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayBandRatesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayBandRatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRelayBandRatesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayBandRatesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayCoinbaseMessages {
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
        if !self.messages.is_empty() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.MsgRelayCoinbaseMessages", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.messages.is_empty() {
            struct_ser.serialize_field(
                "messages",
                &self
                    .messages
                    .iter()
                    .map(pbjson::private::base64::encode)
                    .collect::<Vec<_>>(),
            )?;
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
impl<'de> serde::Deserialize<'de> for MsgRelayCoinbaseMessages {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "messages", "signatures"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Messages,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "messages" => Ok(GeneratedField::Messages),
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
            type Value = MsgRelayCoinbaseMessages;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayCoinbaseMessages")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayCoinbaseMessages, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut messages__ = None;
                let mut signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(
                                map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
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
                Ok(MsgRelayCoinbaseMessages {
                    sender: sender__.unwrap_or_default(),
                    messages: messages__.unwrap_or_default(),
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayCoinbaseMessages",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayCoinbaseMessagesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.MsgRelayCoinbaseMessagesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayCoinbaseMessagesResponse {
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
            type Value = MsgRelayCoinbaseMessagesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.MsgRelayCoinbaseMessagesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayCoinbaseMessagesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRelayCoinbaseMessagesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayCoinbaseMessagesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayPriceFeedPrice {
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
        if !self.base.is_empty() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.MsgRelayPriceFeedPrice", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.base.is_empty() {
            struct_ser.serialize_field("base", &self.base)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayPriceFeedPrice {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "base", "quote", "price"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Base,
            Quote,
            Price,
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
                            "base" => Ok(GeneratedField::Base),
                            "quote" => Ok(GeneratedField::Quote),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRelayPriceFeedPrice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayPriceFeedPrice")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayPriceFeedPrice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut base__ = None;
                let mut quote__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRelayPriceFeedPrice {
                    sender: sender__.unwrap_or_default(),
                    base: base__.unwrap_or_default(),
                    quote: quote__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayPriceFeedPrice",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayPriceFeedPriceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.MsgRelayPriceFeedPriceResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayPriceFeedPriceResponse {
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
            type Value = MsgRelayPriceFeedPriceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.MsgRelayPriceFeedPriceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayPriceFeedPriceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRelayPriceFeedPriceResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayPriceFeedPriceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayProviderPrices {
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
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.symbols.is_empty() {
            len += 1;
        }
        if !self.prices.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.MsgRelayProviderPrices", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.symbols.is_empty() {
            struct_ser.serialize_field("symbols", &self.symbols)?;
        }
        if !self.prices.is_empty() {
            struct_ser.serialize_field("prices", &self.prices)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayProviderPrices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "provider", "symbols", "prices"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Provider,
            Symbols,
            Prices,
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
                            "provider" => Ok(GeneratedField::Provider),
                            "symbols" => Ok(GeneratedField::Symbols),
                            "prices" => Ok(GeneratedField::Prices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRelayProviderPrices;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayProviderPrices")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayProviderPrices, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut provider__ = None;
                let mut symbols__ = None;
                let mut prices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbols => {
                            if symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbols"));
                            }
                            symbols__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Prices => {
                            if prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prices"));
                            }
                            prices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRelayProviderPrices {
                    sender: sender__.unwrap_or_default(),
                    provider: provider__.unwrap_or_default(),
                    symbols: symbols__.unwrap_or_default(),
                    prices: prices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayProviderPrices",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayProviderPricesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.MsgRelayProviderPricesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayProviderPricesResponse {
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
            type Value = MsgRelayProviderPricesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.MsgRelayProviderPricesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayProviderPricesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRelayProviderPricesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayProviderPricesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayPythPrices {
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
        if !self.price_attestations.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.MsgRelayPythPrices", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.price_attestations.is_empty() {
            struct_ser.serialize_field("priceAttestations", &self.price_attestations)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayPythPrices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "price_attestations", "priceAttestations"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            PriceAttestations,
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
                            "priceAttestations" | "price_attestations" => {
                                Ok(GeneratedField::PriceAttestations)
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
            type Value = MsgRelayPythPrices;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayPythPrices")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRelayPythPrices, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut price_attestations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PriceAttestations => {
                            if price_attestations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceAttestations"));
                            }
                            price_attestations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRelayPythPrices {
                    sender: sender__.unwrap_or_default(),
                    price_attestations: price_attestations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayPythPrices",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayPythPricesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.MsgRelayPythPricesResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayPythPricesResponse {
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
            type Value = MsgRelayPythPricesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayPythPricesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayPythPricesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRelayPythPricesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayPythPricesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayStorkPrices {
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
        if !self.asset_pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.MsgRelayStorkPrices", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.asset_pairs.is_empty() {
            struct_ser.serialize_field("assetPairs", &self.asset_pairs)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayStorkPrices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "asset_pairs", "assetPairs"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            AssetPairs,
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
                            "assetPairs" | "asset_pairs" => Ok(GeneratedField::AssetPairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRelayStorkPrices;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayStorkPrices")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRelayStorkPrices, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut asset_pairs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetPairs => {
                            if asset_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetPairs"));
                            }
                            asset_pairs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRelayStorkPrices {
                    sender: sender__.unwrap_or_default(),
                    asset_pairs: asset_pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayStorkPrices",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRelayStorkPricesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.MsgRelayStorkPricesResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRelayStorkPricesResponse {
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
            type Value = MsgRelayStorkPricesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRelayStorkPricesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRelayStorkPricesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRelayStorkPricesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRelayStorkPricesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRequestBandIbcRates {
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
        if self.request_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.MsgRequestBandIBCRates", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if self.request_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("requestId", ToString::to_string(&self.request_id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRequestBandIbcRates {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "request_id", "requestId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            RequestId,
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
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRequestBandIbcRates;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.MsgRequestBandIBCRates")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRequestBandIbcRates, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut request_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgRequestBandIbcRates {
                    sender: sender__.unwrap_or_default(),
                    request_id: request_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRequestBandIBCRates",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRequestBandIbcRatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.MsgRequestBandIBCRatesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRequestBandIbcRatesResponse {
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
            type Value = MsgRequestBandIbcRatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.MsgRequestBandIBCRatesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRequestBandIbcRatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRequestBandIbcRatesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.MsgRequestBandIBCRatesResponse",
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
            serializer.serialize_struct("injective.oracle.v1beta1.MsgUpdateParams", len)?;
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
                formatter.write_str("struct injective.oracle.v1beta1.MsgUpdateParams")
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
            "injective.oracle.v1beta1.MsgUpdateParams",
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
            serializer.serialize_struct("injective.oracle.v1beta1.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct injective.oracle.v1beta1.MsgUpdateParamsResponse")
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
            "injective.oracle.v1beta1.MsgUpdateParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OracleHistoryOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_age != 0 {
            len += 1;
        }
        if self.include_raw_history {
            len += 1;
        }
        if self.include_metadata {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.OracleHistoryOptions", len)?;
        if self.max_age != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxAge", ToString::to_string(&self.max_age).as_str())?;
        }
        if self.include_raw_history {
            struct_ser.serialize_field("includeRawHistory", &self.include_raw_history)?;
        }
        if self.include_metadata {
            struct_ser.serialize_field("includeMetadata", &self.include_metadata)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OracleHistoryOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_age",
            "maxAge",
            "include_raw_history",
            "includeRawHistory",
            "include_metadata",
            "includeMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxAge,
            IncludeRawHistory,
            IncludeMetadata,
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
                            "maxAge" | "max_age" => Ok(GeneratedField::MaxAge),
                            "includeRawHistory" | "include_raw_history" => {
                                Ok(GeneratedField::IncludeRawHistory)
                            }
                            "includeMetadata" | "include_metadata" => {
                                Ok(GeneratedField::IncludeMetadata)
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
            type Value = OracleHistoryOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.OracleHistoryOptions")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<OracleHistoryOptions, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut max_age__ = None;
                let mut include_raw_history__ = None;
                let mut include_metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxAge => {
                            if max_age__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAge"));
                            }
                            max_age__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::IncludeRawHistory => {
                            if include_raw_history__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeRawHistory"));
                            }
                            include_raw_history__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeMetadata => {
                            if include_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeMetadata"));
                            }
                            include_metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OracleHistoryOptions {
                    max_age: max_age__.unwrap_or_default(),
                    include_raw_history: include_raw_history__.unwrap_or_default(),
                    include_metadata: include_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.OracleHistoryOptions",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OracleInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symbol.is_empty() {
            len += 1;
        }
        if self.oracle_type != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.OracleInfo", len)?;
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if self.oracle_type != 0 {
            let v = OracleType::try_from(self.oracle_type).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.oracle_type))
            })?;
            struct_ser.serialize_field("oracleType", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OracleInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["symbol", "oracle_type", "oracleType"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Symbol,
            OracleType,
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
                            "symbol" => Ok(GeneratedField::Symbol),
                            "oracleType" | "oracle_type" => Ok(GeneratedField::OracleType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OracleInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.OracleInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OracleInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut symbol__ = None;
                let mut oracle_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleType => {
                            if oracle_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleType"));
                            }
                            oracle_type__ = Some(map_.next_value::<OracleType>()? as i32);
                        }
                    }
                }
                Ok(OracleInfo {
                    symbol: symbol__.unwrap_or_default(),
                    oracle_type: oracle_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.OracleInfo",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OracleType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "Unspecified",
            Self::Band => "Band",
            Self::PriceFeed => "PriceFeed",
            Self::Coinbase => "Coinbase",
            Self::Chainlink => "Chainlink",
            Self::Razor => "Razor",
            Self::Dia => "Dia",
            Self::Api3 => "API3",
            Self::Uma => "Uma",
            Self::Pyth => "Pyth",
            Self::BandIbc => "BandIBC",
            Self::Provider => "Provider",
            Self::Stork => "Stork",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OracleType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Unspecified",
            "Band",
            "PriceFeed",
            "Coinbase",
            "Chainlink",
            "Razor",
            "Dia",
            "API3",
            "Uma",
            "Pyth",
            "BandIBC",
            "Provider",
            "Stork",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OracleType;

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
                    "Unspecified" => Ok(OracleType::Unspecified),
                    "Band" => Ok(OracleType::Band),
                    "PriceFeed" => Ok(OracleType::PriceFeed),
                    "Coinbase" => Ok(OracleType::Coinbase),
                    "Chainlink" => Ok(OracleType::Chainlink),
                    "Razor" => Ok(OracleType::Razor),
                    "Dia" => Ok(OracleType::Dia),
                    "API3" => Ok(OracleType::Api3),
                    "Uma" => Ok(OracleType::Uma),
                    "Pyth" => Ok(OracleType::Pyth),
                    "BandIBC" => Ok(OracleType::BandIbc),
                    "Provider" => Ok(OracleType::Provider),
                    "Stork" => Ok(OracleType::Stork),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        if !self.pyth_contract.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.oracle.v1beta1.Params", len)?;
        if !self.pyth_contract.is_empty() {
            struct_ser.serialize_field("pythContract", &self.pyth_contract)?;
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
        const FIELDS: &[&str] = &["pyth_contract", "pythContract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PythContract,
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
                            "pythContract" | "pyth_contract" => Ok(GeneratedField::PythContract),
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
                formatter.write_str("struct injective.oracle.v1beta1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pyth_contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PythContract => {
                            if pyth_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pythContract"));
                            }
                            pyth_contract__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    pyth_contract: pyth_contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.oracle.v1beta1.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PriceAttestation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_id.is_empty() {
            len += 1;
        }
        if self.price != 0 {
            len += 1;
        }
        if self.conf != 0 {
            len += 1;
        }
        if self.expo != 0 {
            len += 1;
        }
        if self.ema_price != 0 {
            len += 1;
        }
        if self.ema_conf != 0 {
            len += 1;
        }
        if self.ema_expo != 0 {
            len += 1;
        }
        if self.publish_time != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PriceAttestation", len)?;
        if !self.price_id.is_empty() {
            struct_ser.serialize_field("priceId", &self.price_id)?;
        }
        if self.price != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("price", ToString::to_string(&self.price).as_str())?;
        }
        if self.conf != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("conf", ToString::to_string(&self.conf).as_str())?;
        }
        if self.expo != 0 {
            struct_ser.serialize_field("expo", &self.expo)?;
        }
        if self.ema_price != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("emaPrice", ToString::to_string(&self.ema_price).as_str())?;
        }
        if self.ema_conf != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("emaConf", ToString::to_string(&self.ema_conf).as_str())?;
        }
        if self.ema_expo != 0 {
            struct_ser.serialize_field("emaExpo", &self.ema_expo)?;
        }
        if self.publish_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "publishTime",
                ToString::to_string(&self.publish_time).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PriceAttestation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_id",
            "priceId",
            "price",
            "conf",
            "expo",
            "ema_price",
            "emaPrice",
            "ema_conf",
            "emaConf",
            "ema_expo",
            "emaExpo",
            "publish_time",
            "publishTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceId,
            Price,
            Conf,
            Expo,
            EmaPrice,
            EmaConf,
            EmaExpo,
            PublishTime,
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
                            "priceId" | "price_id" => Ok(GeneratedField::PriceId),
                            "price" => Ok(GeneratedField::Price),
                            "conf" => Ok(GeneratedField::Conf),
                            "expo" => Ok(GeneratedField::Expo),
                            "emaPrice" | "ema_price" => Ok(GeneratedField::EmaPrice),
                            "emaConf" | "ema_conf" => Ok(GeneratedField::EmaConf),
                            "emaExpo" | "ema_expo" => Ok(GeneratedField::EmaExpo),
                            "publishTime" | "publish_time" => Ok(GeneratedField::PublishTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceAttestation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PriceAttestation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceAttestation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_id__ = None;
                let mut price__ = None;
                let mut conf__ = None;
                let mut expo__ = None;
                let mut ema_price__ = None;
                let mut ema_conf__ = None;
                let mut ema_expo__ = None;
                let mut publish_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceId => {
                            if price_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceId"));
                            }
                            price_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Conf => {
                            if conf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conf"));
                            }
                            conf__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Expo => {
                            if expo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expo"));
                            }
                            expo__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EmaPrice => {
                            if ema_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emaPrice"));
                            }
                            ema_price__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EmaConf => {
                            if ema_conf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emaConf"));
                            }
                            ema_conf__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EmaExpo => {
                            if ema_expo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emaExpo"));
                            }
                            ema_expo__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PublishTime => {
                            if publish_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publishTime"));
                            }
                            publish_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PriceAttestation {
                    price_id: price_id__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    conf: conf__.unwrap_or_default(),
                    expo: expo__.unwrap_or_default(),
                    ema_price: ema_price__.unwrap_or_default(),
                    ema_conf: ema_conf__.unwrap_or_default(),
                    ema_expo: ema_expo__.unwrap_or_default(),
                    publish_time: publish_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PriceAttestation",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PriceFeedInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.base.is_empty() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PriceFeedInfo", len)?;
        if !self.base.is_empty() {
            struct_ser.serialize_field("base", &self.base)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PriceFeedInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["base", "quote"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Base,
            Quote,
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
                            "base" => Ok(GeneratedField::Base),
                            "quote" => Ok(GeneratedField::Quote),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceFeedInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PriceFeedInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceFeedInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut base__ = None;
                let mut quote__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PriceFeedInfo {
                    base: base__.unwrap_or_default(),
                    quote: quote__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PriceFeedInfo",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PriceFeedPrice {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PriceFeedPrice", len)?;
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PriceFeedPrice {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Price,
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
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceFeedPrice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PriceFeedPrice")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceFeedPrice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PriceFeedPrice {
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PriceFeedPrice",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PriceFeedState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.base.is_empty() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        if self.price_state.is_some() {
            len += 1;
        }
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PriceFeedState", len)?;
        if !self.base.is_empty() {
            struct_ser.serialize_field("base", &self.base)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        if let Some(v) = self.price_state.as_ref() {
            struct_ser.serialize_field("priceState", v)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PriceFeedState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["base", "quote", "price_state", "priceState", "relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Base,
            Quote,
            PriceState,
            Relayers,
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
                            "base" => Ok(GeneratedField::Base),
                            "quote" => Ok(GeneratedField::Quote),
                            "priceState" | "price_state" => Ok(GeneratedField::PriceState),
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceFeedState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PriceFeedState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceFeedState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut base__ = None;
                let mut quote__ = None;
                let mut price_state__ = None;
                let mut relayers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PriceState => {
                            if price_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceState"));
                            }
                            price_state__ = map_.next_value()?;
                        }
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PriceFeedState {
                    base: base__.unwrap_or_default(),
                    quote: quote__.unwrap_or_default(),
                    price_state: price_state__,
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PriceFeedState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PricePairState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pair_price.is_empty() {
            len += 1;
        }
        if !self.base_price.is_empty() {
            len += 1;
        }
        if !self.quote_price.is_empty() {
            len += 1;
        }
        if !self.base_cumulative_price.is_empty() {
            len += 1;
        }
        if !self.quote_cumulative_price.is_empty() {
            len += 1;
        }
        if self.base_timestamp != 0 {
            len += 1;
        }
        if self.quote_timestamp != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PricePairState", len)?;
        if !self.pair_price.is_empty() {
            struct_ser.serialize_field("pairPrice", &self.pair_price)?;
        }
        if !self.base_price.is_empty() {
            struct_ser.serialize_field("basePrice", &self.base_price)?;
        }
        if !self.quote_price.is_empty() {
            struct_ser.serialize_field("quotePrice", &self.quote_price)?;
        }
        if !self.base_cumulative_price.is_empty() {
            struct_ser.serialize_field("baseCumulativePrice", &self.base_cumulative_price)?;
        }
        if !self.quote_cumulative_price.is_empty() {
            struct_ser.serialize_field("quoteCumulativePrice", &self.quote_cumulative_price)?;
        }
        if self.base_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "baseTimestamp",
                ToString::to_string(&self.base_timestamp).as_str(),
            )?;
        }
        if self.quote_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "quoteTimestamp",
                ToString::to_string(&self.quote_timestamp).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PricePairState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pair_price",
            "pairPrice",
            "base_price",
            "basePrice",
            "quote_price",
            "quotePrice",
            "base_cumulative_price",
            "baseCumulativePrice",
            "quote_cumulative_price",
            "quoteCumulativePrice",
            "base_timestamp",
            "baseTimestamp",
            "quote_timestamp",
            "quoteTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PairPrice,
            BasePrice,
            QuotePrice,
            BaseCumulativePrice,
            QuoteCumulativePrice,
            BaseTimestamp,
            QuoteTimestamp,
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
                            "pairPrice" | "pair_price" => Ok(GeneratedField::PairPrice),
                            "basePrice" | "base_price" => Ok(GeneratedField::BasePrice),
                            "quotePrice" | "quote_price" => Ok(GeneratedField::QuotePrice),
                            "baseCumulativePrice" | "base_cumulative_price" => {
                                Ok(GeneratedField::BaseCumulativePrice)
                            }
                            "quoteCumulativePrice" | "quote_cumulative_price" => {
                                Ok(GeneratedField::QuoteCumulativePrice)
                            }
                            "baseTimestamp" | "base_timestamp" => Ok(GeneratedField::BaseTimestamp),
                            "quoteTimestamp" | "quote_timestamp" => {
                                Ok(GeneratedField::QuoteTimestamp)
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
            type Value = PricePairState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PricePairState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PricePairState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pair_price__ = None;
                let mut base_price__ = None;
                let mut quote_price__ = None;
                let mut base_cumulative_price__ = None;
                let mut quote_cumulative_price__ = None;
                let mut base_timestamp__ = None;
                let mut quote_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PairPrice => {
                            if pair_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairPrice"));
                            }
                            pair_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BasePrice => {
                            if base_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basePrice"));
                            }
                            base_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotePrice => {
                            if quote_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotePrice"));
                            }
                            quote_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BaseCumulativePrice => {
                            if base_cumulative_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "baseCumulativePrice",
                                ));
                            }
                            base_cumulative_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuoteCumulativePrice => {
                            if quote_cumulative_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "quoteCumulativePrice",
                                ));
                            }
                            quote_cumulative_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BaseTimestamp => {
                            if base_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseTimestamp"));
                            }
                            base_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::QuoteTimestamp => {
                            if quote_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteTimestamp"));
                            }
                            quote_timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PricePairState {
                    pair_price: pair_price__.unwrap_or_default(),
                    base_price: base_price__.unwrap_or_default(),
                    quote_price: quote_price__.unwrap_or_default(),
                    base_cumulative_price: base_cumulative_price__.unwrap_or_default(),
                    quote_cumulative_price: quote_cumulative_price__.unwrap_or_default(),
                    base_timestamp: base_timestamp__.unwrap_or_default(),
                    quote_timestamp: quote_timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PricePairState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PriceRecord {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp != 0 {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PriceRecord", len)?;
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PriceRecord {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["timestamp", "price"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            Price,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceRecord;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PriceRecord")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceRecord, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PriceRecord {
                    timestamp: timestamp__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PriceRecord",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PriceRecords {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.oracle != 0 {
            len += 1;
        }
        if !self.symbol_id.is_empty() {
            len += 1;
        }
        if !self.latest_price_records.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PriceRecords", len)?;
        if self.oracle != 0 {
            let v = OracleType::try_from(self.oracle).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.oracle))
            })?;
            struct_ser.serialize_field("oracle", &v)?;
        }
        if !self.symbol_id.is_empty() {
            struct_ser.serialize_field("symbolId", &self.symbol_id)?;
        }
        if !self.latest_price_records.is_empty() {
            struct_ser.serialize_field("latestPriceRecords", &self.latest_price_records)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PriceRecords {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "oracle",
            "symbol_id",
            "symbolId",
            "latest_price_records",
            "latestPriceRecords",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Oracle,
            SymbolId,
            LatestPriceRecords,
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
                            "oracle" => Ok(GeneratedField::Oracle),
                            "symbolId" | "symbol_id" => Ok(GeneratedField::SymbolId),
                            "latestPriceRecords" | "latest_price_records" => {
                                Ok(GeneratedField::LatestPriceRecords)
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
            type Value = PriceRecords;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PriceRecords")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceRecords, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut oracle__ = None;
                let mut symbol_id__ = None;
                let mut latest_price_records__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Oracle => {
                            if oracle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracle"));
                            }
                            oracle__ = Some(map_.next_value::<OracleType>()? as i32);
                        }
                        GeneratedField::SymbolId => {
                            if symbol_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbolId"));
                            }
                            symbol_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LatestPriceRecords => {
                            if latest_price_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "latestPriceRecords",
                                ));
                            }
                            latest_price_records__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PriceRecords {
                    oracle: oracle__.unwrap_or_default(),
                    symbol_id: symbol_id__.unwrap_or_default(),
                    latest_price_records: latest_price_records__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PriceRecords",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PriceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.cumulative_price.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PriceState", len)?;
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.cumulative_price.is_empty() {
            struct_ser.serialize_field("cumulativePrice", &self.cumulative_price)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PriceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price", "cumulative_price", "cumulativePrice", "timestamp"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Price,
            CumulativePrice,
            Timestamp,
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
                            "price" => Ok(GeneratedField::Price),
                            "cumulativePrice" | "cumulative_price" => {
                                Ok(GeneratedField::CumulativePrice)
                            }
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PriceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PriceState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price__ = None;
                let mut cumulative_price__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CumulativePrice => {
                            if cumulative_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cumulativePrice"));
                            }
                            cumulative_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PriceState {
                    price: price__.unwrap_or_default(),
                    cumulative_price: cumulative_price__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PriceState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ProviderInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.ProviderInfo", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ProviderInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider", "relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Relayers,
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
                            "provider" => Ok(GeneratedField::Provider),
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.ProviderInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProviderInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut relayers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProviderInfo {
                    provider: provider__.unwrap_or_default(),
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.ProviderInfo",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ProviderPriceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symbol.is_empty() {
            len += 1;
        }
        if self.state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.ProviderPriceState", len)?;
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if let Some(v) = self.state.as_ref() {
            struct_ser.serialize_field("state", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ProviderPriceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["symbol", "state"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Symbol,
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
                            "symbol" => Ok(GeneratedField::Symbol),
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
            type Value = ProviderPriceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.ProviderPriceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProviderPriceState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut symbol__ = None;
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ProviderPriceState {
                    symbol: symbol__.unwrap_or_default(),
                    state: state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.ProviderPriceState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ProviderState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider_info.is_some() {
            len += 1;
        }
        if !self.provider_price_states.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.ProviderState", len)?;
        if let Some(v) = self.provider_info.as_ref() {
            struct_ser.serialize_field("providerInfo", v)?;
        }
        if !self.provider_price_states.is_empty() {
            struct_ser.serialize_field("providerPriceStates", &self.provider_price_states)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ProviderState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_info",
            "providerInfo",
            "provider_price_states",
            "providerPriceStates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderInfo,
            ProviderPriceStates,
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
                            "providerInfo" | "provider_info" => Ok(GeneratedField::ProviderInfo),
                            "providerPriceStates" | "provider_price_states" => {
                                Ok(GeneratedField::ProviderPriceStates)
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
            type Value = ProviderState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.ProviderState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProviderState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider_info__ = None;
                let mut provider_price_states__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderInfo => {
                            if provider_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerInfo"));
                            }
                            provider_info__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderPriceStates => {
                            if provider_price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "providerPriceStates",
                                ));
                            }
                            provider_price_states__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProviderState {
                    provider_info: provider_info__,
                    provider_price_states: provider_price_states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.ProviderState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PythPriceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_id.is_empty() {
            len += 1;
        }
        if !self.ema_price.is_empty() {
            len += 1;
        }
        if !self.ema_conf.is_empty() {
            len += 1;
        }
        if !self.conf.is_empty() {
            len += 1;
        }
        if self.publish_time != 0 {
            len += 1;
        }
        if self.price_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.PythPriceState", len)?;
        if !self.price_id.is_empty() {
            struct_ser.serialize_field("priceId", &self.price_id)?;
        }
        if !self.ema_price.is_empty() {
            struct_ser.serialize_field("emaPrice", &self.ema_price)?;
        }
        if !self.ema_conf.is_empty() {
            struct_ser.serialize_field("emaConf", &self.ema_conf)?;
        }
        if !self.conf.is_empty() {
            struct_ser.serialize_field("conf", &self.conf)?;
        }
        if self.publish_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "publishTime",
                ToString::to_string(&self.publish_time).as_str(),
            )?;
        }
        if let Some(v) = self.price_state.as_ref() {
            struct_ser.serialize_field("priceState", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PythPriceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_id",
            "priceId",
            "ema_price",
            "emaPrice",
            "ema_conf",
            "emaConf",
            "conf",
            "publish_time",
            "publishTime",
            "price_state",
            "priceState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceId,
            EmaPrice,
            EmaConf,
            Conf,
            PublishTime,
            PriceState,
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
                            "priceId" | "price_id" => Ok(GeneratedField::PriceId),
                            "emaPrice" | "ema_price" => Ok(GeneratedField::EmaPrice),
                            "emaConf" | "ema_conf" => Ok(GeneratedField::EmaConf),
                            "conf" => Ok(GeneratedField::Conf),
                            "publishTime" | "publish_time" => Ok(GeneratedField::PublishTime),
                            "priceState" | "price_state" => Ok(GeneratedField::PriceState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PythPriceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.PythPriceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PythPriceState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_id__ = None;
                let mut ema_price__ = None;
                let mut ema_conf__ = None;
                let mut conf__ = None;
                let mut publish_time__ = None;
                let mut price_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceId => {
                            if price_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceId"));
                            }
                            price_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmaPrice => {
                            if ema_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emaPrice"));
                            }
                            ema_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmaConf => {
                            if ema_conf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emaConf"));
                            }
                            ema_conf__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Conf => {
                            if conf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conf"));
                            }
                            conf__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublishTime => {
                            if publish_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publishTime"));
                            }
                            publish_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PriceState => {
                            if price_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceState"));
                            }
                            price_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PythPriceState {
                    price_id: price_id__.unwrap_or_default(),
                    ema_price: ema_price__.unwrap_or_default(),
                    ema_conf: ema_conf__.unwrap_or_default(),
                    conf: conf__.unwrap_or_default(),
                    publish_time: publish_time__.unwrap_or_default(),
                    price_state: price_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.PythPriceState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBandIbcPriceStatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryBandIBCPriceStatesRequest",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBandIbcPriceStatesRequest {
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
            type Value = QueryBandIbcPriceStatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryBandIBCPriceStatesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBandIbcPriceStatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBandIbcPriceStatesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryBandIBCPriceStatesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBandIbcPriceStatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_states.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryBandIBCPriceStatesResponse",
            len,
        )?;
        if !self.price_states.is_empty() {
            struct_ser.serialize_field("priceStates", &self.price_states)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBandIbcPriceStatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_states", "priceStates"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceStates,
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
                            "priceStates" | "price_states" => Ok(GeneratedField::PriceStates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBandIbcPriceStatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryBandIBCPriceStatesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBandIbcPriceStatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_states__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceStates => {
                            if price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceStates"));
                            }
                            price_states__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBandIbcPriceStatesResponse {
                    price_states: price_states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryBandIBCPriceStatesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBandPriceStatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryBandPriceStatesRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBandPriceStatesRequest {
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
            type Value = QueryBandPriceStatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryBandPriceStatesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBandPriceStatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBandPriceStatesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryBandPriceStatesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBandPriceStatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_states.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryBandPriceStatesResponse", len)?;
        if !self.price_states.is_empty() {
            struct_ser.serialize_field("priceStates", &self.price_states)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBandPriceStatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_states", "priceStates"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceStates,
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
                            "priceStates" | "price_states" => Ok(GeneratedField::PriceStates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBandPriceStatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryBandPriceStatesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBandPriceStatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_states__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceStates => {
                            if price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceStates"));
                            }
                            price_states__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBandPriceStatesResponse {
                    price_states: price_states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryBandPriceStatesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBandRelayersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryBandRelayersRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBandRelayersRequest {
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
            type Value = QueryBandRelayersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryBandRelayersRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBandRelayersRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBandRelayersRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryBandRelayersRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBandRelayersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryBandRelayersResponse", len)?;
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBandRelayersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayers,
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
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBandRelayersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryBandRelayersResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBandRelayersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut relayers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBandRelayersResponse {
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryBandRelayersResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCoinbasePriceStatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryCoinbasePriceStatesRequest",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCoinbasePriceStatesRequest {
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
            type Value = QueryCoinbasePriceStatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryCoinbasePriceStatesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryCoinbasePriceStatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryCoinbasePriceStatesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryCoinbasePriceStatesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCoinbasePriceStatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_states.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryCoinbasePriceStatesResponse",
            len,
        )?;
        if !self.price_states.is_empty() {
            struct_ser.serialize_field("priceStates", &self.price_states)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCoinbasePriceStatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_states", "priceStates"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceStates,
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
                            "priceStates" | "price_states" => Ok(GeneratedField::PriceStates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCoinbasePriceStatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryCoinbasePriceStatesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryCoinbasePriceStatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_states__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceStates => {
                            if price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceStates"));
                            }
                            price_states__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryCoinbasePriceStatesResponse {
                    price_states: price_states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryCoinbasePriceStatesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryHistoricalPriceRecordsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.oracle != 0 {
            len += 1;
        }
        if !self.symbol_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryHistoricalPriceRecordsRequest",
            len,
        )?;
        if self.oracle != 0 {
            let v = OracleType::try_from(self.oracle).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.oracle))
            })?;
            struct_ser.serialize_field("oracle", &v)?;
        }
        if !self.symbol_id.is_empty() {
            struct_ser.serialize_field("symbolId", &self.symbol_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryHistoricalPriceRecordsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["oracle", "symbol_id", "symbolId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Oracle,
            SymbolId,
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
                            "oracle" => Ok(GeneratedField::Oracle),
                            "symbolId" | "symbol_id" => Ok(GeneratedField::SymbolId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryHistoricalPriceRecordsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryHistoricalPriceRecordsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryHistoricalPriceRecordsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut oracle__ = None;
                let mut symbol_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Oracle => {
                            if oracle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracle"));
                            }
                            oracle__ = Some(map_.next_value::<OracleType>()? as i32);
                        }
                        GeneratedField::SymbolId => {
                            if symbol_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbolId"));
                            }
                            symbol_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryHistoricalPriceRecordsRequest {
                    oracle: oracle__.unwrap_or_default(),
                    symbol_id: symbol_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryHistoricalPriceRecordsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryHistoricalPriceRecordsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_records.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryHistoricalPriceRecordsResponse",
            len,
        )?;
        if !self.price_records.is_empty() {
            struct_ser.serialize_field("priceRecords", &self.price_records)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryHistoricalPriceRecordsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_records", "priceRecords"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceRecords,
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
                            "priceRecords" | "price_records" => Ok(GeneratedField::PriceRecords),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryHistoricalPriceRecordsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.oracle.v1beta1.QueryHistoricalPriceRecordsResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryHistoricalPriceRecordsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_records__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceRecords => {
                            if price_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceRecords"));
                            }
                            price_records__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryHistoricalPriceRecordsResponse {
                    price_records: price_records__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryHistoricalPriceRecordsResponse",
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
            serializer.serialize_struct("injective.oracle.v1beta1.QueryModuleStateRequest", len)?;
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
                formatter.write_str("struct injective.oracle.v1beta1.QueryModuleStateRequest")
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
            "injective.oracle.v1beta1.QueryModuleStateRequest",
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
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryModuleStateResponse", len)?;
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
                formatter.write_str("struct injective.oracle.v1beta1.QueryModuleStateResponse")
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
            "injective.oracle.v1beta1.QueryModuleStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOraclePriceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.oracle_type != 0 {
            len += 1;
        }
        if !self.base.is_empty() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        if self.scaling_options.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.QueryOraclePriceRequest", len)?;
        if self.oracle_type != 0 {
            let v = OracleType::try_from(self.oracle_type).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.oracle_type))
            })?;
            struct_ser.serialize_field("oracleType", &v)?;
        }
        if !self.base.is_empty() {
            struct_ser.serialize_field("base", &self.base)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        if let Some(v) = self.scaling_options.as_ref() {
            struct_ser.serialize_field("scalingOptions", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOraclePriceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "oracle_type",
            "oracleType",
            "base",
            "quote",
            "scaling_options",
            "scalingOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OracleType,
            Base,
            Quote,
            ScalingOptions,
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
                            "oracleType" | "oracle_type" => Ok(GeneratedField::OracleType),
                            "base" => Ok(GeneratedField::Base),
                            "quote" => Ok(GeneratedField::Quote),
                            "scalingOptions" | "scaling_options" => {
                                Ok(GeneratedField::ScalingOptions)
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
            type Value = QueryOraclePriceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryOraclePriceRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOraclePriceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut oracle_type__ = None;
                let mut base__ = None;
                let mut quote__ = None;
                let mut scaling_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OracleType => {
                            if oracle_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleType"));
                            }
                            oracle_type__ = Some(map_.next_value::<OracleType>()? as i32);
                        }
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ScalingOptions => {
                            if scaling_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalingOptions"));
                            }
                            scaling_options__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryOraclePriceRequest {
                    oracle_type: oracle_type__.unwrap_or_default(),
                    base: base__.unwrap_or_default(),
                    quote: quote__.unwrap_or_default(),
                    scaling_options: scaling_options__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryOraclePriceRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOraclePriceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price_pair_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryOraclePriceResponse", len)?;
        if let Some(v) = self.price_pair_state.as_ref() {
            struct_ser.serialize_field("pricePairState", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOraclePriceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_pair_state", "pricePairState"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PricePairState,
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
                            "pricePairState" | "price_pair_state" => {
                                Ok(GeneratedField::PricePairState)
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
            type Value = QueryOraclePriceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryOraclePriceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOraclePriceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_pair_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PricePairState => {
                            if price_pair_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pricePairState"));
                            }
                            price_pair_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryOraclePriceResponse {
                    price_pair_state: price_pair_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryOraclePriceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOracleProviderPricesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryOracleProviderPricesRequest",
            len,
        )?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOracleProviderPricesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
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
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOracleProviderPricesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryOracleProviderPricesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOracleProviderPricesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOracleProviderPricesRequest {
                    provider: provider__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryOracleProviderPricesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOracleProviderPricesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider_state.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryOracleProviderPricesResponse",
            len,
        )?;
        if !self.provider_state.is_empty() {
            struct_ser.serialize_field("providerState", &self.provider_state)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOracleProviderPricesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["providerState"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderState,
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
                            "providerState" => Ok(GeneratedField::ProviderState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOracleProviderPricesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryOracleProviderPricesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOracleProviderPricesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderState => {
                            if provider_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerState"));
                            }
                            provider_state__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOracleProviderPricesResponse {
                    provider_state: provider_state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryOracleProviderPricesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOracleProvidersInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryOracleProvidersInfoRequest",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOracleProvidersInfoRequest {
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
            type Value = QueryOracleProvidersInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryOracleProvidersInfoRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOracleProvidersInfoRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryOracleProvidersInfoRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryOracleProvidersInfoRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOracleProvidersInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.providers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryOracleProvidersInfoResponse",
            len,
        )?;
        if !self.providers.is_empty() {
            struct_ser.serialize_field("providers", &self.providers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOracleProvidersInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["providers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Providers,
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
                            "providers" => Ok(GeneratedField::Providers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOracleProvidersInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryOracleProvidersInfoResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOracleProvidersInfoResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut providers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Providers => {
                            if providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providers"));
                            }
                            providers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOracleProvidersInfoResponse {
                    providers: providers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryOracleProvidersInfoResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOracleVolatilityRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_info.is_some() {
            len += 1;
        }
        if self.quote_info.is_some() {
            len += 1;
        }
        if self.oracle_history_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryOracleVolatilityRequest", len)?;
        if let Some(v) = self.base_info.as_ref() {
            struct_ser.serialize_field("baseInfo", v)?;
        }
        if let Some(v) = self.quote_info.as_ref() {
            struct_ser.serialize_field("quoteInfo", v)?;
        }
        if let Some(v) = self.oracle_history_options.as_ref() {
            struct_ser.serialize_field("oracleHistoryOptions", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOracleVolatilityRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_info",
            "baseInfo",
            "quote_info",
            "quoteInfo",
            "oracle_history_options",
            "oracleHistoryOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseInfo,
            QuoteInfo,
            OracleHistoryOptions,
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
                            "baseInfo" | "base_info" => Ok(GeneratedField::BaseInfo),
                            "quoteInfo" | "quote_info" => Ok(GeneratedField::QuoteInfo),
                            "oracleHistoryOptions" | "oracle_history_options" => {
                                Ok(GeneratedField::OracleHistoryOptions)
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
            type Value = QueryOracleVolatilityRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryOracleVolatilityRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOracleVolatilityRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut base_info__ = None;
                let mut quote_info__ = None;
                let mut oracle_history_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseInfo => {
                            if base_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseInfo"));
                            }
                            base_info__ = map_.next_value()?;
                        }
                        GeneratedField::QuoteInfo => {
                            if quote_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteInfo"));
                            }
                            quote_info__ = map_.next_value()?;
                        }
                        GeneratedField::OracleHistoryOptions => {
                            if oracle_history_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "oracleHistoryOptions",
                                ));
                            }
                            oracle_history_options__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryOracleVolatilityRequest {
                    base_info: base_info__,
                    quote_info: quote_info__,
                    oracle_history_options: oracle_history_options__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryOracleVolatilityRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOracleVolatilityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.volatility.is_empty() {
            len += 1;
        }
        if self.history_metadata.is_some() {
            len += 1;
        }
        if !self.raw_history.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryOracleVolatilityResponse",
            len,
        )?;
        if !self.volatility.is_empty() {
            struct_ser.serialize_field("volatility", &self.volatility)?;
        }
        if let Some(v) = self.history_metadata.as_ref() {
            struct_ser.serialize_field("historyMetadata", v)?;
        }
        if !self.raw_history.is_empty() {
            struct_ser.serialize_field("rawHistory", &self.raw_history)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOracleVolatilityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "volatility",
            "history_metadata",
            "historyMetadata",
            "raw_history",
            "rawHistory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Volatility,
            HistoryMetadata,
            RawHistory,
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
                            "volatility" => Ok(GeneratedField::Volatility),
                            "historyMetadata" | "history_metadata" => {
                                Ok(GeneratedField::HistoryMetadata)
                            }
                            "rawHistory" | "raw_history" => Ok(GeneratedField::RawHistory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOracleVolatilityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryOracleVolatilityResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOracleVolatilityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut volatility__ = None;
                let mut history_metadata__ = None;
                let mut raw_history__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Volatility => {
                            if volatility__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volatility"));
                            }
                            volatility__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HistoryMetadata => {
                            if history_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyMetadata"));
                            }
                            history_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::RawHistory => {
                            if raw_history__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawHistory"));
                            }
                            raw_history__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOracleVolatilityResponse {
                    volatility: volatility__.unwrap_or_default(),
                    history_metadata: history_metadata__,
                    raw_history: raw_history__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryOracleVolatilityResponse",
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
            serializer.serialize_struct("injective.oracle.v1beta1.QueryParamsRequest", len)?;
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
                formatter.write_str("struct injective.oracle.v1beta1.QueryParamsRequest")
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
            "injective.oracle.v1beta1.QueryParamsRequest",
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
            serializer.serialize_struct("injective.oracle.v1beta1.QueryParamsResponse", len)?;
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
                formatter.write_str("struct injective.oracle.v1beta1.QueryParamsResponse")
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
            "injective.oracle.v1beta1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPriceFeedPriceStatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryPriceFeedPriceStatesRequest",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPriceFeedPriceStatesRequest {
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
            type Value = QueryPriceFeedPriceStatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryPriceFeedPriceStatesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPriceFeedPriceStatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryPriceFeedPriceStatesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryPriceFeedPriceStatesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPriceFeedPriceStatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_states.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryPriceFeedPriceStatesResponse",
            len,
        )?;
        if !self.price_states.is_empty() {
            struct_ser.serialize_field("priceStates", &self.price_states)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPriceFeedPriceStatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_states", "priceStates"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceStates,
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
                            "priceStates" | "price_states" => Ok(GeneratedField::PriceStates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPriceFeedPriceStatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryPriceFeedPriceStatesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPriceFeedPriceStatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_states__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceStates => {
                            if price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceStates"));
                            }
                            price_states__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPriceFeedPriceStatesResponse {
                    price_states: price_states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryPriceFeedPriceStatesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryProviderPriceStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.symbol.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryProviderPriceStateRequest",
            len,
        )?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryProviderPriceStateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider", "symbol"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Symbol,
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
                            "provider" => Ok(GeneratedField::Provider),
                            "symbol" => Ok(GeneratedField::Symbol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryProviderPriceStateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryProviderPriceStateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryProviderPriceStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut symbol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryProviderPriceStateRequest {
                    provider: provider__.unwrap_or_default(),
                    symbol: symbol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryProviderPriceStateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryProviderPriceStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryProviderPriceStateResponse",
            len,
        )?;
        if let Some(v) = self.price_state.as_ref() {
            struct_ser.serialize_field("priceState", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryProviderPriceStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_state", "priceState"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceState,
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
                            "priceState" | "price_state" => Ok(GeneratedField::PriceState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryProviderPriceStateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.QueryProviderPriceStateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryProviderPriceStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceState => {
                            if price_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceState"));
                            }
                            price_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryProviderPriceStateResponse {
                    price_state: price_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryProviderPriceStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPythPriceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.QueryPythPriceRequest", len)?;
        if !self.price_id.is_empty() {
            struct_ser.serialize_field("priceId", &self.price_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPythPriceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_id", "priceId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceId,
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
                            "priceId" | "price_id" => Ok(GeneratedField::PriceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPythPriceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryPythPriceRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPythPriceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceId => {
                            if price_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceId"));
                            }
                            price_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPythPriceRequest {
                    price_id: price_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryPythPriceRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPythPriceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.QueryPythPriceResponse", len)?;
        if let Some(v) = self.price_state.as_ref() {
            struct_ser.serialize_field("priceState", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPythPriceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_state", "priceState"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceState,
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
                            "priceState" | "price_state" => Ok(GeneratedField::PriceState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPythPriceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryPythPriceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPythPriceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceState => {
                            if price_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceState"));
                            }
                            price_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryPythPriceResponse {
                    price_state: price_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryPythPriceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPythPriceStatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryPythPriceStatesRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPythPriceStatesRequest {
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
            type Value = QueryPythPriceStatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryPythPriceStatesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPythPriceStatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryPythPriceStatesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryPythPriceStatesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPythPriceStatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_states.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryPythPriceStatesResponse", len)?;
        if !self.price_states.is_empty() {
            struct_ser.serialize_field("priceStates", &self.price_states)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPythPriceStatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_states", "priceStates"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceStates,
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
                            "priceStates" | "price_states" => Ok(GeneratedField::PriceStates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPythPriceStatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryPythPriceStatesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPythPriceStatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_states__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceStates => {
                            if price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceStates"));
                            }
                            price_states__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPythPriceStatesResponse {
                    price_states: price_states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryPythPriceStatesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryStorkPriceStatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryStorkPriceStatesRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryStorkPriceStatesRequest {
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
            type Value = QueryStorkPriceStatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryStorkPriceStatesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryStorkPriceStatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryStorkPriceStatesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryStorkPriceStatesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryStorkPriceStatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_states.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.QueryStorkPriceStatesResponse",
            len,
        )?;
        if !self.price_states.is_empty() {
            struct_ser.serialize_field("priceStates", &self.price_states)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryStorkPriceStatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_states", "priceStates"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceStates,
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
                            "priceStates" | "price_states" => Ok(GeneratedField::PriceStates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStorkPriceStatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryStorkPriceStatesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryStorkPriceStatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_states__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceStates => {
                            if price_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceStates"));
                            }
                            price_states__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryStorkPriceStatesResponse {
                    price_states: price_states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryStorkPriceStatesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryStorkPublishersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryStorkPublishersRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryStorkPublishersRequest {
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
            type Value = QueryStorkPublishersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryStorkPublishersRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryStorkPublishersRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryStorkPublishersRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryStorkPublishersRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryStorkPublishersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.publishers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.oracle.v1beta1.QueryStorkPublishersResponse", len)?;
        if !self.publishers.is_empty() {
            struct_ser.serialize_field("publishers", &self.publishers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryStorkPublishersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["publishers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Publishers,
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
                            "publishers" => Ok(GeneratedField::Publishers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStorkPublishersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.QueryStorkPublishersResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryStorkPublishersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut publishers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Publishers => {
                            if publishers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publishers"));
                            }
                            publishers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryStorkPublishersResponse {
                    publishers: publishers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.QueryStorkPublishersResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RevokeBandOraclePrivilegeProposal {
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
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.RevokeBandOraclePrivilegeProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RevokeBandOraclePrivilegeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Relayers,
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
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RevokeBandOraclePrivilegeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.RevokeBandOraclePrivilegeProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RevokeBandOraclePrivilegeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut relayers__ = None;
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
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RevokeBandOraclePrivilegeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.RevokeBandOraclePrivilegeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RevokePriceFeederPrivilegeProposal {
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
        if !self.base.is_empty() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.RevokePriceFeederPrivilegeProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.base.is_empty() {
            struct_ser.serialize_field("base", &self.base)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RevokePriceFeederPrivilegeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "base", "quote", "relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Base,
            Quote,
            Relayers,
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
                            "base" => Ok(GeneratedField::Base),
                            "quote" => Ok(GeneratedField::Quote),
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RevokePriceFeederPrivilegeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.RevokePriceFeederPrivilegeProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RevokePriceFeederPrivilegeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut base__ = None;
                let mut quote__ = None;
                let mut relayers__ = None;
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
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RevokePriceFeederPrivilegeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    base: base__.unwrap_or_default(),
                    quote: quote__.unwrap_or_default(),
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.RevokePriceFeederPrivilegeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RevokeProviderPrivilegeProposal {
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
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.RevokeProviderPrivilegeProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RevokeProviderPrivilegeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "provider", "relayers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Provider,
            Relayers,
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
                            "provider" => Ok(GeneratedField::Provider),
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RevokeProviderPrivilegeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.RevokeProviderPrivilegeProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RevokeProviderPrivilegeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut provider__ = None;
                let mut relayers__ = None;
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
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RevokeProviderPrivilegeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    provider: provider__.unwrap_or_default(),
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.RevokeProviderPrivilegeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RevokeStorkPublisherPrivilegeProposal {
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
        if !self.stork_publishers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.RevokeStorkPublisherPrivilegeProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.stork_publishers.is_empty() {
            struct_ser.serialize_field("storkPublishers", &self.stork_publishers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RevokeStorkPublisherPrivilegeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "stork_publishers",
            "storkPublishers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            StorkPublishers,
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
                            "storkPublishers" | "stork_publishers" => {
                                Ok(GeneratedField::StorkPublishers)
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
            type Value = RevokeStorkPublisherPrivilegeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.oracle.v1beta1.RevokeStorkPublisherPrivilegeProposal",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RevokeStorkPublisherPrivilegeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut stork_publishers__ = None;
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
                        GeneratedField::StorkPublishers => {
                            if stork_publishers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storkPublishers"));
                            }
                            stork_publishers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RevokeStorkPublisherPrivilegeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    stork_publishers: stork_publishers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.RevokeStorkPublisherPrivilegeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ScalingOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_decimals != 0 {
            len += 1;
        }
        if self.quote_decimals != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.ScalingOptions", len)?;
        if self.base_decimals != 0 {
            struct_ser.serialize_field("baseDecimals", &self.base_decimals)?;
        }
        if self.quote_decimals != 0 {
            struct_ser.serialize_field("quoteDecimals", &self.quote_decimals)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ScalingOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_decimals",
            "baseDecimals",
            "quote_decimals",
            "quoteDecimals",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseDecimals,
            QuoteDecimals,
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
                            "baseDecimals" | "base_decimals" => Ok(GeneratedField::BaseDecimals),
                            "quoteDecimals" | "quote_decimals" => Ok(GeneratedField::QuoteDecimals),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScalingOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.ScalingOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScalingOptions, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut base_decimals__ = None;
                let mut quote_decimals__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseDecimals => {
                            if base_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseDecimals"));
                            }
                            base_decimals__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::QuoteDecimals => {
                            if quote_decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteDecimals"));
                            }
                            quote_decimals__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ScalingOptions {
                    base_decimals: base_decimals__.unwrap_or_default(),
                    quote_decimals: quote_decimals__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.ScalingOptions",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetBandIbcPriceEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relayer.is_empty() {
            len += 1;
        }
        if !self.symbols.is_empty() {
            len += 1;
        }
        if !self.prices.is_empty() {
            len += 1;
        }
        if self.resolve_time != 0 {
            len += 1;
        }
        if self.request_id != 0 {
            len += 1;
        }
        if self.client_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.SetBandIBCPriceEvent", len)?;
        if !self.relayer.is_empty() {
            struct_ser.serialize_field("relayer", &self.relayer)?;
        }
        if !self.symbols.is_empty() {
            struct_ser.serialize_field("symbols", &self.symbols)?;
        }
        if !self.prices.is_empty() {
            struct_ser.serialize_field("prices", &self.prices)?;
        }
        if self.resolve_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "resolveTime",
                ToString::to_string(&self.resolve_time).as_str(),
            )?;
        }
        if self.request_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("requestId", ToString::to_string(&self.request_id).as_str())?;
        }
        if self.client_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("clientId", ToString::to_string(&self.client_id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetBandIbcPriceEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relayer",
            "symbols",
            "prices",
            "resolve_time",
            "resolveTime",
            "request_id",
            "requestId",
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayer,
            Symbols,
            Prices,
            ResolveTime,
            RequestId,
            ClientId,
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
                            "relayer" => Ok(GeneratedField::Relayer),
                            "symbols" => Ok(GeneratedField::Symbols),
                            "prices" => Ok(GeneratedField::Prices),
                            "resolveTime" | "resolve_time" => Ok(GeneratedField::ResolveTime),
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetBandIbcPriceEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.SetBandIBCPriceEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SetBandIbcPriceEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut relayer__ = None;
                let mut symbols__ = None;
                let mut prices__ = None;
                let mut resolve_time__ = None;
                let mut request_id__ = None;
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relayer => {
                            if relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayer"));
                            }
                            relayer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbols => {
                            if symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbols"));
                            }
                            symbols__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Prices => {
                            if prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prices"));
                            }
                            prices__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResolveTime => {
                            if resolve_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolveTime"));
                            }
                            resolve_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SetBandIbcPriceEvent {
                    relayer: relayer__.unwrap_or_default(),
                    symbols: symbols__.unwrap_or_default(),
                    prices: prices__.unwrap_or_default(),
                    resolve_time: resolve_time__.unwrap_or_default(),
                    request_id: request_id__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.SetBandIBCPriceEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetBandPriceEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relayer.is_empty() {
            len += 1;
        }
        if !self.symbol.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if self.resolve_time != 0 {
            len += 1;
        }
        if self.request_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.SetBandPriceEvent", len)?;
        if !self.relayer.is_empty() {
            struct_ser.serialize_field("relayer", &self.relayer)?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if self.resolve_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "resolveTime",
                ToString::to_string(&self.resolve_time).as_str(),
            )?;
        }
        if self.request_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("requestId", ToString::to_string(&self.request_id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetBandPriceEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relayer",
            "symbol",
            "price",
            "resolve_time",
            "resolveTime",
            "request_id",
            "requestId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayer,
            Symbol,
            Price,
            ResolveTime,
            RequestId,
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
                            "relayer" => Ok(GeneratedField::Relayer),
                            "symbol" => Ok(GeneratedField::Symbol),
                            "price" => Ok(GeneratedField::Price),
                            "resolveTime" | "resolve_time" => Ok(GeneratedField::ResolveTime),
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetBandPriceEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.SetBandPriceEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetBandPriceEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut relayer__ = None;
                let mut symbol__ = None;
                let mut price__ = None;
                let mut resolve_time__ = None;
                let mut request_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relayer => {
                            if relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayer"));
                            }
                            relayer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResolveTime => {
                            if resolve_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolveTime"));
                            }
                            resolve_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SetBandPriceEvent {
                    relayer: relayer__.unwrap_or_default(),
                    symbol: symbol__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    resolve_time: resolve_time__.unwrap_or_default(),
                    request_id: request_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.SetBandPriceEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetChainlinkPriceEvent {
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
        if !self.answer.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.SetChainlinkPriceEvent", len)?;
        if !self.feed_id.is_empty() {
            struct_ser.serialize_field("feedId", &self.feed_id)?;
        }
        if !self.answer.is_empty() {
            struct_ser.serialize_field("answer", &self.answer)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetChainlinkPriceEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feed_id", "feedId", "answer", "timestamp"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeedId,
            Answer,
            Timestamp,
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
                            "answer" => Ok(GeneratedField::Answer),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetChainlinkPriceEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.SetChainlinkPriceEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SetChainlinkPriceEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feed_id__ = None;
                let mut answer__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeedId => {
                            if feed_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feedId"));
                            }
                            feed_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Answer => {
                            if answer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answer"));
                            }
                            answer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SetChainlinkPriceEvent {
                    feed_id: feed_id__.unwrap_or_default(),
                    answer: answer__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.SetChainlinkPriceEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetCoinbasePriceEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symbol.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.SetCoinbasePriceEvent", len)?;
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetCoinbasePriceEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["symbol", "price", "timestamp"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Symbol,
            Price,
            Timestamp,
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
                            "symbol" => Ok(GeneratedField::Symbol),
                            "price" => Ok(GeneratedField::Price),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetCoinbasePriceEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.SetCoinbasePriceEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SetCoinbasePriceEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut symbol__ = None;
                let mut price__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SetCoinbasePriceEvent {
                    symbol: symbol__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.SetCoinbasePriceEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetPriceFeedPriceEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relayer.is_empty() {
            len += 1;
        }
        if !self.base.is_empty() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.SetPriceFeedPriceEvent", len)?;
        if !self.relayer.is_empty() {
            struct_ser.serialize_field("relayer", &self.relayer)?;
        }
        if !self.base.is_empty() {
            struct_ser.serialize_field("base", &self.base)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetPriceFeedPriceEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["relayer", "base", "quote", "price"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayer,
            Base,
            Quote,
            Price,
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
                            "relayer" => Ok(GeneratedField::Relayer),
                            "base" => Ok(GeneratedField::Base),
                            "quote" => Ok(GeneratedField::Quote),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPriceFeedPriceEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.SetPriceFeedPriceEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SetPriceFeedPriceEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut relayer__ = None;
                let mut base__ = None;
                let mut quote__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relayer => {
                            if relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayer"));
                            }
                            relayer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetPriceFeedPriceEvent {
                    relayer: relayer__.unwrap_or_default(),
                    base: base__.unwrap_or_default(),
                    quote: quote__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.SetPriceFeedPriceEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SetProviderPriceEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.relayer.is_empty() {
            len += 1;
        }
        if !self.symbol.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.SetProviderPriceEvent", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.relayer.is_empty() {
            struct_ser.serialize_field("relayer", &self.relayer)?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SetProviderPriceEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider", "relayer", "symbol", "price"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Relayer,
            Symbol,
            Price,
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
                            "provider" => Ok(GeneratedField::Provider),
                            "relayer" => Ok(GeneratedField::Relayer),
                            "symbol" => Ok(GeneratedField::Symbol),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetProviderPriceEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.SetProviderPriceEvent")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SetProviderPriceEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut relayer__ = None;
                let mut symbol__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relayer => {
                            if relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayer"));
                            }
                            relayer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetProviderPriceEvent {
                    provider: provider__.unwrap_or_default(),
                    relayer: relayer__.unwrap_or_default(),
                    symbol: symbol__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.SetProviderPriceEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SignedPriceOfAssetPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.publisher_key.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.SignedPriceOfAssetPair", len)?;
        if !self.publisher_key.is_empty() {
            struct_ser.serialize_field("publisherKey", &self.publisher_key)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.signature.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "signature",
                pbjson::private::base64::encode(&self.signature).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SignedPriceOfAssetPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "publisher_key",
            "publisherKey",
            "timestamp",
            "price",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublisherKey,
            Timestamp,
            Price,
            Signature,
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
                            "publisherKey" | "publisher_key" => Ok(GeneratedField::PublisherKey),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "price" => Ok(GeneratedField::Price),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignedPriceOfAssetPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.SignedPriceOfAssetPair")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SignedPriceOfAssetPair, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut publisher_key__ = None;
                let mut timestamp__ = None;
                let mut price__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublisherKey => {
                            if publisher_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisherKey"));
                            }
                            publisher_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SignedPriceOfAssetPair {
                    publisher_key: publisher_key__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.SignedPriceOfAssetPair",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StorkPriceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp != 0 {
            len += 1;
        }
        if !self.symbol.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.price_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.StorkPriceState", len)?;
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if let Some(v) = self.price_state.as_ref() {
            struct_ser.serialize_field("priceState", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StorkPriceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["timestamp", "symbol", "value", "price_state", "priceState"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            Symbol,
            Value,
            PriceState,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "symbol" => Ok(GeneratedField::Symbol),
                            "value" => Ok(GeneratedField::Value),
                            "priceState" | "price_state" => Ok(GeneratedField::PriceState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorkPriceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.StorkPriceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorkPriceState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut symbol__ = None;
                let mut value__ = None;
                let mut price_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PriceState => {
                            if price_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceState"));
                            }
                            price_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StorkPriceState {
                    timestamp: timestamp__.unwrap_or_default(),
                    symbol: symbol__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    price_state: price_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.StorkPriceState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SymbolPriceTimestamp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.oracle != 0 {
            len += 1;
        }
        if !self.symbol_id.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.oracle.v1beta1.SymbolPriceTimestamp", len)?;
        if self.oracle != 0 {
            let v = OracleType::try_from(self.oracle).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.oracle))
            })?;
            struct_ser.serialize_field("oracle", &v)?;
        }
        if !self.symbol_id.is_empty() {
            struct_ser.serialize_field("symbolId", &self.symbol_id)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SymbolPriceTimestamp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["oracle", "symbol_id", "symbolId", "timestamp"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Oracle,
            SymbolId,
            Timestamp,
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
                            "oracle" => Ok(GeneratedField::Oracle),
                            "symbolId" | "symbol_id" => Ok(GeneratedField::SymbolId),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SymbolPriceTimestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.oracle.v1beta1.SymbolPriceTimestamp")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SymbolPriceTimestamp, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut oracle__ = None;
                let mut symbol_id__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Oracle => {
                            if oracle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracle"));
                            }
                            oracle__ = Some(map_.next_value::<OracleType>()? as i32);
                        }
                        GeneratedField::SymbolId => {
                            if symbol_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbolId"));
                            }
                            symbol_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SymbolPriceTimestamp {
                    oracle: oracle__.unwrap_or_default(),
                    symbol_id: symbol_id__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.SymbolPriceTimestamp",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for UpdateBandOracleRequestProposal {
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
        if !self.delete_request_ids.is_empty() {
            len += 1;
        }
        if self.update_oracle_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.oracle.v1beta1.UpdateBandOracleRequestProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.delete_request_ids.is_empty() {
            struct_ser.serialize_field(
                "deleteRequestIds",
                &self
                    .delete_request_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        if let Some(v) = self.update_oracle_request.as_ref() {
            struct_ser.serialize_field("updateOracleRequest", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UpdateBandOracleRequestProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "delete_request_ids",
            "deleteRequestIds",
            "update_oracle_request",
            "updateOracleRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            DeleteRequestIds,
            UpdateOracleRequest,
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
                            "deleteRequestIds" | "delete_request_ids" => {
                                Ok(GeneratedField::DeleteRequestIds)
                            }
                            "updateOracleRequest" | "update_oracle_request" => {
                                Ok(GeneratedField::UpdateOracleRequest)
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
            type Value = UpdateBandOracleRequestProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.oracle.v1beta1.UpdateBandOracleRequestProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<UpdateBandOracleRequestProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut delete_request_ids__ = None;
                let mut update_oracle_request__ = None;
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
                        GeneratedField::DeleteRequestIds => {
                            if delete_request_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteRequestIds"));
                            }
                            delete_request_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::UpdateOracleRequest => {
                            if update_oracle_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "updateOracleRequest",
                                ));
                            }
                            update_oracle_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateBandOracleRequestProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    delete_request_ids: delete_request_ids__.unwrap_or_default(),
                    update_oracle_request: update_oracle_request__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.oracle.v1beta1.UpdateBandOracleRequestProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
