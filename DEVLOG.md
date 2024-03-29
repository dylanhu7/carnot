# Carnot Development Log

## 2024-03-27

I found the blog post from Godot very accessible, as it explains concepts without too much technical jargon so that someone who simply uses the engine, for example, might understand it. A few major takeaways include:

- The render graph is a directed acyclic graph (DAG) that represents the rendering pipeline.
- Command buffers are used to record commands for the GPU to execute.
- These commands are executed after submitting the command buffers to a queue.
- Operations in the queue are executed asynchronously, so synchronization is necessary to ensure that operations are executed in the correct order.

  > Operations that are submitted to queues are executed asynchronously. Therefore we have to use synchronization objects like semaphores to ensure a correct order of execution. Execution of the draw command buffer must be set up to wait on image acquisition to finish, otherwise it may occur that we start rendering to an image that is still being read for presentation on the screen. The vkQueuePresentKHR call in turn needs to wait for rendering to be finished, for which we’ll use a second semaphore that is signaled after rendering completes.
  >
  > \- [Khronos Vulkan Tutorial](https://docs.vulkan.org/tutorial/latest/01_Overview.html#_step_7_command_pools_and_command_buffers)

- A render pass is an object that holds references to textures that will be used as render targets. Only certain operations can be performed within a render pass.
- Since the execution of the commands is not guaranteed to match the order in which they were submitted, synchronization barriers must be manually inserted to ensure that results from previous commands are available as necessary before dependent commands are executed.

## 2024-03-26

### Outline

In this first entry, I will outline my existing knowledge, as well as my goals and motivations for this project.

I am coming in with a very introductory understanding of realtime graphics. In the past, I have implemented projects in OpenGL using vertex buffers and attributes, shaders, framebuffers, and textures. In these projects, the rendering pipeline simply consisted of looping through shapes, binding their vertex buffers and uniforms, and drawing them to the screen. It is my hope that this project will allow me to explore more advanced rendering techniques and architectures.

Currently, I am focusing on researching modern approaches to rendering and game engine architecture. In particular, I aim to research and implement:

- A render graph system
- A physically based rendering (PBR) pipeline
- An entity-component-system (ECS) architecture

I am also using this project as an opportunity to gain experience reading and writing idiomatic Rust.

### Render graph

At this point, I am still researching render graph systems to gain a high-level understanding of their use cases, components, and implementation details. Some resources I am using include:

- [FrameGraph: Extensible Rendering Architecture in Frostbite](https://www.gdcvault.com/play/1024612/FrameGraph-Extensible-Rendering-Architecture-in)
- [rend3](https://github.com/BVE-Reborn/rend3)
- [Ponies and Light: Rendergraphs and how to implement one](https://poniesandlight.co.uk/reflect/island_rendergraph_1/)
- [Ponies and Light: Vulkan Render-Queues and how they Sync](https://poniesandlight.co.uk/reflect/island_rendergraph_2/)
- [GPU synchronization in Godot 4.3 is getting a major upgrade](https://godotengine.org/article/rendering-acyclic-graph/)
