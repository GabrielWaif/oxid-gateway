use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::database::entities::{
    consumers::Consumer, routes::Route, targets::Target, upstreams::Upstream,
};

#[derive(ToSchema, Deserialize, Serialize)]
#[aliases(
    UpstreamsPagination = PaginationResponseDto<Upstream>,
    RoutesPagination = PaginationResponseDto<Route>,
    ConsumersPagination = PaginationResponseDto<Consumer>,
    TargetsPagination = PaginationResponseDto<Target>
)]
pub struct PaginationResponseDto<T> {
    pub items: Vec<T>,
    pub count: i32,
}

#[derive(ToSchema, Deserialize, IntoParams)]
pub struct PaginationQueryDto {
    pub offset: i64,
    pub limit: i64,
}