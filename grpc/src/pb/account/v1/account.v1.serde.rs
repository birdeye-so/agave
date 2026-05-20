// @generated
impl serde::Serialize for Account {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pubkey.is_empty() {
            len += 1;
        }
        if self.slot != 0 {
            len += 1;
        }
        if self.lamports != 0 {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if self.executable {
            len += 1;
        }
        if self.rent_epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("account.v1.Account", len)?;
        if !self.pubkey.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pubkey", pbjson::private::base64::encode(&self.pubkey).as_str())?;
        }
        if self.slot != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("slot", ToString::to_string(&self.slot).as_str())?;
        }
        if self.lamports != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lamports", ToString::to_string(&self.lamports).as_str())?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.owner.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("owner", pbjson::private::base64::encode(&self.owner).as_str())?;
        }
        if self.executable {
            struct_ser.serialize_field("executable", &self.executable)?;
        }
        if self.rent_epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rentEpoch", ToString::to_string(&self.rent_epoch).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Account {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pubkey",
            "slot",
            "lamports",
            "data",
            "owner",
            "executable",
            "rent_epoch",
            "rentEpoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pubkey,
            Slot,
            Lamports,
            Data,
            Owner,
            Executable,
            RentEpoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pubkey" => Ok(GeneratedField::Pubkey),
                            "slot" => Ok(GeneratedField::Slot),
                            "lamports" => Ok(GeneratedField::Lamports),
                            "data" => Ok(GeneratedField::Data),
                            "owner" => Ok(GeneratedField::Owner),
                            "executable" => Ok(GeneratedField::Executable),
                            "rentEpoch" | "rent_epoch" => Ok(GeneratedField::RentEpoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Account;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct account.v1.Account")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Account, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pubkey__ = None;
                let mut slot__ = None;
                let mut lamports__ = None;
                let mut data__ = None;
                let mut owner__ = None;
                let mut executable__ = None;
                let mut rent_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pubkey => {
                            if pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubkey"));
                            }
                            pubkey__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Slot => {
                            if slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slot"));
                            }
                            slot__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Lamports => {
                            if lamports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lamports"));
                            }
                            lamports__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Executable => {
                            if executable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executable"));
                            }
                            executable__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RentEpoch => {
                            if rent_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rentEpoch"));
                            }
                            rent_epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Account {
                    pubkey: pubkey__.unwrap_or_default(),
                    slot: slot__.unwrap_or_default(),
                    lamports: lamports__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    executable: executable__.unwrap_or_default(),
                    rent_epoch: rent_epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("account.v1.Account", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProgramAccountsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        if !self.program_id.is_empty() {
            len += 1;
        }
        if !self.discriminators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("account.v1.ListProgramAccountsRequest", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if !self.program_id.is_empty() {
            struct_ser.serialize_field("programId", &self.program_id)?;
        }
        if !self.discriminators.is_empty() {
            struct_ser.serialize_field("discriminators", &self.discriminators.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProgramAccountsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
            "program_id",
            "programId",
            "discriminators",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
            ProgramId,
            Discriminators,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "config" => Ok(GeneratedField::Config),
                            "programId" | "program_id" => Ok(GeneratedField::ProgramId),
                            "discriminators" => Ok(GeneratedField::Discriminators),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListProgramAccountsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct account.v1.ListProgramAccountsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProgramAccountsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                let mut program_id__ = None;
                let mut discriminators__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::ProgramId => {
                            if program_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("programId"));
                            }
                            program_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Discriminators => {
                            if discriminators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("discriminators"));
                            }
                            discriminators__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(ListProgramAccountsRequest {
                    config: config__,
                    program_id: program_id__.unwrap_or_default(),
                    discriminators: discriminators__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("account.v1.ListProgramAccountsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RpcContextConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.commitment.is_some() {
            len += 1;
        }
        if self.min_context_slot.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("account.v1.RpcContextConfig", len)?;
        if let Some(v) = self.commitment.as_ref() {
            let v = rpc_context_config::Commitment::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("commitment", &v)?;
        }
        if let Some(v) = self.min_context_slot.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("minContextSlot", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RpcContextConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commitment",
            "min_context_slot",
            "minContextSlot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Commitment,
            MinContextSlot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "commitment" => Ok(GeneratedField::Commitment),
                            "minContextSlot" | "min_context_slot" => Ok(GeneratedField::MinContextSlot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RpcContextConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct account.v1.RpcContextConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RpcContextConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut commitment__ = None;
                let mut min_context_slot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Commitment => {
                            if commitment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commitment"));
                            }
                            commitment__ = map_.next_value::<::std::option::Option<rpc_context_config::Commitment>>()?.map(|x| x as i32);
                        }
                        GeneratedField::MinContextSlot => {
                            if min_context_slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minContextSlot"));
                            }
                            min_context_slot__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(RpcContextConfig {
                    commitment: commitment__,
                    min_context_slot: min_context_slot__,
                })
            }
        }
        deserializer.deserialize_struct("account.v1.RpcContextConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rpc_context_config::Commitment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COMMITMENT_UNSPECIFIED",
            Self::Processed => "COMMITMENT_PROCESSED",
            Self::Confirmed => "COMMITMENT_CONFIRMED",
            Self::Finalized => "COMMITMENT_FINALIZED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rpc_context_config::Commitment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COMMITMENT_UNSPECIFIED",
            "COMMITMENT_PROCESSED",
            "COMMITMENT_CONFIRMED",
            "COMMITMENT_FINALIZED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rpc_context_config::Commitment;

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
                    "COMMITMENT_UNSPECIFIED" => Ok(rpc_context_config::Commitment::Unspecified),
                    "COMMITMENT_PROCESSED" => Ok(rpc_context_config::Commitment::Processed),
                    "COMMITMENT_CONFIRMED" => Ok(rpc_context_config::Commitment::Confirmed),
                    "COMMITMENT_FINALIZED" => Ok(rpc_context_config::Commitment::Finalized),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
