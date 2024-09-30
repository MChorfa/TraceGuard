use tonic::{Request, Response, Status};
use tracing::{info, error, instrument};
use crate::api::{sbom, provenance, compliance};
use crate::database::Database;
use crate::storage::blob_storage::BlobStorage;

pub mod proto {
    tonic::include_proto!("traceguard.v1");
}

use proto::trace_guard_service_server::TraceGuardService;
use proto::{
    CreateSbomRequest, CreateSbomResponse,
    ListSbomsRequest, ListSbomsResponse,
    CreateProvenanceRecordRequest, CreateProvenanceRecordResponse,
    ListProvenanceRecordsRequest, ListProvenanceRecordsResponse,
    GenerateComplianceReportRequest, GenerateComplianceReportResponse,
};

pub struct TraceGuardGrpcService<S: BlobStorage> {
    db: Database,
    storage: S,
}

#[tonic::async_trait]
impl<S: BlobStorage + Send + Sync + 'static> TraceGuardService for TraceGuardGrpcService<S> {
    #[instrument(skip(self, request))]
    async fn create_sbom(
        &self,
        request: Request<CreateSbomRequest>,
    ) -> Result<Response<CreateSbomResponse>, Status> {
        let req = request.into_inner();
        info!("Received CreateSBOM request: {:?}", req);
        let result = sbom::create_sbom(self.db.clone(), self.storage.clone(), req.into()).await
            .map_err(|e| {
                error!("Error creating SBOM: {:?}", e);
                Status::internal(e.to_string())
            })?;
        Ok(Response::new(result.into()))
    }

    #[instrument(skip(self, request))]
    async fn list_sboms(
        &self,
        request: Request<ListSbomsRequest>,
    ) -> Result<Response<ListSbomsResponse>, Status> {
        let req = request.into_inner();
        info!("Received ListSBOMs request: {:?}", req);
        let result = sbom::list_sboms(self.db.clone(), req.page, req.page_size).await
            .map_err(|e| {
                error!("Error listing SBOMs: {:?}", e);
                Status::internal(e.to_string())
            })?;
        Ok(Response::new(result.into()))
    }

    // Implement other methods (create_provenance_record, list_provenance_records, generate_compliance_report) with similar tracing...
}

pub fn create_grpc_service<S: BlobStorage + Clone + Send + Sync + 'static>(
    db: Database,
    storage: S,
) -> TraceGuardGrpcService<S> {
    TraceGuardGrpcService { db, storage }
}