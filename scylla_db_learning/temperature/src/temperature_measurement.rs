use scylla::{FromRow, ValueList};
use uuid::Uuid;

use std::time::Duration;

struct TemperatureMeasurement {
    pub device: Uuid,
    pub time: Duration,
    pub measurememt: i16,
}
