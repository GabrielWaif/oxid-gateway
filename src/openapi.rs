use utoipa::OpenApi;

use crate::{
    domain::models::{
        result_body_container::{ResultBodyContainerTarget, ResultBodyContainerUpstream},
        route_form_dto::RouteFormDto,
        target_form_dto::TargetFormDto,
    },
    infra::repositories::{
        consumers_repository::{Consumer, NewConsumer}, routes_repository::Route, targets_repository::{NewTarget, Target}, upstream_repository::{NewUpstream, Upstream}
    },
};

#[derive(OpenApi)]
#[openapi(
    info (
        title = "Oxid Gateway",
        description = "API Gateway build using rust",
    ),
    paths (
        crate::handlers::targets::create_target::create_target,
        crate::handlers::targets::delete_target::delete_target,
        crate::handlers::targets::find_target_by_id::find_target_by_id,
        crate::handlers::targets::update_target::update_target,
        crate::handlers::upstreams::create_upstream::create_upstream,
        crate::handlers::upstreams::delete_upstream::delete_upstream,
        crate::handlers::upstreams::find_upstream_by_id::find_upstream_by_id,
        crate::handlers::upstreams::update_upstream::update_upstream,
        crate::handlers::routes::create_route::create_route,
        crate::handlers::routes::delete_route::delete_route,
        crate::handlers::routes::find_route_by_id::find_route_by_id,
        crate::handlers::routes::update_route::update_route,
        crate::handlers::consumers::create_consumer::create_consumer,
        crate::handlers::consumers::delete_consumer::delete_consumer,
        crate::handlers::consumers::find_consumer_by_id::find_consumer_by_id,
        crate::handlers::consumers::update_consumer::update_consumer,
    ),
    components (
        schemas (
            NewUpstream,
            NewConsumer,
            Consumer,
            ResultBodyContainerTarget,
            ResultBodyContainerUpstream,
            Target,
            Upstream,
            Route,
            TargetFormDto,
            RouteFormDto
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
