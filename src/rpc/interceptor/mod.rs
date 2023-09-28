use tonic::metadata::MetadataValue;
use tonic::{service::Interceptor, Request, Status};

use super::ycchat_auth::SignInResponse;

pub struct AuthInterceptor {
    sign_in_res: SignInResponse,
}

impl AuthInterceptor {
    pub fn new(sign_in_res: SignInResponse) -> Self {
        Self { sign_in_res }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let jwt_token = self.sign_in_res.access_token.clone();

        let token: MetadataValue<_> = match format!("Bearer {}", jwt_token).parse() {
            Ok(token) => token,
            Err(_) => return Err(Status::invalid_argument("jwt token error")),
        };

        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    }
}
