use scylla::frame::response::result::CqlValue;
use scylla::{cql_to_rust::FromCqlVal, FromRow};

#[derive(Debug, Clone)]
pub enum PayloadType {
    User,
    Admin,
    Client,
}

impl FromCqlVal<CqlValue> for PayloadType {
    fn from_cql(cql_val: CqlValue) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
        match cql_val {
            CqlValue::Text(txt) => match txt.as_str() {
                "User" => Ok(PayloadType::User),
                "Admin" => Ok(PayloadType::Admin),
                "Client" => Ok(PayloadType::Client),
                _ => Err(scylla::cql_to_rust::FromCqlValError::BadCqlType),
            },
            _ => Err(scylla::cql_to_rust::FromCqlValError::BadCqlType),
        }
    }
}

#[derive(FromRow, Debug, Clone)]
pub struct Payload {
    pub name: String,
    pub payload_type: PayloadType,
}
