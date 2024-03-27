# Carnot Development Log

## 2024-03-26

### Render graph

At this point, I am still researching render graph systems to gain a high-level understanding of their use cases, components, and implementation details. Some resources I am using include:

- [FrameGraph: Extensible Rendering Architecture in Frostbite](https://www.gdcvault.com/play/1024612/FrameGraph-Extensible-Rendering-Architecture-in)
- [rend3](https://github.com/BVE-Reborn/rend3)
- [Ponies and Light: Rendergraphs and how to implement one](https://poniesandlight.co.uk/reflect/island_rendergraph_1/)
- [Ponies and Light: Vulkan Render-Queues and how they Sync](https://poniesandlight.co.uk/reflect/island_rendergraph_2/)
- [GPU synchronization in Godot 4.3 is getting a major upgrade](https://godotengine.org/article/rendering-acyclic-graph/)

### Outline

In this first entry, I will outline my existing knowledge, as well as my goals and motivations for this project.

I am coming in with a very introductory understanding of realtime graphics. In the past, I have implemented projects in OpenGL using vertex buffers and attributes, shaders, framebuffers, and textures. In these projects, the rendering pipeline simply consisted of looping through shapes, binding their vertex buffers and uniforms, and drawing them to the screen. It is my hope that this project will allow me to explore more advanced rendering techniques and architectures.

Currently, I am focusing on researching modern approaches to rendering and game engine architecture. In particular, I aim to research and implement:

- A render graph system
- A physically based rendering (PBR) pipeline
- An entity-component-system (ECS) architecture

I am also using this project as an opportunity to gain experience reading and writing idiomatic Rust.
