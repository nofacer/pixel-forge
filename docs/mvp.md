# MVP Development Plan

## Goal
Create a minimal working prototype of **Pixel Forge** that demonstrates the core loop: **Node Graph -> Rust Backend -> WebGPU Render -> UI Preview**.

## Phase 1: The Skeleton (Infrastructure)
*Objective: Get Rust and React talking, and initialize wgpu.*
- [x] Initialize Tauri project with React + TypeScript.
- [x] Set up basic Rust backend structure (`GraphState` struct).
- [x] Initialize `wgpu` instance in Rust.
- [x] Create a simple "Hello World" Tauri command (e.g., return a string from Rust to React).

## Phase 2: The Graph (Frontend)
*Objective: Visual node editing.*
- [x] Install and configure `React Flow`.
- [x] Create two custom node types on frontend:
    -   **Generator**: "Solid Color" (Inputs: Color (RGBA)).
    -   **Output**: "Canvas" (Displays the final result).
- [x] Implement state management to track nodes and edges.
- [x] Implement `sync_graph` command to send JSON graph data to Rust.

## Phase 3: The Engine (Backend & WebGPU)
*Objective: Process the graph and render a solid color.*
- [x] Implement Graph deserialization in Rust.
- [x] Create the **Render Context**: A struct holding wgpu device, queue, and texture manager.
- [x] Implement `SolidColorNode` logic in Rust:
    -   Create a 1x1 (or variable size) texture.
    -   Write pixel data to it via wgpu queue (or shader).
- [x] Implement data extraction: Read texture back to host memory ( `Vec<u8>`).

## Phase 4: The Loop (Preview)
*Objective: See the result in the app.*
- [x] Create a mechanism to send rendered bytes from Rust to Frontend.
    -   Raw buffer transfer + Canvas `putImageData`.
- [x] Update the "Canvas" node in React to display this image data.

## Phase 5: Composition (The "Cool" Part)
*Objective: Prove we can combine things.*
- [x] Create a **Blend Node** (Inputs: Top, Bottom).
- [x] Write a simple WebGPU render pass that samples two textures and mixes them.
- [x] Connect: `Solid Red` -> `Blend (Top)` + `Solid Blue` -> `Blend (Bottom)` -> `Output`.
- [x] Verify the output is purple.

## Future / Post-MVP
-   Auto-tiling / Pattern generation nodes.
-   Sprite sheet export.
-   Lua scripting for custom nodes.
-   Node groups/prefabs.
