// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for BatchContractDeregistrationProposal {
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
        if !self.contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.wasmx.v1.BatchContractDeregistrationProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.contracts.is_empty() {
            struct_ser.serialize_field("contracts", &self.contracts)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BatchContractDeregistrationProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "contracts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Contracts,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "contracts" => Ok(GeneratedField::Contracts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchContractDeregistrationProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.BatchContractDeregistrationProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BatchContractDeregistrationProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contracts__ = None;
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
                        GeneratedField::Contracts => {
                            if contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contracts"));
                            }
                            contracts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchContractDeregistrationProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contracts: contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.BatchContractDeregistrationProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BatchContractRegistrationRequestProposal {
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
        if !self.contract_registration_requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.wasmx.v1.BatchContractRegistrationRequestProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.contract_registration_requests.is_empty() {
            struct_ser.serialize_field(
                "contractRegistrationRequests",
                &self.contract_registration_requests,
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BatchContractRegistrationRequestProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "contract_registration_requests",
            "contractRegistrationRequests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            ContractRegistrationRequests,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "contractRegistrationRequests" | "contract_registration_requests" => {
                                Ok(GeneratedField::ContractRegistrationRequests)
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
            type Value = BatchContractRegistrationRequestProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.wasmx.v1.BatchContractRegistrationRequestProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BatchContractRegistrationRequestProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contract_registration_requests__ = None;
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
                        GeneratedField::ContractRegistrationRequests => {
                            if contract_registration_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "contractRegistrationRequests",
                                ));
                            }
                            contract_registration_requests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchContractRegistrationRequestProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contract_registration_requests: contract_registration_requests__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.BatchContractRegistrationRequestProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BatchStoreCodeProposal {
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
        if !self.proposals.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.BatchStoreCodeProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.proposals.is_empty() {
            struct_ser.serialize_field("proposals", &self.proposals)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BatchStoreCodeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["title", "description", "proposals"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Proposals,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "proposals" => Ok(GeneratedField::Proposals),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchStoreCodeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.BatchStoreCodeProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BatchStoreCodeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut proposals__ = None;
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
                        GeneratedField::Proposals => {
                            if proposals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposals"));
                            }
                            proposals__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchStoreCodeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    proposals: proposals__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.BatchStoreCodeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ContractRegistrationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.gas_limit != 0 {
            len += 1;
        }
        if self.gas_price != 0 {
            len += 1;
        }
        if self.should_pin_contract {
            len += 1;
        }
        if self.is_migration_allowed {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.admin_address.is_empty() {
            len += 1;
        }
        if !self.granter_address.is_empty() {
            len += 1;
        }
        if self.funding_mode != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.ContractRegistrationRequest", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.gas_limit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("gasLimit", ToString::to_string(&self.gas_limit).as_str())?;
        }
        if self.gas_price != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("gasPrice", ToString::to_string(&self.gas_price).as_str())?;
        }
        if self.should_pin_contract {
            struct_ser.serialize_field("shouldPinContract", &self.should_pin_contract)?;
        }
        if self.is_migration_allowed {
            struct_ser.serialize_field("isMigrationAllowed", &self.is_migration_allowed)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.admin_address.is_empty() {
            struct_ser.serialize_field("adminAddress", &self.admin_address)?;
        }
        if !self.granter_address.is_empty() {
            struct_ser.serialize_field("granterAddress", &self.granter_address)?;
        }
        if self.funding_mode != 0 {
            let v = FundingMode::try_from(self.funding_mode).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.funding_mode))
            })?;
            struct_ser.serialize_field("fundingMode", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ContractRegistrationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "gas_limit",
            "gasLimit",
            "gas_price",
            "gasPrice",
            "should_pin_contract",
            "shouldPinContract",
            "is_migration_allowed",
            "isMigrationAllowed",
            "code_id",
            "codeId",
            "admin_address",
            "adminAddress",
            "granter_address",
            "granterAddress",
            "funding_mode",
            "fundingMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            GasLimit,
            GasPrice,
            ShouldPinContract,
            IsMigrationAllowed,
            CodeId,
            AdminAddress,
            GranterAddress,
            FundingMode,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "gasLimit" | "gas_limit" => Ok(GeneratedField::GasLimit),
                            "gasPrice" | "gas_price" => Ok(GeneratedField::GasPrice),
                            "shouldPinContract" | "should_pin_contract" => {
                                Ok(GeneratedField::ShouldPinContract)
                            }
                            "isMigrationAllowed" | "is_migration_allowed" => {
                                Ok(GeneratedField::IsMigrationAllowed)
                            }
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "adminAddress" | "admin_address" => Ok(GeneratedField::AdminAddress),
                            "granterAddress" | "granter_address" => {
                                Ok(GeneratedField::GranterAddress)
                            }
                            "fundingMode" | "funding_mode" => Ok(GeneratedField::FundingMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContractRegistrationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.ContractRegistrationRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ContractRegistrationRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut gas_limit__ = None;
                let mut gas_price__ = None;
                let mut should_pin_contract__ = None;
                let mut is_migration_allowed__ = None;
                let mut code_id__ = None;
                let mut admin_address__ = None;
                let mut granter_address__ = None;
                let mut funding_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasLimit => {
                            if gas_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasLimit"));
                            }
                            gas_limit__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::GasPrice => {
                            if gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasPrice"));
                            }
                            gas_price__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ShouldPinContract => {
                            if should_pin_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shouldPinContract"));
                            }
                            should_pin_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsMigrationAllowed => {
                            if is_migration_allowed__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "isMigrationAllowed",
                                ));
                            }
                            is_migration_allowed__ = Some(map_.next_value()?);
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
                        GeneratedField::AdminAddress => {
                            if admin_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminAddress"));
                            }
                            admin_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GranterAddress => {
                            if granter_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("granterAddress"));
                            }
                            granter_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FundingMode => {
                            if funding_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fundingMode"));
                            }
                            funding_mode__ = Some(map_.next_value::<FundingMode>()? as i32);
                        }
                    }
                }
                Ok(ContractRegistrationRequest {
                    contract_address: contract_address__.unwrap_or_default(),
                    gas_limit: gas_limit__.unwrap_or_default(),
                    gas_price: gas_price__.unwrap_or_default(),
                    should_pin_contract: should_pin_contract__.unwrap_or_default(),
                    is_migration_allowed: is_migration_allowed__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    admin_address: admin_address__.unwrap_or_default(),
                    granter_address: granter_address__.unwrap_or_default(),
                    funding_mode: funding_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.ContractRegistrationRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ContractRegistrationRequestProposal {
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
        if self.contract_registration_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.wasmx.v1.ContractRegistrationRequestProposal",
            len,
        )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.contract_registration_request.as_ref() {
            struct_ser.serialize_field("contractRegistrationRequest", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ContractRegistrationRequestProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "contract_registration_request",
            "contractRegistrationRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            ContractRegistrationRequest,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "contractRegistrationRequest" | "contract_registration_request" => {
                                Ok(GeneratedField::ContractRegistrationRequest)
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
            type Value = ContractRegistrationRequestProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.ContractRegistrationRequestProposal")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<ContractRegistrationRequestProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut contract_registration_request__ = None;
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
                        GeneratedField::ContractRegistrationRequest => {
                            if contract_registration_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "contractRegistrationRequest",
                                ));
                            }
                            contract_registration_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ContractRegistrationRequestProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    contract_registration_request: contract_registration_request__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.ContractRegistrationRequestProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventContractDeregistered {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.EventContractDeregistered", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventContractDeregistered {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract_address", "contractAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = EventContractDeregistered;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.EventContractDeregistered")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventContractDeregistered, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventContractDeregistered {
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.EventContractDeregistered",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventContractExecution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.response.is_empty() {
            len += 1;
        }
        if !self.other_error.is_empty() {
            len += 1;
        }
        if !self.execution_error.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.EventContractExecution", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if !self.response.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "response",
                pbjson::private::base64::encode(&self.response).as_str(),
            )?;
        }
        if !self.other_error.is_empty() {
            struct_ser.serialize_field("otherError", &self.other_error)?;
        }
        if !self.execution_error.is_empty() {
            struct_ser.serialize_field("executionError", &self.execution_error)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventContractExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "response",
            "other_error",
            "otherError",
            "execution_error",
            "executionError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            Response,
            OtherError,
            ExecutionError,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "response" => Ok(GeneratedField::Response),
                            "otherError" | "other_error" => Ok(GeneratedField::OtherError),
                            "executionError" | "execution_error" => {
                                Ok(GeneratedField::ExecutionError)
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
            type Value = EventContractExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.EventContractExecution")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventContractExecution, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut response__ = None;
                let mut other_error__ = None;
                let mut execution_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OtherError => {
                            if other_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherError"));
                            }
                            other_error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExecutionError => {
                            if execution_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionError"));
                            }
                            execution_error__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventContractExecution {
                    contract_address: contract_address__.unwrap_or_default(),
                    response: response__.unwrap_or_default(),
                    other_error: other_error__.unwrap_or_default(),
                    execution_error: execution_error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.EventContractExecution",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for EventContractRegistered {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.gas_price != 0 {
            len += 1;
        }
        if self.should_pin_contract {
            len += 1;
        }
        if self.is_migration_allowed {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.admin_address.is_empty() {
            len += 1;
        }
        if !self.granter_address.is_empty() {
            len += 1;
        }
        if self.funding_mode != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.EventContractRegistered", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.gas_price != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("gasPrice", ToString::to_string(&self.gas_price).as_str())?;
        }
        if self.should_pin_contract {
            struct_ser.serialize_field("shouldPinContract", &self.should_pin_contract)?;
        }
        if self.is_migration_allowed {
            struct_ser.serialize_field("isMigrationAllowed", &self.is_migration_allowed)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.admin_address.is_empty() {
            struct_ser.serialize_field("adminAddress", &self.admin_address)?;
        }
        if !self.granter_address.is_empty() {
            struct_ser.serialize_field("granterAddress", &self.granter_address)?;
        }
        if self.funding_mode != 0 {
            let v = FundingMode::try_from(self.funding_mode).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.funding_mode))
            })?;
            struct_ser.serialize_field("fundingMode", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EventContractRegistered {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_address",
            "contractAddress",
            "gas_price",
            "gasPrice",
            "should_pin_contract",
            "shouldPinContract",
            "is_migration_allowed",
            "isMigrationAllowed",
            "code_id",
            "codeId",
            "admin_address",
            "adminAddress",
            "granter_address",
            "granterAddress",
            "funding_mode",
            "fundingMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            GasPrice,
            ShouldPinContract,
            IsMigrationAllowed,
            CodeId,
            AdminAddress,
            GranterAddress,
            FundingMode,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "gasPrice" | "gas_price" => Ok(GeneratedField::GasPrice),
                            "shouldPinContract" | "should_pin_contract" => {
                                Ok(GeneratedField::ShouldPinContract)
                            }
                            "isMigrationAllowed" | "is_migration_allowed" => {
                                Ok(GeneratedField::IsMigrationAllowed)
                            }
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "adminAddress" | "admin_address" => Ok(GeneratedField::AdminAddress),
                            "granterAddress" | "granter_address" => {
                                Ok(GeneratedField::GranterAddress)
                            }
                            "fundingMode" | "funding_mode" => Ok(GeneratedField::FundingMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventContractRegistered;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.EventContractRegistered")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<EventContractRegistered, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut gas_price__ = None;
                let mut should_pin_contract__ = None;
                let mut is_migration_allowed__ = None;
                let mut code_id__ = None;
                let mut admin_address__ = None;
                let mut granter_address__ = None;
                let mut funding_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasPrice => {
                            if gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasPrice"));
                            }
                            gas_price__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ShouldPinContract => {
                            if should_pin_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shouldPinContract"));
                            }
                            should_pin_contract__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsMigrationAllowed => {
                            if is_migration_allowed__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "isMigrationAllowed",
                                ));
                            }
                            is_migration_allowed__ = Some(map_.next_value()?);
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
                        GeneratedField::AdminAddress => {
                            if admin_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminAddress"));
                            }
                            admin_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GranterAddress => {
                            if granter_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("granterAddress"));
                            }
                            granter_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FundingMode => {
                            if funding_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fundingMode"));
                            }
                            funding_mode__ = Some(map_.next_value::<FundingMode>()? as i32);
                        }
                    }
                }
                Ok(EventContractRegistered {
                    contract_address: contract_address__.unwrap_or_default(),
                    gas_price: gas_price__.unwrap_or_default(),
                    should_pin_contract: should_pin_contract__.unwrap_or_default(),
                    is_migration_allowed: is_migration_allowed__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    admin_address: admin_address__.unwrap_or_default(),
                    granter_address: granter_address__.unwrap_or_default(),
                    funding_mode: funding_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.EventContractRegistered",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FundingMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "Unspecified",
            Self::SelfFunded => "SelfFunded",
            Self::GrantOnly => "GrantOnly",
            Self::Dual => "Dual",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FundingMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["Unspecified", "SelfFunded", "GrantOnly", "Dual"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FundingMode;

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
                    "Unspecified" => Ok(FundingMode::Unspecified),
                    "SelfFunded" => Ok(FundingMode::SelfFunded),
                    "GrantOnly" => Ok(FundingMode::GrantOnly),
                    "Dual" => Ok(FundingMode::Dual),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        if !self.registered_contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.wasmx.v1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.registered_contracts.is_empty() {
            struct_ser.serialize_field("registeredContracts", &self.registered_contracts)?;
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
        const FIELDS: &[&str] = &["params", "registered_contracts", "registeredContracts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            RegisteredContracts,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "registeredContracts" | "registered_contracts" => {
                                Ok(GeneratedField::RegisteredContracts)
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
                formatter.write_str("struct injective.wasmx.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut registered_contracts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::RegisteredContracts => {
                            if registered_contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredContracts",
                                ));
                            }
                            registered_contracts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    registered_contracts: registered_contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("injective.wasmx.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgActivateContract {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.MsgActivateContract", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgActivateContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "contract_address", "contractAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
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
                            "sender" => Ok(GeneratedField::Sender),
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
            type Value = MsgActivateContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgActivateContract")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgActivateContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgActivateContract {
                    sender: sender__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgActivateContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgActivateContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.MsgActivateContractResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgActivateContractResponse {
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
            type Value = MsgActivateContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgActivateContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgActivateContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgActivateContractResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgActivateContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgDeactivateContract {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.MsgDeactivateContract", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgDeactivateContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "contract_address", "contractAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
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
                            "sender" => Ok(GeneratedField::Sender),
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
            type Value = MsgDeactivateContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgDeactivateContract")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgDeactivateContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgDeactivateContract {
                    sender: sender__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgDeactivateContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgDeactivateContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.MsgDeactivateContractResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgDeactivateContractResponse {
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
            type Value = MsgDeactivateContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgDeactivateContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgDeactivateContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDeactivateContractResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgDeactivateContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgExecuteContractCompat {
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
            serializer.serialize_struct("injective.wasmx.v1.MsgExecuteContractCompat", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.msg.is_empty() {
            struct_ser.serialize_field("msg", &self.msg)?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgExecuteContractCompat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "contract", "msg", "funds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
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
                            "sender" => Ok(GeneratedField::Sender),
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
            type Value = MsgExecuteContractCompat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgExecuteContractCompat")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgExecuteContractCompat, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract__ = None;
                let mut msg__ = None;
                let mut funds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
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
                            msg__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgExecuteContractCompat {
                    sender: sender__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgExecuteContractCompat",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgExecuteContractCompatResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("injective.wasmx.v1.MsgExecuteContractCompatResponse", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgExecuteContractCompatResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MsgExecuteContractCompatResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgExecuteContractCompatResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgExecuteContractCompatResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(MsgExecuteContractCompatResponse {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgExecuteContractCompatResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRegisterContract {
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
        if self.contract_registration_request.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.MsgRegisterContract", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if let Some(v) = self.contract_registration_request.as_ref() {
            struct_ser.serialize_field("contractRegistrationRequest", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRegisterContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "contract_registration_request",
            "contractRegistrationRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            ContractRegistrationRequest,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "contractRegistrationRequest" | "contract_registration_request" => {
                                Ok(GeneratedField::ContractRegistrationRequest)
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
            type Value = MsgRegisterContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgRegisterContract")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRegisterContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract_registration_request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContractRegistrationRequest => {
                            if contract_registration_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "contractRegistrationRequest",
                                ));
                            }
                            contract_registration_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgRegisterContract {
                    sender: sender__.unwrap_or_default(),
                    contract_registration_request: contract_registration_request__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgRegisterContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRegisterContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.MsgRegisterContractResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRegisterContractResponse {
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
            type Value = MsgRegisterContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgRegisterContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgRegisterContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRegisterContractResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgRegisterContractResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateContract {
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if self.gas_limit != 0 {
            len += 1;
        }
        if self.gas_price != 0 {
            len += 1;
        }
        if !self.admin_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.MsgUpdateContract", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if self.gas_limit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("gasLimit", ToString::to_string(&self.gas_limit).as_str())?;
        }
        if self.gas_price != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("gasPrice", ToString::to_string(&self.gas_price).as_str())?;
        }
        if !self.admin_address.is_empty() {
            struct_ser.serialize_field("adminAddress", &self.admin_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "contract_address",
            "contractAddress",
            "gas_limit",
            "gasLimit",
            "gas_price",
            "gasPrice",
            "admin_address",
            "adminAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            ContractAddress,
            GasLimit,
            GasPrice,
            AdminAddress,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "contractAddress" | "contract_address" => {
                                Ok(GeneratedField::ContractAddress)
                            }
                            "gasLimit" | "gas_limit" => Ok(GeneratedField::GasLimit),
                            "gasPrice" | "gas_price" => Ok(GeneratedField::GasPrice),
                            "adminAddress" | "admin_address" => Ok(GeneratedField::AdminAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgUpdateContract")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut contract_address__ = None;
                let mut gas_limit__ = None;
                let mut gas_price__ = None;
                let mut admin_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasLimit => {
                            if gas_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasLimit"));
                            }
                            gas_limit__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::GasPrice => {
                            if gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasPrice"));
                            }
                            gas_price__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AdminAddress => {
                            if admin_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminAddress"));
                            }
                            admin_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateContract {
                    sender: sender__.unwrap_or_default(),
                    contract_address: contract_address__.unwrap_or_default(),
                    gas_limit: gas_limit__.unwrap_or_default(),
                    gas_price: gas_price__.unwrap_or_default(),
                    admin_address: admin_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgUpdateContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateContractResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.MsgUpdateContractResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateContractResponse {
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
            type Value = MsgUpdateContractResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.MsgUpdateContractResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<MsgUpdateContractResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateContractResponse {})
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.MsgUpdateContractResponse",
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
            serializer.serialize_struct("injective.wasmx.v1.MsgUpdateParams", len)?;
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
                formatter.write_str("struct injective.wasmx.v1.MsgUpdateParams")
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
            "injective.wasmx.v1.MsgUpdateParams",
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
            serializer.serialize_struct("injective.wasmx.v1.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct injective.wasmx.v1.MsgUpdateParamsResponse")
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
            "injective.wasmx.v1.MsgUpdateParamsResponse",
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
        if self.is_execution_enabled {
            len += 1;
        }
        if self.max_begin_block_total_gas != 0 {
            len += 1;
        }
        if self.max_contract_gas_limit != 0 {
            len += 1;
        }
        if self.min_gas_price != 0 {
            len += 1;
        }
        if self.register_contract_access.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("injective.wasmx.v1.Params", len)?;
        if self.is_execution_enabled {
            struct_ser.serialize_field("isExecutionEnabled", &self.is_execution_enabled)?;
        }
        if self.max_begin_block_total_gas != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "maxBeginBlockTotalGas",
                ToString::to_string(&self.max_begin_block_total_gas).as_str(),
            )?;
        }
        if self.max_contract_gas_limit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "maxContractGasLimit",
                ToString::to_string(&self.max_contract_gas_limit).as_str(),
            )?;
        }
        if self.min_gas_price != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "minGasPrice",
                ToString::to_string(&self.min_gas_price).as_str(),
            )?;
        }
        if let Some(v) = self.register_contract_access.as_ref() {
            struct_ser.serialize_field("registerContractAccess", v)?;
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
            "is_execution_enabled",
            "isExecutionEnabled",
            "max_begin_block_total_gas",
            "maxBeginBlockTotalGas",
            "max_contract_gas_limit",
            "maxContractGasLimit",
            "min_gas_price",
            "minGasPrice",
            "register_contract_access",
            "registerContractAccess",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsExecutionEnabled,
            MaxBeginBlockTotalGas,
            MaxContractGasLimit,
            MinGasPrice,
            RegisterContractAccess,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "isExecutionEnabled" | "is_execution_enabled" => {
                                Ok(GeneratedField::IsExecutionEnabled)
                            }
                            "maxBeginBlockTotalGas" | "max_begin_block_total_gas" => {
                                Ok(GeneratedField::MaxBeginBlockTotalGas)
                            }
                            "maxContractGasLimit" | "max_contract_gas_limit" => {
                                Ok(GeneratedField::MaxContractGasLimit)
                            }
                            "minGasPrice" | "min_gas_price" => Ok(GeneratedField::MinGasPrice),
                            "registerContractAccess" | "register_contract_access" => {
                                Ok(GeneratedField::RegisterContractAccess)
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
                formatter.write_str("struct injective.wasmx.v1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut is_execution_enabled__ = None;
                let mut max_begin_block_total_gas__ = None;
                let mut max_contract_gas_limit__ = None;
                let mut min_gas_price__ = None;
                let mut register_contract_access__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsExecutionEnabled => {
                            if is_execution_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "isExecutionEnabled",
                                ));
                            }
                            is_execution_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxBeginBlockTotalGas => {
                            if max_begin_block_total_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxBeginBlockTotalGas",
                                ));
                            }
                            max_begin_block_total_gas__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MaxContractGasLimit => {
                            if max_contract_gas_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxContractGasLimit",
                                ));
                            }
                            max_contract_gas_limit__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinGasPrice => {
                            if min_gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minGasPrice"));
                            }
                            min_gas_price__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RegisterContractAccess => {
                            if register_contract_access__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registerContractAccess",
                                ));
                            }
                            register_contract_access__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Params {
                    is_execution_enabled: is_execution_enabled__.unwrap_or_default(),
                    max_begin_block_total_gas: max_begin_block_total_gas__.unwrap_or_default(),
                    max_contract_gas_limit: max_contract_gas_limit__.unwrap_or_default(),
                    min_gas_price: min_gas_price__.unwrap_or_default(),
                    register_contract_access: register_contract_access__,
                })
            }
        }
        deserializer.deserialize_struct("injective.wasmx.v1.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryContractRegistrationInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contract_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.wasmx.v1.QueryContractRegistrationInfoRequest",
            len,
        )?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryContractRegistrationInfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract_address", "contractAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryContractRegistrationInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.wasmx.v1.QueryContractRegistrationInfoRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractRegistrationInfoRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryContractRegistrationInfoRequest {
                    contract_address: contract_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.QueryContractRegistrationInfoRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryContractRegistrationInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.contract.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "injective.wasmx.v1.QueryContractRegistrationInfoResponse",
            len,
        )?;
        if let Some(v) = self.contract.as_ref() {
            struct_ser.serialize_field("contract", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryContractRegistrationInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["contract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryContractRegistrationInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct injective.wasmx.v1.QueryContractRegistrationInfoResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryContractRegistrationInfoResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryContractRegistrationInfoResponse {
                    contract: contract__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.QueryContractRegistrationInfoResponse",
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
            serializer.serialize_struct("injective.wasmx.v1.QueryModuleStateRequest", len)?;
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
                formatter.write_str("struct injective.wasmx.v1.QueryModuleStateRequest")
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
            "injective.wasmx.v1.QueryModuleStateRequest",
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
            serializer.serialize_struct("injective.wasmx.v1.QueryModuleStateResponse", len)?;
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
                formatter.write_str("struct injective.wasmx.v1.QueryModuleStateResponse")
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
            "injective.wasmx.v1.QueryModuleStateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryWasmxParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.QueryWasmxParamsRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryWasmxParamsRequest {
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
            type Value = QueryWasmxParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.QueryWasmxParamsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryWasmxParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryWasmxParamsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.QueryWasmxParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryWasmxParamsResponse {
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
            serializer.serialize_struct("injective.wasmx.v1.QueryWasmxParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryWasmxParamsResponse {
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
            type Value = QueryWasmxParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.QueryWasmxParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<QueryWasmxParamsResponse, V::Error>
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
                Ok(QueryWasmxParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.QueryWasmxParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RegisteredContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gas_limit != 0 {
            len += 1;
        }
        if self.gas_price != 0 {
            len += 1;
        }
        if self.is_executable {
            len += 1;
        }
        if self.code_id != 0 {
            len += 1;
        }
        if !self.admin_address.is_empty() {
            len += 1;
        }
        if !self.granter_address.is_empty() {
            len += 1;
        }
        if self.fund_mode != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.RegisteredContract", len)?;
        if self.gas_limit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("gasLimit", ToString::to_string(&self.gas_limit).as_str())?;
        }
        if self.gas_price != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("gasPrice", ToString::to_string(&self.gas_price).as_str())?;
        }
        if self.is_executable {
            struct_ser.serialize_field("isExecutable", &self.is_executable)?;
        }
        if self.code_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("codeId", ToString::to_string(&self.code_id).as_str())?;
        }
        if !self.admin_address.is_empty() {
            struct_ser.serialize_field("adminAddress", &self.admin_address)?;
        }
        if !self.granter_address.is_empty() {
            struct_ser.serialize_field("granterAddress", &self.granter_address)?;
        }
        if self.fund_mode != 0 {
            let v = FundingMode::try_from(self.fund_mode).map_err(|_| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.fund_mode))
            })?;
            struct_ser.serialize_field("fundMode", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RegisteredContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gas_limit",
            "gasLimit",
            "gas_price",
            "gasPrice",
            "is_executable",
            "isExecutable",
            "code_id",
            "codeId",
            "admin_address",
            "adminAddress",
            "granter_address",
            "granterAddress",
            "fund_mode",
            "fundMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GasLimit,
            GasPrice,
            IsExecutable,
            CodeId,
            AdminAddress,
            GranterAddress,
            FundMode,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "gasLimit" | "gas_limit" => Ok(GeneratedField::GasLimit),
                            "gasPrice" | "gas_price" => Ok(GeneratedField::GasPrice),
                            "isExecutable" | "is_executable" => Ok(GeneratedField::IsExecutable),
                            "codeId" | "code_id" => Ok(GeneratedField::CodeId),
                            "adminAddress" | "admin_address" => Ok(GeneratedField::AdminAddress),
                            "granterAddress" | "granter_address" => {
                                Ok(GeneratedField::GranterAddress)
                            }
                            "fundMode" | "fund_mode" => Ok(GeneratedField::FundMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisteredContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.RegisteredContract")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisteredContract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut gas_limit__ = None;
                let mut gas_price__ = None;
                let mut is_executable__ = None;
                let mut code_id__ = None;
                let mut admin_address__ = None;
                let mut granter_address__ = None;
                let mut fund_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GasLimit => {
                            if gas_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasLimit"));
                            }
                            gas_limit__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::GasPrice => {
                            if gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasPrice"));
                            }
                            gas_price__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::IsExecutable => {
                            if is_executable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExecutable"));
                            }
                            is_executable__ = Some(map_.next_value()?);
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
                        GeneratedField::AdminAddress => {
                            if admin_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminAddress"));
                            }
                            admin_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GranterAddress => {
                            if granter_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("granterAddress"));
                            }
                            granter_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FundMode => {
                            if fund_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fundMode"));
                            }
                            fund_mode__ = Some(map_.next_value::<FundingMode>()? as i32);
                        }
                    }
                }
                Ok(RegisteredContract {
                    gas_limit: gas_limit__.unwrap_or_default(),
                    gas_price: gas_price__.unwrap_or_default(),
                    is_executable: is_executable__.unwrap_or_default(),
                    code_id: code_id__.unwrap_or_default(),
                    admin_address: admin_address__.unwrap_or_default(),
                    granter_address: granter_address__.unwrap_or_default(),
                    fund_mode: fund_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.RegisteredContract",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RegisteredContractWithAddress {
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
        if self.registered_contract.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("injective.wasmx.v1.RegisteredContractWithAddress", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.registered_contract.as_ref() {
            struct_ser.serialize_field("registeredContract", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RegisteredContractWithAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "registered_contract", "registeredContract"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            RegisteredContract,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
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
                            "registeredContract" | "registered_contract" => {
                                Ok(GeneratedField::RegisteredContract)
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
            type Value = RegisteredContractWithAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct injective.wasmx.v1.RegisteredContractWithAddress")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<RegisteredContractWithAddress, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut registered_contract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegisteredContract => {
                            if registered_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredContract",
                                ));
                            }
                            registered_contract__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegisteredContractWithAddress {
                    address: address__.unwrap_or_default(),
                    registered_contract: registered_contract__,
                })
            }
        }
        deserializer.deserialize_struct(
            "injective.wasmx.v1.RegisteredContractWithAddress",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
