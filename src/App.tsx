import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import './App.css';

function App() {
	const [greetMsg, setGreetMsg] = useState('');

	async function initWgpu() {
		try {
			setGreetMsg('Initializing WebGPU...');
			const result = await invoke('init_wgpu');
			setGreetMsg(result as string);
		} catch (e) {
			setGreetMsg(`Error: ${e}`);
		}
	}

	return (
		<div className="container">
			<h1 className="text-3xl font-bold underline">Pixel Forge</h1>

			<div className="row">
				<button type="button" onClick={initWgpu}>
					Init WebGPU
				</button>
			</div>

			<p className="mt-4 break-all">{greetMsg}</p>
		</div>
	);
}

export default App;
