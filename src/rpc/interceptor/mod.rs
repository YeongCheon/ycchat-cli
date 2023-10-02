use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::{HeaderValue, Request};
use hyper::Body;
use tokio::sync::Mutex;
use tonic::body::BoxBody;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{service::Interceptor, Status};
use tower::Service;

use super::ycchat_auth::SignInResponse;

#[deprecated]
pub struct AuthInterceptor {
    sign_in_res: SignInResponse,
}

impl AuthInterceptor {
    pub fn new(sign_in_res: SignInResponse) -> Self {
        Self { sign_in_res }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let jwt_token = self.sign_in_res.access_token.clone();

        let token: MetadataValue<_> = match format!("Bearer {}", jwt_token).parse() {
            Ok(token) => token,
            Err(_) => return Err(Status::invalid_argument("jwt token error")),
        };

        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    }
}

pub struct AuthMiddleware {
    inner: Channel,
    auth_state: Arc<Mutex<SignInResponse>>,
}

impl AuthMiddleware {
    pub fn new(inner: Channel, auth_state: Arc<Mutex<SignInResponse>>) -> Self {
        Self { inner, auth_state }
    }
}

impl Service<hyper::Request<BoxBody>> for AuthMiddleware {
    type Response = http::Response<Body>;

    type Error = Box<dyn std::error::Error + Send + Sync>;

    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: hyper::Request<BoxBody>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        let auth_state = Arc::clone(&self.auth_state);

        Box::pin(async move {
            let jwt_token = auth_state.lock().await.access_token.clone();

            let jwt_token =
                HeaderValue::from_bytes(format!("Bearer {}", jwt_token).as_bytes()).unwrap();

            req.headers_mut().append("authorization", jwt_token);

            let response = inner.call(req).await?;

            Ok(response)
        })
    }
}

pub struct AuthSvc {
    inner: Channel,
}

impl AuthSvc {
    pub fn new(inner: Channel) -> Self {
        AuthSvc { inner }
    }
}

impl Service<Request<BoxBody>> for AuthSvc {
    type Response = http::Response<Body>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            // Do extra async work here...
            let response = inner.call(req).await?;

            Ok(response)
        })
    }
}
