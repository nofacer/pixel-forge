import { Handle, Position } from '@xyflow/react';
import { useState } from 'react';

function ColorNode({ data: _data }: { data: { label: string } }) {
	const [color, setColor] = useState({ r: 255, g: 0, b: 0, a: 1 });

	const handleChange = (channel: keyof typeof color, value: number) => {
		setColor((prev) => ({ ...prev, [channel]: value }));
		// TODO: Propagate change to parent/graph state
	};

	return (
		<div className="px-4 py-2 shadow-md rounded-md bg-white border-2 border-stone-400">
			<div className="flex flex-col">
				<div className="font-bold text-sm mb-2">Solid Color</div>

				<div className="flex flex-col gap-1 w-full">
					<label className="text-xs flex justify-between">
						<span>R</span>
						<input
							type="range"
							min="0"
							max="255"
							value={color.r}
							onChange={(e) => handleChange('r', Number(e.target.value))}
							className="w-20 nodrag"
						/>
					</label>
					<label className="text-xs flex justify-between">
						<span>G</span>
						<input
							type="range"
							min="0"
							max="255"
							value={color.g}
							onChange={(e) => handleChange('g', Number(e.target.value))}
							className="w-20 nodrag"
						/>
					</label>
					<label className="text-xs flex justify-between">
						<span>B</span>
						<input
							type="range"
							min="0"
							max="255"
							value={color.b}
							onChange={(e) => handleChange('b', Number(e.target.value))}
							className="w-20 nodrag"
						/>
					</label>
				</div>

				<div
					className="mt-2 w-full h-8 rounded border border-gray-300"
					style={{
						backgroundColor: `rgba(${color.r}, ${color.g}, ${color.b}, ${color.a})`,
					}}
				/>
			</div>

			<Handle
				type="source"
				position={Position.Right}
				className="w-16 !bg-teal-500"
			/>
		</div>
	);
}

export default ColorNode;
