// rustfmt-group_imports: ExternalCrate
// rustfmt-imports_granularity: Crate
use alloc::{alloc::Layout, vec::Vec};
use broker::database::PooledConnection;
use chrono::Utc;
use core::f32;
use juniper::{FieldError, FieldResult};
use std::sync::Arc;
use uuid::Uuid;

use super::{
    schema::{Context, Payload},
    update::convert_publish_payload,
};
use crate::models::Event;
