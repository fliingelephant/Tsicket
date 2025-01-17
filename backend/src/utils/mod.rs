pub mod auth;

/*
// create auth message
#[derive(Debug)]
pub struct GenerateAuth {
    pub token: String,
}

pub fn authenticate(state: &Data<AppState>, req: &HttpRequest) -> impl Future<Item = Auth, Error = Error> {
    let db = state.db.clone();

    result(preprocess_authz_token(req.headers().get(AUTHORIZATION)))
        .and_then(move |token| db.send(GenerateAuth { token }).from_err())
        .flatten()
}

fn preprocess_authz_token(token: Option<&HeaderValue>) -> Result<String> {
    let token = match token {
        Some(token) => token.to_str().unwrap(),
        None => {
            return Err(Error::Unauthorized(json!({
                "error": "No authorization was provided",
            })))
        }
    };

    if !token.starts_with(TOKEN_PREFIX) {
        return Err(Error::Unauthorized(json!({
            "error": "Invalid authorization method",
        })));
    }

    let token = token.replacen(TOKEN_PREFIX, "", 1);

    Ok(token)
}*/