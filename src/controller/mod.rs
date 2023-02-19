use actix_web::HttpRequest;

pub mod account;
pub mod bot;
pub mod graphql;

pub fn extract_token(request: &HttpRequest) -> Option<String> {
    let cookie_token = request.cookie("session");
    if cookie_token.is_none() {
        let header_token = request.headers().get("session");
        if header_token.is_none() {
            return None;
        }
        return Some(header_token.unwrap().to_str().unwrap().to_string());
    }
    let token = cookie_token.unwrap();
    Some(token.value().to_string())
}
