import React, { useEffect, useRef } from 'react';
import { ForceGraph2D } from 'react-force-graph';
import { SBOM } from '../../types';
import { Spin } from 'antd';

interface SBOMGraphProps {
  sboms: SBOM[];
  relationships: [string, string][];
  loading: boolean;
}

const SBOMGraph: React.FC<SBOMGraphProps> = ({ sboms, relationships, loading }) => {
  const graphRef = useRef<any>();

  useEffect(() => {
    if (graphRef.current) {
      graphRef.current.d3Force('charge').strength(-300);
    }
  }, []);

  const graphData = {
    nodes: sboms.map(sbom => ({ id: sbom.id, name: sbom.name })),
    links: relationships.map(([source, target]) => ({ source, target }))
  };

  if (loading) {
    return <Spin size="large" />;
  }

  return (
    <ForceGraph2D
      ref={graphRef}
      graphData={graphData}
      nodeLabel="name"
      nodeColor={() => "#38a3a5"}
      linkColor={() => "#4a4e6d"}
      width={600}
      height={400}
    />
  );
};

export default SBOMGraph;