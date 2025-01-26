use actix_web::web;

mod user_route;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(web::scope("/user").configure(user_route::user_route_config)),
    );
}
