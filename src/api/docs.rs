use utoipa::OpenApi;

use crate::database::entities::{
    consumers::{ApiConsumer, NewConsumer}, consumers_routes::ConsumerRoute, routes::Route, targets::Target, upstreams::{NewUpstream, Upstream}
};

use super::dtos::{
    pagination::{ConsumersPagination, RoutesPagination, TargetsPagination, UpstreamsPagination},
    routes::RouteFormDto,
    targets::TargetFormDto,
};

#[derive(OpenApi)]
#[openapi(
    info (
        title = "Oxid Gateway",
        description = "API Gateway build using rust",
    ),
    paths (
        crate::api::handlers::targets::create_target,
        crate::api::handlers::targets::delete_target,
        crate::api::handlers::targets::find_target_by_id,
        crate::api::handlers::targets::update_target,
        crate::api::handlers::targets::find_targets,
        crate::api::handlers::upstreams::find_upstreams,
        crate::api::handlers::upstreams::create_upstream,
        crate::api::handlers::upstreams::delete_upstream,
        crate::api::handlers::upstreams::find_upstream_by_id,
        crate::api::handlers::upstreams::find_upstreams,
        crate::api::handlers::upstreams::update_upstream,
        crate::api::handlers::routes::find_routes,
        crate::api::handlers::routes::find_routes_in_upstream,
        crate::api::handlers::routes::create_route,
        crate::api::handlers::routes::delete_route,
        crate::api::handlers::routes::find_route_by_id,
        crate::api::handlers::routes::update_route,
        crate::api::handlers::routes::link_consumer_to_route,
        crate::api::handlers::routes::find_consumer_routes,
        crate::api::handlers::consumers::find_consumers,
        crate::api::handlers::consumers::create_consumer,
        crate::api::handlers::consumers::delete_consumer,
        crate::api::handlers::consumers::find_consumer_by_id,
        crate::api::handlers::consumers::update_consumer,
    ),
    components (
        schemas (
            NewUpstream,
            NewConsumer,
            ApiConsumer,
            Target,
            Upstream,
            Route,
            TargetFormDto,
            RouteFormDto,
            UpstreamsPagination,
            TargetsPagination,
            ConsumersPagination,
            RoutesPagination,
            ConsumerRoute
        )
    ),
    tags (
        (name = "Targets", description = "Upstream targets"),
        (name = "Upstreams", description = "Upstreams"),
        (name = "Routes", description = "Routes"),
        (name = "Consumers", description = "Consumers of routes")
    ),
)]
pub struct ApiDoc;
