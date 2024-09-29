use tonic::{Request, Response, Status};
use crate::proto::traceguard::v1::trace_guard_service_server::{TraceGuardService, TraceGuardServiceServer};
use crate::proto::traceguard::v1::{StreamUpdatesRequest, Update, GetProvenanceRequest, ProvenanceRecord, ListSBOMsRequest, ListSBOMsResponse, SBOM};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use tracing::{info, error};

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
        info!("Streaming updates for user: {}", user_id);

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            for i in 0..10 {
                let update = Update {
                    message: format!("Update {} for user {}", i, user_id),
                };
                if tx.send(Ok(update)).await.is_err() {
                    error!("Failed to send update for user: {}", user_id);
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
        let artifact_id = request.into_inner().artifact_id;
        info!("Getting provenance for artifact: {}", artifact_id);

        // TODO: Implement actual provenance retrieval logic
        let provenance = ProvenanceRecord {
            artifact_id: artifact_id.clone(),
            slsa_level: 2,
            metadata: "Sample metadata".to_string(),
        };

        Ok(Response::new(provenance))
    }

    async fn list_sboms(
        &self,
        request: Request<ListSBOMsRequest>,
    ) -> Result<Response<ListSBOMsResponse>, Status> {
        let _filter = request.into_inner().filter;
        info!("Listing SBOMs");

        // TODO: Implement actual SBOM listing logic
        let sboms = vec![
            SBOM {
                id: "sbom1".to_string(),
                name: "SBOM 1".to_string(),
                version: "1.0.0".to_string(),
                content: "Sample content".to_string(),
            },
            SBOM {
                id: "sbom2".to_string(),
                name: "SBOM 2".to_string(),
                version: "2.0.0".to_string(),
                content: "Sample content".to_string(),
            },
        ];

        Ok(Response::new(ListSBOMsResponse {
            sboms,
            next_page_token: "".to_string(),
        }))
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