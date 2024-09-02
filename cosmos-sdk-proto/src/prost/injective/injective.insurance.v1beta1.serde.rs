// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for EventInsuranceFundUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fund.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.EventInsuranceFundUpdate", len)?;
        if let Some(v) = self.fund.as_ref() {
            struct_ser.serialize_field("fund", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventInsuranceFundUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["fund"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fund,
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
                            "fund" => Ok(GeneratedField::Fund),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventInsuranceFundUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.EventInsuranceFundUpdate")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventInsuranceFundUpdate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut fund__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fund => {
                            if fund__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fund"));
                            }
                            fund__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventInsuranceFundUpdate { fund: fund__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.EventInsuranceFundUpdate",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventInsuranceWithdraw {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.market_id.is_empty() {
            len += 1;
        }
        if !self.market_ticker.is_empty() {
            len += 1;
        }
        if self.withdrawal.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.EventInsuranceWithdraw", len)?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if !self.market_ticker.is_empty() {
            struct_ser.serialize_field("marketTicker", &self.market_ticker)?;
        }
        if let Some(v) = self.withdrawal.as_ref() {
            struct_ser.serialize_field("withdrawal", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventInsuranceWithdraw {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "market_id",
            "marketId",
            "market_ticker",
            "marketTicker",
            "withdrawal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
            MarketTicker,
            Withdrawal,
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
                            "marketId" | "market_id" => Ok(GeneratedField::MarketId),
                            "marketTicker" | "market_ticker" => Ok(GeneratedField::MarketTicker),
                            "withdrawal" => Ok(GeneratedField::Withdrawal),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventInsuranceWithdraw;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.EventInsuranceWithdraw")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventInsuranceWithdraw, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut market_ticker__ = None;
                let mut withdrawal__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketTicker => {
                            if market_ticker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketTicker"));
                            }
                            market_ticker__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Withdrawal => {
                            if withdrawal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawal"));
                            }
                            withdrawal__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventInsuranceWithdraw {
                    market_id: market_id__.unwrap_or_default(),
                    market_ticker: market_ticker__.unwrap_or_default(),
                    withdrawal: withdrawal__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.EventInsuranceWithdraw",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventRequestRedemption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.schedule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.EventRequestRedemption", len)?;
        if let Some(v) = self.schedule.as_ref() {
            struct_ser.serialize_field("schedule", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventRequestRedemption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["schedule"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schedule,
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
                            "schedule" => Ok(GeneratedField::Schedule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventRequestRedemption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.EventRequestRedemption")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventRequestRedemption, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut schedule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Schedule => {
                            if schedule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            schedule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventRequestRedemption {
                    schedule: schedule__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.EventRequestRedemption",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventUnderwrite {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.underwriter.is_empty() {
            len += 1;
        }
        if !self.market_id.is_empty() {
            len += 1;
        }
        if self.deposit.is_some() {
            len += 1;
        }
        if self.shares.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.insurance.v1beta1.EventUnderwrite", len)?;
        if !self.underwriter.is_empty() {
            struct_ser.serialize_field("underwriter", &self.underwriter)?;
        }
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        if let Some(v) = self.shares.as_ref() {
            struct_ser.serialize_field("shares", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventUnderwrite {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["underwriter", "marketId", "deposit", "shares"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Underwriter,
            MarketId,
            Deposit,
            Shares,
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
                            "underwriter" => Ok(GeneratedField::Underwriter),
                            "marketId" => Ok(GeneratedField::MarketId),
                            "deposit" => Ok(GeneratedField::Deposit),
                            "shares" => Ok(GeneratedField::Shares),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventUnderwrite;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.EventUnderwrite")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventUnderwrite, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut underwriter__ = None;
                let mut market_id__ = None;
                let mut deposit__ = None;
                let mut shares__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Underwriter => {
                            if underwriter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("underwriter"));
                            }
                            underwriter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Deposit => {
                            if deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposit"));
                            }
                            deposit__ = map_.next_value()?;
                        }
                        GeneratedField::Shares => {
                            if shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shares"));
                            }
                            shares__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventUnderwrite {
                    underwriter: underwriter__.unwrap_or_default(),
                    market_id: market_id__.unwrap_or_default(),
                    deposit: deposit__,
                    shares: shares__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.EventUnderwrite",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventWithdrawRedemption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.schedule.is_some() {
            len += 1;
        }
        if self.redeem_coin.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.EventWithdrawRedemption", len)?;
        if let Some(v) = self.schedule.as_ref() {
            struct_ser.serialize_field("schedule", v)?;
        }
        if let Some(v) = self.redeem_coin.as_ref() {
            struct_ser.serialize_field("redeemCoin", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventWithdrawRedemption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["schedule", "redeem_coin", "redeemCoin"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schedule,
            RedeemCoin,
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
                            "schedule" => Ok(GeneratedField::Schedule),
                            "redeemCoin" | "redeem_coin" => Ok(GeneratedField::RedeemCoin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventWithdrawRedemption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.EventWithdrawRedemption")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventWithdrawRedemption, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut schedule__ = None;
                let mut redeem_coin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Schedule => {
                            if schedule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            schedule__ = map_.next_value()?;
                        }
                        GeneratedField::RedeemCoin => {
                            if redeem_coin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redeemCoin"));
                            }
                            redeem_coin__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventWithdrawRedemption {
                    schedule: schedule__,
                    redeem_coin: redeem_coin__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.EventWithdrawRedemption",
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
        if !self.insurance_funds.is_empty() {
            len += 1;
        }
        if !self.redemption_schedule.is_empty() {
            len += 1;
        }
        if self.next_share_denom_id != 0 {
            len += 1;
        }
        if self.next_redemption_schedule_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.insurance.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.insurance_funds.is_empty() {
            struct_ser.serialize_field("insuranceFunds", &self.insurance_funds)?;
        }
        if !self.redemption_schedule.is_empty() {
            struct_ser.serialize_field("redemptionSchedule", &self.redemption_schedule)?;
        }
        if self.next_share_denom_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "nextShareDenomId",
                ToString::to_string(&self.next_share_denom_id).as_str(),
            )?;
        }
        if self.next_redemption_schedule_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "nextRedemptionScheduleId",
                ToString::to_string(&self.next_redemption_schedule_id).as_str(),
            )?;
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
            "insurance_funds",
            "insuranceFunds",
            "redemption_schedule",
            "redemptionSchedule",
            "next_share_denom_id",
            "nextShareDenomId",
            "next_redemption_schedule_id",
            "nextRedemptionScheduleId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            InsuranceFunds,
            RedemptionSchedule,
            NextShareDenomId,
            NextRedemptionScheduleId,
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
                            "insuranceFunds" | "insurance_funds" => {
                                Ok(GeneratedField::InsuranceFunds)
                            }
                            "redemptionSchedule" | "redemption_schedule" => {
                                Ok(GeneratedField::RedemptionSchedule)
                            }
                            "nextShareDenomId" | "next_share_denom_id" => {
                                Ok(GeneratedField::NextShareDenomId)
                            }
                            "nextRedemptionScheduleId" | "next_redemption_schedule_id" => {
                                Ok(GeneratedField::NextRedemptionScheduleId)
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
                formatter.write_str("struct injective.insurance.v1beta1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut insurance_funds__ = None;
                let mut redemption_schedule__ = None;
                let mut next_share_denom_id__ = None;
                let mut next_redemption_schedule_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::InsuranceFunds => {
                            if insurance_funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insuranceFunds"));
                            }
                            insurance_funds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RedemptionSchedule => {
                            if redemption_schedule__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "redemptionSchedule",
                                ));
                            }
                            redemption_schedule__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextShareDenomId => {
                            if next_share_denom_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextShareDenomId"));
                            }
                            next_share_denom_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::NextRedemptionScheduleId => {
                            if next_redemption_schedule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "nextRedemptionScheduleId",
                                ));
                            }
                            next_redemption_schedule_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    insurance_funds: insurance_funds__.unwrap_or_default(),
                    redemption_schedule: redemption_schedule__.unwrap_or_default(),
                    next_share_denom_id: next_share_denom_id__.unwrap_or_default(),
                    next_redemption_schedule_id: next_redemption_schedule_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.GenesisState",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for InsuranceFund {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deposit_denom.is_empty() {
            len += 1;
        }
        if !self.insurance_pool_token_denom.is_empty() {
            len += 1;
        }
        if self.redemption_notice_period_duration.is_some() {
            len += 1;
        }
        if !self.balance.is_empty() {
            len += 1;
        }
        if !self.total_share.is_empty() {
            len += 1;
        }
        if !self.market_id.is_empty() {
            len += 1;
        }
        if !self.market_ticker.is_empty() {
            len += 1;
        }
        if !self.oracle_base.is_empty() {
            len += 1;
        }
        if !self.oracle_quote.is_empty() {
            len += 1;
        }
        if self.oracle_type != 0 {
            len += 1;
        }
        if self.expiry != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.insurance.v1beta1.InsuranceFund", len)?;
        if !self.deposit_denom.is_empty() {
            struct_ser.serialize_field("depositDenom", &self.deposit_denom)?;
        }
        if !self.insurance_pool_token_denom.is_empty() {
            struct_ser
                .serialize_field("insurancePoolTokenDenom", &self.insurance_pool_token_denom)?;
        }
        if let Some(v) = self.redemption_notice_period_duration.as_ref() {
            struct_ser.serialize_field("redemptionNoticePeriodDuration", v)?;
        }
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        if !self.total_share.is_empty() {
            struct_ser.serialize_field("totalShare", &self.total_share)?;
        }
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if !self.market_ticker.is_empty() {
            struct_ser.serialize_field("marketTicker", &self.market_ticker)?;
        }
        if !self.oracle_base.is_empty() {
            struct_ser.serialize_field("oracleBase", &self.oracle_base)?;
        }
        if !self.oracle_quote.is_empty() {
            struct_ser.serialize_field("oracleQuote", &self.oracle_quote)?;
        }
        if self.oracle_type != 0 {
            let v = super::super::oracle::v1beta1::OracleType::try_from(self.oracle_type).map_err(
                |_| serde::ser::Error::custom(format!("Invalid variant {}", self.oracle_type)),
            )?;
            struct_ser.serialize_field("oracleType", &v)?;
        }
        if self.expiry != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("expiry", ToString::to_string(&self.expiry).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for InsuranceFund {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposit_denom",
            "depositDenom",
            "insurance_pool_token_denom",
            "insurancePoolTokenDenom",
            "redemption_notice_period_duration",
            "redemptionNoticePeriodDuration",
            "balance",
            "total_share",
            "totalShare",
            "market_id",
            "marketId",
            "market_ticker",
            "marketTicker",
            "oracle_base",
            "oracleBase",
            "oracle_quote",
            "oracleQuote",
            "oracle_type",
            "oracleType",
            "expiry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DepositDenom,
            InsurancePoolTokenDenom,
            RedemptionNoticePeriodDuration,
            Balance,
            TotalShare,
            MarketId,
            MarketTicker,
            OracleBase,
            OracleQuote,
            OracleType,
            Expiry,
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
                            "depositDenom" | "deposit_denom" => Ok(GeneratedField::DepositDenom),
                            "insurancePoolTokenDenom" | "insurance_pool_token_denom" => {
                                Ok(GeneratedField::InsurancePoolTokenDenom)
                            }
                            "redemptionNoticePeriodDuration"
                            | "redemption_notice_period_duration" => {
                                Ok(GeneratedField::RedemptionNoticePeriodDuration)
                            }
                            "balance" => Ok(GeneratedField::Balance),
                            "totalShare" | "total_share" => Ok(GeneratedField::TotalShare),
                            "marketId" | "market_id" => Ok(GeneratedField::MarketId),
                            "marketTicker" | "market_ticker" => Ok(GeneratedField::MarketTicker),
                            "oracleBase" | "oracle_base" => Ok(GeneratedField::OracleBase),
                            "oracleQuote" | "oracle_quote" => Ok(GeneratedField::OracleQuote),
                            "oracleType" | "oracle_type" => Ok(GeneratedField::OracleType),
                            "expiry" => Ok(GeneratedField::Expiry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InsuranceFund;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.InsuranceFund")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InsuranceFund, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut deposit_denom__ = None;
                let mut insurance_pool_token_denom__ = None;
                let mut redemption_notice_period_duration__ = None;
                let mut balance__ = None;
                let mut total_share__ = None;
                let mut market_id__ = None;
                let mut market_ticker__ = None;
                let mut oracle_base__ = None;
                let mut oracle_quote__ = None;
                let mut oracle_type__ = None;
                let mut expiry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DepositDenom => {
                            if deposit_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositDenom"));
                            }
                            deposit_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InsurancePoolTokenDenom => {
                            if insurance_pool_token_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "insurancePoolTokenDenom",
                                ));
                            }
                            insurance_pool_token_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RedemptionNoticePeriodDuration => {
                            if redemption_notice_period_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "redemptionNoticePeriodDuration",
                                ));
                            }
                            redemption_notice_period_duration__ = map_.next_value()?;
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalShare => {
                            if total_share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalShare"));
                            }
                            total_share__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketTicker => {
                            if market_ticker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketTicker"));
                            }
                            market_ticker__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleBase => {
                            if oracle_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleBase"));
                            }
                            oracle_base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleQuote => {
                            if oracle_quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleQuote"));
                            }
                            oracle_quote__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleType => {
                            if oracle_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleType"));
                            }
                            oracle_type__ = Some(
                                map_.next_value::<super::super::oracle::v1beta1::OracleType>()?
                                    as i32,
                            );
                        }
                        GeneratedField::Expiry => {
                            if expiry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiry"));
                            }
                            expiry__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(InsuranceFund {
                    deposit_denom: deposit_denom__.unwrap_or_default(),
                    insurance_pool_token_denom: insurance_pool_token_denom__.unwrap_or_default(),
                    redemption_notice_period_duration: redemption_notice_period_duration__,
                    balance: balance__.unwrap_or_default(),
                    total_share: total_share__.unwrap_or_default(),
                    market_id: market_id__.unwrap_or_default(),
                    market_ticker: market_ticker__.unwrap_or_default(),
                    oracle_base: oracle_base__.unwrap_or_default(),
                    oracle_quote: oracle_quote__.unwrap_or_default(),
                    oracle_type: oracle_type__.unwrap_or_default(),
                    expiry: expiry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.InsuranceFund",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreateInsuranceFund {
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
        if !self.ticker.is_empty() {
            len += 1;
        }
        if !self.quote_denom.is_empty() {
            len += 1;
        }
        if !self.oracle_base.is_empty() {
            len += 1;
        }
        if !self.oracle_quote.is_empty() {
            len += 1;
        }
        if self.oracle_type != 0 {
            len += 1;
        }
        if self.expiry != 0 {
            len += 1;
        }
        if self.initial_deposit.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.MsgCreateInsuranceFund", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.ticker.is_empty() {
            struct_ser.serialize_field("ticker", &self.ticker)?;
        }
        if !self.quote_denom.is_empty() {
            struct_ser.serialize_field("quoteDenom", &self.quote_denom)?;
        }
        if !self.oracle_base.is_empty() {
            struct_ser.serialize_field("oracleBase", &self.oracle_base)?;
        }
        if !self.oracle_quote.is_empty() {
            struct_ser.serialize_field("oracleQuote", &self.oracle_quote)?;
        }
        if self.oracle_type != 0 {
            let v = super::super::oracle::v1beta1::OracleType::try_from(self.oracle_type).map_err(
                |_| serde::ser::Error::custom(format!("Invalid variant {}", self.oracle_type)),
            )?;
            struct_ser.serialize_field("oracleType", &v)?;
        }
        if self.expiry != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("expiry", ToString::to_string(&self.expiry).as_str())?;
        }
        if let Some(v) = self.initial_deposit.as_ref() {
            struct_ser.serialize_field("initialDeposit", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreateInsuranceFund {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "ticker",
            "quote_denom",
            "quoteDenom",
            "oracle_base",
            "oracleBase",
            "oracle_quote",
            "oracleQuote",
            "oracle_type",
            "oracleType",
            "expiry",
            "initial_deposit",
            "initialDeposit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Ticker,
            QuoteDenom,
            OracleBase,
            OracleQuote,
            OracleType,
            Expiry,
            InitialDeposit,
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
                            "ticker" => Ok(GeneratedField::Ticker),
                            "quoteDenom" | "quote_denom" => Ok(GeneratedField::QuoteDenom),
                            "oracleBase" | "oracle_base" => Ok(GeneratedField::OracleBase),
                            "oracleQuote" | "oracle_quote" => Ok(GeneratedField::OracleQuote),
                            "oracleType" | "oracle_type" => Ok(GeneratedField::OracleType),
                            "expiry" => Ok(GeneratedField::Expiry),
                            "initialDeposit" | "initial_deposit" => {
                                Ok(GeneratedField::InitialDeposit)
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
            type Value = MsgCreateInsuranceFund;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.MsgCreateInsuranceFund")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgCreateInsuranceFund, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut ticker__ = None;
                let mut quote_denom__ = None;
                let mut oracle_base__ = None;
                let mut oracle_quote__ = None;
                let mut oracle_type__ = None;
                let mut expiry__ = None;
                let mut initial_deposit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ticker => {
                            if ticker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ticker"));
                            }
                            ticker__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuoteDenom => {
                            if quote_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteDenom"));
                            }
                            quote_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleBase => {
                            if oracle_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleBase"));
                            }
                            oracle_base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleQuote => {
                            if oracle_quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleQuote"));
                            }
                            oracle_quote__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleType => {
                            if oracle_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleType"));
                            }
                            oracle_type__ = Some(
                                map_.next_value::<super::super::oracle::v1beta1::OracleType>()?
                                    as i32,
                            );
                        }
                        GeneratedField::Expiry => {
                            if expiry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiry"));
                            }
                            expiry__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::InitialDeposit => {
                            if initial_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialDeposit"));
                            }
                            initial_deposit__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgCreateInsuranceFund {
                    sender: sender__.unwrap_or_default(),
                    ticker: ticker__.unwrap_or_default(),
                    quote_denom: quote_denom__.unwrap_or_default(),
                    oracle_base: oracle_base__.unwrap_or_default(),
                    oracle_quote: oracle_quote__.unwrap_or_default(),
                    oracle_type: oracle_type__.unwrap_or_default(),
                    expiry: expiry__.unwrap_or_default(),
                    initial_deposit: initial_deposit__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.MsgCreateInsuranceFund",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreateInsuranceFundResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.MsgCreateInsuranceFundResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreateInsuranceFundResponse {
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
            type Value = MsgCreateInsuranceFundResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.insurance.v1beta1.MsgCreateInsuranceFundResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgCreateInsuranceFundResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreateInsuranceFundResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.MsgCreateInsuranceFundResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRequestRedemption {
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
        if !self.market_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.insurance.v1beta1.MsgRequestRedemption", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRequestRedemption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "market_id", "marketId", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            MarketId,
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
                            "marketId" | "market_id" => Ok(GeneratedField::MarketId),
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
            type Value = MsgRequestRedemption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.MsgRequestRedemption")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRequestRedemption, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut market_id__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgRequestRedemption {
                    sender: sender__.unwrap_or_default(),
                    market_id: market_id__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.MsgRequestRedemption",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRequestRedemptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.MsgRequestRedemptionResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRequestRedemptionResponse {
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
            type Value = MsgRequestRedemptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.insurance.v1beta1.MsgRequestRedemptionResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRequestRedemptionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRequestRedemptionResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.MsgRequestRedemptionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUnderwrite {
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
        if !self.market_id.is_empty() {
            len += 1;
        }
        if self.deposit.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.insurance.v1beta1.MsgUnderwrite", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUnderwrite {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "market_id", "marketId", "deposit"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            MarketId,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "marketId" | "market_id" => Ok(GeneratedField::MarketId),
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
            type Value = MsgUnderwrite;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.MsgUnderwrite")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUnderwrite, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut market_id__ = None;
                let mut deposit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Deposit => {
                            if deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposit"));
                            }
                            deposit__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUnderwrite {
                    sender: sender__.unwrap_or_default(),
                    market_id: market_id__.unwrap_or_default(),
                    deposit: deposit__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.MsgUnderwrite",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUnderwriteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.MsgUnderwriteResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUnderwriteResponse {
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
            type Value = MsgUnderwriteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.MsgUnderwriteResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUnderwriteResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnderwriteResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.MsgUnderwriteResponse",
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
            serializer.serialize_struct("injective.insurance.v1beta1.MsgUpdateParams", len)?;
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
                formatter.write_str("struct injective.insurance.v1beta1.MsgUpdateParams")
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
            "injective.insurance.v1beta1.MsgUpdateParams",
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
        let struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct injective.insurance.v1beta1.MsgUpdateParamsResponse")
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
            "injective.insurance.v1beta1.MsgUpdateParamsResponse",
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
        if self.default_redemption_notice_period_duration.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.insurance.v1beta1.Params", len)?;
        if let Some(v) = self.default_redemption_notice_period_duration.as_ref() {
            struct_ser.serialize_field("defaultRedemptionNoticePeriodDuration", v)?;
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
            "default_redemption_notice_period_duration",
            "defaultRedemptionNoticePeriodDuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultRedemptionNoticePeriodDuration,
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
                            "defaultRedemptionNoticePeriodDuration"
                            | "default_redemption_notice_period_duration" => {
                                Ok(GeneratedField::DefaultRedemptionNoticePeriodDuration)
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
                formatter.write_str("struct injective.insurance.v1beta1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut default_redemption_notice_period_duration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DefaultRedemptionNoticePeriodDuration => {
                            if default_redemption_notice_period_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultRedemptionNoticePeriodDuration",
                                ));
                            }
                            default_redemption_notice_period_duration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Params {
                    default_redemption_notice_period_duration:
                        default_redemption_notice_period_duration__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.Params",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryEstimatedRedemptionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.market_id.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryEstimatedRedemptionsRequest",
            len,
        )?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryEstimatedRedemptionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["marketId", "address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
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
                            "marketId" => Ok(GeneratedField::MarketId),
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
            type Value = QueryEstimatedRedemptionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.insurance.v1beta1.QueryEstimatedRedemptionsRequest",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryEstimatedRedemptionsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryEstimatedRedemptionsRequest {
                    market_id: market_id__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryEstimatedRedemptionsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryEstimatedRedemptionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryEstimatedRedemptionsResponse",
            len,
        )?;
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryEstimatedRedemptionsResponse {
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
            type Value = QueryEstimatedRedemptionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct injective.insurance.v1beta1.QueryEstimatedRedemptionsResponse",
                )
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryEstimatedRedemptionsResponse, V::Error>
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
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryEstimatedRedemptionsResponse {
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryEstimatedRedemptionsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryInsuranceFundRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.market_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.QueryInsuranceFundRequest", len)?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryInsuranceFundRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["market_id", "marketId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
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
                            "marketId" | "market_id" => Ok(GeneratedField::MarketId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryInsuranceFundRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.QueryInsuranceFundRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryInsuranceFundRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryInsuranceFundRequest {
                    market_id: market_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceFundRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryInsuranceFundResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fund.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceFundResponse",
            len,
        )?;
        if let Some(v) = self.fund.as_ref() {
            struct_ser.serialize_field("fund", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryInsuranceFundResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["fund"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fund,
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
                            "fund" => Ok(GeneratedField::Fund),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryInsuranceFundResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.QueryInsuranceFundResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryInsuranceFundResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut fund__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fund => {
                            if fund__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fund"));
                            }
                            fund__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryInsuranceFundResponse { fund: fund__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceFundResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryInsuranceFundsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceFundsRequest",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryInsuranceFundsRequest {
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
            type Value = QueryInsuranceFundsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.QueryInsuranceFundsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryInsuranceFundsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryInsuranceFundsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceFundsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryInsuranceFundsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.funds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceFundsResponse",
            len,
        )?;
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryInsuranceFundsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["funds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryInsuranceFundsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.insurance.v1beta1.QueryInsuranceFundsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryInsuranceFundsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut funds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryInsuranceFundsResponse {
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceFundsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryInsuranceParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceParamsRequest",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryInsuranceParamsRequest {
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
            type Value = QueryInsuranceParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.insurance.v1beta1.QueryInsuranceParamsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryInsuranceParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryInsuranceParamsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryInsuranceParamsResponse {
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
        let mut struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceParamsResponse",
            len,
        )?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryInsuranceParamsResponse {
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
            type Value = QueryInsuranceParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.insurance.v1beta1.QueryInsuranceParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryInsuranceParamsResponse, V::Error>
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
                Ok(QueryInsuranceParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryInsuranceParamsResponse",
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
        let struct_ser = serializer
            .serialize_struct("injective.insurance.v1beta1.QueryModuleStateRequest", len)?;
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
                formatter.write_str("struct injective.insurance.v1beta1.QueryModuleStateRequest")
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
            "injective.insurance.v1beta1.QueryModuleStateRequest",
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
            .serialize_struct("injective.insurance.v1beta1.QueryModuleStateResponse", len)?;
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
                formatter.write_str("struct injective.insurance.v1beta1.QueryModuleStateResponse")
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
            "injective.insurance.v1beta1.QueryModuleStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPendingRedemptionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.market_id.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryPendingRedemptionsRequest",
            len,
        )?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPendingRedemptionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["marketId", "address"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
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
                            "marketId" => Ok(GeneratedField::MarketId),
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
            type Value = QueryPendingRedemptionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.insurance.v1beta1.QueryPendingRedemptionsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPendingRedemptionsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPendingRedemptionsRequest {
                    market_id: market_id__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryPendingRedemptionsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPendingRedemptionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.insurance.v1beta1.QueryPendingRedemptionsResponse",
            len,
        )?;
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPendingRedemptionsResponse {
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
            type Value = QueryPendingRedemptionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.insurance.v1beta1.QueryPendingRedemptionsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryPendingRedemptionsResponse, V::Error>
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
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPendingRedemptionsResponse {
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.QueryPendingRedemptionsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RedemptionSchedule {
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
        if !self.market_id.is_empty() {
            len += 1;
        }
        if !self.redeemer.is_empty() {
            len += 1;
        }
        if self.claimable_redemption_time.is_some() {
            len += 1;
        }
        if self.redemption_amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.insurance.v1beta1.RedemptionSchedule", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if !self.redeemer.is_empty() {
            struct_ser.serialize_field("redeemer", &self.redeemer)?;
        }
        if let Some(v) = self.claimable_redemption_time.as_ref() {
            struct_ser.serialize_field("claimableRedemptionTime", v)?;
        }
        if let Some(v) = self.redemption_amount.as_ref() {
            struct_ser.serialize_field("redemptionAmount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RedemptionSchedule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "marketId",
            "redeemer",
            "claimable_redemption_time",
            "claimableRedemptionTime",
            "redemption_amount",
            "redemptionAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            MarketId,
            Redeemer,
            ClaimableRedemptionTime,
            RedemptionAmount,
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
                            "marketId" => Ok(GeneratedField::MarketId),
                            "redeemer" => Ok(GeneratedField::Redeemer),
                            "claimableRedemptionTime" | "claimable_redemption_time" => {
                                Ok(GeneratedField::ClaimableRedemptionTime)
                            }
                            "redemptionAmount" | "redemption_amount" => {
                                Ok(GeneratedField::RedemptionAmount)
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
            type Value = RedemptionSchedule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.insurance.v1beta1.RedemptionSchedule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RedemptionSchedule, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut market_id__ = None;
                let mut redeemer__ = None;
                let mut claimable_redemption_time__ = None;
                let mut redemption_amount__ = None;
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
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Redeemer => {
                            if redeemer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redeemer"));
                            }
                            redeemer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClaimableRedemptionTime => {
                            if claimable_redemption_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "claimableRedemptionTime",
                                ));
                            }
                            claimable_redemption_time__ = map_.next_value()?;
                        }
                        GeneratedField::RedemptionAmount => {
                            if redemption_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redemptionAmount"));
                            }
                            redemption_amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RedemptionSchedule {
                    id: id__.unwrap_or_default(),
                    market_id: market_id__.unwrap_or_default(),
                    redeemer: redeemer__.unwrap_or_default(),
                    claimable_redemption_time: claimable_redemption_time__,
                    redemption_amount: redemption_amount__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.insurance.v1beta1.RedemptionSchedule",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
