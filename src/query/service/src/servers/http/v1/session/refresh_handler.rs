// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use jwt_simple::prelude::Deserialize;
use jwt_simple::prelude::Serialize;
use poem::error::Result as PoemResult;
use poem::web::Json;
use poem::IntoResponse;

use crate::auth::Credential;
use crate::servers::http::error::HttpErrorCode;
use crate::servers::http::v1::session::client_session_manager::ClientSessionManager;
use crate::servers::http::v1::session::consts::SESSION_TOKEN_TTL;
use crate::servers::http::v1::HttpQueryContext;

#[derive(Deserialize, Clone)]
struct RefreshRequest {
    // to drop the old token earlier instead of waiting for expiration
    pub session_token: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct RefreshResponse {
    session_token: Option<String>,
    refresh_token: Option<String>,
    session_token_ttl_in_secs: u64,
}

#[poem::handler]
#[async_backtrace::framed]
pub async fn refresh_handler(
    ctx: &HttpQueryContext,
    Json(req): Json<RefreshRequest>,
) -> PoemResult<impl IntoResponse> {
    let mgr = ClientSessionManager::instance();
    match &ctx.credential {
        Credential::DatabendToken { token, .. } => {
            let (_, token_pair) = mgr
                .new_token_pair(&ctx.session, Some(token.clone()), req.session_token)
                .await
                .map_err(HttpErrorCode::server_error)?;
            Ok(Json(RefreshResponse {
                session_token_ttl_in_secs: SESSION_TOKEN_TTL.as_secs(),
                session_token: Some(token_pair.session.clone()),
                refresh_token: Some(token_pair.refresh.clone()),
            }))
        }
        _ => {
            unreachable!("/v1/session/refresh should be authed by databend refresh token")
        }
    }
}
