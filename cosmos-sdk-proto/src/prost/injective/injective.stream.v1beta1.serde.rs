// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for BankBalance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account.is_empty() {
            len += 1;
        }
        if !self.balances.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.BankBalance", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BankBalance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["account", "balances"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
            Balances,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "account" => Ok(GeneratedField::Account),
                            "balances" => Ok(GeneratedField::Balances),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BankBalance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.BankBalance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BankBalance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                let mut balances__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BankBalance {
                    account: account__.unwrap_or_default(),
                    balances: balances__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.BankBalance",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BankBalancesFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.accounts.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.BankBalancesFilter", len)?;
        if !self.accounts.is_empty() {
            struct_ser.serialize_field("accounts", &self.accounts)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BankBalancesFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["accounts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Accounts,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accounts" => Ok(GeneratedField::Accounts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BankBalancesFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.BankBalancesFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BankBalancesFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut accounts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Accounts => {
                            if accounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accounts"));
                            }
                            accounts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BankBalancesFilter {
                    accounts: accounts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.BankBalancesFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DerivativeOrder {
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
        if self.order.is_some() {
            len += 1;
        }
        if self.is_market {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.DerivativeOrder", len)?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        if self.is_market {
            struct_ser.serialize_field("isMarket", &self.is_market)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DerivativeOrder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["market_id", "marketId", "order", "is_market", "isMarket"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
            Order,
            IsMarket,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "order" => Ok(GeneratedField::Order),
                            "isMarket" | "is_market" => Ok(GeneratedField::IsMarket),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DerivativeOrder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.DerivativeOrder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DerivativeOrder, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut order__ = None;
                let mut is_market__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                        GeneratedField::IsMarket => {
                            if is_market__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isMarket"));
                            }
                            is_market__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DerivativeOrder {
                    market_id: market_id__.unwrap_or_default(),
                    order: order__,
                    is_market: is_market__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.DerivativeOrder",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DerivativeOrderUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if !self.order_hash.is_empty() {
            len += 1;
        }
        if !self.cid.is_empty() {
            len += 1;
        }
        if self.order.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.DerivativeOrderUpdate", len)?;
        if self.status != 0 {
            let v = OrderUpdateStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.order_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "orderHash",
                pbjson::private::base64::encode(&self.order_hash).as_str(),
            )?;
        }
        if !self.cid.is_empty() {
            struct_ser.serialize_field("cid", &self.cid)?;
        }
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DerivativeOrderUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["status", "order_hash", "orderHash", "cid", "order"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            OrderHash,
            Cid,
            Order,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "orderHash" | "order_hash" => Ok(GeneratedField::OrderHash),
                            "cid" => Ok(GeneratedField::Cid),
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DerivativeOrderUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.DerivativeOrderUpdate")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<DerivativeOrderUpdate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut order_hash__ = None;
                let mut cid__ = None;
                let mut order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<OrderUpdateStatus>()? as i32);
                        }
                        GeneratedField::OrderHash => {
                            if order_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderHash"));
                            }
                            order_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Cid => {
                            if cid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cid"));
                            }
                            cid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DerivativeOrderUpdate {
                    status: status__.unwrap_or_default(),
                    order_hash: order_hash__.unwrap_or_default(),
                    cid: cid__.unwrap_or_default(),
                    order: order__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.DerivativeOrderUpdate",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DerivativeTrade {
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
        if self.is_buy {
            len += 1;
        }
        if !self.execution_type.is_empty() {
            len += 1;
        }
        if !self.subaccount_id.is_empty() {
            len += 1;
        }
        if self.position_delta.is_some() {
            len += 1;
        }
        if !self.payout.is_empty() {
            len += 1;
        }
        if !self.fee.is_empty() {
            len += 1;
        }
        if !self.order_hash.is_empty() {
            len += 1;
        }
        if !self.fee_recipient_address.is_empty() {
            len += 1;
        }
        if !self.cid.is_empty() {
            len += 1;
        }
        if !self.trade_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.DerivativeTrade", len)?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if self.is_buy {
            struct_ser.serialize_field("isBuy", &self.is_buy)?;
        }
        if !self.execution_type.is_empty() {
            struct_ser.serialize_field("executionType", &self.execution_type)?;
        }
        if !self.subaccount_id.is_empty() {
            struct_ser.serialize_field("subaccountId", &self.subaccount_id)?;
        }
        if let Some(v) = self.position_delta.as_ref() {
            struct_ser.serialize_field("positionDelta", v)?;
        }
        if !self.payout.is_empty() {
            struct_ser.serialize_field("payout", &self.payout)?;
        }
        if !self.fee.is_empty() {
            struct_ser.serialize_field("fee", &self.fee)?;
        }
        if !self.order_hash.is_empty() {
            struct_ser.serialize_field("orderHash", &self.order_hash)?;
        }
        if !self.fee_recipient_address.is_empty() {
            struct_ser.serialize_field("feeRecipientAddress", &self.fee_recipient_address)?;
        }
        if !self.cid.is_empty() {
            struct_ser.serialize_field("cid", &self.cid)?;
        }
        if !self.trade_id.is_empty() {
            struct_ser.serialize_field("tradeId", &self.trade_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DerivativeTrade {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "market_id",
            "marketId",
            "is_buy",
            "isBuy",
            "executionType",
            "subaccount_id",
            "subaccountId",
            "position_delta",
            "positionDelta",
            "payout",
            "fee",
            "order_hash",
            "orderHash",
            "fee_recipient_address",
            "feeRecipientAddress",
            "cid",
            "trade_id",
            "tradeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
            IsBuy,
            ExecutionType,
            SubaccountId,
            PositionDelta,
            Payout,
            Fee,
            OrderHash,
            FeeRecipientAddress,
            Cid,
            TradeId,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "isBuy" | "is_buy" => Ok(GeneratedField::IsBuy),
                            "executionType" => Ok(GeneratedField::ExecutionType),
                            "subaccountId" | "subaccount_id" => Ok(GeneratedField::SubaccountId),
                            "positionDelta" | "position_delta" => Ok(GeneratedField::PositionDelta),
                            "payout" => Ok(GeneratedField::Payout),
                            "fee" => Ok(GeneratedField::Fee),
                            "orderHash" | "order_hash" => Ok(GeneratedField::OrderHash),
                            "feeRecipientAddress" | "fee_recipient_address" => {
                                Ok(GeneratedField::FeeRecipientAddress)
                            }
                            "cid" => Ok(GeneratedField::Cid),
                            "tradeId" | "trade_id" => Ok(GeneratedField::TradeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DerivativeTrade;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.DerivativeTrade")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DerivativeTrade, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut is_buy__ = None;
                let mut execution_type__ = None;
                let mut subaccount_id__ = None;
                let mut position_delta__ = None;
                let mut payout__ = None;
                let mut fee__ = None;
                let mut order_hash__ = None;
                let mut fee_recipient_address__ = None;
                let mut cid__ = None;
                let mut trade_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsBuy => {
                            if is_buy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isBuy"));
                            }
                            is_buy__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExecutionType => {
                            if execution_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionType"));
                            }
                            execution_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubaccountId => {
                            if subaccount_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subaccountId"));
                            }
                            subaccount_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PositionDelta => {
                            if position_delta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positionDelta"));
                            }
                            position_delta__ = map_.next_value()?;
                        }
                        GeneratedField::Payout => {
                            if payout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payout"));
                            }
                            payout__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderHash => {
                            if order_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderHash"));
                            }
                            order_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeeRecipientAddress => {
                            if fee_recipient_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "feeRecipientAddress",
                                ));
                            }
                            fee_recipient_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cid => {
                            if cid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cid"));
                            }
                            cid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TradeId => {
                            if trade_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradeId"));
                            }
                            trade_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DerivativeTrade {
                    market_id: market_id__.unwrap_or_default(),
                    is_buy: is_buy__.unwrap_or_default(),
                    execution_type: execution_type__.unwrap_or_default(),
                    subaccount_id: subaccount_id__.unwrap_or_default(),
                    position_delta: position_delta__,
                    payout: payout__.unwrap_or_default(),
                    fee: fee__.unwrap_or_default(),
                    order_hash: order_hash__.unwrap_or_default(),
                    fee_recipient_address: fee_recipient_address__.unwrap_or_default(),
                    cid: cid__.unwrap_or_default(),
                    trade_id: trade_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.DerivativeTrade",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OraclePrice {
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
        if !self.r#type.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.OraclePrice", len)?;
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OraclePrice {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["symbol", "price", "type"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Symbol,
            Price,
            Type,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OraclePrice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.OraclePrice")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OraclePrice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut symbol__ = None;
                let mut price__ = None;
                let mut r#type__ = None;
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OraclePrice {
                    symbol: symbol__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.OraclePrice",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OraclePriceFilter {
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
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.OraclePriceFilter", len)?;
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OraclePriceFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["symbol"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = OraclePriceFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.OraclePriceFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OraclePriceFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut symbol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OraclePriceFilter {
                    symbol: symbol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.OraclePriceFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OrderUpdateStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "Unspecified",
            Self::Booked => "Booked",
            Self::Matched => "Matched",
            Self::Cancelled => "Cancelled",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OrderUpdateStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["Unspecified", "Booked", "Matched", "Cancelled"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderUpdateStatus;

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
                    "Unspecified" => Ok(OrderUpdateStatus::Unspecified),
                    "Booked" => Ok(OrderUpdateStatus::Booked),
                    "Matched" => Ok(OrderUpdateStatus::Matched),
                    "Cancelled" => Ok(OrderUpdateStatus::Cancelled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Orderbook {
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
        if !self.buy_levels.is_empty() {
            len += 1;
        }
        if !self.sell_levels.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.Orderbook", len)?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if !self.buy_levels.is_empty() {
            struct_ser.serialize_field("buyLevels", &self.buy_levels)?;
        }
        if !self.sell_levels.is_empty() {
            struct_ser.serialize_field("sellLevels", &self.sell_levels)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Orderbook {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "market_id",
            "marketId",
            "buy_levels",
            "buyLevels",
            "sell_levels",
            "sellLevels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
            BuyLevels,
            SellLevels,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "buyLevels" | "buy_levels" => Ok(GeneratedField::BuyLevels),
                            "sellLevels" | "sell_levels" => Ok(GeneratedField::SellLevels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Orderbook;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.Orderbook")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Orderbook, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut buy_levels__ = None;
                let mut sell_levels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BuyLevels => {
                            if buy_levels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyLevels"));
                            }
                            buy_levels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SellLevels => {
                            if sell_levels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellLevels"));
                            }
                            sell_levels__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Orderbook {
                    market_id: market_id__.unwrap_or_default(),
                    buy_levels: buy_levels__.unwrap_or_default(),
                    sell_levels: sell_levels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.Orderbook",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OrderbookFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.market_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.OrderbookFilter", len)?;
        if !self.market_ids.is_empty() {
            struct_ser.serialize_field("marketIds", &self.market_ids)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OrderbookFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["market_ids", "marketIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketIds,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "marketIds" | "market_ids" => Ok(GeneratedField::MarketIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderbookFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.OrderbookFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderbookFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketIds => {
                            if market_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketIds"));
                            }
                            market_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OrderbookFilter {
                    market_ids: market_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.OrderbookFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OrderbookUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seq != 0 {
            len += 1;
        }
        if self.orderbook.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.OrderbookUpdate", len)?;
        if self.seq != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("seq", ToString::to_string(&self.seq).as_str())?;
        }
        if let Some(v) = self.orderbook.as_ref() {
            struct_ser.serialize_field("orderbook", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OrderbookUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["seq", "orderbook"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seq,
            Orderbook,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "seq" => Ok(GeneratedField::Seq),
                            "orderbook" => Ok(GeneratedField::Orderbook),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderbookUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.OrderbookUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderbookUpdate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut seq__ = None;
                let mut orderbook__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seq => {
                            if seq__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seq"));
                            }
                            seq__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Orderbook => {
                            if orderbook__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderbook"));
                            }
                            orderbook__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OrderbookUpdate {
                    seq: seq__.unwrap_or_default(),
                    orderbook: orderbook__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.OrderbookUpdate",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for OrdersFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subaccount_ids.is_empty() {
            len += 1;
        }
        if !self.market_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.OrdersFilter", len)?;
        if !self.subaccount_ids.is_empty() {
            struct_ser.serialize_field("subaccountIds", &self.subaccount_ids)?;
        }
        if !self.market_ids.is_empty() {
            struct_ser.serialize_field("marketIds", &self.market_ids)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OrdersFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subaccount_ids", "subaccountIds", "market_ids", "marketIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubaccountIds,
            MarketIds,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subaccountIds" | "subaccount_ids" => Ok(GeneratedField::SubaccountIds),
                            "marketIds" | "market_ids" => Ok(GeneratedField::MarketIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrdersFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.OrdersFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrdersFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subaccount_ids__ = None;
                let mut market_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubaccountIds => {
                            if subaccount_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subaccountIds"));
                            }
                            subaccount_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketIds => {
                            if market_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketIds"));
                            }
                            market_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OrdersFilter {
                    subaccount_ids: subaccount_ids__.unwrap_or_default(),
                    market_ids: market_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.OrdersFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Position {
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
        if !self.subaccount_id.is_empty() {
            len += 1;
        }
        if self.is_long {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if !self.entry_price.is_empty() {
            len += 1;
        }
        if !self.margin.is_empty() {
            len += 1;
        }
        if !self.cumulative_funding_entry.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.Position", len)?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if !self.subaccount_id.is_empty() {
            struct_ser.serialize_field("subaccountId", &self.subaccount_id)?;
        }
        if self.is_long {
            struct_ser.serialize_field("isLong", &self.is_long)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.entry_price.is_empty() {
            struct_ser.serialize_field("entryPrice", &self.entry_price)?;
        }
        if !self.margin.is_empty() {
            struct_ser.serialize_field("margin", &self.margin)?;
        }
        if !self.cumulative_funding_entry.is_empty() {
            struct_ser.serialize_field("cumulativeFundingEntry", &self.cumulative_funding_entry)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Position {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "market_id",
            "marketId",
            "subaccount_id",
            "subaccountId",
            "isLong",
            "quantity",
            "entry_price",
            "entryPrice",
            "margin",
            "cumulative_funding_entry",
            "cumulativeFundingEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
            SubaccountId,
            IsLong,
            Quantity,
            EntryPrice,
            Margin,
            CumulativeFundingEntry,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "subaccountId" | "subaccount_id" => Ok(GeneratedField::SubaccountId),
                            "isLong" => Ok(GeneratedField::IsLong),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "entryPrice" | "entry_price" => Ok(GeneratedField::EntryPrice),
                            "margin" => Ok(GeneratedField::Margin),
                            "cumulativeFundingEntry" | "cumulative_funding_entry" => {
                                Ok(GeneratedField::CumulativeFundingEntry)
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
            type Value = Position;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.Position")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Position, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut subaccount_id__ = None;
                let mut is_long__ = None;
                let mut quantity__ = None;
                let mut entry_price__ = None;
                let mut margin__ = None;
                let mut cumulative_funding_entry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubaccountId => {
                            if subaccount_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subaccountId"));
                            }
                            subaccount_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsLong => {
                            if is_long__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isLong"));
                            }
                            is_long__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntryPrice => {
                            if entry_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entryPrice"));
                            }
                            entry_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Margin => {
                            if margin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("margin"));
                            }
                            margin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CumulativeFundingEntry => {
                            if cumulative_funding_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "cumulativeFundingEntry",
                                ));
                            }
                            cumulative_funding_entry__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Position {
                    market_id: market_id__.unwrap_or_default(),
                    subaccount_id: subaccount_id__.unwrap_or_default(),
                    is_long: is_long__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    entry_price: entry_price__.unwrap_or_default(),
                    margin: margin__.unwrap_or_default(),
                    cumulative_funding_entry: cumulative_funding_entry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.Position",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PositionsFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subaccount_ids.is_empty() {
            len += 1;
        }
        if !self.market_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.PositionsFilter", len)?;
        if !self.subaccount_ids.is_empty() {
            struct_ser.serialize_field("subaccountIds", &self.subaccount_ids)?;
        }
        if !self.market_ids.is_empty() {
            struct_ser.serialize_field("marketIds", &self.market_ids)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PositionsFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subaccount_ids", "subaccountIds", "market_ids", "marketIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubaccountIds,
            MarketIds,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subaccountIds" | "subaccount_ids" => Ok(GeneratedField::SubaccountIds),
                            "marketIds" | "market_ids" => Ok(GeneratedField::MarketIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PositionsFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.PositionsFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PositionsFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subaccount_ids__ = None;
                let mut market_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubaccountIds => {
                            if subaccount_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subaccountIds"));
                            }
                            subaccount_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketIds => {
                            if market_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketIds"));
                            }
                            market_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PositionsFilter {
                    subaccount_ids: subaccount_ids__.unwrap_or_default(),
                    market_ids: market_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.PositionsFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SpotOrder {
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
        if self.order.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.SpotOrder", len)?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SpotOrder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["market_id", "marketId", "order"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
            Order,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpotOrder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.SpotOrder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpotOrder, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SpotOrder {
                    market_id: market_id__.unwrap_or_default(),
                    order: order__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.SpotOrder",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SpotOrderUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if !self.order_hash.is_empty() {
            len += 1;
        }
        if !self.cid.is_empty() {
            len += 1;
        }
        if self.order.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.SpotOrderUpdate", len)?;
        if self.status != 0 {
            let v = OrderUpdateStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.order_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "orderHash",
                pbjson::private::base64::encode(&self.order_hash).as_str(),
            )?;
        }
        if !self.cid.is_empty() {
            struct_ser.serialize_field("cid", &self.cid)?;
        }
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SpotOrderUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["status", "order_hash", "orderHash", "cid", "order"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            OrderHash,
            Cid,
            Order,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "orderHash" | "order_hash" => Ok(GeneratedField::OrderHash),
                            "cid" => Ok(GeneratedField::Cid),
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpotOrderUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.SpotOrderUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpotOrderUpdate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut order_hash__ = None;
                let mut cid__ = None;
                let mut order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<OrderUpdateStatus>()? as i32);
                        }
                        GeneratedField::OrderHash => {
                            if order_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderHash"));
                            }
                            order_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Cid => {
                            if cid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cid"));
                            }
                            cid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SpotOrderUpdate {
                    status: status__.unwrap_or_default(),
                    order_hash: order_hash__.unwrap_or_default(),
                    cid: cid__.unwrap_or_default(),
                    order: order__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.SpotOrderUpdate",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SpotTrade {
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
        if self.is_buy {
            len += 1;
        }
        if !self.execution_type.is_empty() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.subaccount_id.is_empty() {
            len += 1;
        }
        if !self.fee.is_empty() {
            len += 1;
        }
        if !self.order_hash.is_empty() {
            len += 1;
        }
        if !self.fee_recipient_address.is_empty() {
            len += 1;
        }
        if !self.cid.is_empty() {
            len += 1;
        }
        if !self.trade_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.SpotTrade", len)?;
        if !self.market_id.is_empty() {
            struct_ser.serialize_field("marketId", &self.market_id)?;
        }
        if self.is_buy {
            struct_ser.serialize_field("isBuy", &self.is_buy)?;
        }
        if !self.execution_type.is_empty() {
            struct_ser.serialize_field("executionType", &self.execution_type)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.subaccount_id.is_empty() {
            struct_ser.serialize_field("subaccountId", &self.subaccount_id)?;
        }
        if !self.fee.is_empty() {
            struct_ser.serialize_field("fee", &self.fee)?;
        }
        if !self.order_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "orderHash",
                pbjson::private::base64::encode(&self.order_hash).as_str(),
            )?;
        }
        if !self.fee_recipient_address.is_empty() {
            struct_ser.serialize_field("feeRecipientAddress", &self.fee_recipient_address)?;
        }
        if !self.cid.is_empty() {
            struct_ser.serialize_field("cid", &self.cid)?;
        }
        if !self.trade_id.is_empty() {
            struct_ser.serialize_field("tradeId", &self.trade_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SpotTrade {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "market_id",
            "marketId",
            "is_buy",
            "isBuy",
            "executionType",
            "quantity",
            "price",
            "subaccount_id",
            "subaccountId",
            "fee",
            "order_hash",
            "orderHash",
            "fee_recipient_address",
            "feeRecipientAddress",
            "cid",
            "trade_id",
            "tradeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
            IsBuy,
            ExecutionType,
            Quantity,
            Price,
            SubaccountId,
            Fee,
            OrderHash,
            FeeRecipientAddress,
            Cid,
            TradeId,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "isBuy" | "is_buy" => Ok(GeneratedField::IsBuy),
                            "executionType" => Ok(GeneratedField::ExecutionType),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "price" => Ok(GeneratedField::Price),
                            "subaccountId" | "subaccount_id" => Ok(GeneratedField::SubaccountId),
                            "fee" => Ok(GeneratedField::Fee),
                            "orderHash" | "order_hash" => Ok(GeneratedField::OrderHash),
                            "feeRecipientAddress" | "fee_recipient_address" => {
                                Ok(GeneratedField::FeeRecipientAddress)
                            }
                            "cid" => Ok(GeneratedField::Cid),
                            "tradeId" | "trade_id" => Ok(GeneratedField::TradeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpotTrade;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.SpotTrade")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpotTrade, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut is_buy__ = None;
                let mut execution_type__ = None;
                let mut quantity__ = None;
                let mut price__ = None;
                let mut subaccount_id__ = None;
                let mut fee__ = None;
                let mut order_hash__ = None;
                let mut fee_recipient_address__ = None;
                let mut cid__ = None;
                let mut trade_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsBuy => {
                            if is_buy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isBuy"));
                            }
                            is_buy__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExecutionType => {
                            if execution_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionType"));
                            }
                            execution_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubaccountId => {
                            if subaccount_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subaccountId"));
                            }
                            subaccount_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderHash => {
                            if order_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderHash"));
                            }
                            order_hash__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FeeRecipientAddress => {
                            if fee_recipient_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "feeRecipientAddress",
                                ));
                            }
                            fee_recipient_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cid => {
                            if cid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cid"));
                            }
                            cid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TradeId => {
                            if trade_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradeId"));
                            }
                            trade_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SpotTrade {
                    market_id: market_id__.unwrap_or_default(),
                    is_buy: is_buy__.unwrap_or_default(),
                    execution_type: execution_type__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    subaccount_id: subaccount_id__.unwrap_or_default(),
                    fee: fee__.unwrap_or_default(),
                    order_hash: order_hash__.unwrap_or_default(),
                    fee_recipient_address: fee_recipient_address__.unwrap_or_default(),
                    cid: cid__.unwrap_or_default(),
                    trade_id: trade_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.SpotTrade",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StreamRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bank_balances_filter.is_some() {
            len += 1;
        }
        if self.subaccount_deposits_filter.is_some() {
            len += 1;
        }
        if self.spot_trades_filter.is_some() {
            len += 1;
        }
        if self.derivative_trades_filter.is_some() {
            len += 1;
        }
        if self.spot_orders_filter.is_some() {
            len += 1;
        }
        if self.derivative_orders_filter.is_some() {
            len += 1;
        }
        if self.spot_orderbooks_filter.is_some() {
            len += 1;
        }
        if self.derivative_orderbooks_filter.is_some() {
            len += 1;
        }
        if self.positions_filter.is_some() {
            len += 1;
        }
        if self.oracle_price_filter.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.StreamRequest", len)?;
        if let Some(v) = self.bank_balances_filter.as_ref() {
            struct_ser.serialize_field("bankBalancesFilter", v)?;
        }
        if let Some(v) = self.subaccount_deposits_filter.as_ref() {
            struct_ser.serialize_field("subaccountDepositsFilter", v)?;
        }
        if let Some(v) = self.spot_trades_filter.as_ref() {
            struct_ser.serialize_field("spotTradesFilter", v)?;
        }
        if let Some(v) = self.derivative_trades_filter.as_ref() {
            struct_ser.serialize_field("derivativeTradesFilter", v)?;
        }
        if let Some(v) = self.spot_orders_filter.as_ref() {
            struct_ser.serialize_field("spotOrdersFilter", v)?;
        }
        if let Some(v) = self.derivative_orders_filter.as_ref() {
            struct_ser.serialize_field("derivativeOrdersFilter", v)?;
        }
        if let Some(v) = self.spot_orderbooks_filter.as_ref() {
            struct_ser.serialize_field("spotOrderbooksFilter", v)?;
        }
        if let Some(v) = self.derivative_orderbooks_filter.as_ref() {
            struct_ser.serialize_field("derivativeOrderbooksFilter", v)?;
        }
        if let Some(v) = self.positions_filter.as_ref() {
            struct_ser.serialize_field("positionsFilter", v)?;
        }
        if let Some(v) = self.oracle_price_filter.as_ref() {
            struct_ser.serialize_field("oraclePriceFilter", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StreamRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bank_balances_filter",
            "bankBalancesFilter",
            "subaccount_deposits_filter",
            "subaccountDepositsFilter",
            "spot_trades_filter",
            "spotTradesFilter",
            "derivative_trades_filter",
            "derivativeTradesFilter",
            "spot_orders_filter",
            "spotOrdersFilter",
            "derivative_orders_filter",
            "derivativeOrdersFilter",
            "spot_orderbooks_filter",
            "spotOrderbooksFilter",
            "derivative_orderbooks_filter",
            "derivativeOrderbooksFilter",
            "positions_filter",
            "positionsFilter",
            "oracle_price_filter",
            "oraclePriceFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BankBalancesFilter,
            SubaccountDepositsFilter,
            SpotTradesFilter,
            DerivativeTradesFilter,
            SpotOrdersFilter,
            DerivativeOrdersFilter,
            SpotOrderbooksFilter,
            DerivativeOrderbooksFilter,
            PositionsFilter,
            OraclePriceFilter,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bankBalancesFilter" | "bank_balances_filter" => {
                                Ok(GeneratedField::BankBalancesFilter)
                            }
                            "subaccountDepositsFilter" | "subaccount_deposits_filter" => {
                                Ok(GeneratedField::SubaccountDepositsFilter)
                            }
                            "spotTradesFilter" | "spot_trades_filter" => {
                                Ok(GeneratedField::SpotTradesFilter)
                            }
                            "derivativeTradesFilter" | "derivative_trades_filter" => {
                                Ok(GeneratedField::DerivativeTradesFilter)
                            }
                            "spotOrdersFilter" | "spot_orders_filter" => {
                                Ok(GeneratedField::SpotOrdersFilter)
                            }
                            "derivativeOrdersFilter" | "derivative_orders_filter" => {
                                Ok(GeneratedField::DerivativeOrdersFilter)
                            }
                            "spotOrderbooksFilter" | "spot_orderbooks_filter" => {
                                Ok(GeneratedField::SpotOrderbooksFilter)
                            }
                            "derivativeOrderbooksFilter" | "derivative_orderbooks_filter" => {
                                Ok(GeneratedField::DerivativeOrderbooksFilter)
                            }
                            "positionsFilter" | "positions_filter" => {
                                Ok(GeneratedField::PositionsFilter)
                            }
                            "oraclePriceFilter" | "oracle_price_filter" => {
                                Ok(GeneratedField::OraclePriceFilter)
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
            type Value = StreamRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.StreamRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut bank_balances_filter__ = None;
                let mut subaccount_deposits_filter__ = None;
                let mut spot_trades_filter__ = None;
                let mut derivative_trades_filter__ = None;
                let mut spot_orders_filter__ = None;
                let mut derivative_orders_filter__ = None;
                let mut spot_orderbooks_filter__ = None;
                let mut derivative_orderbooks_filter__ = None;
                let mut positions_filter__ = None;
                let mut oracle_price_filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BankBalancesFilter => {
                            if bank_balances_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "bankBalancesFilter",
                                ));
                            }
                            bank_balances_filter__ = map_.next_value()?;
                        }
                        GeneratedField::SubaccountDepositsFilter => {
                            if subaccount_deposits_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subaccountDepositsFilter",
                                ));
                            }
                            subaccount_deposits_filter__ = map_.next_value()?;
                        }
                        GeneratedField::SpotTradesFilter => {
                            if spot_trades_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spotTradesFilter"));
                            }
                            spot_trades_filter__ = map_.next_value()?;
                        }
                        GeneratedField::DerivativeTradesFilter => {
                            if derivative_trades_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "derivativeTradesFilter",
                                ));
                            }
                            derivative_trades_filter__ = map_.next_value()?;
                        }
                        GeneratedField::SpotOrdersFilter => {
                            if spot_orders_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spotOrdersFilter"));
                            }
                            spot_orders_filter__ = map_.next_value()?;
                        }
                        GeneratedField::DerivativeOrdersFilter => {
                            if derivative_orders_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "derivativeOrdersFilter",
                                ));
                            }
                            derivative_orders_filter__ = map_.next_value()?;
                        }
                        GeneratedField::SpotOrderbooksFilter => {
                            if spot_orderbooks_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "spotOrderbooksFilter",
                                ));
                            }
                            spot_orderbooks_filter__ = map_.next_value()?;
                        }
                        GeneratedField::DerivativeOrderbooksFilter => {
                            if derivative_orderbooks_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "derivativeOrderbooksFilter",
                                ));
                            }
                            derivative_orderbooks_filter__ = map_.next_value()?;
                        }
                        GeneratedField::PositionsFilter => {
                            if positions_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positionsFilter"));
                            }
                            positions_filter__ = map_.next_value()?;
                        }
                        GeneratedField::OraclePriceFilter => {
                            if oracle_price_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oraclePriceFilter"));
                            }
                            oracle_price_filter__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamRequest {
                    bank_balances_filter: bank_balances_filter__,
                    subaccount_deposits_filter: subaccount_deposits_filter__,
                    spot_trades_filter: spot_trades_filter__,
                    derivative_trades_filter: derivative_trades_filter__,
                    spot_orders_filter: spot_orders_filter__,
                    derivative_orders_filter: derivative_orders_filter__,
                    spot_orderbooks_filter: spot_orderbooks_filter__,
                    derivative_orderbooks_filter: derivative_orderbooks_filter__,
                    positions_filter: positions_filter__,
                    oracle_price_filter: oracle_price_filter__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.StreamRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for StreamResponse {
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
        if self.block_time != 0 {
            len += 1;
        }
        if !self.bank_balances.is_empty() {
            len += 1;
        }
        if !self.subaccount_deposits.is_empty() {
            len += 1;
        }
        if !self.spot_trades.is_empty() {
            len += 1;
        }
        if !self.derivative_trades.is_empty() {
            len += 1;
        }
        if !self.spot_orders.is_empty() {
            len += 1;
        }
        if !self.derivative_orders.is_empty() {
            len += 1;
        }
        if !self.spot_orderbook_updates.is_empty() {
            len += 1;
        }
        if !self.derivative_orderbook_updates.is_empty() {
            len += 1;
        }
        if !self.positions.is_empty() {
            len += 1;
        }
        if !self.oracle_prices.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.StreamResponse", len)?;
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "blockHeight",
                ToString::to_string(&self.block_height).as_str(),
            )?;
        }
        if self.block_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("blockTime", ToString::to_string(&self.block_time).as_str())?;
        }
        if !self.bank_balances.is_empty() {
            struct_ser.serialize_field("bankBalances", &self.bank_balances)?;
        }
        if !self.subaccount_deposits.is_empty() {
            struct_ser.serialize_field("subaccountDeposits", &self.subaccount_deposits)?;
        }
        if !self.spot_trades.is_empty() {
            struct_ser.serialize_field("spotTrades", &self.spot_trades)?;
        }
        if !self.derivative_trades.is_empty() {
            struct_ser.serialize_field("derivativeTrades", &self.derivative_trades)?;
        }
        if !self.spot_orders.is_empty() {
            struct_ser.serialize_field("spotOrders", &self.spot_orders)?;
        }
        if !self.derivative_orders.is_empty() {
            struct_ser.serialize_field("derivativeOrders", &self.derivative_orders)?;
        }
        if !self.spot_orderbook_updates.is_empty() {
            struct_ser.serialize_field("spotOrderbookUpdates", &self.spot_orderbook_updates)?;
        }
        if !self.derivative_orderbook_updates.is_empty() {
            struct_ser.serialize_field(
                "derivativeOrderbookUpdates",
                &self.derivative_orderbook_updates,
            )?;
        }
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        if !self.oracle_prices.is_empty() {
            struct_ser.serialize_field("oraclePrices", &self.oracle_prices)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StreamResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_height",
            "blockHeight",
            "block_time",
            "blockTime",
            "bank_balances",
            "bankBalances",
            "subaccount_deposits",
            "subaccountDeposits",
            "spot_trades",
            "spotTrades",
            "derivative_trades",
            "derivativeTrades",
            "spot_orders",
            "spotOrders",
            "derivative_orders",
            "derivativeOrders",
            "spot_orderbook_updates",
            "spotOrderbookUpdates",
            "derivative_orderbook_updates",
            "derivativeOrderbookUpdates",
            "positions",
            "oracle_prices",
            "oraclePrices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockHeight,
            BlockTime,
            BankBalances,
            SubaccountDeposits,
            SpotTrades,
            DerivativeTrades,
            SpotOrders,
            DerivativeOrders,
            SpotOrderbookUpdates,
            DerivativeOrderbookUpdates,
            Positions,
            OraclePrices,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "blockTime" | "block_time" => Ok(GeneratedField::BlockTime),
                            "bankBalances" | "bank_balances" => Ok(GeneratedField::BankBalances),
                            "subaccountDeposits" | "subaccount_deposits" => {
                                Ok(GeneratedField::SubaccountDeposits)
                            }
                            "spotTrades" | "spot_trades" => Ok(GeneratedField::SpotTrades),
                            "derivativeTrades" | "derivative_trades" => {
                                Ok(GeneratedField::DerivativeTrades)
                            }
                            "spotOrders" | "spot_orders" => Ok(GeneratedField::SpotOrders),
                            "derivativeOrders" | "derivative_orders" => {
                                Ok(GeneratedField::DerivativeOrders)
                            }
                            "spotOrderbookUpdates" | "spot_orderbook_updates" => {
                                Ok(GeneratedField::SpotOrderbookUpdates)
                            }
                            "derivativeOrderbookUpdates" | "derivative_orderbook_updates" => {
                                Ok(GeneratedField::DerivativeOrderbookUpdates)
                            }
                            "positions" => Ok(GeneratedField::Positions),
                            "oraclePrices" | "oracle_prices" => Ok(GeneratedField::OraclePrices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.StreamResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut block_height__ = None;
                let mut block_time__ = None;
                let mut bank_balances__ = None;
                let mut subaccount_deposits__ = None;
                let mut spot_trades__ = None;
                let mut derivative_trades__ = None;
                let mut spot_orders__ = None;
                let mut derivative_orders__ = None;
                let mut spot_orderbook_updates__ = None;
                let mut derivative_orderbook_updates__ = None;
                let mut positions__ = None;
                let mut oracle_prices__ = None;
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
                        GeneratedField::BlockTime => {
                            if block_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockTime"));
                            }
                            block_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BankBalances => {
                            if bank_balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bankBalances"));
                            }
                            bank_balances__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubaccountDeposits => {
                            if subaccount_deposits__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subaccountDeposits",
                                ));
                            }
                            subaccount_deposits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpotTrades => {
                            if spot_trades__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spotTrades"));
                            }
                            spot_trades__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DerivativeTrades => {
                            if derivative_trades__.is_some() {
                                return Err(serde::de::Error::duplicate_field("derivativeTrades"));
                            }
                            derivative_trades__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpotOrders => {
                            if spot_orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spotOrders"));
                            }
                            spot_orders__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DerivativeOrders => {
                            if derivative_orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("derivativeOrders"));
                            }
                            derivative_orders__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpotOrderbookUpdates => {
                            if spot_orderbook_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "spotOrderbookUpdates",
                                ));
                            }
                            spot_orderbook_updates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DerivativeOrderbookUpdates => {
                            if derivative_orderbook_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "derivativeOrderbookUpdates",
                                ));
                            }
                            derivative_orderbook_updates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OraclePrices => {
                            if oracle_prices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oraclePrices"));
                            }
                            oracle_prices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamResponse {
                    block_height: block_height__.unwrap_or_default(),
                    block_time: block_time__.unwrap_or_default(),
                    bank_balances: bank_balances__.unwrap_or_default(),
                    subaccount_deposits: subaccount_deposits__.unwrap_or_default(),
                    spot_trades: spot_trades__.unwrap_or_default(),
                    derivative_trades: derivative_trades__.unwrap_or_default(),
                    spot_orders: spot_orders__.unwrap_or_default(),
                    derivative_orders: derivative_orders__.unwrap_or_default(),
                    spot_orderbook_updates: spot_orderbook_updates__.unwrap_or_default(),
                    derivative_orderbook_updates: derivative_orderbook_updates__
                        .unwrap_or_default(),
                    positions: positions__.unwrap_or_default(),
                    oracle_prices: oracle_prices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.StreamResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SubaccountDeposit {
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
        if self.deposit.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.SubaccountDeposit", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if let Some(v) = self.deposit.as_ref() {
            struct_ser.serialize_field("deposit", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SubaccountDeposit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["denom", "deposit"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
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
                            "denom" => Ok(GeneratedField::Denom),
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
            type Value = SubaccountDeposit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.SubaccountDeposit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubaccountDeposit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut deposit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Deposit => {
                            if deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposit"));
                            }
                            deposit__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SubaccountDeposit {
                    denom: denom__.unwrap_or_default(),
                    deposit: deposit__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.SubaccountDeposit",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SubaccountDeposits {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subaccount_id.is_empty() {
            len += 1;
        }
        if !self.deposits.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.SubaccountDeposits", len)?;
        if !self.subaccount_id.is_empty() {
            struct_ser.serialize_field("subaccountId", &self.subaccount_id)?;
        }
        if !self.deposits.is_empty() {
            struct_ser.serialize_field("deposits", &self.deposits)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SubaccountDeposits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subaccount_id", "subaccountId", "deposits"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubaccountId,
            Deposits,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subaccountId" | "subaccount_id" => Ok(GeneratedField::SubaccountId),
                            "deposits" => Ok(GeneratedField::Deposits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubaccountDeposits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.SubaccountDeposits")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubaccountDeposits, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subaccount_id__ = None;
                let mut deposits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubaccountId => {
                            if subaccount_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subaccountId"));
                            }
                            subaccount_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Deposits => {
                            if deposits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposits"));
                            }
                            deposits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubaccountDeposits {
                    subaccount_id: subaccount_id__.unwrap_or_default(),
                    deposits: deposits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.SubaccountDeposits",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SubaccountDepositsFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subaccount_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.stream.v1beta1.SubaccountDepositsFilter", len)?;
        if !self.subaccount_ids.is_empty() {
            struct_ser.serialize_field("subaccountIds", &self.subaccount_ids)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SubaccountDepositsFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subaccount_ids", "subaccountIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubaccountIds,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subaccountIds" | "subaccount_ids" => Ok(GeneratedField::SubaccountIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubaccountDepositsFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.SubaccountDepositsFilter")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<SubaccountDepositsFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subaccount_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubaccountIds => {
                            if subaccount_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subaccountIds"));
                            }
                            subaccount_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubaccountDepositsFilter {
                    subaccount_ids: subaccount_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.SubaccountDepositsFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for TradesFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subaccount_ids.is_empty() {
            len += 1;
        }
        if !self.market_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.stream.v1beta1.TradesFilter", len)?;
        if !self.subaccount_ids.is_empty() {
            struct_ser.serialize_field("subaccountIds", &self.subaccount_ids)?;
        }
        if !self.market_ids.is_empty() {
            struct_ser.serialize_field("marketIds", &self.market_ids)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TradesFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subaccount_ids", "subaccountIds", "market_ids", "marketIds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubaccountIds,
            MarketIds,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subaccountIds" | "subaccount_ids" => Ok(GeneratedField::SubaccountIds),
                            "marketIds" | "market_ids" => Ok(GeneratedField::MarketIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TradesFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.stream.v1beta1.TradesFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TradesFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subaccount_ids__ = None;
                let mut market_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubaccountIds => {
                            if subaccount_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subaccountIds"));
                            }
                            subaccount_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketIds => {
                            if market_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketIds"));
                            }
                            market_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TradesFilter {
                    subaccount_ids: subaccount_ids__.unwrap_or_default(),
                    market_ids: market_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.stream.v1beta1.TradesFilter",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
