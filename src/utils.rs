use jwt_simple::prelude::HS256Key;

use crate::error;
use crate::types::UserClaim;
use jwt_simple::algorithms::MACLike;

/// Validates if the token is valid and contains desired user claim
pub async fn validate_token(token: &str, key: &HS256Key) -> Result<UserClaim, error::Error> {
    let claim = key.verify_token::<UserClaim>(token, None)?;
    Ok(claim.custom)
}
