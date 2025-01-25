use crate::controller::user_controller::{create_user, get_users};
use actix_web::web;
pub fn user_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(create_user);
}
