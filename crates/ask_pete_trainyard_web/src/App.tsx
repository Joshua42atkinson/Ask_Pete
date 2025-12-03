import { useCallback } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  type Connection,
  type Edge,
  type Node,
} from 'reactflow';
import 'reactflow/dist/style.css';

import DialogueNode from './components/DialogueNode';
import type { NarrativeGraph, NarrativeNode } from './types/NarrativeGraph';

const nodeTypes = {
  dialogue: DialogueNode,
};

const initialNodes: Node[] = [
  {
    id: '1',
    type: 'dialogue',
    position: { x: 250, y: 5 },
    data: {
      speaker: 'Pete',
      text: 'Hello there! Welcome to the Train Yard.',
      choices: [{ text: 'Hi Pete!', next_node_id: '2', conditions: [] }]
    },
  },
  {
    id: '2',
    type: 'dialogue',
    position: { x: 250, y: 200 },
    data: {
      speaker: 'Player',
      text: 'Thanks! How does this work?',
      choices: []
    },
  },
];

const initialEdges: Edge[] = [
  { id: 'e1-2', source: '1', target: '2', sourceHandle: 'choice-0' }
];

export default function App() {
  const [nodes, , onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

  const exportJson = () => {
    const narrativeNodes: Record<string, NarrativeNode> = {};

    nodes.forEach((node) => {
      // Map edges to choices
      const nodeEdges = edges.filter((edge) => edge.source === node.id);

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const choices = (node.data.choices || []).map((choice: any, index: number) => {
        // Find edge connected to this choice handle
        const edge = nodeEdges.find(e => e.sourceHandle === `choice-${index}`);
        return {
          ...choice,
          next_node_id: edge ? edge.target : undefined
        };
      });

      narrativeNodes[node.id] = {
        id: node.id,
        speaker: node.data.speaker,
        text: node.data.text,
        choices: choices,
        events: node.data.events || [],
        position: node.position,
      };
    });

    const graph: NarrativeGraph = {
      nodes: narrativeNodes,
      start_node_id: '1', // TODO: Make configurable
      metadata: {
        "exported_at": new Date().toISOString()
      }
    };

    const json = JSON.stringify(graph, null, 2);
    console.log(json);

    // Download file
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'narrative_graph.json';
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="app-container">
      <button onClick={exportJson} className="export-button">
        Export JSON
      </button>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        nodeTypes={nodeTypes}
        fitView
      >
        <Controls />
        <MiniMap />
        <Background gap={12} size={1} />
      </ReactFlow>
    </div>
  );
}
