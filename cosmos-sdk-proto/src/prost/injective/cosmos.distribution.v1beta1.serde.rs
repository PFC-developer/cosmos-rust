// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for CommunityPoolSpendProposal {
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
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "cosmos.distribution.v1beta1.CommunityPoolSpendProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CommunityPoolSpendProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "recipient", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Recipient,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "recipient" => Ok(GeneratedField::Recipient),
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
            type Value = CommunityPoolSpendProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.distribution.v1beta1.CommunityPoolSpendProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<CommunityPoolSpendProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut recipient__ = None;
                let mut amount__ = None;
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
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CommunityPoolSpendProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.CommunityPoolSpendProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CommunityPoolSpendProposalWithDeposit {
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
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.deposit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "cosmos.distribution.v1beta1.CommunityPoolSpendProposalWithDeposit",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.deposit.is_empty() {
            struct_ser.serialize_field("deposit", &self.deposit)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CommunityPoolSpendProposalWithDeposit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "recipient", "amount", "deposit"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Recipient,
            Amount,
            Deposit,
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
                            "recipient" => Ok(GeneratedField::Recipient),
                            "amount" => Ok(GeneratedField::Amount),
                            "deposit" => Ok(GeneratedField::Deposit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommunityPoolSpendProposalWithDeposit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct cosmos.distribution.v1beta1.CommunityPoolSpendProposalWithDeposit",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<CommunityPoolSpendProposalWithDeposit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut recipient__ = None;
                let mut amount__ = None;
                let mut deposit__ = None;
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
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Deposit => {
                            if deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposit"));
                            }
                            deposit__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CommunityPoolSpendProposalWithDeposit {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    deposit: deposit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.CommunityPoolSpendProposalWithDeposit",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DelegationDelegatorReward {
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
        if !self.reward.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.distribution.v1beta1.DelegationDelegatorReward", len)?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.reward.is_empty() {
            struct_ser.serialize_field("reward", &self.reward)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DelegationDelegatorReward {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_address", "validatorAddress", "reward"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
            Reward,
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
                            "reward" => Ok(GeneratedField::Reward),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DelegationDelegatorReward;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.distribution.v1beta1.DelegationDelegatorReward")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<DelegationDelegatorReward, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                let mut reward__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reward => {
                            if reward__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reward"));
                            }
                            reward__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DelegationDelegatorReward {
                    validator_address: validator_address__.unwrap_or_default(),
                    reward: reward__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.DelegationDelegatorReward",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DelegatorStartingInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.previous_period != 0 {
            len += 1;
        }
        if !self.stake.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.distribution.v1beta1.DelegatorStartingInfo", len)?;
        if self.previous_period != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "previousPeriod",
                ToString::to_string(&self.previous_period).as_str(),
            )?;
        }
        if !self.stake.is_empty() {
            struct_ser.serialize_field("stake", &self.stake)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DelegatorStartingInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["previous_period", "previousPeriod", "stake", "height"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviousPeriod,
            Stake,
            Height,
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
                            "previousPeriod" | "previous_period" => {
                                Ok(GeneratedField::PreviousPeriod)
                            }
                            "stake" => Ok(GeneratedField::Stake),
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DelegatorStartingInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.distribution.v1beta1.DelegatorStartingInfo")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<DelegatorStartingInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut previous_period__ = None;
                let mut stake__ = None;
                let mut height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreviousPeriod => {
                            if previous_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousPeriod"));
                            }
                            previous_period__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Stake => {
                            if stake__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stake"));
                            }
                            stake__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(DelegatorStartingInfo {
                    previous_period: previous_period__.unwrap_or_default(),
                    stake: stake__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.DelegatorStartingInfo",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeePool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.community_pool.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.distribution.v1beta1.FeePool", len)?;
        if !self.community_pool.is_empty() {
            struct_ser.serialize_field("communityPool", &self.community_pool)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeePool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["community_pool", "communityPool"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommunityPool,
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
                            "communityPool" | "community_pool" => Ok(GeneratedField::CommunityPool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeePool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.distribution.v1beta1.FeePool")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeePool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut community_pool__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CommunityPool => {
                            if community_pool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("communityPool"));
                            }
                            community_pool__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FeePool {
                    community_pool: community_pool__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.FeePool",
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
        if !self.community_tax.is_empty() {
            len += 1;
        }
        if !self.base_proposer_reward.is_empty() {
            len += 1;
        }
        if !self.bonus_proposer_reward.is_empty() {
            len += 1;
        }
        if self.withdraw_addr_enabled {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.distribution.v1beta1.Params", len)?;
        if !self.community_tax.is_empty() {
            struct_ser.serialize_field("communityTax", &self.community_tax)?;
        }
        if !self.base_proposer_reward.is_empty() {
            struct_ser.serialize_field("baseProposerReward", &self.base_proposer_reward)?;
        }
        if !self.bonus_proposer_reward.is_empty() {
            struct_ser.serialize_field("bonusProposerReward", &self.bonus_proposer_reward)?;
        }
        if self.withdraw_addr_enabled {
            struct_ser.serialize_field("withdrawAddrEnabled", &self.withdraw_addr_enabled)?;
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
            "community_tax",
            "communityTax",
            "base_proposer_reward",
            "baseProposerReward",
            "bonus_proposer_reward",
            "bonusProposerReward",
            "withdraw_addr_enabled",
            "withdrawAddrEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommunityTax,
            BaseProposerReward,
            BonusProposerReward,
            WithdrawAddrEnabled,
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
                            "communityTax" | "community_tax" => Ok(GeneratedField::CommunityTax),
                            "baseProposerReward" | "base_proposer_reward" => {
                                Ok(GeneratedField::BaseProposerReward)
                            }
                            "bonusProposerReward" | "bonus_proposer_reward" => {
                                Ok(GeneratedField::BonusProposerReward)
                            }
                            "withdrawAddrEnabled" | "withdraw_addr_enabled" => {
                                Ok(GeneratedField::WithdrawAddrEnabled)
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
                formatter.write_str("struct cosmos.distribution.v1beta1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut community_tax__ = None;
                let mut base_proposer_reward__ = None;
                let mut bonus_proposer_reward__ = None;
                let mut withdraw_addr_enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CommunityTax => {
                            if community_tax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("communityTax"));
                            }
                            community_tax__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BaseProposerReward => {
                            if base_proposer_reward__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "baseProposerReward",
                                ));
                            }
                            base_proposer_reward__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BonusProposerReward => {
                            if bonus_proposer_reward__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bonusProposerReward",
                                ));
                            }
                            bonus_proposer_reward__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithdrawAddrEnabled => {
                            if withdraw_addr_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "withdrawAddrEnabled",
                                ));
                            }
                            withdraw_addr_enabled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    community_tax: community_tax__.unwrap_or_default(),
                    base_proposer_reward: base_proposer_reward__.unwrap_or_default(),
                    bonus_proposer_reward: bonus_proposer_reward__.unwrap_or_default(),
                    withdraw_addr_enabled: withdraw_addr_enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.Params",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ValidatorAccumulatedCommission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.commission.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "cosmos.distribution.v1beta1.ValidatorAccumulatedCommission",
            len,
        )?;
        if !self.commission.is_empty() {
            struct_ser.serialize_field("commission", &self.commission)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ValidatorAccumulatedCommission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["commission"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Commission,
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
                            "commission" => Ok(GeneratedField::Commission),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorAccumulatedCommission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.distribution.v1beta1.ValidatorAccumulatedCommission")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ValidatorAccumulatedCommission, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut commission__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Commission => {
                            if commission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commission"));
                            }
                            commission__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidatorAccumulatedCommission {
                    commission: commission__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.ValidatorAccumulatedCommission",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ValidatorCurrentRewards {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rewards.is_empty() {
            len += 1;
        }
        if self.period != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.distribution.v1beta1.ValidatorCurrentRewards", len)?;
        if !self.rewards.is_empty() {
            struct_ser.serialize_field("rewards", &self.rewards)?;
        }
        if self.period != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("period", ToString::to_string(&self.period).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ValidatorCurrentRewards {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["rewards", "period"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rewards,
            Period,
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
                            "rewards" => Ok(GeneratedField::Rewards),
                            "period" => Ok(GeneratedField::Period),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorCurrentRewards;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.distribution.v1beta1.ValidatorCurrentRewards")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ValidatorCurrentRewards, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut rewards__ = None;
                let mut period__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rewards => {
                            if rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewards"));
                            }
                            rewards__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Period => {
                            if period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            period__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ValidatorCurrentRewards {
                    rewards: rewards__.unwrap_or_default(),
                    period: period__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.ValidatorCurrentRewards",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ValidatorHistoricalRewards {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cumulative_reward_ratio.is_empty() {
            len += 1;
        }
        if self.reference_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "cosmos.distribution.v1beta1.ValidatorHistoricalRewards",
            len,
        )?;
        if !self.cumulative_reward_ratio.is_empty() {
            struct_ser.serialize_field("cumulativeRewardRatio", &self.cumulative_reward_ratio)?;
        }
        if self.reference_count != 0 {
            struct_ser.serialize_field("referenceCount", &self.reference_count)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ValidatorHistoricalRewards {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cumulative_reward_ratio",
            "cumulativeRewardRatio",
            "reference_count",
            "referenceCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CumulativeRewardRatio,
            ReferenceCount,
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
                            "cumulativeRewardRatio" | "cumulative_reward_ratio" => {
                                Ok(GeneratedField::CumulativeRewardRatio)
                            }
                            "referenceCount" | "reference_count" => {
                                Ok(GeneratedField::ReferenceCount)
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
            type Value = ValidatorHistoricalRewards;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.distribution.v1beta1.ValidatorHistoricalRewards")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ValidatorHistoricalRewards, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cumulative_reward_ratio__ = None;
                let mut reference_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CumulativeRewardRatio => {
                            if cumulative_reward_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "cumulativeRewardRatio",
                                ));
                            }
                            cumulative_reward_ratio__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReferenceCount => {
                            if reference_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceCount"));
                            }
                            reference_count__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(ValidatorHistoricalRewards {
                    cumulative_reward_ratio: cumulative_reward_ratio__.unwrap_or_default(),
                    reference_count: reference_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.ValidatorHistoricalRewards",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ValidatorOutstandingRewards {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rewards.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "cosmos.distribution.v1beta1.ValidatorOutstandingRewards",
            len,
        )?;
        if !self.rewards.is_empty() {
            struct_ser.serialize_field("rewards", &self.rewards)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ValidatorOutstandingRewards {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["rewards"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rewards,
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
                            "rewards" => Ok(GeneratedField::Rewards),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorOutstandingRewards;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.distribution.v1beta1.ValidatorOutstandingRewards")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ValidatorOutstandingRewards, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut rewards__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rewards => {
                            if rewards__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewards"));
                            }
                            rewards__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidatorOutstandingRewards {
                    rewards: rewards__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.ValidatorOutstandingRewards",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ValidatorSlashEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.validator_period != 0 {
            len += 1;
        }
        if !self.fraction.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.distribution.v1beta1.ValidatorSlashEvent", len)?;
        if self.validator_period != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "validatorPeriod",
                ToString::to_string(&self.validator_period).as_str(),
            )?;
        }
        if !self.fraction.is_empty() {
            struct_ser.serialize_field("fraction", &self.fraction)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ValidatorSlashEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_period", "validatorPeriod", "fraction"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorPeriod,
            Fraction,
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
                            "validatorPeriod" | "validator_period" => {
                                Ok(GeneratedField::ValidatorPeriod)
                            }
                            "fraction" => Ok(GeneratedField::Fraction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorSlashEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.distribution.v1beta1.ValidatorSlashEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidatorSlashEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_period__ = None;
                let mut fraction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidatorPeriod => {
                            if validator_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorPeriod"));
                            }
                            validator_period__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Fraction => {
                            if fraction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fraction"));
                            }
                            fraction__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidatorSlashEvent {
                    validator_period: validator_period__.unwrap_or_default(),
                    fraction: fraction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.ValidatorSlashEvent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ValidatorSlashEvents {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_slash_events.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.distribution.v1beta1.ValidatorSlashEvents", len)?;
        if !self.validator_slash_events.is_empty() {
            struct_ser.serialize_field("validatorSlashEvents", &self.validator_slash_events)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ValidatorSlashEvents {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_slash_events", "validatorSlashEvents"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorSlashEvents,
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
                            "validatorSlashEvents" | "validator_slash_events" => {
                                Ok(GeneratedField::ValidatorSlashEvents)
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
            type Value = ValidatorSlashEvents;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.distribution.v1beta1.ValidatorSlashEvents")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ValidatorSlashEvents, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_slash_events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidatorSlashEvents => {
                            if validator_slash_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "validatorSlashEvents",
                                ));
                            }
                            validator_slash_events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidatorSlashEvents {
                    validator_slash_events: validator_slash_events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.distribution.v1beta1.ValidatorSlashEvents",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
