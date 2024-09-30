import React, { useEffect, useState } from 'react';
import { createPromiseClient } from "@bufbuild/connect";
import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { TraceGuardService } from "../gen/traceguard/v1/traceguard_connect";
import { SBOM, ProvenanceRecord } from "../gen/traceguard/v1/traceguard_pb";

const transport = createGrpcWebTransport({
  baseUrl: "http://localhost:8080",
});

const client = createPromiseClient(TraceGuardService, transport);

const UpdateStream: React.FC = () => {
  const [updates, setUpdates] = useState<string[]>([]);
  const [sboms, setSboms] = useState<SBOM[]>([]);
  const [provenance, setProvenance] = useState<ProvenanceRecord | null>(null);

  useEffect(() => {
    const fetchUpdates = async () => {
      const stream = client.streamUpdates({ userId: "current-user-id" });
      for await (const update of stream) {
        setUpdates(prev => [...prev, update.message]);
      }
    };

    const fetchSBOMs = async () => {
      const response = await client.listSBOMs({});
      setSboms(response.sboms);
    };

    fetchUpdates();
    fetchSBOMs();
  }, []);

  const handleGetProvenance = async (artifactId: string) => {
    try {
      const response = await client.getProvenance({ artifactId });
      setProvenance(response);
    } catch (error) {
      console.error("Error fetching provenance:", error);
    }
  };

  return (
    <div className="update-stream">
      <h2>Real-time Updates</h2>
      <ul>
        {updates.map((update, index) => (
          <li key={index}>{update}</li>
        ))}
      </ul>

      <h2>SBOMs</h2>
      <ul>
        {sboms.map((sbom) => (
          <li key={sbom.id}>
            {sbom.name} (v{sbom.version})
            <button onClick={() => handleGetProvenance(sbom.id)}>Get Provenance</button>
          </li>
        ))}
      </ul>

      {provenance && (
        <div>
          <h3>Provenance for {provenance.artifactId}</h3>
          <p>SLSA Level: {provenance.slsaLevel}</p>
          <pre>{JSON.stringify(JSON.parse(provenance.metadata), null, 2)}</pre>
        </div>
      )}
    </div>
  );
};

export default UpdateStream;