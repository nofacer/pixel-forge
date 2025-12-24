import { Handle, Position, useReactFlow } from '@xyflow/react';
import { useEffect, useState } from 'react';

function MixNode({
	id,
	data,
}: {
	id: string;
	data: {
		label: string;
		factor?: number;
	};
}) {
	const { updateNodeData } = useReactFlow();
	const [factor, setFactor] = useState(data.factor ?? 0.5);

	useEffect(() => {
		updateNodeData(id, { factor });
	}, [factor, id, updateNodeData]);

	return (
		<div className="px-4 py-2 shadow-md rounded-md bg-white border-2 border-stone-400 min-w-[150px]">
			<div className="flex flex-col gap-2">
				<div className="font-bold text-sm text-center">Mix</div>

				{/* Input Handles */}
				<div className="relative h-12">
					<div className="absolute left-0 top-0 flex flex-col gap-3 w-full">
						<div className="flex items-center relative">
							<span className="text-xs ml-2">A</span>
							<Handle
								type="target"
								position={Position.Left}
								id="a"
								className="!bg-red-400"
								style={{ top: '50%' }}
							/>
						</div>
						<div className="flex items-center relative mt-4">
							<span className="text-xs ml-2">B</span>
							<Handle
								type="target"
								position={Position.Left}
								id="b"
								className="!bg-blue-400"
								style={{ top: '50%' }}
							/>
						</div>
					</div>
				</div>

				<div className="flex flex-col gap-1 w-full mt-2">
					<label className="text-xs flex flex-col gap-1">
						<span className="flex justify-between">
							<span>Factor</span>
							<span>{factor.toFixed(2)}</span>
						</span>
						<input
							type="range"
							min="0"
							max="1"
							step="0.01"
							value={factor}
							onChange={(e) => setFactor(Number(e.target.value))}
							className="nodrag w-full"
						/>
					</label>
				</div>
			</div>

			<Handle
				type="source"
				position={Position.Right}
				className="w-16 !bg-purple-500"
			/>
		</div>
	);
}

export default MixNode;
