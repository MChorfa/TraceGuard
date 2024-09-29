import React, { useEffect, useState } from 'react';
import { createPromiseClient } from "@bufbuild/connect";
import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { TraceGuardService } from "../gen/traceguard/v1/traceguard_connect";

const transport = createGrpcWebTransport({
  baseUrl: "http://localhost:8080",
});

const client = createPromiseClient(TraceGuardService, transport);

const UpdateStream: React.FC = () => {
  const [updates, setUpdates] = useState<string[]>([]);

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

    fetchUpdates();
  }, []);

  return (
    <div>
      <h2>Updates</h2>
      <ul>
        {updates.map((update, index) => (
          <li key={index}>{update}</li>
        ))}
      </ul>
    </div>
  );
};

export default UpdateStream;