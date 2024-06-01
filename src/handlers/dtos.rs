use crate::{
    domain::models::result_body_container::ResultBodyContainer,
    infra::repositories::targets_repository::Target,
};

pub type ResultBodyContainerTarget = ResultBodyContainer<Target>;
