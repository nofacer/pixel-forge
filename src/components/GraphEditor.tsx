import { invoke } from '@tauri-apps/api/core';
import {
	addEdge,
	Background,
	type Connection,
	Controls,
	type Edge,
	type Node,
	Panel,
	ReactFlow,
	useEdgesState,
	useNodesState,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { useCallback, useEffect } from 'react';

import ColorNode from './nodes/ColorNode';
import OutputNode from './nodes/OutputNode';

const nodeTypes = {
	colorNode: ColorNode,
	outputNode: OutputNode,
};

const initialNodes: Node[] = [
	{
		id: '1',
		type: 'colorNode',
		position: { x: 100, y: 100 },
		data: { label: 'Solid Color' },
	},
	{
		id: '2',
		type: 'outputNode',
		position: { x: 400, y: 100 },
		data: { label: 'Output' },
	},
];

const initialEdges: Edge[] = [];

function GraphEditor() {
	const [nodes, , onNodesChange] = useNodesState(initialNodes);
	const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

	const onConnect = useCallback(
		(params: Connection) => setEdges((eds) => addEdge(params, eds)),
		[setEdges],
	);

	// Initialize WebGPU on mount
	useEffect(() => {
		invoke('init_wgpu')
			.then(() => console.log('WebGPU Initialized'))
			.catch(console.error);
	}, []);

	const handleSync = async () => {
		const graph = { nodes, edges };
		try {
			await invoke('sync_graph', { graphJson: JSON.stringify(graph) });
			console.log('Graph synced!');
		} catch (e) {
			console.error('Failed to sync graph:', e);
		}
	};

	return (
		<div style={{ width: '100vw', height: '100vh', background: '#1a1a1a' }}>
			<ReactFlow
				nodes={nodes}
				edges={edges}
				onNodesChange={onNodesChange}
				onEdgesChange={onEdgesChange}
				onConnect={onConnect}
				nodeTypes={nodeTypes}
				colorMode="dark"
				fitView
			>
				<Background />
				<Controls />
				<Panel position="top-right">
					<button
						type="button"
						onClick={handleSync}
						className="bg-teal-600 hover:bg-teal-700 text-white font-bold py-2 px-4 rounded"
					>
						Sync Graph
					</button>
				</Panel>
			</ReactFlow>
		</div>
	);
}

export default GraphEditor;
