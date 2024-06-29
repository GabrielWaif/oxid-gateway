use deadpool_diesel::InteractError;

use super::errors::adapt_infra_error;

pub mod consumers;
pub mod routes;
pub mod targets;
pub mod upstreams;

pub fn extract_interact_error<T>(
    interact_result: Result<Result<T, diesel::result::Error>, InteractError>,
) -> super::Result<T> {
    let diesel_result = match interact_result {
        Ok(diesel_result) => diesel_result,
        Err(e) => {
            tracing::warn!("{}", e);
            return Err(adapt_infra_error(e));
        }
    };

    match diesel_result {
        Ok(diesel_output) => return Ok(diesel_output),
        Err(e) => {
            tracing::warn!("{:?}", e);
            return Err(adapt_infra_error(e));
        }
    };
}
