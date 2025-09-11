// rustfmt-group_imports: StdExternalCrate
// rustfmt-reorder_imports: false

use alloc::alloc::Layout;
use std::sync::Arc;
use core::f32;

use chrono::Utc;
use juniper::{FieldError, FieldResult};
use uuid::Uuid;
use broker::database::PooledConnection;

use super::{
    schema::{Context, Payload},
    update::convert_publish_payload,
};
use crate::models::Event;
