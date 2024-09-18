use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures::future::{ok, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Instant;

pub struct ResponseTime;

impl<S, B> Transform<S, ServiceRequest> for ResponseTime
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ResponseTimeMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ResponseTimeMiddleware {
            service: Rc::new(service),
        })
    }
}

// Add 'static to ensure that S can live long enough
pub struct ResponseTimeMiddleware<S>
where
    S: 'static, // Adding the 'static lifetime bound here
{
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ResponseTimeMiddleware<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let svc = Rc::clone(&self.service);
        Box::pin(async move {
            let mut res = svc.call(req).await?;
            let elapsed_time = start_time.elapsed();
            let res_time = format!("{} ms", elapsed_time.as_millis());

            // Inserting the dynamic response time into headers
            res.headers_mut().insert(
                HeaderName::from_static("x-response-time"),
                HeaderValue::try_from(res_time.as_str()).unwrap(),
            );
            Ok(res)
        })
    }
}
