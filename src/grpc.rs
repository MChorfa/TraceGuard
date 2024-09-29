use tonic::{Request, Response, Status};
use crate::proto::traceguard::v1::trace_guard_service_server::{TraceGuardService, TraceGuardServiceServer};
use crate::proto::traceguard::v1::{StreamUpdatesRequest, Update, GetProvenanceRequest, ProvenanceRecord, ListSBOMsRequest, ListSBOMsResponse};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

pub mod proto {
    pub mod traceguard {
        pub mod v1 {
            tonic::include_proto!("traceguard.v1");
        }
    }
}

#[derive(Default)]
pub struct TraceGuardServiceImpl {}

#[tonic::async_trait]
impl TraceGuardService for TraceGuardServiceImpl {
    type StreamUpdatesStream = tokio_stream::wrappers::ReceiverStream<Result<Update, Status>>;

    async fn stream_updates(
        &self,
        request: Request<StreamUpdatesRequest>,
    ) -> Result<Response<Self::StreamUpdatesStream>, Status> {
        let user_id = request.into_inner().user_id;
        tracing::info!("Streaming updates for user: {}", user_id);

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            for i in 0..10 {
                let update = Update {
                    message: format!("Update {} for user {}", i, user_id),
                };
                if tx.send(Ok(update)).await.is_err() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    async fn get_provenance(
        &self,
        request: Request<GetProvenanceRequest>,
    ) -> Result<Response<ProvenanceRecord>, Status> {
        // Implement get_provenance logic
        unimplemented!()
    }

    async fn list_sboms(
        &self,
        request: Request<ListSBOMsRequest>,
    ) -> Result<Response<ListSBOMsResponse>, Status> {
        // Implement list_sboms logic
        unimplemented!()
    }
}

pub fn create_grpc_service() -> tonic::transport::Server {
    let trace_guard_service = TraceGuardServiceImpl::default();
    tonic::transport::Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .layer(CorsLayer::permissive())
        .add_service(TraceGuardServiceServer::new(trace_guard_service))
}