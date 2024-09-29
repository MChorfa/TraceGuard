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
      const stream = client.streamUpdates({ userId: 'user123' });

      try {
        for await (const update of stream) {
          setUpdates((prevUpdates) => [...prevUpdates, update.message]);
        }
      } catch (err) {
        console.error('Error:', err);
      } finally {
        console.log('Stream ended');
      }
    };

    const fetchSBOMs = async () => {
      try {
        const response = await client.listSBOMs({});
        setSboms(response.sboms);
      } catch (err) {
        console.error('Error fetching SBOMs:', err);
      }
    };

    fetchUpdates();
    fetchSBOMs();
  }, []);

  const handleGetProvenance = async (artifactId: string) => {
    try {
      const response = await client.getProvenance({ artifactId });
      setProvenance(response);
    } catch (err) {
      console.error('Error fetching provenance:', err);
    }
  };

  return (
    <div>
      <h2>Updates</h2>
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
          <h2>Provenance</h2>
          <p>Artifact ID: {provenance.artifactId}</p>
          <p>SLSA Level: {provenance.slsaLevel}</p>
          <p>Metadata: {provenance.metadata}</p>
        </div>
      )}
    </div>
  );
}

export default UpdateStream;