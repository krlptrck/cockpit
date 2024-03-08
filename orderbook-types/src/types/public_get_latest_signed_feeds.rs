#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///ForwardFeedDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "confidence",
    "currency",
    "deadline",
    "expiry",
    "fwd_diff",
    "signatures",
    "spot_aggregate_latest",
    "spot_aggregate_start",
    "timestamp"
  ],
  "properties": {
    "confidence": {
      "title": "confidence",
      "description": "The confidence score of the price",
      "type": "string",
      "format": "decimal"
    },
    "currency": {
      "title": "currency",
      "description": "The currency for which the spot feed represents",
      "type": "string"
    },
    "deadline": {
      "title": "deadline",
      "description": "The latest time the data can be submitted on chain",
      "type": "integer"
    },
    "expiry": {
      "title": "expiry",
      "description": "The expiry for the forward feed",
      "type": "integer"
    },
    "fwd_diff": {
      "title": "fwd_diff",
      "description": "difference of forward price from current spot price",
      "type": "string",
      "format": "decimal"
    },
    "signatures": {
      "description": "Signatures",
      "type": "object",
      "$ref": "#/definitions/OracleSignatureDataSchema",
      "field_many": false
    },
    "spot_aggregate_latest": {
      "title": "spot_aggregate_latest",
      "description": "expiry -> spot * time value at the latest timestamp",
      "type": "string",
      "format": "decimal"
    },
    "spot_aggregate_start": {
      "title": "spot_aggregate_start",
      "description": "spot * time value at the start of the settlement period",
      "type": "string",
      "format": "decimal"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "The timestamp for which the data was created",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct ForwardFeedDataSchema {
    ///The confidence score of the price
    pub confidence: bigdecimal::BigDecimal,
    ///The currency for which the spot feed represents
    pub currency: String,
    ///The latest time the data can be submitted on chain
    pub deadline: i64,
    ///The expiry for the forward feed
    pub expiry: i64,
    ///difference of forward price from current spot price
    pub fwd_diff: bigdecimal::BigDecimal,
    ///Signatures
    pub signatures: OracleSignatureDataSchema,
    ///expiry -> spot * time value at the latest timestamp
    pub spot_aggregate_latest: bigdecimal::BigDecimal,
    ///spot * time value at the start of the settlement period
    pub spot_aggregate_start: bigdecimal::BigDecimal,
    ///The timestamp for which the data was created
    pub timestamp: i64,
}
impl From<&ForwardFeedDataSchema> for ForwardFeedDataSchema {
    fn from(value: &ForwardFeedDataSchema) -> Self {
        value.clone()
    }
}
///OracleSignatureDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "properties": {
    "signatures": {
      "title": "signatures",
      "description": "The signatures of the given signers",
      "type": "array",
      "items": {
        "title": "signatures",
        "type": "string"
      }
    },
    "signers": {
      "title": "signers",
      "description": "The signers who verify the data integrity",
      "type": "array",
      "items": {
        "title": "signers",
        "type": "string"
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OracleSignatureDataSchema {
    ///The signatures of the given signers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<String>,
    ///The signers who verify the data integrity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signers: Vec<String>,
}
impl From<&OracleSignatureDataSchema> for OracleSignatureDataSchema {
    fn from(value: &OracleSignatureDataSchema) -> Self {
        value.clone()
    }
}
///PerpFeedDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "confidence",
    "currency",
    "deadline",
    "signatures",
    "spot_diff_value",
    "timestamp",
    "type"
  ],
  "properties": {
    "confidence": {
      "title": "confidence",
      "description": "The confidence score of the price",
      "type": "string",
      "format": "decimal"
    },
    "currency": {
      "title": "currency",
      "description": "The currency for which the spot feed represents",
      "type": "string"
    },
    "deadline": {
      "title": "deadline",
      "description": "The latest time the data can be submitted on chain",
      "type": "integer"
    },
    "signatures": {
      "description": "Signatures",
      "type": "object",
      "$ref": "#/definitions/OracleSignatureDataSchema",
      "field_many": false
    },
    "spot_diff_value": {
      "title": "spot_diff_value",
      "description": "The difference between the spot price and the perp price",
      "type": "string",
      "format": "decimal"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "The timestamp for which the data was created",
      "type": "integer"
    },
    "type": {
      "title": "type",
      "description": "The type of perp feed; mid price, ask impact or bid impact",
      "type": "string",
      "enum": [
        "P",
        "A",
        "B"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PerpFeedDataSchema {
    ///The confidence score of the price
    pub confidence: bigdecimal::BigDecimal,
    ///The currency for which the spot feed represents
    pub currency: String,
    ///The latest time the data can be submitted on chain
    pub deadline: i64,
    ///Signatures
    pub signatures: OracleSignatureDataSchema,
    ///The difference between the spot price and the perp price
    pub spot_diff_value: bigdecimal::BigDecimal,
    ///The timestamp for which the data was created
    pub timestamp: i64,
    ///The type of perp feed; mid price, ask impact or bid impact
    #[serde(rename = "type")]
    pub type_: Type,
}
impl From<&PerpFeedDataSchema> for PerpFeedDataSchema {
    fn from(value: &PerpFeedDataSchema) -> Self {
        value.clone()
    }
}
///Get latest signed data feeds
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_latest_signed_feeds",
  "description": "Get latest signed data feeds",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetLatestSignedFeedsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetLatestSignedFeeds(pub PublicGetLatestSignedFeedsJsonrpcSchema);
impl std::ops::Deref for PublicGetLatestSignedFeeds {
    type Target = PublicGetLatestSignedFeedsJsonrpcSchema;
    fn deref(&self) -> &PublicGetLatestSignedFeedsJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetLatestSignedFeeds> for PublicGetLatestSignedFeedsJsonrpcSchema {
    fn from(value: PublicGetLatestSignedFeeds) -> Self {
        value.0
    }
}
impl From<&PublicGetLatestSignedFeeds> for PublicGetLatestSignedFeeds {
    fn from(value: &PublicGetLatestSignedFeeds) -> Self {
        value.clone()
    }
}
impl From<PublicGetLatestSignedFeedsJsonrpcSchema> for PublicGetLatestSignedFeeds {
    fn from(value: PublicGetLatestSignedFeedsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetLatestSignedFeedsJsonrpcSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "request",
    "response"
  ],
  "properties": {
    "request": {
      "type": "object",
      "$ref": "#/definitions/PublicGetLatestSignedFeedsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetLatestSignedFeedsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetLatestSignedFeedsJsonrpcSchema {
    pub request: PublicGetLatestSignedFeedsRequestSchema,
    pub response: PublicGetLatestSignedFeedsResponseSchema,
}
impl From<&PublicGetLatestSignedFeedsJsonrpcSchema>
for PublicGetLatestSignedFeedsJsonrpcSchema {
    fn from(value: &PublicGetLatestSignedFeedsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetLatestSignedFeedsParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency filter, (defaults to all currencies)",
      "type": [
        "string",
        "null"
      ]
    },
    "expiry": {
      "title": "expiry",
      "description": "Expiry filter for options and forward data (defaults to all expiries). Use `0` to get data only for spot and perpetuals",
      "type": [
        "integer",
        "null"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetLatestSignedFeedsParamsSchema {
    ///Currency filter, (defaults to all currencies)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Expiry filter for options and forward data (defaults to all expiries). Use `0` to get data only for spot and perpetuals
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
}
impl From<&PublicGetLatestSignedFeedsParamsSchema>
for PublicGetLatestSignedFeedsParamsSchema {
    fn from(value: &PublicGetLatestSignedFeedsParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetLatestSignedFeedsRequestSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "method",
    "params"
  ],
  "properties": {
    "id": {
      "oneOf": [
        {
          "title": "",
          "type": "string"
        },
        {
          "title": "",
          "type": "integer"
        }
      ]
    },
    "method": {
      "title": "method",
      "type": "string",
      "const": "public/get_latest_signed_feeds"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetLatestSignedFeedsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetLatestSignedFeedsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetLatestSignedFeedsRequestSchemaId>,
    pub method: String,
    pub params: PublicGetLatestSignedFeedsParamsSchema,
}
impl From<&PublicGetLatestSignedFeedsRequestSchema>
for PublicGetLatestSignedFeedsRequestSchema {
    fn from(value: &PublicGetLatestSignedFeedsRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetLatestSignedFeedsRequestSchemaId
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "oneOf": [
    {
      "title": "",
      "type": "string"
    },
    {
      "title": "",
      "type": "integer"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PublicGetLatestSignedFeedsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetLatestSignedFeedsRequestSchemaId>
for PublicGetLatestSignedFeedsRequestSchemaId {
    fn from(value: &PublicGetLatestSignedFeedsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetLatestSignedFeedsRequestSchemaId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for PublicGetLatestSignedFeedsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetLatestSignedFeedsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetLatestSignedFeedsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetLatestSignedFeedsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetLatestSignedFeedsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetLatestSignedFeedsResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "id",
    "result"
  ],
  "properties": {
    "id": {
      "oneOf": [
        {
          "title": "",
          "type": "string"
        },
        {
          "title": "",
          "type": "integer"
        }
      ]
    },
    "result": {
      "description": "",
      "type": "object",
      "$ref": "#/definitions/PublicGetLatestSignedFeedsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetLatestSignedFeedsResponseSchema {
    pub id: PublicGetLatestSignedFeedsResponseSchemaId,
    ///
    pub result: PublicGetLatestSignedFeedsResultSchema,
}
impl From<&PublicGetLatestSignedFeedsResponseSchema>
for PublicGetLatestSignedFeedsResponseSchema {
    fn from(value: &PublicGetLatestSignedFeedsResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetLatestSignedFeedsResponseSchemaId
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "oneOf": [
    {
      "title": "",
      "type": "string"
    },
    {
      "title": "",
      "type": "integer"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PublicGetLatestSignedFeedsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetLatestSignedFeedsResponseSchemaId>
for PublicGetLatestSignedFeedsResponseSchemaId {
    fn from(value: &PublicGetLatestSignedFeedsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetLatestSignedFeedsResponseSchemaId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for PublicGetLatestSignedFeedsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetLatestSignedFeedsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetLatestSignedFeedsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetLatestSignedFeedsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetLatestSignedFeedsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetLatestSignedFeedsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "fwd_data",
    "perp_data",
    "spot_data",
    "vol_data"
  ],
  "properties": {
    "fwd_data": {
      "title": "fwd_data",
      "description": "currency -> expiry -> latest forward feed data",
      "type": "object",
      "additionalProperties": {
        "title": "fwd_data",
        "type": "object",
        "additionalProperties": {
          "type": "object",
          "$ref": "#/definitions/ForwardFeedDataSchema",
          "field_many": true
        }
      }
    },
    "perp_data": {
      "title": "perp_data",
      "description": "currency -> feed type -> latest perp feed data",
      "type": "object",
      "additionalProperties": {
        "title": "perp_data",
        "type": "object",
        "additionalProperties": {
          "type": "object",
          "$ref": "#/definitions/PerpFeedDataSchema",
          "field_many": true
        }
      }
    },
    "spot_data": {
      "title": "spot_data",
      "description": "currency -> latest spot feed data",
      "type": "object",
      "additionalProperties": {
        "type": "object",
        "$ref": "#/definitions/SpotFeedDataSchema",
        "field_many": true
      }
    },
    "vol_data": {
      "title": "vol_data",
      "description": "currency -> expiry -> latest vol feed data",
      "type": "object",
      "additionalProperties": {
        "title": "vol_data",
        "type": "object",
        "additionalProperties": {
          "type": "object",
          "$ref": "#/definitions/VolFeedDataSchema",
          "field_many": true
        }
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetLatestSignedFeedsResultSchema {
    ///currency -> expiry -> latest forward feed data
    pub fwd_data: std::collections::HashMap<
        String,
        std::collections::HashMap<String, ForwardFeedDataSchema>,
    >,
    ///currency -> feed type -> latest perp feed data
    pub perp_data: std::collections::HashMap<
        String,
        std::collections::HashMap<String, PerpFeedDataSchema>,
    >,
    ///currency -> latest spot feed data
    pub spot_data: std::collections::HashMap<String, SpotFeedDataSchema>,
    ///currency -> expiry -> latest vol feed data
    pub vol_data: std::collections::HashMap<
        String,
        std::collections::HashMap<String, VolFeedDataSchema>,
    >,
}
impl From<&PublicGetLatestSignedFeedsResultSchema>
for PublicGetLatestSignedFeedsResultSchema {
    fn from(value: &PublicGetLatestSignedFeedsResultSchema) -> Self {
        value.clone()
    }
}
///SpotFeedDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "confidence",
    "currency",
    "deadline",
    "price",
    "signatures",
    "timestamp"
  ],
  "properties": {
    "confidence": {
      "title": "confidence",
      "description": "The confidence score of the price",
      "type": "string",
      "format": "decimal"
    },
    "currency": {
      "title": "currency",
      "description": "The currency for which the spot feed represents",
      "type": "string"
    },
    "deadline": {
      "title": "deadline",
      "description": "The latest time the data can be submitted on chain",
      "type": "integer"
    },
    "price": {
      "title": "price",
      "description": "The price of the currency in USD",
      "type": "string",
      "format": "decimal"
    },
    "signatures": {
      "description": "Signatures",
      "type": "object",
      "$ref": "#/definitions/OracleSignatureDataSchema",
      "field_many": false
    },
    "timestamp": {
      "title": "timestamp",
      "description": "The timestamp for which the data was created",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SpotFeedDataSchema {
    ///The confidence score of the price
    pub confidence: bigdecimal::BigDecimal,
    ///The currency for which the spot feed represents
    pub currency: String,
    ///The latest time the data can be submitted on chain
    pub deadline: i64,
    ///The price of the currency in USD
    pub price: bigdecimal::BigDecimal,
    ///Signatures
    pub signatures: OracleSignatureDataSchema,
    ///The timestamp for which the data was created
    pub timestamp: i64,
}
impl From<&SpotFeedDataSchema> for SpotFeedDataSchema {
    fn from(value: &SpotFeedDataSchema) -> Self {
        value.clone()
    }
}
///The type of perp feed; mid price, ask impact or bid impact
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "type",
  "description": "The type of perp feed; mid price, ask impact or bid impact",
  "type": "string",
  "enum": [
    "P",
    "A",
    "B"
  ]
}*/
/// ```
/// </details>
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum Type {
    P,
    A,
    B,
}
impl From<&Type> for Type {
    fn from(value: &Type) -> Self {
        value.clone()
    }
}
impl ToString for Type {
    fn to_string(&self) -> String {
        match *self {
            Self::P => "P".to_string(),
            Self::A => "A".to_string(),
            Self::B => "B".to_string(),
        }
    }
}
impl std::str::FromStr for Type {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "P" => Ok(Self::P),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Type {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Type {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Type {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///VolFeedDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "confidence",
    "currency",
    "deadline",
    "expiry",
    "signatures",
    "timestamp",
    "vol_data"
  ],
  "properties": {
    "confidence": {
      "title": "confidence",
      "description": "The confidence score of the price",
      "type": "string",
      "format": "decimal"
    },
    "currency": {
      "title": "currency",
      "description": "The currency for which the spot feed represents",
      "type": "string"
    },
    "deadline": {
      "title": "deadline",
      "description": "The latest time the data can be submitted on chain",
      "type": "integer"
    },
    "expiry": {
      "title": "expiry",
      "description": "The expiry for the options for the vol feed",
      "type": "integer"
    },
    "signatures": {
      "description": "Signatures",
      "type": "object",
      "$ref": "#/definitions/OracleSignatureDataSchema",
      "field_many": false
    },
    "timestamp": {
      "title": "timestamp",
      "description": "The timestamp for which the data was created",
      "type": "integer"
    },
    "vol_data": {
      "description": "The SVI parameters for the vol curve",
      "type": "object",
      "$ref": "#/definitions/VolSVIParamDataSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct VolFeedDataSchema {
    ///The confidence score of the price
    pub confidence: bigdecimal::BigDecimal,
    ///The currency for which the spot feed represents
    pub currency: String,
    ///The latest time the data can be submitted on chain
    pub deadline: i64,
    ///The expiry for the options for the vol feed
    pub expiry: i64,
    ///Signatures
    pub signatures: OracleSignatureDataSchema,
    ///The timestamp for which the data was created
    pub timestamp: i64,
    ///The SVI parameters for the vol curve
    pub vol_data: VolSviParamDataSchema,
}
impl From<&VolFeedDataSchema> for VolFeedDataSchema {
    fn from(value: &VolFeedDataSchema) -> Self {
        value.clone()
    }
}
///VolSviParamDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "SVI_a",
    "SVI_b",
    "SVI_fwd",
    "SVI_m",
    "SVI_refTau",
    "SVI_rho",
    "SVI_sigma"
  ],
  "properties": {
    "SVI_a": {
      "title": "SVI_a",
      "type": "string",
      "format": "decimal"
    },
    "SVI_b": {
      "title": "SVI_b",
      "type": "string",
      "format": "decimal"
    },
    "SVI_fwd": {
      "title": "SVI_fwd",
      "type": "string",
      "format": "decimal"
    },
    "SVI_m": {
      "title": "SVI_m",
      "type": "string",
      "format": "decimal"
    },
    "SVI_refTau": {
      "title": "SVI_refTau",
      "type": "string",
      "format": "decimal"
    },
    "SVI_rho": {
      "title": "SVI_rho",
      "type": "string",
      "format": "decimal"
    },
    "SVI_sigma": {
      "title": "SVI_sigma",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct VolSviParamDataSchema {
    #[serde(rename = "SVI_a")]
    pub svi_a: bigdecimal::BigDecimal,
    #[serde(rename = "SVI_b")]
    pub svi_b: bigdecimal::BigDecimal,
    #[serde(rename = "SVI_fwd")]
    pub svi_fwd: bigdecimal::BigDecimal,
    #[serde(rename = "SVI_m")]
    pub svi_m: bigdecimal::BigDecimal,
    #[serde(rename = "SVI_refTau")]
    pub svi_ref_tau: bigdecimal::BigDecimal,
    #[serde(rename = "SVI_rho")]
    pub svi_rho: bigdecimal::BigDecimal,
    #[serde(rename = "SVI_sigma")]
    pub svi_sigma: bigdecimal::BigDecimal,
}
impl From<&VolSviParamDataSchema> for VolSviParamDataSchema {
    fn from(value: &VolSviParamDataSchema) -> Self {
        value.clone()
    }
}
