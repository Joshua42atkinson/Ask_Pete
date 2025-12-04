import { useCallback, useState, useMemo } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  type Edge,
  type Node,
  type OnConnect,
} from 'reactflow';
import 'reactflow/dist/style.css';
import './App.css';

import DialogueNode from './components/DialogueNode';
import PropertyEditor from './components/PropertyEditor';
import Layout from './components/Layout';
import WordSmithy, { type WordDefinition } from './components/WordSmithy';
import PersonaEngine from './components/PersonaEngine';
import OwlDiagnostic from './components/OwlDiagnostic';
import type { NodeData, StoryGraph } from './types/NarrativeGraph';



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
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
  const [selectedNode, setSelectedNode] = useState<Node | null>(null);
  const [showWordSmithy, setShowWordSmithy] = useState(false);
  const [showPersonaEngine, setShowPersonaEngine] = useState(false);
  const [showOwlDiagnostic, setShowOwlDiagnostic] = useState(false);
  const [forgedWords, setForgedWords] = useState<WordDefinition[]>([]);

  const nodeTypes = useMemo(() => ({
    dialogue: DialogueNode,
  }), []);

  const onConnect: OnConnect = useCallback(
    (params) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

  // ... (rest of handlers)

  const onNodeClick = useCallback((_event: React.MouseEvent, node: Node) => {
    setSelectedNode(node);
  }, []);

  const onPaneClick = useCallback(() => {
    setSelectedNode(null);
  }, []);

  const handleNodeUpdate = (id: string, newData: NodeData) => {
    setNodes((nds) =>
      nds.map((node) => {
        if (node.id === id) {
          return { ...node, data: newData };
        }
        return node;
      })
    );
    // Update selected node reference as well
    if (selectedNode && selectedNode.id === id) {
      setSelectedNode({ ...selectedNode, data: newData });
    }
  };

  const addNode = () => {
    const id = (nodes.length + 1).toString();
    const newNode: Node = {
      id,
      type: 'dialogue',
      position: { x: Math.random() * 400, y: Math.random() * 400 },
      data: {
        speaker: 'New Station',
        text: 'Station content...',
        choices: []
      },
    };
    setNodes((nds) => nds.concat(newNode));
  };

  const handleImportGraph = (graph: StoryGraph) => {
    // Convert backend nodes to React Flow nodes
    const newNodes: Node[] = graph.nodes.map(n => ({
      id: n.id,
      type: 'dialogue',
      position: { x: n.x || Math.random() * 400 + 250, y: n.y || Math.random() * 400 + 250 }, // Offset to avoid overlap
      data: {
        speaker: n.title,
        text: n.content,
        choices: [], // Connections handled by edges
        literary_device: undefined,
        quest: n.quest
      }
    }));

    // Convert backend connections to React Flow edges
    const newEdges: Edge[] = graph.connections.map(c => ({
      id: c.id,
      source: c.from_node,
      target: c.to_node
    }));

    // Merge with existing
    setNodes((nds) => [...nds, ...newNodes]);
    setEdges((eds) => [...eds, ...newEdges]);

    console.log(`Imported ${newNodes.length} nodes and ${newEdges.length} connections.`);
  };

  const saveGraph = async () => {
    // Map React Flow nodes to Backend StoryNodes
    const storyNodes = nodes.map((node) => ({
      id: node.id,
      title: node.data.speaker,
      content: node.data.text,
      x: node.position.x,
      y: node.position.y,
      // Default values for required backend fields
      passenger_count: 0,
      complexity_level: 1,
      learner_profiles: [],
      gardens_active: [],
      required_stats: {},
      logic: { conditions: [], effects: [] }, // Assuming empty LogicBlock
      style: { contrast: false, alignment: "left", proximity: 1.0 }
    }));

    const storyConnections = edges.map(e => ({
      id: e.id,
      from_node: e.source,
      to_node: e.target
    }));

    const graphData = {
      id: "react_demo_graph",
      title: "React Demo Story",
      nodes: storyNodes,
      connections: storyConnections,
    };

    const payload = {
      title: "React Demo Story",
      subject: "General",
      literary_device: "None",
      focus: 0.5,
      vocabulary: forgedWords.map(w => w.word),
      graph_data: graphData
    };

    try {
      const response = await fetch('/api/story_graphs', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });

      if (response.ok) {
        const data = await response.json();
        console.log("Saved Graph Response:", data);
        alert('Story saved successfully!');
      } else {
        const errorText = await response.text();
        console.error("Save failed:", errorText);
        alert(`Failed to save story: ${errorText}`);
      }
    } catch (error) {
      console.error('Error saving graph:', error);
      alert('Error saving graph.');
    }
  };

  const playStory = () => {
    window.location.href = 'http://localhost:8080/cab/journey/react_demo_graph';
  };

  const handleForgeWord = (word: WordDefinition) => {
    setForgedWords([...forgedWords, word]);
    console.log("Forged Word:", word);
    // Ideally, we'd save this to the backend or local storage
  };

  return (
    <Layout>
      <div className="toolbar">
        <button onClick={addNode} className="toolbar-btn btn-add">
          + Add Node
        </button>
        <button onClick={saveGraph} className="toolbar-btn btn-save">
          💾 Save Story
        </button>
        <button onClick={playStory} className="toolbar-btn btn-play">
          ▶ Play Story
        </button>
        <button onClick={() => setShowWordSmithy(true)} className="toolbar-btn btn-word-smithy">
          ⚒️ Word Smithy
        </button>
        <button onClick={() => setShowPersonaEngine(true)} className="toolbar-btn btn-persona-engine">
          🎭 Persona Engine
        </button>
        <button onClick={() => setShowOwlDiagnostic(true)} className="toolbar-btn btn-owl">
          🦉 O.W.L.
        </button>
      </div>

      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onNodeClick={onNodeClick}
        onPaneClick={onPaneClick}
        nodeTypes={nodeTypes}
        fitView
        className="bg-railyard-dark"
        style={{ width: '100%', height: '100%' }}
      >
        <Controls />
        <MiniMap />
        <Background gap={40} size={1} color="#3d3d5c" />
      </ReactFlow>

      <PropertyEditor
        key={selectedNode ? selectedNode.id : 'none'}
        selectedNode={selectedNode}
        onNodeChange={handleNodeUpdate}
        onImportGraph={handleImportGraph}
        onClose={() => setSelectedNode(null)}
      />

      {showWordSmithy && (
        <WordSmithy
          onClose={() => setShowWordSmithy(false)}
          onSave={handleForgeWord}
        />
      )}

      {showPersonaEngine && (
        <PersonaEngine
          onClose={() => setShowPersonaEngine(false)}
        />
      )}

      {showOwlDiagnostic && (
        <OwlDiagnostic
          nodes={nodes}
          onClose={() => setShowOwlDiagnostic(false)}
        />
      )}
    </Layout>
  );
}
