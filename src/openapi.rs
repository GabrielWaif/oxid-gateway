use utoipa::OpenApi;

use crate::{
    domain::models::result_body_container::{ResultBodyContainerTarget, ResultBodyContainerUpstream}, infra::repositories::{targets_repository::{NewTarget, Target}, upstream_repository::{NewUpstream, Upstream}}
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
    ),
    components (
        schemas (
            NewTarget,
            NewUpstream,
            ResultBodyContainerTarget,
            ResultBodyContainerUpstream,
            Target,
            Upstream
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
