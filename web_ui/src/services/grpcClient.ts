import { grpc } from "@improbable-eng/grpc-web";
import { TraceGuardService } from "../generated/traceguard_pb_service";
import { CreateSbomRequest, ListSbomsRequest } from "../generated/traceguard_pb";

const host = "http://localhost:8080";

const propagateTrace = () => {
  const traceId = Math.random().toString(36).substring(2, 15);
  const spanId = Math.random().toString(36).substring(2, 15);
  return {
    "x-b3-traceid": traceId,
    "x-b3-spanid": spanId,
    "x-b3-sampled": "1"
  };
};

export const createSBOM = (name: string, version: string, format: string, content: string): Promise<string> => {
  return new Promise((resolve, reject) => {
    const request = new CreateSbomRequest();
    request.setName(name);
    request.setVersion(version);
    request.setFormat(format);
    request.setContent(content);

    grpc.unary(TraceGuardService.CreateSBOM, {
      request: request,
      host: host,
      metadata: propagateTrace(),
      onEnd: res => {
        const { status, statusMessage, headers, message, trailers } = res;
        if (status === grpc.Code.OK && message) {
          resolve(message.getId());
        } else {
          reject(new Error(statusMessage));
        }
      }
    });
  });
};

export const listSBOMs = (page: number, pageSize: number): Promise<any> => {
  return new Promise((resolve, reject) => {
    const request = new ListSbomsRequest();
    request.setPage(page);
    request.setPageSize(pageSize);

    grpc.unary(TraceGuardService.ListSBOMs, {
      request: request,
      host: host,
      onEnd: res => {
        const { status, statusMessage, headers, message, trailers } = res;
        if (status === grpc.Code.OK && message) {
          resolve({
            sboms: message.getSbomsList(),
            total: message.getTotal()
          });
        } else {
          reject(new Error(statusMessage));
        }
      }
    });
  });
};

// Implement other gRPC methods (createProvenanceRecord, listProvenanceRecords, generateComplianceReport) similarly...