import { Handle, Position } from '@xyflow/react';

function OutputNode({ data: _data }: { data: { label: string } }) {
	return (
		<div className="px-4 py-2 shadow-md rounded-md bg-slate-900 border-2 border-stone-400 text-white">
			<div className="flex flex-col items-center">
				<div className="font-bold text-sm mb-2">Output Canvas</div>

				<div className="w-32 h-32 bg-black border border-gray-600 flex items-center justify-center">
					<span className="text-xs text-gray-500">Preview</span>
					{/* This will be replaced by the actual canvas later */}
				</div>
			</div>

			<Handle
				type="target"
				position={Position.Left}
				className="w-16 !bg-teal-500"
			/>
		</div>
	);
}

export default OutputNode;
