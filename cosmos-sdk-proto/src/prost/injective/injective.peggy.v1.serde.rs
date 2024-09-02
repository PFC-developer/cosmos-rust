// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for Attestation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.observed {
            len += 1;
        }
        if !self.votes.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.claim.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.Attestation", len)?;
        if self.observed {
            struct_ser.serialize_field("observed", &self.observed)?;
        }
        if !self.votes.is_empty() {
            struct_ser.serialize_field("votes", &self.votes)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.claim.as_ref() {
            struct_ser.serialize_field("claim", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Attestation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["observed", "votes", "height", "claim"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Observed,
            Votes,
            Height,
            Claim,
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
                            "observed" => Ok(GeneratedField::Observed),
                            "votes" => Ok(GeneratedField::Votes),
                            "height" => Ok(GeneratedField::Height),
                            "claim" => Ok(GeneratedField::Claim),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Attestation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.Attestation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Attestation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut observed__ = None;
                let mut votes__ = None;
                let mut height__ = None;
                let mut claim__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Observed => {
                            if observed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("observed"));
                            }
                            observed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Votes => {
                            if votes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votes"));
                            }
                            votes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Claim => {
                            if claim__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claim"));
                            }
                            claim__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Attestation {
                    observed: observed__.unwrap_or_default(),
                    votes: votes__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    claim: claim__,
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.Attestation", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BatchFees {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token.is_empty() {
            len += 1;
        }
        if !self.total_fees.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.BatchFees", len)?;
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        if !self.total_fees.is_empty() {
            struct_ser.serialize_field("totalFees", &self.total_fees)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BatchFees {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["token", "total_fees", "totalFees"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
            TotalFees,
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
                            "token" => Ok(GeneratedField::Token),
                            "totalFees" | "total_fees" => Ok(GeneratedField::TotalFees),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchFees;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.BatchFees")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchFees, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                let mut total_fees__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalFees => {
                            if total_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalFees"));
                            }
                            total_fees__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchFees {
                    token: token__.unwrap_or_default(),
                    total_fees: total_fees__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.BatchFees", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BridgeValidator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.power != 0 {
            len += 1;
        }
        if !self.ethereum_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.BridgeValidator", len)?;
        if self.power != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("power", ToString::to_string(&self.power).as_str())?;
        }
        if !self.ethereum_address.is_empty() {
            struct_ser.serialize_field("ethereumAddress", &self.ethereum_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BridgeValidator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["power", "ethereum_address", "ethereumAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Power,
            EthereumAddress,
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
                            "power" => Ok(GeneratedField::Power),
                            "ethereumAddress" | "ethereum_address" => {
                                Ok(GeneratedField::EthereumAddress)
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
            type Value = BridgeValidator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.BridgeValidator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BridgeValidator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut power__ = None;
                let mut ethereum_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Power => {
                            if power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("power"));
                            }
                            power__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EthereumAddress => {
                            if ethereum_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethereumAddress"));
                            }
                            ethereum_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BridgeValidator {
                    power: power__.unwrap_or_default(),
                    ethereum_address: ethereum_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.BridgeValidator",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ClaimType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "CLAIM_TYPE_UNKNOWN",
            Self::Deposit => "CLAIM_TYPE_DEPOSIT",
            Self::Withdraw => "CLAIM_TYPE_WITHDRAW",
            Self::Erc20Deployed => "CLAIM_TYPE_ERC20_DEPLOYED",
            Self::ValsetUpdated => "CLAIM_TYPE_VALSET_UPDATED",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ClaimType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CLAIM_TYPE_UNKNOWN",
            "CLAIM_TYPE_DEPOSIT",
            "CLAIM_TYPE_WITHDRAW",
            "CLAIM_TYPE_ERC20_DEPLOYED",
            "CLAIM_TYPE_VALSET_UPDATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClaimType;

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
                    "CLAIM_TYPE_UNKNOWN" => Ok(ClaimType::Unknown),
                    "CLAIM_TYPE_DEPOSIT" => Ok(ClaimType::Deposit),
                    "CLAIM_TYPE_WITHDRAW" => Ok(ClaimType::Withdraw),
                    "CLAIM_TYPE_ERC20_DEPLOYED" => Ok(ClaimType::Erc20Deployed),
                    "CLAIM_TYPE_VALSET_UPDATED" => Ok(ClaimType::ValsetUpdated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Erc20ToDenom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.erc20.is_empty() {
            len += 1;
        }
        if !self.denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.ERC20ToDenom", len)?;
        if !self.erc20.is_empty() {
            struct_ser.serialize_field("erc20", &self.erc20)?;
        }
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Erc20ToDenom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["erc20", "denom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Erc20,
            Denom,
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
                            "erc20" => Ok(GeneratedField::Erc20),
                            "denom" => Ok(GeneratedField::Denom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Erc20ToDenom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.ERC20ToDenom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Erc20ToDenom, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut erc20__ = None;
                let mut denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Erc20 => {
                            if erc20__.is_some() {
                                return Err(serde::de::Error::duplicate_field("erc20"));
                            }
                            erc20__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Erc20ToDenom {
                    erc20: erc20__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.ERC20ToDenom", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Erc20Token {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.ERC20Token", len)?;
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Erc20Token {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Contract,
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
                            "contract" => Ok(GeneratedField::Contract),
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
            type Value = Erc20Token;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.ERC20Token")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Erc20Token, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Erc20Token {
                    contract: contract__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.ERC20Token", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventAttestationObserved {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attestation_type != 0 {
            len += 1;
        }
        if !self.bridge_contract.is_empty() {
            len += 1;
        }
        if self.bridge_chain_id != 0 {
            len += 1;
        }
        if !self.attestation_id.is_empty() {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventAttestationObserved", len)?;
        if self.attestation_type != 0 {
            let v = ClaimType::try_from(self.attestation_type).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.attestation_type))
            })?;
            struct_ser.serialize_field("attestationType", &v)?;
        }
        if !self.bridge_contract.is_empty() {
            struct_ser.serialize_field("bridgeContract", &self.bridge_contract)?;
        }
        if self.bridge_chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "bridgeChainId",
                ToString::to_string(&self.bridge_chain_id).as_str(),
            )?;
        }
        if !self.attestation_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "attestationId",
                pbjson::private::base64::encode(&self.attestation_id).as_str(),
            )?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventAttestationObserved {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attestation_type",
            "attestationType",
            "bridge_contract",
            "bridgeContract",
            "bridge_chain_id",
            "bridgeChainId",
            "attestation_id",
            "attestationId",
            "nonce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttestationType,
            BridgeContract,
            BridgeChainId,
            AttestationId,
            Nonce,
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
                            "attestationType" | "attestation_type" => {
                                Ok(GeneratedField::AttestationType)
                            }
                            "bridgeContract" | "bridge_contract" => {
                                Ok(GeneratedField::BridgeContract)
                            }
                            "bridgeChainId" | "bridge_chain_id" => {
                                Ok(GeneratedField::BridgeChainId)
                            }
                            "attestationId" | "attestation_id" => Ok(GeneratedField::AttestationId),
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventAttestationObserved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventAttestationObserved")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventAttestationObserved, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attestation_type__ = None;
                let mut bridge_contract__ = None;
                let mut bridge_chain_id__ = None;
                let mut attestation_id__ = None;
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttestationType => {
                            if attestation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationType"));
                            }
                            attestation_type__ = Some(map_.next_value::<ClaimType>()? as i32);
                        }
                        GeneratedField::BridgeContract => {
                            if bridge_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeContract"));
                            }
                            bridge_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BridgeChainId => {
                            if bridge_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeChainId"));
                            }
                            bridge_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AttestationId => {
                            if attestation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationId"));
                            }
                            attestation_id__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EventAttestationObserved {
                    attestation_type: attestation_type__.unwrap_or_default(),
                    bridge_contract: bridge_contract__.unwrap_or_default(),
                    bridge_chain_id: bridge_chain_id__.unwrap_or_default(),
                    attestation_id: attestation_id__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventAttestationObserved",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventAttestationVote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if !self.attestation_id.is_empty() {
            len += 1;
        }
        if !self.voter.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventAttestationVote", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if !self.attestation_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "attestationId",
                pbjson::private::base64::encode(&self.attestation_id).as_str(),
            )?;
        }
        if !self.voter.is_empty() {
            struct_ser.serialize_field("voter", &self.voter)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventAttestationVote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "attestation_id",
            "attestationId",
            "voter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            AttestationId,
            Voter,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "attestationId" | "attestation_id" => Ok(GeneratedField::AttestationId),
                            "voter" => Ok(GeneratedField::Voter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventAttestationVote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventAttestationVote")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventAttestationVote, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut attestation_id__ = None;
                let mut voter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AttestationId => {
                            if attestation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationId"));
                            }
                            attestation_id__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Voter => {
                            if voter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voter"));
                            }
                            voter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventAttestationVote {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    attestation_id: attestation_id__.unwrap_or_default(),
                    voter: voter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventAttestationVote",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventBridgeWithdrawCanceled {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bridge_contract.is_empty() {
            len += 1;
        }
        if self.bridge_chain_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventBridgeWithdrawCanceled", len)?;
        if !self.bridge_contract.is_empty() {
            struct_ser.serialize_field("bridgeContract", &self.bridge_contract)?;
        }
        if self.bridge_chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "bridgeChainId",
                ToString::to_string(&self.bridge_chain_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventBridgeWithdrawCanceled {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bridge_contract",
            "bridgeContract",
            "bridge_chain_id",
            "bridgeChainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BridgeContract,
            BridgeChainId,
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
                            "bridgeContract" | "bridge_contract" => {
                                Ok(GeneratedField::BridgeContract)
                            }
                            "bridgeChainId" | "bridge_chain_id" => {
                                Ok(GeneratedField::BridgeChainId)
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
            type Value = EventBridgeWithdrawCanceled;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventBridgeWithdrawCanceled")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventBridgeWithdrawCanceled, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut bridge_contract__ = None;
                let mut bridge_chain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BridgeContract => {
                            if bridge_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeContract"));
                            }
                            bridge_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BridgeChainId => {
                            if bridge_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeChainId"));
                            }
                            bridge_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EventBridgeWithdrawCanceled {
                    bridge_contract: bridge_contract__.unwrap_or_default(),
                    bridge_chain_id: bridge_chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventBridgeWithdrawCanceled",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventCancelSendToEth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.outgoing_tx_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventCancelSendToEth", len)?;
        if self.outgoing_tx_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "outgoingTxId",
                ToString::to_string(&self.outgoing_tx_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventCancelSendToEth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["outgoing_tx_id", "outgoingTxId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OutgoingTxId,
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
                            "outgoingTxId" | "outgoing_tx_id" => Ok(GeneratedField::OutgoingTxId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventCancelSendToEth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventCancelSendToEth")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventCancelSendToEth, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut outgoing_tx_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OutgoingTxId => {
                            if outgoing_tx_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outgoingTxId"));
                            }
                            outgoing_tx_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EventCancelSendToEth {
                    outgoing_tx_id: outgoing_tx_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventCancelSendToEth",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventConfirmBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch_nonce != 0 {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventConfirmBatch", len)?;
        if self.batch_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "batchNonce",
                ToString::to_string(&self.batch_nonce).as_str(),
            )?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventConfirmBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_nonce",
            "batchNonce",
            "orchestrator_address",
            "orchestratorAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchNonce,
            OrchestratorAddress,
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
                            "batchNonce" | "batch_nonce" => Ok(GeneratedField::BatchNonce),
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = EventConfirmBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventConfirmBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventConfirmBatch, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut batch_nonce__ = None;
                let mut orchestrator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchNonce => {
                            if batch_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchNonce"));
                            }
                            batch_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventConfirmBatch {
                    batch_nonce: batch_nonce__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventConfirmBatch",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventDepositClaim {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if self.event_height != 0 {
            len += 1;
        }
        if !self.attestation_id.is_empty() {
            len += 1;
        }
        if !self.ethereum_sender.is_empty() {
            len += 1;
        }
        if !self.cosmos_receiver.is_empty() {
            len += 1;
        }
        if !self.token_contract.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventDepositClaim", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if self.event_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventHeight",
                ToString::to_string(&self.event_height).as_str(),
            )?;
        }
        if !self.attestation_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "attestationId",
                pbjson::private::base64::encode(&self.attestation_id).as_str(),
            )?;
        }
        if !self.ethereum_sender.is_empty() {
            struct_ser.serialize_field("ethereumSender", &self.ethereum_sender)?;
        }
        if !self.cosmos_receiver.is_empty() {
            struct_ser.serialize_field("cosmosReceiver", &self.cosmos_receiver)?;
        }
        if !self.token_contract.is_empty() {
            struct_ser.serialize_field("tokenContract", &self.token_contract)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventDepositClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "event_height",
            "eventHeight",
            "attestation_id",
            "attestationId",
            "ethereum_sender",
            "ethereumSender",
            "cosmos_receiver",
            "cosmosReceiver",
            "token_contract",
            "tokenContract",
            "amount",
            "orchestrator_address",
            "orchestratorAddress",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            EventHeight,
            AttestationId,
            EthereumSender,
            CosmosReceiver,
            TokenContract,
            Amount,
            OrchestratorAddress,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "eventHeight" | "event_height" => Ok(GeneratedField::EventHeight),
                            "attestationId" | "attestation_id" => Ok(GeneratedField::AttestationId),
                            "ethereumSender" | "ethereum_sender" => {
                                Ok(GeneratedField::EthereumSender)
                            }
                            "cosmosReceiver" | "cosmos_receiver" => {
                                Ok(GeneratedField::CosmosReceiver)
                            }
                            "tokenContract" | "token_contract" => Ok(GeneratedField::TokenContract),
                            "amount" => Ok(GeneratedField::Amount),
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = EventDepositClaim;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventDepositClaim")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventDepositClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut event_height__ = None;
                let mut attestation_id__ = None;
                let mut ethereum_sender__ = None;
                let mut cosmos_receiver__ = None;
                let mut token_contract__ = None;
                let mut amount__ = None;
                let mut orchestrator_address__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EventHeight => {
                            if event_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventHeight"));
                            }
                            event_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AttestationId => {
                            if attestation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationId"));
                            }
                            attestation_id__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EthereumSender => {
                            if ethereum_sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethereumSender"));
                            }
                            ethereum_sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CosmosReceiver => {
                            if cosmos_receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosReceiver"));
                            }
                            cosmos_receiver__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenContract => {
                            if token_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenContract"));
                            }
                            token_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventDepositClaim {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    event_height: event_height__.unwrap_or_default(),
                    attestation_id: attestation_id__.unwrap_or_default(),
                    ethereum_sender: ethereum_sender__.unwrap_or_default(),
                    cosmos_receiver: cosmos_receiver__.unwrap_or_default(),
                    token_contract: token_contract__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventDepositClaim",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventErc20DeployedClaim {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if self.event_height != 0 {
            len += 1;
        }
        if !self.attestation_id.is_empty() {
            len += 1;
        }
        if !self.cosmos_denom.is_empty() {
            len += 1;
        }
        if !self.token_contract.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.symbol.is_empty() {
            len += 1;
        }
        if self.decimals != 0 {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventERC20DeployedClaim", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if self.event_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventHeight",
                ToString::to_string(&self.event_height).as_str(),
            )?;
        }
        if !self.attestation_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "attestationId",
                pbjson::private::base64::encode(&self.attestation_id).as_str(),
            )?;
        }
        if !self.cosmos_denom.is_empty() {
            struct_ser.serialize_field("cosmosDenom", &self.cosmos_denom)?;
        }
        if !self.token_contract.is_empty() {
            struct_ser.serialize_field("tokenContract", &self.token_contract)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if self.decimals != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("decimals", ToString::to_string(&self.decimals).as_str())?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventErc20DeployedClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "event_height",
            "eventHeight",
            "attestation_id",
            "attestationId",
            "cosmos_denom",
            "cosmosDenom",
            "token_contract",
            "tokenContract",
            "name",
            "symbol",
            "decimals",
            "orchestrator_address",
            "orchestratorAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            EventHeight,
            AttestationId,
            CosmosDenom,
            TokenContract,
            Name,
            Symbol,
            Decimals,
            OrchestratorAddress,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "eventHeight" | "event_height" => Ok(GeneratedField::EventHeight),
                            "attestationId" | "attestation_id" => Ok(GeneratedField::AttestationId),
                            "cosmosDenom" | "cosmos_denom" => Ok(GeneratedField::CosmosDenom),
                            "tokenContract" | "token_contract" => Ok(GeneratedField::TokenContract),
                            "name" => Ok(GeneratedField::Name),
                            "symbol" => Ok(GeneratedField::Symbol),
                            "decimals" => Ok(GeneratedField::Decimals),
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = EventErc20DeployedClaim;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventERC20DeployedClaim")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventErc20DeployedClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut event_height__ = None;
                let mut attestation_id__ = None;
                let mut cosmos_denom__ = None;
                let mut token_contract__ = None;
                let mut name__ = None;
                let mut symbol__ = None;
                let mut decimals__ = None;
                let mut orchestrator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EventHeight => {
                            if event_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventHeight"));
                            }
                            event_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AttestationId => {
                            if attestation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationId"));
                            }
                            attestation_id__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CosmosDenom => {
                            if cosmos_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosDenom"));
                            }
                            cosmos_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenContract => {
                            if token_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenContract"));
                            }
                            token_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Decimals => {
                            if decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decimals"));
                            }
                            decimals__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventErc20DeployedClaim {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    event_height: event_height__.unwrap_or_default(),
                    attestation_id: attestation_id__.unwrap_or_default(),
                    cosmos_denom: cosmos_denom__.unwrap_or_default(),
                    token_contract: token_contract__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    symbol: symbol__.unwrap_or_default(),
                    decimals: decimals__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventERC20DeployedClaim",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventOutgoingBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        if self.batch_nonce != 0 {
            len += 1;
        }
        if self.batch_timeout != 0 {
            len += 1;
        }
        if !self.batch_tx_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventOutgoingBatch", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        if self.batch_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "batchNonce",
                ToString::to_string(&self.batch_nonce).as_str(),
            )?;
        }
        if self.batch_timeout != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "batchTimeout",
                ToString::to_string(&self.batch_timeout).as_str(),
            )?;
        }
        if !self.batch_tx_ids.is_empty() {
            struct_ser.serialize_field(
                "batchTxIds",
                &self
                    .batch_tx_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventOutgoingBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denom",
            "orchestrator_address",
            "orchestratorAddress",
            "batch_nonce",
            "batchNonce",
            "batch_timeout",
            "batchTimeout",
            "batch_tx_ids",
            "batchTxIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            OrchestratorAddress,
            BatchNonce,
            BatchTimeout,
            BatchTxIds,
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
                            "denom" => Ok(GeneratedField::Denom),
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
                            }
                            "batchNonce" | "batch_nonce" => Ok(GeneratedField::BatchNonce),
                            "batchTimeout" | "batch_timeout" => Ok(GeneratedField::BatchTimeout),
                            "batchTxIds" | "batch_tx_ids" => Ok(GeneratedField::BatchTxIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventOutgoingBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventOutgoingBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventOutgoingBatch, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut orchestrator_address__ = None;
                let mut batch_nonce__ = None;
                let mut batch_timeout__ = None;
                let mut batch_tx_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchNonce => {
                            if batch_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchNonce"));
                            }
                            batch_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BatchTimeout => {
                            if batch_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchTimeout"));
                            }
                            batch_timeout__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BatchTxIds => {
                            if batch_tx_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchTxIds"));
                            }
                            batch_tx_ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(EventOutgoingBatch {
                    denom: denom__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                    batch_nonce: batch_nonce__.unwrap_or_default(),
                    batch_timeout: batch_timeout__.unwrap_or_default(),
                    batch_tx_ids: batch_tx_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventOutgoingBatch",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventOutgoingBatchCanceled {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bridge_contract.is_empty() {
            len += 1;
        }
        if self.bridge_chain_id != 0 {
            len += 1;
        }
        if self.batch_id != 0 {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventOutgoingBatchCanceled", len)?;
        if !self.bridge_contract.is_empty() {
            struct_ser.serialize_field("bridgeContract", &self.bridge_contract)?;
        }
        if self.bridge_chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "bridgeChainId",
                ToString::to_string(&self.bridge_chain_id).as_str(),
            )?;
        }
        if self.batch_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("batchId", ToString::to_string(&self.batch_id).as_str())?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventOutgoingBatchCanceled {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bridge_contract",
            "bridgeContract",
            "bridge_chain_id",
            "bridgeChainId",
            "batch_id",
            "batchId",
            "nonce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BridgeContract,
            BridgeChainId,
            BatchId,
            Nonce,
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
                            "bridgeContract" | "bridge_contract" => {
                                Ok(GeneratedField::BridgeContract)
                            }
                            "bridgeChainId" | "bridge_chain_id" => {
                                Ok(GeneratedField::BridgeChainId)
                            }
                            "batchId" | "batch_id" => Ok(GeneratedField::BatchId),
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventOutgoingBatchCanceled;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventOutgoingBatchCanceled")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventOutgoingBatchCanceled, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut bridge_contract__ = None;
                let mut bridge_chain_id__ = None;
                let mut batch_id__ = None;
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BridgeContract => {
                            if bridge_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeContract"));
                            }
                            bridge_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BridgeChainId => {
                            if bridge_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeChainId"));
                            }
                            bridge_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BatchId => {
                            if batch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchId"));
                            }
                            batch_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(EventOutgoingBatchCanceled {
                    bridge_contract: bridge_contract__.unwrap_or_default(),
                    bridge_chain_id: bridge_chain_id__.unwrap_or_default(),
                    batch_id: batch_id__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventOutgoingBatchCanceled",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventSendToEth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.outgoing_tx_id != 0 {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.receiver.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.bridge_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventSendToEth", len)?;
        if self.outgoing_tx_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "outgoingTxId",
                ToString::to_string(&self.outgoing_tx_id).as_str(),
            )?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.receiver.is_empty() {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.bridge_fee.is_empty() {
            struct_ser.serialize_field("bridgeFee", &self.bridge_fee)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventSendToEth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "outgoing_tx_id",
            "outgoingTxId",
            "sender",
            "receiver",
            "amount",
            "bridge_fee",
            "bridgeFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OutgoingTxId,
            Sender,
            Receiver,
            Amount,
            BridgeFee,
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
                            "outgoingTxId" | "outgoing_tx_id" => Ok(GeneratedField::OutgoingTxId),
                            "sender" => Ok(GeneratedField::Sender),
                            "receiver" => Ok(GeneratedField::Receiver),
                            "amount" => Ok(GeneratedField::Amount),
                            "bridgeFee" | "bridge_fee" => Ok(GeneratedField::BridgeFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventSendToEth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventSendToEth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventSendToEth, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut outgoing_tx_id__ = None;
                let mut sender__ = None;
                let mut receiver__ = None;
                let mut amount__ = None;
                let mut bridge_fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OutgoingTxId => {
                            if outgoing_tx_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outgoingTxId"));
                            }
                            outgoing_tx_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BridgeFee => {
                            if bridge_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFee"));
                            }
                            bridge_fee__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventSendToEth {
                    outgoing_tx_id: outgoing_tx_id__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    bridge_fee: bridge_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventSendToEth",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventSetOrchestratorAddresses {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        if !self.operator_eth_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventSetOrchestratorAddresses", len)?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        if !self.operator_eth_address.is_empty() {
            struct_ser.serialize_field("operatorEthAddress", &self.operator_eth_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventSetOrchestratorAddresses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator_address",
            "validatorAddress",
            "orchestrator_address",
            "orchestratorAddress",
            "operator_eth_address",
            "operatorEthAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
            OrchestratorAddress,
            OperatorEthAddress,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
                            }
                            "operatorEthAddress" | "operator_eth_address" => {
                                Ok(GeneratedField::OperatorEthAddress)
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
            type Value = EventSetOrchestratorAddresses;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventSetOrchestratorAddresses")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventSetOrchestratorAddresses, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                let mut orchestrator_address__ = None;
                let mut operator_eth_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OperatorEthAddress => {
                            if operator_eth_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "operatorEthAddress",
                                ));
                            }
                            operator_eth_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventSetOrchestratorAddresses {
                    validator_address: validator_address__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                    operator_eth_address: operator_eth_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventSetOrchestratorAddresses",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventSubmitBadSignatureEvidence {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bad_eth_signature.is_empty() {
            len += 1;
        }
        if !self.bad_eth_signature_subject.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.peggy.v1.EventSubmitBadSignatureEvidence", len)?;
        if !self.bad_eth_signature.is_empty() {
            struct_ser.serialize_field("badEthSignature", &self.bad_eth_signature)?;
        }
        if !self.bad_eth_signature_subject.is_empty() {
            struct_ser
                .serialize_field("badEthSignatureSubject", &self.bad_eth_signature_subject)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventSubmitBadSignatureEvidence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bad_eth_signature",
            "badEthSignature",
            "bad_eth_signature_subject",
            "badEthSignatureSubject",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BadEthSignature,
            BadEthSignatureSubject,
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
                            "badEthSignature" | "bad_eth_signature" => {
                                Ok(GeneratedField::BadEthSignature)
                            }
                            "badEthSignatureSubject" | "bad_eth_signature_subject" => {
                                Ok(GeneratedField::BadEthSignatureSubject)
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
            type Value = EventSubmitBadSignatureEvidence;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventSubmitBadSignatureEvidence")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventSubmitBadSignatureEvidence, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut bad_eth_signature__ = None;
                let mut bad_eth_signature_subject__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BadEthSignature => {
                            if bad_eth_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("badEthSignature"));
                            }
                            bad_eth_signature__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BadEthSignatureSubject => {
                            if bad_eth_signature_subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "badEthSignatureSubject",
                                ));
                            }
                            bad_eth_signature_subject__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventSubmitBadSignatureEvidence {
                    bad_eth_signature: bad_eth_signature__.unwrap_or_default(),
                    bad_eth_signature_subject: bad_eth_signature_subject__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventSubmitBadSignatureEvidence",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventValidatorSlash {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.power != 0 {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if !self.consensus_address.is_empty() {
            len += 1;
        }
        if !self.operator_address.is_empty() {
            len += 1;
        }
        if !self.moniker.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventValidatorSlash", len)?;
        if self.power != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("power", ToString::to_string(&self.power).as_str())?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if !self.consensus_address.is_empty() {
            struct_ser.serialize_field("consensusAddress", &self.consensus_address)?;
        }
        if !self.operator_address.is_empty() {
            struct_ser.serialize_field("operatorAddress", &self.operator_address)?;
        }
        if !self.moniker.is_empty() {
            struct_ser.serialize_field("moniker", &self.moniker)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventValidatorSlash {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "power",
            "reason",
            "consensus_address",
            "consensusAddress",
            "operator_address",
            "operatorAddress",
            "moniker",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Power,
            Reason,
            ConsensusAddress,
            OperatorAddress,
            Moniker,
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
                            "power" => Ok(GeneratedField::Power),
                            "reason" => Ok(GeneratedField::Reason),
                            "consensusAddress" | "consensus_address" => {
                                Ok(GeneratedField::ConsensusAddress)
                            }
                            "operatorAddress" | "operator_address" => {
                                Ok(GeneratedField::OperatorAddress)
                            }
                            "moniker" => Ok(GeneratedField::Moniker),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventValidatorSlash;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventValidatorSlash")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventValidatorSlash, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut power__ = None;
                let mut reason__ = None;
                let mut consensus_address__ = None;
                let mut operator_address__ = None;
                let mut moniker__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Power => {
                            if power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("power"));
                            }
                            power__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsensusAddress => {
                            if consensus_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusAddress"));
                            }
                            consensus_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OperatorAddress => {
                            if operator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operatorAddress"));
                            }
                            operator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Moniker => {
                            if moniker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moniker"));
                            }
                            moniker__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventValidatorSlash {
                    power: power__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    consensus_address: consensus_address__.unwrap_or_default(),
                    operator_address: operator_address__.unwrap_or_default(),
                    moniker: moniker__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventValidatorSlash",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventValsetConfirm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.valset_nonce != 0 {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventValsetConfirm", len)?;
        if self.valset_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "valsetNonce",
                ToString::to_string(&self.valset_nonce).as_str(),
            )?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventValsetConfirm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "valset_nonce",
            "valsetNonce",
            "orchestrator_address",
            "orchestratorAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValsetNonce,
            OrchestratorAddress,
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
                            "valsetNonce" | "valset_nonce" => Ok(GeneratedField::ValsetNonce),
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = EventValsetConfirm;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventValsetConfirm")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventValsetConfirm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut valset_nonce__ = None;
                let mut orchestrator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValsetNonce => {
                            if valset_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetNonce"));
                            }
                            valset_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventValsetConfirm {
                    valset_nonce: valset_nonce__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventValsetConfirm",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventValsetUpdateClaim {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if self.event_height != 0 {
            len += 1;
        }
        if !self.attestation_id.is_empty() {
            len += 1;
        }
        if self.valset_nonce != 0 {
            len += 1;
        }
        if !self.valset_members.is_empty() {
            len += 1;
        }
        if !self.reward_amount.is_empty() {
            len += 1;
        }
        if !self.reward_token.is_empty() {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventValsetUpdateClaim", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if self.event_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventHeight",
                ToString::to_string(&self.event_height).as_str(),
            )?;
        }
        if !self.attestation_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "attestationId",
                pbjson::private::base64::encode(&self.attestation_id).as_str(),
            )?;
        }
        if self.valset_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "valsetNonce",
                ToString::to_string(&self.valset_nonce).as_str(),
            )?;
        }
        if !self.valset_members.is_empty() {
            struct_ser.serialize_field("valsetMembers", &self.valset_members)?;
        }
        if !self.reward_amount.is_empty() {
            struct_ser.serialize_field("rewardAmount", &self.reward_amount)?;
        }
        if !self.reward_token.is_empty() {
            struct_ser.serialize_field("rewardToken", &self.reward_token)?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventValsetUpdateClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "event_height",
            "eventHeight",
            "attestation_id",
            "attestationId",
            "valset_nonce",
            "valsetNonce",
            "valset_members",
            "valsetMembers",
            "reward_amount",
            "rewardAmount",
            "reward_token",
            "rewardToken",
            "orchestrator_address",
            "orchestratorAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            EventHeight,
            AttestationId,
            ValsetNonce,
            ValsetMembers,
            RewardAmount,
            RewardToken,
            OrchestratorAddress,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "eventHeight" | "event_height" => Ok(GeneratedField::EventHeight),
                            "attestationId" | "attestation_id" => Ok(GeneratedField::AttestationId),
                            "valsetNonce" | "valset_nonce" => Ok(GeneratedField::ValsetNonce),
                            "valsetMembers" | "valset_members" => Ok(GeneratedField::ValsetMembers),
                            "rewardAmount" | "reward_amount" => Ok(GeneratedField::RewardAmount),
                            "rewardToken" | "reward_token" => Ok(GeneratedField::RewardToken),
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = EventValsetUpdateClaim;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventValsetUpdateClaim")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventValsetUpdateClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut event_height__ = None;
                let mut attestation_id__ = None;
                let mut valset_nonce__ = None;
                let mut valset_members__ = None;
                let mut reward_amount__ = None;
                let mut reward_token__ = None;
                let mut orchestrator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EventHeight => {
                            if event_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventHeight"));
                            }
                            event_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AttestationId => {
                            if attestation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationId"));
                            }
                            attestation_id__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValsetNonce => {
                            if valset_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetNonce"));
                            }
                            valset_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValsetMembers => {
                            if valset_members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetMembers"));
                            }
                            valset_members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RewardAmount => {
                            if reward_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAmount"));
                            }
                            reward_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RewardToken => {
                            if reward_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardToken"));
                            }
                            reward_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventValsetUpdateClaim {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    event_height: event_height__.unwrap_or_default(),
                    attestation_id: attestation_id__.unwrap_or_default(),
                    valset_nonce: valset_nonce__.unwrap_or_default(),
                    valset_members: valset_members__.unwrap_or_default(),
                    reward_amount: reward_amount__.unwrap_or_default(),
                    reward_token: reward_token__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventValsetUpdateClaim",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventValsetUpdateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.valset_nonce != 0 {
            len += 1;
        }
        if self.valset_height != 0 {
            len += 1;
        }
        if !self.valset_members.is_empty() {
            len += 1;
        }
        if !self.reward_amount.is_empty() {
            len += 1;
        }
        if !self.reward_token.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventValsetUpdateRequest", len)?;
        if self.valset_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "valsetNonce",
                ToString::to_string(&self.valset_nonce).as_str(),
            )?;
        }
        if self.valset_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "valsetHeight",
                ToString::to_string(&self.valset_height).as_str(),
            )?;
        }
        if !self.valset_members.is_empty() {
            struct_ser.serialize_field("valsetMembers", &self.valset_members)?;
        }
        if !self.reward_amount.is_empty() {
            struct_ser.serialize_field("rewardAmount", &self.reward_amount)?;
        }
        if !self.reward_token.is_empty() {
            struct_ser.serialize_field("rewardToken", &self.reward_token)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventValsetUpdateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "valset_nonce",
            "valsetNonce",
            "valset_height",
            "valsetHeight",
            "valset_members",
            "valsetMembers",
            "reward_amount",
            "rewardAmount",
            "reward_token",
            "rewardToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValsetNonce,
            ValsetHeight,
            ValsetMembers,
            RewardAmount,
            RewardToken,
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
                            "valsetNonce" | "valset_nonce" => Ok(GeneratedField::ValsetNonce),
                            "valsetHeight" | "valset_height" => Ok(GeneratedField::ValsetHeight),
                            "valsetMembers" | "valset_members" => Ok(GeneratedField::ValsetMembers),
                            "rewardAmount" | "reward_amount" => Ok(GeneratedField::RewardAmount),
                            "rewardToken" | "reward_token" => Ok(GeneratedField::RewardToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventValsetUpdateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventValsetUpdateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventValsetUpdateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut valset_nonce__ = None;
                let mut valset_height__ = None;
                let mut valset_members__ = None;
                let mut reward_amount__ = None;
                let mut reward_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValsetNonce => {
                            if valset_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetNonce"));
                            }
                            valset_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValsetHeight => {
                            if valset_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetHeight"));
                            }
                            valset_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValsetMembers => {
                            if valset_members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetMembers"));
                            }
                            valset_members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RewardAmount => {
                            if reward_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAmount"));
                            }
                            reward_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RewardToken => {
                            if reward_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardToken"));
                            }
                            reward_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventValsetUpdateRequest {
                    valset_nonce: valset_nonce__.unwrap_or_default(),
                    valset_height: valset_height__.unwrap_or_default(),
                    valset_members: valset_members__.unwrap_or_default(),
                    reward_amount: reward_amount__.unwrap_or_default(),
                    reward_token: reward_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventValsetUpdateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventWithdrawClaim {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if self.event_height != 0 {
            len += 1;
        }
        if !self.attestation_id.is_empty() {
            len += 1;
        }
        if self.batch_nonce != 0 {
            len += 1;
        }
        if !self.token_contract.is_empty() {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.EventWithdrawClaim", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if self.event_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventHeight",
                ToString::to_string(&self.event_height).as_str(),
            )?;
        }
        if !self.attestation_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "attestationId",
                pbjson::private::base64::encode(&self.attestation_id).as_str(),
            )?;
        }
        if self.batch_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "batchNonce",
                ToString::to_string(&self.batch_nonce).as_str(),
            )?;
        }
        if !self.token_contract.is_empty() {
            struct_ser.serialize_field("tokenContract", &self.token_contract)?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventWithdrawClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "event_height",
            "eventHeight",
            "attestation_id",
            "attestationId",
            "batch_nonce",
            "batchNonce",
            "token_contract",
            "tokenContract",
            "orchestrator_address",
            "orchestratorAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            EventHeight,
            AttestationId,
            BatchNonce,
            TokenContract,
            OrchestratorAddress,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "eventHeight" | "event_height" => Ok(GeneratedField::EventHeight),
                            "attestationId" | "attestation_id" => Ok(GeneratedField::AttestationId),
                            "batchNonce" | "batch_nonce" => Ok(GeneratedField::BatchNonce),
                            "tokenContract" | "token_contract" => Ok(GeneratedField::TokenContract),
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = EventWithdrawClaim;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.EventWithdrawClaim")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventWithdrawClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut event_height__ = None;
                let mut attestation_id__ = None;
                let mut batch_nonce__ = None;
                let mut token_contract__ = None;
                let mut orchestrator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EventHeight => {
                            if event_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventHeight"));
                            }
                            event_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AttestationId => {
                            if attestation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationId"));
                            }
                            attestation_id__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BatchNonce => {
                            if batch_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchNonce"));
                            }
                            batch_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TokenContract => {
                            if token_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenContract"));
                            }
                            token_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventWithdrawClaim {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    event_height: event_height__.unwrap_or_default(),
                    attestation_id: attestation_id__.unwrap_or_default(),
                    batch_nonce: batch_nonce__.unwrap_or_default(),
                    token_contract: token_contract__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.EventWithdrawClaim",
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
        if self.last_observed_nonce != 0 {
            len += 1;
        }
        if !self.valsets.is_empty() {
            len += 1;
        }
        if !self.valset_confirms.is_empty() {
            len += 1;
        }
        if !self.batches.is_empty() {
            len += 1;
        }
        if !self.batch_confirms.is_empty() {
            len += 1;
        }
        if !self.attestations.is_empty() {
            len += 1;
        }
        if !self.orchestrator_addresses.is_empty() {
            len += 1;
        }
        if !self.erc20_to_denoms.is_empty() {
            len += 1;
        }
        if !self.unbatched_transfers.is_empty() {
            len += 1;
        }
        if self.last_observed_ethereum_height != 0 {
            len += 1;
        }
        if self.last_outgoing_batch_id != 0 {
            len += 1;
        }
        if self.last_outgoing_pool_id != 0 {
            len += 1;
        }
        if self.last_observed_valset.is_some() {
            len += 1;
        }
        if !self.ethereum_blacklist.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if self.last_observed_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "lastObservedNonce",
                ToString::to_string(&self.last_observed_nonce).as_str(),
            )?;
        }
        if !self.valsets.is_empty() {
            struct_ser.serialize_field("valsets", &self.valsets)?;
        }
        if !self.valset_confirms.is_empty() {
            struct_ser.serialize_field("valsetConfirms", &self.valset_confirms)?;
        }
        if !self.batches.is_empty() {
            struct_ser.serialize_field("batches", &self.batches)?;
        }
        if !self.batch_confirms.is_empty() {
            struct_ser.serialize_field("batchConfirms", &self.batch_confirms)?;
        }
        if !self.attestations.is_empty() {
            struct_ser.serialize_field("attestations", &self.attestations)?;
        }
        if !self.orchestrator_addresses.is_empty() {
            struct_ser.serialize_field("orchestratorAddresses", &self.orchestrator_addresses)?;
        }
        if !self.erc20_to_denoms.is_empty() {
            struct_ser.serialize_field("erc20ToDenoms", &self.erc20_to_denoms)?;
        }
        if !self.unbatched_transfers.is_empty() {
            struct_ser.serialize_field("unbatchedTransfers", &self.unbatched_transfers)?;
        }
        if self.last_observed_ethereum_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "lastObservedEthereumHeight",
                ToString::to_string(&self.last_observed_ethereum_height).as_str(),
            )?;
        }
        if self.last_outgoing_batch_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "lastOutgoingBatchId",
                ToString::to_string(&self.last_outgoing_batch_id).as_str(),
            )?;
        }
        if self.last_outgoing_pool_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "lastOutgoingPoolId",
                ToString::to_string(&self.last_outgoing_pool_id).as_str(),
            )?;
        }
        if let Some(v) = self.last_observed_valset.as_ref() {
            struct_ser.serialize_field("lastObservedValset", v)?;
        }
        if !self.ethereum_blacklist.is_empty() {
            struct_ser.serialize_field("ethereumBlacklist", &self.ethereum_blacklist)?;
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
            "last_observed_nonce",
            "lastObservedNonce",
            "valsets",
            "valset_confirms",
            "valsetConfirms",
            "batches",
            "batch_confirms",
            "batchConfirms",
            "attestations",
            "orchestrator_addresses",
            "orchestratorAddresses",
            "erc20_to_denoms",
            "erc20ToDenoms",
            "unbatched_transfers",
            "unbatchedTransfers",
            "last_observed_ethereum_height",
            "lastObservedEthereumHeight",
            "last_outgoing_batch_id",
            "lastOutgoingBatchId",
            "last_outgoing_pool_id",
            "lastOutgoingPoolId",
            "last_observed_valset",
            "lastObservedValset",
            "ethereum_blacklist",
            "ethereumBlacklist",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            LastObservedNonce,
            Valsets,
            ValsetConfirms,
            Batches,
            BatchConfirms,
            Attestations,
            OrchestratorAddresses,
            Erc20ToDenoms,
            UnbatchedTransfers,
            LastObservedEthereumHeight,
            LastOutgoingBatchId,
            LastOutgoingPoolId,
            LastObservedValset,
            EthereumBlacklist,
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
                            "lastObservedNonce" | "last_observed_nonce" => {
                                Ok(GeneratedField::LastObservedNonce)
                            }
                            "valsets" => Ok(GeneratedField::Valsets),
                            "valsetConfirms" | "valset_confirms" => {
                                Ok(GeneratedField::ValsetConfirms)
                            }
                            "batches" => Ok(GeneratedField::Batches),
                            "batchConfirms" | "batch_confirms" => Ok(GeneratedField::BatchConfirms),
                            "attestations" => Ok(GeneratedField::Attestations),
                            "orchestratorAddresses" | "orchestrator_addresses" => {
                                Ok(GeneratedField::OrchestratorAddresses)
                            }
                            "erc20ToDenoms" | "erc20_to_denoms" => {
                                Ok(GeneratedField::Erc20ToDenoms)
                            }
                            "unbatchedTransfers" | "unbatched_transfers" => {
                                Ok(GeneratedField::UnbatchedTransfers)
                            }
                            "lastObservedEthereumHeight" | "last_observed_ethereum_height" => {
                                Ok(GeneratedField::LastObservedEthereumHeight)
                            }
                            "lastOutgoingBatchId" | "last_outgoing_batch_id" => {
                                Ok(GeneratedField::LastOutgoingBatchId)
                            }
                            "lastOutgoingPoolId" | "last_outgoing_pool_id" => {
                                Ok(GeneratedField::LastOutgoingPoolId)
                            }
                            "lastObservedValset" | "last_observed_valset" => {
                                Ok(GeneratedField::LastObservedValset)
                            }
                            "ethereumBlacklist" | "ethereum_blacklist" => {
                                Ok(GeneratedField::EthereumBlacklist)
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
                formatter.write_str("struct injective.peggy.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut last_observed_nonce__ = None;
                let mut valsets__ = None;
                let mut valset_confirms__ = None;
                let mut batches__ = None;
                let mut batch_confirms__ = None;
                let mut attestations__ = None;
                let mut orchestrator_addresses__ = None;
                let mut erc20_to_denoms__ = None;
                let mut unbatched_transfers__ = None;
                let mut last_observed_ethereum_height__ = None;
                let mut last_outgoing_batch_id__ = None;
                let mut last_outgoing_pool_id__ = None;
                let mut last_observed_valset__ = None;
                let mut ethereum_blacklist__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::LastObservedNonce => {
                            if last_observed_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastObservedNonce"));
                            }
                            last_observed_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Valsets => {
                            if valsets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsets"));
                            }
                            valsets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValsetConfirms => {
                            if valset_confirms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetConfirms"));
                            }
                            valset_confirms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Batches => {
                            if batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            batches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchConfirms => {
                            if batch_confirms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchConfirms"));
                            }
                            batch_confirms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Attestations => {
                            if attestations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestations"));
                            }
                            attestations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrchestratorAddresses => {
                            if orchestrator_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddresses",
                                ));
                            }
                            orchestrator_addresses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Erc20ToDenoms => {
                            if erc20_to_denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("erc20ToDenoms"));
                            }
                            erc20_to_denoms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnbatchedTransfers => {
                            if unbatched_transfers__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unbatchedTransfers",
                                ));
                            }
                            unbatched_transfers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastObservedEthereumHeight => {
                            if last_observed_ethereum_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "lastObservedEthereumHeight",
                                ));
                            }
                            last_observed_ethereum_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LastOutgoingBatchId => {
                            if last_outgoing_batch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "lastOutgoingBatchId",
                                ));
                            }
                            last_outgoing_batch_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LastOutgoingPoolId => {
                            if last_outgoing_pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "lastOutgoingPoolId",
                                ));
                            }
                            last_outgoing_pool_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LastObservedValset => {
                            if last_observed_valset__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "lastObservedValset",
                                ));
                            }
                            last_observed_valset__ = map_.next_value()?;
                        }
                        GeneratedField::EthereumBlacklist => {
                            if ethereum_blacklist__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethereumBlacklist"));
                            }
                            ethereum_blacklist__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    last_observed_nonce: last_observed_nonce__.unwrap_or_default(),
                    valsets: valsets__.unwrap_or_default(),
                    valset_confirms: valset_confirms__.unwrap_or_default(),
                    batches: batches__.unwrap_or_default(),
                    batch_confirms: batch_confirms__.unwrap_or_default(),
                    attestations: attestations__.unwrap_or_default(),
                    orchestrator_addresses: orchestrator_addresses__.unwrap_or_default(),
                    erc20_to_denoms: erc20_to_denoms__.unwrap_or_default(),
                    unbatched_transfers: unbatched_transfers__.unwrap_or_default(),
                    last_observed_ethereum_height: last_observed_ethereum_height__
                        .unwrap_or_default(),
                    last_outgoing_batch_id: last_outgoing_batch_id__.unwrap_or_default(),
                    last_outgoing_pool_id: last_outgoing_pool_id__.unwrap_or_default(),
                    last_observed_valset: last_observed_valset__,
                    ethereum_blacklist: ethereum_blacklist__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for IdSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.IDSet", len)?;
        if !self.ids.is_empty() {
            struct_ser.serialize_field(
                "ids",
                &self.ids.iter().map(ToString::to_string).collect::<Vec<_>>(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for IdSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ids"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ids,
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
                            "ids" => Ok(GeneratedField::Ids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.IDSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdSet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ids => {
                            if ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            ids__ = Some(
                                map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                    }
                }
                Ok(IdSet {
                    ids: ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.IDSet", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LastClaimEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ethereum_event_nonce != 0 {
            len += 1;
        }
        if self.ethereum_event_height != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.LastClaimEvent", len)?;
        if self.ethereum_event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "ethereumEventNonce",
                ToString::to_string(&self.ethereum_event_nonce).as_str(),
            )?;
        }
        if self.ethereum_event_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "ethereumEventHeight",
                ToString::to_string(&self.ethereum_event_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LastClaimEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ethereum_event_nonce",
            "ethereumEventNonce",
            "ethereum_event_height",
            "ethereumEventHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EthereumEventNonce,
            EthereumEventHeight,
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
                            "ethereumEventNonce" | "ethereum_event_nonce" => {
                                Ok(GeneratedField::EthereumEventNonce)
                            }
                            "ethereumEventHeight" | "ethereum_event_height" => {
                                Ok(GeneratedField::EthereumEventHeight)
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
            type Value = LastClaimEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.LastClaimEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LastClaimEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ethereum_event_nonce__ = None;
                let mut ethereum_event_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EthereumEventNonce => {
                            if ethereum_event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ethereumEventNonce",
                                ));
                            }
                            ethereum_event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EthereumEventHeight => {
                            if ethereum_event_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ethereumEventHeight",
                                ));
                            }
                            ethereum_event_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(LastClaimEvent {
                    ethereum_event_nonce: ethereum_event_nonce__.unwrap_or_default(),
                    ethereum_event_height: ethereum_event_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.LastClaimEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LastObservedEthereumBlockHeight {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cosmos_block_height != 0 {
            len += 1;
        }
        if self.ethereum_block_height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.peggy.v1.LastObservedEthereumBlockHeight", len)?;
        if self.cosmos_block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "cosmosBlockHeight",
                ToString::to_string(&self.cosmos_block_height).as_str(),
            )?;
        }
        if self.ethereum_block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "ethereumBlockHeight",
                ToString::to_string(&self.ethereum_block_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LastObservedEthereumBlockHeight {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cosmos_block_height",
            "cosmosBlockHeight",
            "ethereum_block_height",
            "ethereumBlockHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CosmosBlockHeight,
            EthereumBlockHeight,
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
                            "cosmosBlockHeight" | "cosmos_block_height" => {
                                Ok(GeneratedField::CosmosBlockHeight)
                            }
                            "ethereumBlockHeight" | "ethereum_block_height" => {
                                Ok(GeneratedField::EthereumBlockHeight)
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
            type Value = LastObservedEthereumBlockHeight;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.LastObservedEthereumBlockHeight")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<LastObservedEthereumBlockHeight, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cosmos_block_height__ = None;
                let mut ethereum_block_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CosmosBlockHeight => {
                            if cosmos_block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosBlockHeight"));
                            }
                            cosmos_block_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EthereumBlockHeight => {
                            if ethereum_block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ethereumBlockHeight",
                                ));
                            }
                            ethereum_block_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(LastObservedEthereumBlockHeight {
                    cosmos_block_height: cosmos_block_height__.unwrap_or_default(),
                    ethereum_block_height: ethereum_block_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.LastObservedEthereumBlockHeight",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MissingNoncesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MissingNoncesRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MissingNoncesRequest {
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
            type Value = MissingNoncesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MissingNoncesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MissingNoncesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MissingNoncesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MissingNoncesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MissingNoncesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operator_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MissingNoncesResponse", len)?;
        if !self.operator_addresses.is_empty() {
            struct_ser.serialize_field("operatorAddresses", &self.operator_addresses)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MissingNoncesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["operator_addresses", "operatorAddresses"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperatorAddresses,
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
                            "operatorAddresses" | "operator_addresses" => {
                                Ok(GeneratedField::OperatorAddresses)
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
            type Value = MissingNoncesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MissingNoncesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MissingNoncesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut operator_addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OperatorAddresses => {
                            if operator_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operatorAddresses"));
                            }
                            operator_addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MissingNoncesResponse {
                    operator_addresses: operator_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MissingNoncesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgBlacklistEthereumAddresses {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signer.is_empty() {
            len += 1;
        }
        if !self.blacklist_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgBlacklistEthereumAddresses", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.blacklist_addresses.is_empty() {
            struct_ser.serialize_field("blacklistAddresses", &self.blacklist_addresses)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgBlacklistEthereumAddresses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "blacklist_addresses", "blacklistAddresses"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            BlacklistAddresses,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "blacklistAddresses" | "blacklist_addresses" => {
                                Ok(GeneratedField::BlacklistAddresses)
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
            type Value = MsgBlacklistEthereumAddresses;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgBlacklistEthereumAddresses")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgBlacklistEthereumAddresses, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut blacklist_addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlacklistAddresses => {
                            if blacklist_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "blacklistAddresses",
                                ));
                            }
                            blacklist_addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgBlacklistEthereumAddresses {
                    signer: signer__.unwrap_or_default(),
                    blacklist_addresses: blacklist_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgBlacklistEthereumAddresses",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgBlacklistEthereumAddressesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.MsgBlacklistEthereumAddressesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgBlacklistEthereumAddressesResponse {
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
            type Value = MsgBlacklistEthereumAddressesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.peggy.v1.MsgBlacklistEthereumAddressesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgBlacklistEthereumAddressesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgBlacklistEthereumAddressesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgBlacklistEthereumAddressesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCancelSendToEth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transaction_id != 0 {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgCancelSendToEth", len)?;
        if self.transaction_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "transactionId",
                ToString::to_string(&self.transaction_id).as_str(),
            )?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCancelSendToEth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["transaction_id", "transactionId", "sender"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionId,
            Sender,
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
                            "transactionId" | "transaction_id" => Ok(GeneratedField::TransactionId),
                            "sender" => Ok(GeneratedField::Sender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCancelSendToEth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgCancelSendToEth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCancelSendToEth, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut transaction_id__ = None;
                let mut sender__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransactionId => {
                            if transaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionId"));
                            }
                            transaction_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCancelSendToEth {
                    transaction_id: transaction_id__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgCancelSendToEth",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCancelSendToEthResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgCancelSendToEthResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCancelSendToEthResponse {
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
            type Value = MsgCancelSendToEthResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgCancelSendToEthResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgCancelSendToEthResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCancelSendToEthResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgCancelSendToEthResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgConfirmBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        if !self.token_contract.is_empty() {
            len += 1;
        }
        if !self.eth_signer.is_empty() {
            len += 1;
        }
        if !self.orchestrator.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgConfirmBatch", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.token_contract.is_empty() {
            struct_ser.serialize_field("tokenContract", &self.token_contract)?;
        }
        if !self.eth_signer.is_empty() {
            struct_ser.serialize_field("ethSigner", &self.eth_signer)?;
        }
        if !self.orchestrator.is_empty() {
            struct_ser.serialize_field("orchestrator", &self.orchestrator)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgConfirmBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nonce",
            "token_contract",
            "tokenContract",
            "eth_signer",
            "ethSigner",
            "orchestrator",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
            TokenContract,
            EthSigner,
            Orchestrator,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            "tokenContract" | "token_contract" => Ok(GeneratedField::TokenContract),
                            "ethSigner" | "eth_signer" => Ok(GeneratedField::EthSigner),
                            "orchestrator" => Ok(GeneratedField::Orchestrator),
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
            type Value = MsgConfirmBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgConfirmBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgConfirmBatch, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                let mut token_contract__ = None;
                let mut eth_signer__ = None;
                let mut orchestrator__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TokenContract => {
                            if token_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenContract"));
                            }
                            token_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EthSigner => {
                            if eth_signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethSigner"));
                            }
                            eth_signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Orchestrator => {
                            if orchestrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orchestrator"));
                            }
                            orchestrator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgConfirmBatch {
                    nonce: nonce__.unwrap_or_default(),
                    token_contract: token_contract__.unwrap_or_default(),
                    eth_signer: eth_signer__.unwrap_or_default(),
                    orchestrator: orchestrator__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgConfirmBatch",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgConfirmBatchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgConfirmBatchResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgConfirmBatchResponse {
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
            type Value = MsgConfirmBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgConfirmBatchResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgConfirmBatchResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgConfirmBatchResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgConfirmBatchResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgDepositClaim {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if self.block_height != 0 {
            len += 1;
        }
        if !self.token_contract.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.ethereum_sender.is_empty() {
            len += 1;
        }
        if !self.cosmos_receiver.is_empty() {
            len += 1;
        }
        if !self.orchestrator.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgDepositClaim", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        if !self.token_contract.is_empty() {
            struct_ser.serialize_field("tokenContract", &self.token_contract)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.ethereum_sender.is_empty() {
            struct_ser.serialize_field("ethereumSender", &self.ethereum_sender)?;
        }
        if !self.cosmos_receiver.is_empty() {
            struct_ser.serialize_field("cosmosReceiver", &self.cosmos_receiver)?;
        }
        if !self.orchestrator.is_empty() {
            struct_ser.serialize_field("orchestrator", &self.orchestrator)?;
        }
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgDepositClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "block_height",
            "blockHeight",
            "token_contract",
            "tokenContract",
            "amount",
            "ethereum_sender",
            "ethereumSender",
            "cosmos_receiver",
            "cosmosReceiver",
            "orchestrator",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            BlockHeight,
            TokenContract,
            Amount,
            EthereumSender,
            CosmosReceiver,
            Orchestrator,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "tokenContract" | "token_contract" => Ok(GeneratedField::TokenContract),
                            "amount" => Ok(GeneratedField::Amount),
                            "ethereumSender" | "ethereum_sender" => {
                                Ok(GeneratedField::EthereumSender)
                            }
                            "cosmosReceiver" | "cosmos_receiver" => {
                                Ok(GeneratedField::CosmosReceiver)
                            }
                            "orchestrator" => Ok(GeneratedField::Orchestrator),
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
            type Value = MsgDepositClaim;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgDepositClaim")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgDepositClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut block_height__ = None;
                let mut token_contract__ = None;
                let mut amount__ = None;
                let mut ethereum_sender__ = None;
                let mut cosmos_receiver__ = None;
                let mut orchestrator__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TokenContract => {
                            if token_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenContract"));
                            }
                            token_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EthereumSender => {
                            if ethereum_sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethereumSender"));
                            }
                            ethereum_sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CosmosReceiver => {
                            if cosmos_receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosReceiver"));
                            }
                            cosmos_receiver__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Orchestrator => {
                            if orchestrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orchestrator"));
                            }
                            orchestrator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgDepositClaim {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    block_height: block_height__.unwrap_or_default(),
                    token_contract: token_contract__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    ethereum_sender: ethereum_sender__.unwrap_or_default(),
                    cosmos_receiver: cosmos_receiver__.unwrap_or_default(),
                    orchestrator: orchestrator__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgDepositClaim",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgDepositClaimResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgDepositClaimResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgDepositClaimResponse {
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
            type Value = MsgDepositClaimResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgDepositClaimResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgDepositClaimResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDepositClaimResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgDepositClaimResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgErc20DeployedClaim {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if self.block_height != 0 {
            len += 1;
        }
        if !self.cosmos_denom.is_empty() {
            len += 1;
        }
        if !self.token_contract.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.symbol.is_empty() {
            len += 1;
        }
        if self.decimals != 0 {
            len += 1;
        }
        if !self.orchestrator.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgERC20DeployedClaim", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        if !self.cosmos_denom.is_empty() {
            struct_ser.serialize_field("cosmosDenom", &self.cosmos_denom)?;
        }
        if !self.token_contract.is_empty() {
            struct_ser.serialize_field("tokenContract", &self.token_contract)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if self.decimals != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("decimals", ToString::to_string(&self.decimals).as_str())?;
        }
        if !self.orchestrator.is_empty() {
            struct_ser.serialize_field("orchestrator", &self.orchestrator)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgErc20DeployedClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "block_height",
            "blockHeight",
            "cosmos_denom",
            "cosmosDenom",
            "token_contract",
            "tokenContract",
            "name",
            "symbol",
            "decimals",
            "orchestrator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            BlockHeight,
            CosmosDenom,
            TokenContract,
            Name,
            Symbol,
            Decimals,
            Orchestrator,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "cosmosDenom" | "cosmos_denom" => Ok(GeneratedField::CosmosDenom),
                            "tokenContract" | "token_contract" => Ok(GeneratedField::TokenContract),
                            "name" => Ok(GeneratedField::Name),
                            "symbol" => Ok(GeneratedField::Symbol),
                            "decimals" => Ok(GeneratedField::Decimals),
                            "orchestrator" => Ok(GeneratedField::Orchestrator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgErc20DeployedClaim;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgERC20DeployedClaim")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgErc20DeployedClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut block_height__ = None;
                let mut cosmos_denom__ = None;
                let mut token_contract__ = None;
                let mut name__ = None;
                let mut symbol__ = None;
                let mut decimals__ = None;
                let mut orchestrator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CosmosDenom => {
                            if cosmos_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosDenom"));
                            }
                            cosmos_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenContract => {
                            if token_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenContract"));
                            }
                            token_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Decimals => {
                            if decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decimals"));
                            }
                            decimals__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Orchestrator => {
                            if orchestrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orchestrator"));
                            }
                            orchestrator__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgErc20DeployedClaim {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    block_height: block_height__.unwrap_or_default(),
                    cosmos_denom: cosmos_denom__.unwrap_or_default(),
                    token_contract: token_contract__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    symbol: symbol__.unwrap_or_default(),
                    decimals: decimals__.unwrap_or_default(),
                    orchestrator: orchestrator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgERC20DeployedClaim",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgErc20DeployedClaimResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgERC20DeployedClaimResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgErc20DeployedClaimResponse {
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
            type Value = MsgErc20DeployedClaimResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgERC20DeployedClaimResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgErc20DeployedClaimResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgErc20DeployedClaimResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgERC20DeployedClaimResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRequestBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.orchestrator.is_empty() {
            len += 1;
        }
        if !self.denom.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgRequestBatch", len)?;
        if !self.orchestrator.is_empty() {
            struct_ser.serialize_field("orchestrator", &self.orchestrator)?;
        }
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRequestBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["orchestrator", "denom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Orchestrator,
            Denom,
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
                            "orchestrator" => Ok(GeneratedField::Orchestrator),
                            "denom" => Ok(GeneratedField::Denom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRequestBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgRequestBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRequestBatch, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut orchestrator__ = None;
                let mut denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Orchestrator => {
                            if orchestrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orchestrator"));
                            }
                            orchestrator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRequestBatch {
                    orchestrator: orchestrator__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgRequestBatch",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRequestBatchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgRequestBatchResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRequestBatchResponse {
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
            type Value = MsgRequestBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgRequestBatchResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRequestBatchResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRequestBatchResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgRequestBatchResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRevokeEthereumBlacklist {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signer.is_empty() {
            len += 1;
        }
        if !self.blacklist_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgRevokeEthereumBlacklist", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.blacklist_addresses.is_empty() {
            struct_ser.serialize_field("blacklistAddresses", &self.blacklist_addresses)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRevokeEthereumBlacklist {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "blacklist_addresses", "blacklistAddresses"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            BlacklistAddresses,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "blacklistAddresses" | "blacklist_addresses" => {
                                Ok(GeneratedField::BlacklistAddresses)
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
            type Value = MsgRevokeEthereumBlacklist;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgRevokeEthereumBlacklist")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRevokeEthereumBlacklist, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut blacklist_addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlacklistAddresses => {
                            if blacklist_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "blacklistAddresses",
                                ));
                            }
                            blacklist_addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRevokeEthereumBlacklist {
                    signer: signer__.unwrap_or_default(),
                    blacklist_addresses: blacklist_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgRevokeEthereumBlacklist",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRevokeEthereumBlacklistResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.peggy.v1.MsgRevokeEthereumBlacklistResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRevokeEthereumBlacklistResponse {
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
            type Value = MsgRevokeEthereumBlacklistResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgRevokeEthereumBlacklistResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRevokeEthereumBlacklistResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRevokeEthereumBlacklistResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgRevokeEthereumBlacklistResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSendToEth {
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
        if !self.eth_dest.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.bridge_fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.MsgSendToEth", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.eth_dest.is_empty() {
            struct_ser.serialize_field("ethDest", &self.eth_dest)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if let Some(v) = self.bridge_fee.as_ref() {
            struct_ser.serialize_field("bridgeFee", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSendToEth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "eth_dest",
            "ethDest",
            "amount",
            "bridge_fee",
            "bridgeFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            EthDest,
            Amount,
            BridgeFee,
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
                            "ethDest" | "eth_dest" => Ok(GeneratedField::EthDest),
                            "amount" => Ok(GeneratedField::Amount),
                            "bridgeFee" | "bridge_fee" => Ok(GeneratedField::BridgeFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSendToEth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgSendToEth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSendToEth, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut eth_dest__ = None;
                let mut amount__ = None;
                let mut bridge_fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EthDest => {
                            if eth_dest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethDest"));
                            }
                            eth_dest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                        GeneratedField::BridgeFee => {
                            if bridge_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeFee"));
                            }
                            bridge_fee__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgSendToEth {
                    sender: sender__.unwrap_or_default(),
                    eth_dest: eth_dest__.unwrap_or_default(),
                    amount: amount__,
                    bridge_fee: bridge_fee__,
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.MsgSendToEth", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSendToEthResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgSendToEthResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSendToEthResponse {
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
            type Value = MsgSendToEthResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgSendToEthResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSendToEthResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSendToEthResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgSendToEthResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSetOrchestratorAddresses {
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
        if !self.orchestrator.is_empty() {
            len += 1;
        }
        if !self.eth_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgSetOrchestratorAddresses", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.orchestrator.is_empty() {
            struct_ser.serialize_field("orchestrator", &self.orchestrator)?;
        }
        if !self.eth_address.is_empty() {
            struct_ser.serialize_field("ethAddress", &self.eth_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSetOrchestratorAddresses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "orchestrator", "eth_address", "ethAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Orchestrator,
            EthAddress,
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
                            "orchestrator" => Ok(GeneratedField::Orchestrator),
                            "ethAddress" | "eth_address" => Ok(GeneratedField::EthAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetOrchestratorAddresses;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgSetOrchestratorAddresses")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetOrchestratorAddresses, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut orchestrator__ = None;
                let mut eth_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Orchestrator => {
                            if orchestrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orchestrator"));
                            }
                            orchestrator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EthAddress => {
                            if eth_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethAddress"));
                            }
                            eth_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSetOrchestratorAddresses {
                    sender: sender__.unwrap_or_default(),
                    orchestrator: orchestrator__.unwrap_or_default(),
                    eth_address: eth_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgSetOrchestratorAddresses",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSetOrchestratorAddressesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.MsgSetOrchestratorAddressesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSetOrchestratorAddressesResponse {
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
            type Value = MsgSetOrchestratorAddressesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgSetOrchestratorAddressesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSetOrchestratorAddressesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetOrchestratorAddressesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgSetOrchestratorAddressesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitBadSignatureEvidence {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subject.is_some() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgSubmitBadSignatureEvidence", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitBadSignatureEvidence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subject", "signature", "sender"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Signature,
            Sender,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "signature" => Ok(GeneratedField::Signature),
                            "sender" => Ok(GeneratedField::Sender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitBadSignatureEvidence;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgSubmitBadSignatureEvidence")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSubmitBadSignatureEvidence, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut signature__ = None;
                let mut sender__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map_.next_value()?;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitBadSignatureEvidence {
                    subject: subject__,
                    signature: signature__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgSubmitBadSignatureEvidence",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitBadSignatureEvidenceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.MsgSubmitBadSignatureEvidenceResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitBadSignatureEvidenceResponse {
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
            type Value = MsgSubmitBadSignatureEvidenceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.peggy.v1.MsgSubmitBadSignatureEvidenceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgSubmitBadSignatureEvidenceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitBadSignatureEvidenceResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgSubmitBadSignatureEvidenceResponse",
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
            serializer.serialize_struct("injective.peggy.v1.MsgUpdateParams", len)?;
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
                formatter.write_str("struct injective.peggy.v1.MsgUpdateParams")
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
            "injective.peggy.v1.MsgUpdateParams",
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
            serializer.serialize_struct("injective.peggy.v1.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct injective.peggy.v1.MsgUpdateParamsResponse")
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
            "injective.peggy.v1.MsgUpdateParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgValsetConfirm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        if !self.orchestrator.is_empty() {
            len += 1;
        }
        if !self.eth_address.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgValsetConfirm", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.orchestrator.is_empty() {
            struct_ser.serialize_field("orchestrator", &self.orchestrator)?;
        }
        if !self.eth_address.is_empty() {
            struct_ser.serialize_field("ethAddress", &self.eth_address)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgValsetConfirm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nonce",
            "orchestrator",
            "eth_address",
            "ethAddress",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
            Orchestrator,
            EthAddress,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            "orchestrator" => Ok(GeneratedField::Orchestrator),
                            "ethAddress" | "eth_address" => Ok(GeneratedField::EthAddress),
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
            type Value = MsgValsetConfirm;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgValsetConfirm")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgValsetConfirm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                let mut orchestrator__ = None;
                let mut eth_address__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Orchestrator => {
                            if orchestrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orchestrator"));
                            }
                            orchestrator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EthAddress => {
                            if eth_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethAddress"));
                            }
                            eth_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgValsetConfirm {
                    nonce: nonce__.unwrap_or_default(),
                    orchestrator: orchestrator__.unwrap_or_default(),
                    eth_address: eth_address__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgValsetConfirm",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgValsetConfirmResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgValsetConfirmResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgValsetConfirmResponse {
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
            type Value = MsgValsetConfirmResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgValsetConfirmResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgValsetConfirmResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgValsetConfirmResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgValsetConfirmResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgValsetUpdatedClaim {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if self.valset_nonce != 0 {
            len += 1;
        }
        if self.block_height != 0 {
            len += 1;
        }
        if !self.members.is_empty() {
            len += 1;
        }
        if !self.reward_amount.is_empty() {
            len += 1;
        }
        if !self.reward_token.is_empty() {
            len += 1;
        }
        if !self.orchestrator.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgValsetUpdatedClaim", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if self.valset_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "valsetNonce",
                ToString::to_string(&self.valset_nonce).as_str(),
            )?;
        }
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if !self.reward_amount.is_empty() {
            struct_ser.serialize_field("rewardAmount", &self.reward_amount)?;
        }
        if !self.reward_token.is_empty() {
            struct_ser.serialize_field("rewardToken", &self.reward_token)?;
        }
        if !self.orchestrator.is_empty() {
            struct_ser.serialize_field("orchestrator", &self.orchestrator)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgValsetUpdatedClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "valset_nonce",
            "valsetNonce",
            "block_height",
            "blockHeight",
            "members",
            "reward_amount",
            "rewardAmount",
            "reward_token",
            "rewardToken",
            "orchestrator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            ValsetNonce,
            BlockHeight,
            Members,
            RewardAmount,
            RewardToken,
            Orchestrator,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "valsetNonce" | "valset_nonce" => Ok(GeneratedField::ValsetNonce),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "members" => Ok(GeneratedField::Members),
                            "rewardAmount" | "reward_amount" => Ok(GeneratedField::RewardAmount),
                            "rewardToken" | "reward_token" => Ok(GeneratedField::RewardToken),
                            "orchestrator" => Ok(GeneratedField::Orchestrator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgValsetUpdatedClaim;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgValsetUpdatedClaim")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgValsetUpdatedClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut valset_nonce__ = None;
                let mut block_height__ = None;
                let mut members__ = None;
                let mut reward_amount__ = None;
                let mut reward_token__ = None;
                let mut orchestrator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValsetNonce => {
                            if valset_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetNonce"));
                            }
                            valset_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RewardAmount => {
                            if reward_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAmount"));
                            }
                            reward_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RewardToken => {
                            if reward_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardToken"));
                            }
                            reward_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Orchestrator => {
                            if orchestrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orchestrator"));
                            }
                            orchestrator__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgValsetUpdatedClaim {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    valset_nonce: valset_nonce__.unwrap_or_default(),
                    block_height: block_height__.unwrap_or_default(),
                    members: members__.unwrap_or_default(),
                    reward_amount: reward_amount__.unwrap_or_default(),
                    reward_token: reward_token__.unwrap_or_default(),
                    orchestrator: orchestrator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgValsetUpdatedClaim",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgValsetUpdatedClaimResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgValsetUpdatedClaimResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgValsetUpdatedClaimResponse {
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
            type Value = MsgValsetUpdatedClaimResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgValsetUpdatedClaimResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgValsetUpdatedClaimResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgValsetUpdatedClaimResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgValsetUpdatedClaimResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgWithdrawClaim {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_nonce != 0 {
            len += 1;
        }
        if self.block_height != 0 {
            len += 1;
        }
        if self.batch_nonce != 0 {
            len += 1;
        }
        if !self.token_contract.is_empty() {
            len += 1;
        }
        if !self.orchestrator.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgWithdrawClaim", len)?;
        if self.event_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventNonce",
                ToString::to_string(&self.event_nonce).as_str(),
            )?;
        }
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        if self.batch_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "batchNonce",
                ToString::to_string(&self.batch_nonce).as_str(),
            )?;
        }
        if !self.token_contract.is_empty() {
            struct_ser.serialize_field("tokenContract", &self.token_contract)?;
        }
        if !self.orchestrator.is_empty() {
            struct_ser.serialize_field("orchestrator", &self.orchestrator)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgWithdrawClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_nonce",
            "eventNonce",
            "block_height",
            "blockHeight",
            "batch_nonce",
            "batchNonce",
            "token_contract",
            "tokenContract",
            "orchestrator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventNonce,
            BlockHeight,
            BatchNonce,
            TokenContract,
            Orchestrator,
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
                            "eventNonce" | "event_nonce" => Ok(GeneratedField::EventNonce),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "batchNonce" | "batch_nonce" => Ok(GeneratedField::BatchNonce),
                            "tokenContract" | "token_contract" => Ok(GeneratedField::TokenContract),
                            "orchestrator" => Ok(GeneratedField::Orchestrator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgWithdrawClaim;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgWithdrawClaim")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgWithdrawClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_nonce__ = None;
                let mut block_height__ = None;
                let mut batch_nonce__ = None;
                let mut token_contract__ = None;
                let mut orchestrator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventNonce => {
                            if event_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventNonce"));
                            }
                            event_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BatchNonce => {
                            if batch_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchNonce"));
                            }
                            batch_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TokenContract => {
                            if token_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenContract"));
                            }
                            token_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Orchestrator => {
                            if orchestrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orchestrator"));
                            }
                            orchestrator__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgWithdrawClaim {
                    event_nonce: event_nonce__.unwrap_or_default(),
                    block_height: block_height__.unwrap_or_default(),
                    batch_nonce: batch_nonce__.unwrap_or_default(),
                    token_contract: token_contract__.unwrap_or_default(),
                    orchestrator: orchestrator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgWithdrawClaim",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgWithdrawClaimResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.MsgWithdrawClaimResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgWithdrawClaimResponse {
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
            type Value = MsgWithdrawClaimResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.MsgWithdrawClaimResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgWithdrawClaimResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgWithdrawClaimResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.MsgWithdrawClaimResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OutgoingTransferTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.dest_address.is_empty() {
            len += 1;
        }
        if self.erc20_token.is_some() {
            len += 1;
        }
        if self.erc20_fee.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.OutgoingTransferTx", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.dest_address.is_empty() {
            struct_ser.serialize_field("destAddress", &self.dest_address)?;
        }
        if let Some(v) = self.erc20_token.as_ref() {
            struct_ser.serialize_field("erc20Token", v)?;
        }
        if let Some(v) = self.erc20_fee.as_ref() {
            struct_ser.serialize_field("erc20Fee", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OutgoingTransferTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "sender",
            "dest_address",
            "destAddress",
            "erc20_token",
            "erc20Token",
            "erc20_fee",
            "erc20Fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Sender,
            DestAddress,
            Erc20Token,
            Erc20Fee,
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
                            "id" => Ok(GeneratedField::Id),
                            "sender" => Ok(GeneratedField::Sender),
                            "destAddress" | "dest_address" => Ok(GeneratedField::DestAddress),
                            "erc20Token" | "erc20_token" => Ok(GeneratedField::Erc20Token),
                            "erc20Fee" | "erc20_fee" => Ok(GeneratedField::Erc20Fee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutgoingTransferTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.OutgoingTransferTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OutgoingTransferTx, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut sender__ = None;
                let mut dest_address__ = None;
                let mut erc20_token__ = None;
                let mut erc20_fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestAddress => {
                            if dest_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destAddress"));
                            }
                            dest_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Erc20Token => {
                            if erc20_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("erc20Token"));
                            }
                            erc20_token__ = map_.next_value()?;
                        }
                        GeneratedField::Erc20Fee => {
                            if erc20_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("erc20Fee"));
                            }
                            erc20_fee__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OutgoingTransferTx {
                    id: id__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    dest_address: dest_address__.unwrap_or_default(),
                    erc20_token: erc20_token__,
                    erc20_fee: erc20_fee__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.OutgoingTransferTx",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OutgoingTxBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch_nonce != 0 {
            len += 1;
        }
        if self.batch_timeout != 0 {
            len += 1;
        }
        if !self.transactions.is_empty() {
            len += 1;
        }
        if !self.token_contract.is_empty() {
            len += 1;
        }
        if self.block != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.OutgoingTxBatch", len)?;
        if self.batch_nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "batchNonce",
                ToString::to_string(&self.batch_nonce).as_str(),
            )?;
        }
        if self.batch_timeout != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "batchTimeout",
                ToString::to_string(&self.batch_timeout).as_str(),
            )?;
        }
        if !self.transactions.is_empty() {
            struct_ser.serialize_field("transactions", &self.transactions)?;
        }
        if !self.token_contract.is_empty() {
            struct_ser.serialize_field("tokenContract", &self.token_contract)?;
        }
        if self.block != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("block", ToString::to_string(&self.block).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OutgoingTxBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_nonce",
            "batchNonce",
            "batch_timeout",
            "batchTimeout",
            "transactions",
            "token_contract",
            "tokenContract",
            "block",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchNonce,
            BatchTimeout,
            Transactions,
            TokenContract,
            Block,
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
                            "batchNonce" | "batch_nonce" => Ok(GeneratedField::BatchNonce),
                            "batchTimeout" | "batch_timeout" => Ok(GeneratedField::BatchTimeout),
                            "transactions" => Ok(GeneratedField::Transactions),
                            "tokenContract" | "token_contract" => Ok(GeneratedField::TokenContract),
                            "block" => Ok(GeneratedField::Block),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutgoingTxBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.OutgoingTxBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OutgoingTxBatch, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut batch_nonce__ = None;
                let mut batch_timeout__ = None;
                let mut transactions__ = None;
                let mut token_contract__ = None;
                let mut block__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchNonce => {
                            if batch_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchNonce"));
                            }
                            batch_nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BatchTimeout => {
                            if batch_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchTimeout"));
                            }
                            batch_timeout__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Transactions => {
                            if transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactions"));
                            }
                            transactions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenContract => {
                            if token_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenContract"));
                            }
                            token_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(OutgoingTxBatch {
                    batch_nonce: batch_nonce__.unwrap_or_default(),
                    batch_timeout: batch_timeout__.unwrap_or_default(),
                    transactions: transactions__.unwrap_or_default(),
                    token_contract: token_contract__.unwrap_or_default(),
                    block: block__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.OutgoingTxBatch",
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
        if !self.peggy_id.is_empty() {
            len += 1;
        }
        if !self.contract_source_hash.is_empty() {
            len += 1;
        }
        if !self.bridge_ethereum_address.is_empty() {
            len += 1;
        }
        if self.bridge_chain_id != 0 {
            len += 1;
        }
        if self.signed_valsets_window != 0 {
            len += 1;
        }
        if self.signed_batches_window != 0 {
            len += 1;
        }
        if self.signed_claims_window != 0 {
            len += 1;
        }
        if self.target_batch_timeout != 0 {
            len += 1;
        }
        if self.average_block_time != 0 {
            len += 1;
        }
        if self.average_ethereum_block_time != 0 {
            len += 1;
        }
        if !self.slash_fraction_valset.is_empty() {
            len += 1;
        }
        if !self.slash_fraction_batch.is_empty() {
            len += 1;
        }
        if !self.slash_fraction_claim.is_empty() {
            len += 1;
        }
        if !self.slash_fraction_conflicting_claim.is_empty() {
            len += 1;
        }
        if self.unbond_slashing_valsets_window != 0 {
            len += 1;
        }
        if !self.slash_fraction_bad_eth_signature.is_empty() {
            len += 1;
        }
        if !self.cosmos_coin_denom.is_empty() {
            len += 1;
        }
        if !self.cosmos_coin_erc20_contract.is_empty() {
            len += 1;
        }
        if self.claim_slashing_enabled {
            len += 1;
        }
        if self.bridge_contract_start_height != 0 {
            len += 1;
        }
        if self.valset_reward.is_some() {
            len += 1;
        }
        if !self.admins.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.Params", len)?;
        if !self.peggy_id.is_empty() {
            struct_ser.serialize_field("peggyId", &self.peggy_id)?;
        }
        if !self.contract_source_hash.is_empty() {
            struct_ser.serialize_field("contractSourceHash", &self.contract_source_hash)?;
        }
        if !self.bridge_ethereum_address.is_empty() {
            struct_ser.serialize_field("bridgeEthereumAddress", &self.bridge_ethereum_address)?;
        }
        if self.bridge_chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "bridgeChainId",
                ToString::to_string(&self.bridge_chain_id).as_str(),
            )?;
        }
        if self.signed_valsets_window != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "signedValsetsWindow",
                ToString::to_string(&self.signed_valsets_window).as_str(),
            )?;
        }
        if self.signed_batches_window != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "signedBatchesWindow",
                ToString::to_string(&self.signed_batches_window).as_str(),
            )?;
        }
        if self.signed_claims_window != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "signedClaimsWindow",
                ToString::to_string(&self.signed_claims_window).as_str(),
            )?;
        }
        if self.target_batch_timeout != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "targetBatchTimeout",
                ToString::to_string(&self.target_batch_timeout).as_str(),
            )?;
        }
        if self.average_block_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "averageBlockTime",
                ToString::to_string(&self.average_block_time).as_str(),
            )?;
        }
        if self.average_ethereum_block_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "averageEthereumBlockTime",
                ToString::to_string(&self.average_ethereum_block_time).as_str(),
            )?;
        }
        if !self.slash_fraction_valset.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "slashFractionValset",
                pbjson::private::base64::encode(&self.slash_fraction_valset).as_str(),
            )?;
        }
        if !self.slash_fraction_batch.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "slashFractionBatch",
                pbjson::private::base64::encode(&self.slash_fraction_batch).as_str(),
            )?;
        }
        if !self.slash_fraction_claim.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "slashFractionClaim",
                pbjson::private::base64::encode(&self.slash_fraction_claim).as_str(),
            )?;
        }
        if !self.slash_fraction_conflicting_claim.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "slashFractionConflictingClaim",
                pbjson::private::base64::encode(&self.slash_fraction_conflicting_claim).as_str(),
            )?;
        }
        if self.unbond_slashing_valsets_window != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "unbondSlashingValsetsWindow",
                ToString::to_string(&self.unbond_slashing_valsets_window).as_str(),
            )?;
        }
        if !self.slash_fraction_bad_eth_signature.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "slashFractionBadEthSignature",
                pbjson::private::base64::encode(&self.slash_fraction_bad_eth_signature).as_str(),
            )?;
        }
        if !self.cosmos_coin_denom.is_empty() {
            struct_ser.serialize_field("cosmosCoinDenom", &self.cosmos_coin_denom)?;
        }
        if !self.cosmos_coin_erc20_contract.is_empty() {
            struct_ser
                .serialize_field("cosmosCoinErc20Contract", &self.cosmos_coin_erc20_contract)?;
        }
        if self.claim_slashing_enabled {
            struct_ser.serialize_field("claimSlashingEnabled", &self.claim_slashing_enabled)?;
        }
        if self.bridge_contract_start_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "bridgeContractStartHeight",
                ToString::to_string(&self.bridge_contract_start_height).as_str(),
            )?;
        }
        if let Some(v) = self.valset_reward.as_ref() {
            struct_ser.serialize_field("valsetReward", v)?;
        }
        if !self.admins.is_empty() {
            struct_ser.serialize_field("admins", &self.admins)?;
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
            "peggy_id",
            "peggyId",
            "contract_source_hash",
            "contractSourceHash",
            "bridge_ethereum_address",
            "bridgeEthereumAddress",
            "bridge_chain_id",
            "bridgeChainId",
            "signed_valsets_window",
            "signedValsetsWindow",
            "signed_batches_window",
            "signedBatchesWindow",
            "signed_claims_window",
            "signedClaimsWindow",
            "target_batch_timeout",
            "targetBatchTimeout",
            "average_block_time",
            "averageBlockTime",
            "average_ethereum_block_time",
            "averageEthereumBlockTime",
            "slash_fraction_valset",
            "slashFractionValset",
            "slash_fraction_batch",
            "slashFractionBatch",
            "slash_fraction_claim",
            "slashFractionClaim",
            "slash_fraction_conflicting_claim",
            "slashFractionConflictingClaim",
            "unbond_slashing_valsets_window",
            "unbondSlashingValsetsWindow",
            "slash_fraction_bad_eth_signature",
            "slashFractionBadEthSignature",
            "cosmos_coin_denom",
            "cosmosCoinDenom",
            "cosmos_coin_erc20_contract",
            "cosmosCoinErc20Contract",
            "claim_slashing_enabled",
            "claimSlashingEnabled",
            "bridge_contract_start_height",
            "bridgeContractStartHeight",
            "valset_reward",
            "valsetReward",
            "admins",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PeggyId,
            ContractSourceHash,
            BridgeEthereumAddress,
            BridgeChainId,
            SignedValsetsWindow,
            SignedBatchesWindow,
            SignedClaimsWindow,
            TargetBatchTimeout,
            AverageBlockTime,
            AverageEthereumBlockTime,
            SlashFractionValset,
            SlashFractionBatch,
            SlashFractionClaim,
            SlashFractionConflictingClaim,
            UnbondSlashingValsetsWindow,
            SlashFractionBadEthSignature,
            CosmosCoinDenom,
            CosmosCoinErc20Contract,
            ClaimSlashingEnabled,
            BridgeContractStartHeight,
            ValsetReward,
            Admins,
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
                            "peggyId" | "peggy_id" => Ok(GeneratedField::PeggyId),
                            "contractSourceHash" | "contract_source_hash" => {
                                Ok(GeneratedField::ContractSourceHash)
                            }
                            "bridgeEthereumAddress" | "bridge_ethereum_address" => {
                                Ok(GeneratedField::BridgeEthereumAddress)
                            }
                            "bridgeChainId" | "bridge_chain_id" => {
                                Ok(GeneratedField::BridgeChainId)
                            }
                            "signedValsetsWindow" | "signed_valsets_window" => {
                                Ok(GeneratedField::SignedValsetsWindow)
                            }
                            "signedBatchesWindow" | "signed_batches_window" => {
                                Ok(GeneratedField::SignedBatchesWindow)
                            }
                            "signedClaimsWindow" | "signed_claims_window" => {
                                Ok(GeneratedField::SignedClaimsWindow)
                            }
                            "targetBatchTimeout" | "target_batch_timeout" => {
                                Ok(GeneratedField::TargetBatchTimeout)
                            }
                            "averageBlockTime" | "average_block_time" => {
                                Ok(GeneratedField::AverageBlockTime)
                            }
                            "averageEthereumBlockTime" | "average_ethereum_block_time" => {
                                Ok(GeneratedField::AverageEthereumBlockTime)
                            }
                            "slashFractionValset" | "slash_fraction_valset" => {
                                Ok(GeneratedField::SlashFractionValset)
                            }
                            "slashFractionBatch" | "slash_fraction_batch" => {
                                Ok(GeneratedField::SlashFractionBatch)
                            }
                            "slashFractionClaim" | "slash_fraction_claim" => {
                                Ok(GeneratedField::SlashFractionClaim)
                            }
                            "slashFractionConflictingClaim"
                            | "slash_fraction_conflicting_claim" => {
                                Ok(GeneratedField::SlashFractionConflictingClaim)
                            }
                            "unbondSlashingValsetsWindow" | "unbond_slashing_valsets_window" => {
                                Ok(GeneratedField::UnbondSlashingValsetsWindow)
                            }
                            "slashFractionBadEthSignature" | "slash_fraction_bad_eth_signature" => {
                                Ok(GeneratedField::SlashFractionBadEthSignature)
                            }
                            "cosmosCoinDenom" | "cosmos_coin_denom" => {
                                Ok(GeneratedField::CosmosCoinDenom)
                            }
                            "cosmosCoinErc20Contract" | "cosmos_coin_erc20_contract" => {
                                Ok(GeneratedField::CosmosCoinErc20Contract)
                            }
                            "claimSlashingEnabled" | "claim_slashing_enabled" => {
                                Ok(GeneratedField::ClaimSlashingEnabled)
                            }
                            "bridgeContractStartHeight" | "bridge_contract_start_height" => {
                                Ok(GeneratedField::BridgeContractStartHeight)
                            }
                            "valsetReward" | "valset_reward" => Ok(GeneratedField::ValsetReward),
                            "admins" => Ok(GeneratedField::Admins),
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
                formatter.write_str("struct injective.peggy.v1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut peggy_id__ = None;
                let mut contract_source_hash__ = None;
                let mut bridge_ethereum_address__ = None;
                let mut bridge_chain_id__ = None;
                let mut signed_valsets_window__ = None;
                let mut signed_batches_window__ = None;
                let mut signed_claims_window__ = None;
                let mut target_batch_timeout__ = None;
                let mut average_block_time__ = None;
                let mut average_ethereum_block_time__ = None;
                let mut slash_fraction_valset__ = None;
                let mut slash_fraction_batch__ = None;
                let mut slash_fraction_claim__ = None;
                let mut slash_fraction_conflicting_claim__ = None;
                let mut unbond_slashing_valsets_window__ = None;
                let mut slash_fraction_bad_eth_signature__ = None;
                let mut cosmos_coin_denom__ = None;
                let mut cosmos_coin_erc20_contract__ = None;
                let mut claim_slashing_enabled__ = None;
                let mut bridge_contract_start_height__ = None;
                let mut valset_reward__ = None;
                let mut admins__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PeggyId => {
                            if peggy_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peggyId"));
                            }
                            peggy_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContractSourceHash => {
                            if contract_source_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "contractSourceHash",
                                ));
                            }
                            contract_source_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BridgeEthereumAddress => {
                            if bridge_ethereum_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bridgeEthereumAddress",
                                ));
                            }
                            bridge_ethereum_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BridgeChainId => {
                            if bridge_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeChainId"));
                            }
                            bridge_chain_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SignedValsetsWindow => {
                            if signed_valsets_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "signedValsetsWindow",
                                ));
                            }
                            signed_valsets_window__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SignedBatchesWindow => {
                            if signed_batches_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "signedBatchesWindow",
                                ));
                            }
                            signed_batches_window__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SignedClaimsWindow => {
                            if signed_claims_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "signedClaimsWindow",
                                ));
                            }
                            signed_claims_window__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TargetBatchTimeout => {
                            if target_batch_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "targetBatchTimeout",
                                ));
                            }
                            target_batch_timeout__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AverageBlockTime => {
                            if average_block_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("averageBlockTime"));
                            }
                            average_block_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AverageEthereumBlockTime => {
                            if average_ethereum_block_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "averageEthereumBlockTime",
                                ));
                            }
                            average_ethereum_block_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SlashFractionValset => {
                            if slash_fraction_valset__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "slashFractionValset",
                                ));
                            }
                            slash_fraction_valset__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SlashFractionBatch => {
                            if slash_fraction_batch__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "slashFractionBatch",
                                ));
                            }
                            slash_fraction_batch__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SlashFractionClaim => {
                            if slash_fraction_claim__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "slashFractionClaim",
                                ));
                            }
                            slash_fraction_claim__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SlashFractionConflictingClaim => {
                            if slash_fraction_conflicting_claim__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "slashFractionConflictingClaim",
                                ));
                            }
                            slash_fraction_conflicting_claim__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::UnbondSlashingValsetsWindow => {
                            if unbond_slashing_valsets_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unbondSlashingValsetsWindow",
                                ));
                            }
                            unbond_slashing_valsets_window__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SlashFractionBadEthSignature => {
                            if slash_fraction_bad_eth_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "slashFractionBadEthSignature",
                                ));
                            }
                            slash_fraction_bad_eth_signature__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CosmosCoinDenom => {
                            if cosmos_coin_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosCoinDenom"));
                            }
                            cosmos_coin_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CosmosCoinErc20Contract => {
                            if cosmos_coin_erc20_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "cosmosCoinErc20Contract",
                                ));
                            }
                            cosmos_coin_erc20_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClaimSlashingEnabled => {
                            if claim_slashing_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "claimSlashingEnabled",
                                ));
                            }
                            claim_slashing_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BridgeContractStartHeight => {
                            if bridge_contract_start_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bridgeContractStartHeight",
                                ));
                            }
                            bridge_contract_start_height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValsetReward => {
                            if valset_reward__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsetReward"));
                            }
                            valset_reward__ = map_.next_value()?;
                        }
                        GeneratedField::Admins => {
                            if admins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admins"));
                            }
                            admins__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    peggy_id: peggy_id__.unwrap_or_default(),
                    contract_source_hash: contract_source_hash__.unwrap_or_default(),
                    bridge_ethereum_address: bridge_ethereum_address__.unwrap_or_default(),
                    bridge_chain_id: bridge_chain_id__.unwrap_or_default(),
                    signed_valsets_window: signed_valsets_window__.unwrap_or_default(),
                    signed_batches_window: signed_batches_window__.unwrap_or_default(),
                    signed_claims_window: signed_claims_window__.unwrap_or_default(),
                    target_batch_timeout: target_batch_timeout__.unwrap_or_default(),
                    average_block_time: average_block_time__.unwrap_or_default(),
                    average_ethereum_block_time: average_ethereum_block_time__.unwrap_or_default(),
                    slash_fraction_valset: slash_fraction_valset__.unwrap_or_default(),
                    slash_fraction_batch: slash_fraction_batch__.unwrap_or_default(),
                    slash_fraction_claim: slash_fraction_claim__.unwrap_or_default(),
                    slash_fraction_conflicting_claim: slash_fraction_conflicting_claim__
                        .unwrap_or_default(),
                    unbond_slashing_valsets_window: unbond_slashing_valsets_window__
                        .unwrap_or_default(),
                    slash_fraction_bad_eth_signature: slash_fraction_bad_eth_signature__
                        .unwrap_or_default(),
                    cosmos_coin_denom: cosmos_coin_denom__.unwrap_or_default(),
                    cosmos_coin_erc20_contract: cosmos_coin_erc20_contract__.unwrap_or_default(),
                    claim_slashing_enabled: claim_slashing_enabled__.unwrap_or_default(),
                    bridge_contract_start_height: bridge_contract_start_height__
                        .unwrap_or_default(),
                    valset_reward: valset_reward__,
                    admins: admins__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBatchConfirmsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryBatchConfirmsRequest", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBatchConfirmsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nonce", "contract_address", "contractAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
            ContractAddress,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
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
            type Value = QueryBatchConfirmsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryBatchConfirmsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBatchConfirmsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                let mut contract_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBatchConfirmsRequest {
                    nonce: nonce__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryBatchConfirmsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBatchConfirmsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.confirms.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryBatchConfirmsResponse", len)?;
        if !self.confirms.is_empty() {
            struct_ser.serialize_field("confirms", &self.confirms)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBatchConfirmsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["confirms"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Confirms,
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
                            "confirms" => Ok(GeneratedField::Confirms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBatchConfirmsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryBatchConfirmsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBatchConfirmsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut confirms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Confirms => {
                            if confirms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confirms"));
                            }
                            confirms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBatchConfirmsResponse {
                    confirms: confirms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryBatchConfirmsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBatchFeeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryBatchFeeRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBatchFeeRequest {
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
            type Value = QueryBatchFeeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryBatchFeeRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBatchFeeRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBatchFeeRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryBatchFeeRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBatchFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batch_fees.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryBatchFeeResponse", len)?;
        if !self.batch_fees.is_empty() {
            struct_ser.serialize_field("batchFees", &self.batch_fees)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBatchFeeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["batchFees"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchFees,
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
                            "batchFees" => Ok(GeneratedField::BatchFees),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBatchFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryBatchFeeResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBatchFeeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut batch_fees__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchFees => {
                            if batch_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchFees"));
                            }
                            batch_fees__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBatchFeeResponse {
                    batch_fees: batch_fees__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryBatchFeeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBatchRequestByNonceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.peggy.v1.QueryBatchRequestByNonceRequest", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBatchRequestByNonceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nonce", "contract_address", "contractAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
            ContractAddress,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
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
            type Value = QueryBatchRequestByNonceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryBatchRequestByNonceRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBatchRequestByNonceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                let mut contract_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBatchRequestByNonceRequest {
                    nonce: nonce__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryBatchRequestByNonceRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBatchRequestByNonceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.peggy.v1.QueryBatchRequestByNonceResponse", len)?;
        if let Some(v) = self.batch.as_ref() {
            struct_ser.serialize_field("batch", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBatchRequestByNonceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["batch"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batch,
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
                            "batch" => Ok(GeneratedField::Batch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBatchRequestByNonceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryBatchRequestByNonceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryBatchRequestByNonceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut batch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Batch => {
                            if batch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch"));
                            }
                            batch__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchRequestByNonceResponse { batch: batch__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryBatchRequestByNonceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCurrentValsetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryCurrentValsetRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCurrentValsetRequest {
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
            type Value = QueryCurrentValsetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryCurrentValsetRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryCurrentValsetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryCurrentValsetRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryCurrentValsetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCurrentValsetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.valset.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryCurrentValsetResponse", len)?;
        if let Some(v) = self.valset.as_ref() {
            struct_ser.serialize_field("valset", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCurrentValsetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["valset"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Valset,
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
                            "valset" => Ok(GeneratedField::Valset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCurrentValsetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryCurrentValsetResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryCurrentValsetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut valset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Valset => {
                            if valset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valset"));
                            }
                            valset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryCurrentValsetResponse { valset: valset__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryCurrentValsetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDelegateKeysByEthAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.eth_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryDelegateKeysByEthAddress", len)?;
        if !self.eth_address.is_empty() {
            struct_ser.serialize_field("ethAddress", &self.eth_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDelegateKeysByEthAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["eth_address", "ethAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EthAddress,
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
                            "ethAddress" | "eth_address" => Ok(GeneratedField::EthAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDelegateKeysByEthAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryDelegateKeysByEthAddress")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDelegateKeysByEthAddress, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut eth_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EthAddress => {
                            if eth_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethAddress"));
                            }
                            eth_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDelegateKeysByEthAddress {
                    eth_address: eth_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByEthAddress",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDelegateKeysByEthAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByEthAddressResponse",
            len,
        )?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDelegateKeysByEthAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator_address",
            "validatorAddress",
            "orchestrator_address",
            "orchestratorAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
            OrchestratorAddress,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = QueryDelegateKeysByEthAddressResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.peggy.v1.QueryDelegateKeysByEthAddressResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDelegateKeysByEthAddressResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                let mut orchestrator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDelegateKeysByEthAddressResponse {
                    validator_address: validator_address__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByEthAddressResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDelegateKeysByOrchestratorAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByOrchestratorAddress",
            len,
        )?;
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDelegateKeysByOrchestratorAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["orchestrator_address", "orchestratorAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrchestratorAddress,
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
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = QueryDelegateKeysByOrchestratorAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.peggy.v1.QueryDelegateKeysByOrchestratorAddress")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDelegateKeysByOrchestratorAddress, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut orchestrator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDelegateKeysByOrchestratorAddress {
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByOrchestratorAddress",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDelegateKeysByOrchestratorAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if !self.eth_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByOrchestratorAddressResponse",
            len,
        )?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.eth_address.is_empty() {
            struct_ser.serialize_field("ethAddress", &self.eth_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDelegateKeysByOrchestratorAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator_address",
            "validatorAddress",
            "eth_address",
            "ethAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
            EthAddress,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "ethAddress" | "eth_address" => Ok(GeneratedField::EthAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDelegateKeysByOrchestratorAddressResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.peggy.v1.QueryDelegateKeysByOrchestratorAddressResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDelegateKeysByOrchestratorAddressResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                let mut eth_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EthAddress => {
                            if eth_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethAddress"));
                            }
                            eth_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDelegateKeysByOrchestratorAddressResponse {
                    validator_address: validator_address__.unwrap_or_default(),
                    eth_address: eth_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByOrchestratorAddressResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDelegateKeysByValidatorAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByValidatorAddress",
            len,
        )?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDelegateKeysByValidatorAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_address", "validatorAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
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
            type Value = QueryDelegateKeysByValidatorAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryDelegateKeysByValidatorAddress")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDelegateKeysByValidatorAddress, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDelegateKeysByValidatorAddress {
                    validator_address: validator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByValidatorAddress",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDelegateKeysByValidatorAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.eth_address.is_empty() {
            len += 1;
        }
        if !self.orchestrator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByValidatorAddressResponse",
            len,
        )?;
        if !self.eth_address.is_empty() {
            struct_ser.serialize_field("ethAddress", &self.eth_address)?;
        }
        if !self.orchestrator_address.is_empty() {
            struct_ser.serialize_field("orchestratorAddress", &self.orchestrator_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDelegateKeysByValidatorAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "eth_address",
            "ethAddress",
            "orchestrator_address",
            "orchestratorAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EthAddress,
            OrchestratorAddress,
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
                            "ethAddress" | "eth_address" => Ok(GeneratedField::EthAddress),
                            "orchestratorAddress" | "orchestrator_address" => {
                                Ok(GeneratedField::OrchestratorAddress)
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
            type Value = QueryDelegateKeysByValidatorAddressResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.peggy.v1.QueryDelegateKeysByValidatorAddressResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDelegateKeysByValidatorAddressResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut eth_address__ = None;
                let mut orchestrator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EthAddress => {
                            if eth_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethAddress"));
                            }
                            eth_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrchestratorAddress => {
                            if orchestrator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "orchestratorAddress",
                                ));
                            }
                            orchestrator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDelegateKeysByValidatorAddressResponse {
                    eth_address: eth_address__.unwrap_or_default(),
                    orchestrator_address: orchestrator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryDelegateKeysByValidatorAddressResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDenomToErc20Request {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryDenomToERC20Request", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDenomToErc20Request {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["denom"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
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
                            "denom" => Ok(GeneratedField::Denom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDenomToErc20Request;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryDenomToERC20Request")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDenomToErc20Request, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDenomToErc20Request {
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryDenomToERC20Request",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDenomToErc20Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.erc20.is_empty() {
            len += 1;
        }
        if self.cosmos_originated {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryDenomToERC20Response", len)?;
        if !self.erc20.is_empty() {
            struct_ser.serialize_field("erc20", &self.erc20)?;
        }
        if self.cosmos_originated {
            struct_ser.serialize_field("cosmosOriginated", &self.cosmos_originated)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDenomToErc20Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["erc20", "cosmos_originated", "cosmosOriginated"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Erc20,
            CosmosOriginated,
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
                            "erc20" => Ok(GeneratedField::Erc20),
                            "cosmosOriginated" | "cosmos_originated" => {
                                Ok(GeneratedField::CosmosOriginated)
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
            type Value = QueryDenomToErc20Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryDenomToERC20Response")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryDenomToErc20Response, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut erc20__ = None;
                let mut cosmos_originated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Erc20 => {
                            if erc20__.is_some() {
                                return Err(serde::de::Error::duplicate_field("erc20"));
                            }
                            erc20__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CosmosOriginated => {
                            if cosmos_originated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosOriginated"));
                            }
                            cosmos_originated__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDenomToErc20Response {
                    erc20: erc20__.unwrap_or_default(),
                    cosmos_originated: cosmos_originated__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryDenomToERC20Response",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryErc20ToDenomRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.erc20.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryERC20ToDenomRequest", len)?;
        if !self.erc20.is_empty() {
            struct_ser.serialize_field("erc20", &self.erc20)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryErc20ToDenomRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["erc20"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Erc20,
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
                            "erc20" => Ok(GeneratedField::Erc20),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryErc20ToDenomRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryERC20ToDenomRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryErc20ToDenomRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut erc20__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Erc20 => {
                            if erc20__.is_some() {
                                return Err(serde::de::Error::duplicate_field("erc20"));
                            }
                            erc20__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryErc20ToDenomRequest {
                    erc20: erc20__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryERC20ToDenomRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryErc20ToDenomResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        if self.cosmos_originated {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryERC20ToDenomResponse", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if self.cosmos_originated {
            struct_ser.serialize_field("cosmosOriginated", &self.cosmos_originated)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryErc20ToDenomResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["denom", "cosmos_originated", "cosmosOriginated"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            CosmosOriginated,
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
                            "denom" => Ok(GeneratedField::Denom),
                            "cosmosOriginated" | "cosmos_originated" => {
                                Ok(GeneratedField::CosmosOriginated)
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
            type Value = QueryErc20ToDenomResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryERC20ToDenomResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryErc20ToDenomResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut cosmos_originated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CosmosOriginated => {
                            if cosmos_originated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosOriginated"));
                            }
                            cosmos_originated__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryErc20ToDenomResponse {
                    denom: denom__.unwrap_or_default(),
                    cosmos_originated: cosmos_originated__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryERC20ToDenomResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLastEventByAddrRequest {
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
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryLastEventByAddrRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLastEventByAddrRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLastEventByAddrRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryLastEventByAddrRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLastEventByAddrRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLastEventByAddrRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryLastEventByAddrRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLastEventByAddrResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.last_claim_event.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryLastEventByAddrResponse", len)?;
        if let Some(v) = self.last_claim_event.as_ref() {
            struct_ser.serialize_field("lastClaimEvent", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLastEventByAddrResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["last_claim_event", "lastClaimEvent"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LastClaimEvent,
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
                            "lastClaimEvent" | "last_claim_event" => {
                                Ok(GeneratedField::LastClaimEvent)
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
            type Value = QueryLastEventByAddrResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryLastEventByAddrResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLastEventByAddrResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut last_claim_event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LastClaimEvent => {
                            if last_claim_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastClaimEvent"));
                            }
                            last_claim_event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLastEventByAddrResponse {
                    last_claim_event: last_claim_event__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryLastEventByAddrResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLastPendingBatchRequestByAddrRequest {
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
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryLastPendingBatchRequestByAddrRequest",
            len,
        )?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLastPendingBatchRequestByAddrRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLastPendingBatchRequestByAddrRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.peggy.v1.QueryLastPendingBatchRequestByAddrRequest",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLastPendingBatchRequestByAddrRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLastPendingBatchRequestByAddrRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryLastPendingBatchRequestByAddrRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLastPendingBatchRequestByAddrResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryLastPendingBatchRequestByAddrResponse",
            len,
        )?;
        if let Some(v) = self.batch.as_ref() {
            struct_ser.serialize_field("batch", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLastPendingBatchRequestByAddrResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["batch"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batch,
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
                            "batch" => Ok(GeneratedField::Batch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLastPendingBatchRequestByAddrResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.peggy.v1.QueryLastPendingBatchRequestByAddrResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLastPendingBatchRequestByAddrResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut batch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Batch => {
                            if batch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch"));
                            }
                            batch__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLastPendingBatchRequestByAddrResponse { batch: batch__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryLastPendingBatchRequestByAddrResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLastPendingValsetRequestByAddrRequest {
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
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryLastPendingValsetRequestByAddrRequest",
            len,
        )?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLastPendingValsetRequestByAddrRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLastPendingValsetRequestByAddrRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.peggy.v1.QueryLastPendingValsetRequestByAddrRequest",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLastPendingValsetRequestByAddrRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLastPendingValsetRequestByAddrRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryLastPendingValsetRequestByAddrRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLastPendingValsetRequestByAddrResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.valsets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.peggy.v1.QueryLastPendingValsetRequestByAddrResponse",
            len,
        )?;
        if !self.valsets.is_empty() {
            struct_ser.serialize_field("valsets", &self.valsets)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLastPendingValsetRequestByAddrResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["valsets"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Valsets,
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
                            "valsets" => Ok(GeneratedField::Valsets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLastPendingValsetRequestByAddrResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.peggy.v1.QueryLastPendingValsetRequestByAddrResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLastPendingValsetRequestByAddrResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut valsets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Valsets => {
                            if valsets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsets"));
                            }
                            valsets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLastPendingValsetRequestByAddrResponse {
                    valsets: valsets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryLastPendingValsetRequestByAddrResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLastValsetRequestsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.peggy.v1.QueryLastValsetRequestsRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLastValsetRequestsRequest {
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
            type Value = QueryLastValsetRequestsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryLastValsetRequestsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLastValsetRequestsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryLastValsetRequestsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryLastValsetRequestsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLastValsetRequestsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.valsets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.peggy.v1.QueryLastValsetRequestsResponse", len)?;
        if !self.valsets.is_empty() {
            struct_ser.serialize_field("valsets", &self.valsets)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLastValsetRequestsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["valsets"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Valsets,
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
                            "valsets" => Ok(GeneratedField::Valsets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLastValsetRequestsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryLastValsetRequestsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryLastValsetRequestsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut valsets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Valsets => {
                            if valsets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valsets"));
                            }
                            valsets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLastValsetRequestsResponse {
                    valsets: valsets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryLastValsetRequestsResponse",
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
            serializer.serialize_struct("injective.peggy.v1.QueryModuleStateRequest", len)?;
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
                formatter.write_str("struct injective.peggy.v1.QueryModuleStateRequest")
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
            "injective.peggy.v1.QueryModuleStateRequest",
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
            serializer.serialize_struct("injective.peggy.v1.QueryModuleStateResponse", len)?;
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
                formatter.write_str("struct injective.peggy.v1.QueryModuleStateResponse")
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
            "injective.peggy.v1.QueryModuleStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOutgoingTxBatchesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryOutgoingTxBatchesRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOutgoingTxBatchesRequest {
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
            type Value = QueryOutgoingTxBatchesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryOutgoingTxBatchesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOutgoingTxBatchesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryOutgoingTxBatchesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryOutgoingTxBatchesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOutgoingTxBatchesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batches.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.peggy.v1.QueryOutgoingTxBatchesResponse", len)?;
        if !self.batches.is_empty() {
            struct_ser.serialize_field("batches", &self.batches)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOutgoingTxBatchesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["batches"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batches,
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
                            "batches" => Ok(GeneratedField::Batches),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOutgoingTxBatchesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryOutgoingTxBatchesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryOutgoingTxBatchesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut batches__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Batches => {
                            if batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            batches__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOutgoingTxBatchesResponse {
                    batches: batches__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryOutgoingTxBatchesResponse",
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
            serializer.serialize_struct("injective.peggy.v1.QueryParamsRequest", len)?;
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
                formatter.write_str("struct injective.peggy.v1.QueryParamsRequest")
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
            "injective.peggy.v1.QueryParamsRequest",
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
            serializer.serialize_struct("injective.peggy.v1.QueryParamsResponse", len)?;
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
                formatter.write_str("struct injective.peggy.v1.QueryParamsResponse")
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
            "injective.peggy.v1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPendingSendToEth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryPendingSendToEth", len)?;
        if !self.sender_address.is_empty() {
            struct_ser.serialize_field("senderAddress", &self.sender_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPendingSendToEth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender_address", "senderAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SenderAddress,
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
                            "senderAddress" | "sender_address" => Ok(GeneratedField::SenderAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPendingSendToEth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryPendingSendToEth")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPendingSendToEth, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SenderAddress => {
                            if sender_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderAddress"));
                            }
                            sender_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPendingSendToEth {
                    sender_address: sender_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryPendingSendToEth",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPendingSendToEthResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transfers_in_batches.is_empty() {
            len += 1;
        }
        if !self.unbatched_transfers.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryPendingSendToEthResponse", len)?;
        if !self.transfers_in_batches.is_empty() {
            struct_ser.serialize_field("transfersInBatches", &self.transfers_in_batches)?;
        }
        if !self.unbatched_transfers.is_empty() {
            struct_ser.serialize_field("unbatchedTransfers", &self.unbatched_transfers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPendingSendToEthResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transfers_in_batches",
            "transfersInBatches",
            "unbatched_transfers",
            "unbatchedTransfers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransfersInBatches,
            UnbatchedTransfers,
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
                            "transfersInBatches" | "transfers_in_batches" => {
                                Ok(GeneratedField::TransfersInBatches)
                            }
                            "unbatchedTransfers" | "unbatched_transfers" => {
                                Ok(GeneratedField::UnbatchedTransfers)
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
            type Value = QueryPendingSendToEthResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryPendingSendToEthResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPendingSendToEthResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut transfers_in_batches__ = None;
                let mut unbatched_transfers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransfersInBatches => {
                            if transfers_in_batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "transfersInBatches",
                                ));
                            }
                            transfers_in_batches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnbatchedTransfers => {
                            if unbatched_transfers__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unbatchedTransfers",
                                ));
                            }
                            unbatched_transfers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPendingSendToEthResponse {
                    transfers_in_batches: transfers_in_batches__.unwrap_or_default(),
                    unbatched_transfers: unbatched_transfers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryPendingSendToEthResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryValsetConfirmRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryValsetConfirmRequest", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryValsetConfirmRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nonce", "address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
            Address,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValsetConfirmRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryValsetConfirmRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryValsetConfirmRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryValsetConfirmRequest {
                    nonce: nonce__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryValsetConfirmRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryValsetConfirmResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.confirm.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryValsetConfirmResponse", len)?;
        if let Some(v) = self.confirm.as_ref() {
            struct_ser.serialize_field("confirm", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryValsetConfirmResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["confirm"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Confirm,
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
                            "confirm" => Ok(GeneratedField::Confirm),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValsetConfirmResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryValsetConfirmResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryValsetConfirmResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut confirm__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Confirm => {
                            if confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confirm"));
                            }
                            confirm__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryValsetConfirmResponse { confirm: confirm__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryValsetConfirmResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryValsetConfirmsByNonceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.peggy.v1.QueryValsetConfirmsByNonceRequest", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryValsetConfirmsByNonceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nonce"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValsetConfirmsByNonceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryValsetConfirmsByNonceRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryValsetConfirmsByNonceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryValsetConfirmsByNonceRequest {
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryValsetConfirmsByNonceRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryValsetConfirmsByNonceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.confirms.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.peggy.v1.QueryValsetConfirmsByNonceResponse", len)?;
        if !self.confirms.is_empty() {
            struct_ser.serialize_field("confirms", &self.confirms)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryValsetConfirmsByNonceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["confirms"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Confirms,
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
                            "confirms" => Ok(GeneratedField::Confirms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValsetConfirmsByNonceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryValsetConfirmsByNonceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryValsetConfirmsByNonceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut confirms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Confirms => {
                            if confirms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confirms"));
                            }
                            confirms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryValsetConfirmsByNonceResponse {
                    confirms: confirms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryValsetConfirmsByNonceResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryValsetRequestRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryValsetRequestRequest", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryValsetRequestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nonce"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValsetRequestRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryValsetRequestRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryValsetRequestRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryValsetRequestRequest {
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryValsetRequestRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryValsetRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.valset.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.peggy.v1.QueryValsetRequestResponse", len)?;
        if let Some(v) = self.valset.as_ref() {
            struct_ser.serialize_field("valset", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryValsetRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["valset"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Valset,
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
                            "valset" => Ok(GeneratedField::Valset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValsetRequestResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.QueryValsetRequestResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryValsetRequestResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut valset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Valset => {
                            if valset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valset"));
                            }
                            valset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryValsetRequestResponse { valset: valset__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.peggy.v1.QueryValsetRequestResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SignType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "SIGN_TYPE_UNKNOWN",
            Self::OrchestratorSignedMultiSigUpdate => {
                "SIGN_TYPE_ORCHESTRATOR_SIGNED_MULTI_SIG_UPDATE"
            }
            Self::OrchestratorSignedWithdrawBatch => "SIGN_TYPE_ORCHESTRATOR_SIGNED_WITHDRAW_BATCH",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SignType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGN_TYPE_UNKNOWN",
            "SIGN_TYPE_ORCHESTRATOR_SIGNED_MULTI_SIG_UPDATE",
            "SIGN_TYPE_ORCHESTRATOR_SIGNED_WITHDRAW_BATCH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignType;

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
                    "SIGN_TYPE_UNKNOWN" => Ok(SignType::Unknown),
                    "SIGN_TYPE_ORCHESTRATOR_SIGNED_MULTI_SIG_UPDATE" => {
                        Ok(SignType::OrchestratorSignedMultiSigUpdate)
                    }
                    "SIGN_TYPE_ORCHESTRATOR_SIGNED_WITHDRAW_BATCH" => {
                        Ok(SignType::OrchestratorSignedWithdrawBatch)
                    }
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Valset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        if !self.members.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if !self.reward_amount.is_empty() {
            len += 1;
        }
        if !self.reward_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.peggy.v1.Valset", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if !self.reward_amount.is_empty() {
            struct_ser.serialize_field("rewardAmount", &self.reward_amount)?;
        }
        if !self.reward_token.is_empty() {
            struct_ser.serialize_field("rewardToken", &self.reward_token)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Valset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nonce",
            "members",
            "height",
            "reward_amount",
            "rewardAmount",
            "reward_token",
            "rewardToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
            Members,
            Height,
            RewardAmount,
            RewardToken,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            "members" => Ok(GeneratedField::Members),
                            "height" => Ok(GeneratedField::Height),
                            "rewardAmount" | "reward_amount" => Ok(GeneratedField::RewardAmount),
                            "rewardToken" | "reward_token" => Ok(GeneratedField::RewardToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Valset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.peggy.v1.Valset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Valset, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                let mut members__ = None;
                let mut height__ = None;
                let mut reward_amount__ = None;
                let mut reward_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RewardAmount => {
                            if reward_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardAmount"));
                            }
                            reward_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RewardToken => {
                            if reward_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardToken"));
                            }
                            reward_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Valset {
                    nonce: nonce__.unwrap_or_default(),
                    members: members__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    reward_amount: reward_amount__.unwrap_or_default(),
                    reward_token: reward_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.peggy.v1.Valset", FIELDS, GeneratedVisitor)
    }
}
