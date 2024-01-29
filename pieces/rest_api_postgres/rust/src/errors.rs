use thiserror::Error;

#[derive(Debug, Error)]
#[error("Rating must be between 0 and 5")]
pub struct InvalidRatingError();
