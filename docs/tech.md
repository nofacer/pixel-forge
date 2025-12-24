# Technical Architecture

## Overview
**Pixel Forge** is a node-based pixel art compositing application. It allows users to procedurally generate and composite pixel art using a visual node graph.

The system is built for high performance using **Rust** and **WebGPU** for the rendering backend, with **Tauri** and **React** providing a responsive, cross-platform UI.

## Technology Stack

| Layer | Technology | Purpose |
| :--- | :--- | :--- |
| **Frontend** | React | UI Components & Application State |
| | React Flow | Node Graph Visualization & Interaction |
| | TypeScript | Type Safety for Frontend Logic |
| **Bridge** | Tauri | IPC between Frontend (WebView) and Backend (Rust) |
| **Backend** | Rust | Application Logic, State Management, High-performance calculations |
| **Graphics** | WebGPU (wgpu) | Hardware accelerated rendering & compute shaders |

## Architecture Components

### 1. Frontend (The View)
*   **Node Graph**: Built with `React Flow`. Handles user interaction (adding nodes, connecting wires, values).
*   **State Management**: React state holds the *visual* representation of the graph.
*   **Preview Window**: A canvas element that displays the rendered output. It receives texture data (or shares a GPU surface) from the Rust backend.
*   **Property Inspector**: UI for editing selected node parameters.

### 2. The Bridge (Tauri Command Interface)
Tauri commands serve as the API between the UI and the Engine.
*   `update_graph(graph_data)`: Sends the current node graph structure to Rust.
*   `render_node(node_id)`: Requests a render update for a specific node (used for previews).
*   `get_preview_image(node_id)`: Retrieves the rendered buffer (e.g., as base64 or shared buffer) for display.

### 3. Backend (The Engine)
*   **Graph Processor**: A Rust struct that mirrors the React Flow graph but strictly typed for execution. It handles:
    *   Topological sorting of nodes.
    *   Dependency tracking (which nodes need re-rendering).
*   **Node System**: Trait-based system where each Node Type (e.g., `SolidColor`, `Blend`, `Noise`) implements a `process` function.

### 4. Rendering Pipeline (wgpu)
*   **Texture Management**: Each node output is essentially a GPU Texture.
*   **Compute/Fragment Shaders**: Actual pixel manipulation happens here.
    *   *Generators*: Compute shaders that create patterns (Noise, Shapes).
    *   *Filters*: Fragment shaders taking input textures and producing an output texture (Blur, Color Correct).
    *   *Compositors*: Shaders that blend multiple textures (Over, Multiply, Add).

## Data Flow
1.  **User Action**: User connects "Noise Node" to "Colorize Node".
2.  **Update**: Frontend sends the new connection data to Rust via Tauri.
3.  **Graph Analysis**: Backend marks "Colorize Node" and downstream nodes as "Dirty".
4.  **Execution**:
    *   Rust commands wgpu to run the "Noise" shader (if dirty).
    *   Rust commands wgpu to run the "Colorize" shader, using the Noise texture as input.
5.  **Display**:
    *   The final texture is read from GPU memory.
    *   Data is sent back to Frontend (or blitted to a window surface) to be displayed in the Preview pane.
