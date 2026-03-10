use std::sync::Arc;

use tonic::{transport::Server, Request, Response, Status};

use crate::services::core::CoreService;

use core_proto::core_service_server::{CoreService as CoreGrpc, CoreServiceServer};
use core_proto::{LoginRequest, LoginResponse, CreateUserRequest, CreateUserResponse};

pub mod core_proto {
    tonic::include_proto!("core");
}

#[derive(Clone)]
pub struct CoreGrpcServer {
    core_service: Arc<CoreService>,
}

#[tonic::async_trait]
impl CoreGrpc for CoreGrpcServer {

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
    
            let req = request.into_inner();
    
            let result = self.core_service
                .create_user(&req.username, &req.password, &req.token)
                .await
                .map_err(|e: String| Status::internal(e))?;
    
            Ok(Response::new(CreateUserResponse {
                success: result,
            }))
        }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {

        let req = request.into_inner();

        let token = self.core_service
            .login_user(&req.username, &req.password)
            .await
            .map_err(|e: String| Status::internal(e))?;

        Ok(Response::new(LoginResponse {
            token,
            refresh_token: "".into(),
        }))
    }
}

pub async fn start_grpc_server(
    core_service: Arc<CoreService>,
) -> Result<(), Box<dyn std::error::Error>> {

    let addr = "0.0.0.0:50050".parse()?;

    let grpc_server = CoreGrpcServer { core_service };

    println!("Core gRPC server listening on {}", addr);

    Server::builder()
        .add_service(CoreServiceServer::new(grpc_server))
        .serve(addr)
        .await?;

    Ok(())
}