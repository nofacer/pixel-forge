import { Handle, Position } from '@xyflow/react';
import { useEffect, useRef } from 'react';

function OutputNode({
	data,
}: {
	data: { label: string; imageData?: Uint8Array };
}) {
	const canvasRef = useRef<HTMLCanvasElement>(null);

	useEffect(() => {
		if (data.imageData && canvasRef.current) {
			const canvas = canvasRef.current;
			const ctx = canvas.getContext('2d');
			if (ctx) {
				const imageData = new ImageData(
					new Uint8ClampedArray(data.imageData),
					256,
					256,
				);
				ctx.putImageData(imageData, 0, 0);
			}
		}
	}, [data.imageData]);

	return (
		<div className="px-4 py-2 shadow-md rounded-md bg-slate-900 border-2 border-stone-400 text-white">
			<div className="flex flex-col items-center">
				<div className="font-bold text-sm mb-2">Output Canvas</div>

				<div className="bg-black border border-gray-600 flex items-center justify-center overflow-hidden">
					<canvas
						ref={canvasRef}
						width={256}
						height={256}
						className="w-32 h-32 object-contain rendering-pixelated"
					/>
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
