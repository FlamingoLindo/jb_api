use crate::middlewares::auth::Claims;
use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::{LocalBoxFuture, Ready, ok};
use serde_json::json;
use std::rc::Rc;

pub struct RoleGuard(pub &'static [&'static str]);

impl<S, B> Transform<S, ServiceRequest> for RoleGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RoleGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RoleGuardMiddleware {
            service: Rc::new(service),
            required_roles: self.0,
        })
    }
}

pub struct RoleGuardMiddleware<S> {
    service: Rc<S>,
    required_roles: &'static [&'static str],
}

impl<S, B> Service<ServiceRequest> for RoleGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let required_roles = self.required_roles;

        Box::pin(async move {
            let has_role = {
                let extensions = req.extensions();
                match extensions.get::<Claims>().and_then(|c| c.role.as_deref()) {
                    Some(role) => required_roles.contains(&role),
                    None => false,
                }
            };

            if !has_role {
                let response = HttpResponse::Unauthorized().json(json!({
                    "status": "error",
                    "message": "You do not have permission to perform this action"
                }));
                return Ok(req.into_response(response.map_into_right_body()));
            }

            service.call(req).await.map(|res| res.map_into_left_body())
        })
    }
}
