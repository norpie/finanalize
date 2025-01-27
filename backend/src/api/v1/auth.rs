use tide::{Request, Result};

use crate::prelude::ApiResult;


pub async fn refresh(mut req: Request<()>) -> Result {
    // TODO: Implement refresh
    ApiResult::Ok("Hello, world!".to_string()).into()
}

pub async fn login(mut req: Request<()>) -> Result {
    // TODO: Implement login
    ApiResult::Ok("Hello, world!".to_string()).into()
}

pub async fn register(mut req: Request<()>) -> Result {
    // TODO: Implement register
    ApiResult::Ok("Hello, world!".to_string()).into()
}

pub async fn logout(mut req: Request<()>) -> Result {
    // TODO: Implement logout
    ApiResult::Ok("Hello, world!".to_string()).into()
}

pub async fn me(mut req: Request<()>) -> Result {
    // TODO: Implement me
    ApiResult::Ok("Hello, world!".to_string()).into()
}
