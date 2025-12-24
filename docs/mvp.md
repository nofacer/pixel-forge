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
- [ ] Install and configure `React Flow`.
- [ ] Create two custom node types on frontend:
    -   **Generator**: "Solid Color" (Inputs: Color (RGBA)).
    -   **Output**: "Canvas" (Displays the final result).
- [ ] Implement state management to track nodes and edges.
- [ ] Implement `sync_graph` command to send JSON graph data to Rust.

## Phase 3: The Engine (Backend & WebGPU)
*Objective: Process the graph and render a solid color.*
- [ ] Implement Graph deserialization in Rust.
- [ ] Create the **Render Context**: A struct holding wgpu device, queue, and texture manager.
- [ ] Implement `SolidColorNode` logic in Rust:
    -   Create a 1x1 (or variable size) texture.
    -   Write pixel data to it via wgpu queue (or shader).
- [ ] Implement data extraction: Read texture back to host memory ( `Vec<u8>`).

## Phase 4: The Loop (Preview)
*Objective: See the result in the app.*
- [ ] Create a mechanism to send rendered bytes from Rust to Frontend.
    -   *Approach A*: Convert to Base64 (Slow, but easy for MVP).
    -   *Approach B*: Raw buffer transfer + Canvas `putImageData`.
- [ ] Update the "Canvas" node in React to display this image data.

## Phase 5: Composition (The "Cool" Part)
*Objective: Prove we can combine things.*
- [ ] Create a **Blend Node** (Inputs: Top, Bottom).
- [ ] Write a simple WebGPU render pass that samples two textures and mixes them.
- [ ] Connect: `Solid Red` -> `Blend (Top)` + `Solid Blue` -> `Blend (Bottom)` -> `Output`.
- [ ] Verify the output is purple.

## Future / Post-MVP
-   Auto-tiling / Pattern generation nodes.
-   Sprite sheet export.
-   Lua scripting for custom nodes.
-   Node groups/prefabs.
