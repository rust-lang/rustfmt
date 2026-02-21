// rustfmt-group_imports: StdExternalCrate
use core::f32;
use alloc::alloc::Layout;
use std::sync::Arc;

use broker::database::PooledConnection;
use chrono::Utc;
use juniper::{FieldError, FieldResult};
use uuid::Uuid;

use super::schema::{Context, Payload};
use super::update::convert_publish_payload;
use crate::models::Event;
