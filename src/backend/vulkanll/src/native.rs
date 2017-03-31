// Copyright 2017 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::pso;
use ash::vk;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

pub use command::CommandBuffer;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ShaderLib {
    // TODO:
    // There is currently no tool to merge the SPIR-V modules generated by glslang.
    // We can later think about merging them into a single shader module.
    pub shaders: BTreeMap<pso::EntryPoint, vk::ShaderModule>,
}
unsafe impl Send for ShaderLib {}
unsafe impl Sync for ShaderLib {}

#[derive(Debug, Hash)]
pub struct PipelineLayout {
    pub layout: vk::PipelineLayout,
}
unsafe impl Send for PipelineLayout {}
unsafe impl Sync for PipelineLayout {}

#[derive(Debug, Hash)]
pub struct RenderPass {
    pub inner: vk::RenderPass,
}
unsafe impl Send for RenderPass {}
unsafe impl Sync for RenderPass {}

#[derive(Debug, Hash)]
pub struct FrameBuffer {
    pub inner: vk::Framebuffer,
}
unsafe impl Send for FrameBuffer {}
unsafe impl Sync for FrameBuffer {}

#[derive(Debug, Hash)]
pub struct GraphicsPipeline {
    pub pipeline: vk::Pipeline,
}
unsafe impl Send for GraphicsPipeline {}
unsafe impl Sync for GraphicsPipeline {}

#[derive(Debug, Hash)]
pub struct ComputePipeline {
    pub pipeline: vk::Pipeline,
}
unsafe impl Send for ComputePipeline {}
unsafe impl Sync for ComputePipeline {}

pub struct GeneralCommandBuffer(pub CommandBuffer);
impl Deref for GeneralCommandBuffer {
    type Target = CommandBuffer;
    fn deref(&self) -> &CommandBuffer {
        &self.0
    }
}
impl DerefMut for GeneralCommandBuffer {
    fn deref_mut(&mut self) -> &mut CommandBuffer {
        &mut self.0
    }
}

pub struct GraphicsCommandBuffer(pub CommandBuffer);
impl Deref for GraphicsCommandBuffer {
    type Target = CommandBuffer;
    fn deref(&self) -> &CommandBuffer {
        &self.0
    }
}
impl DerefMut for GraphicsCommandBuffer {
    fn deref_mut(&mut self) -> &mut CommandBuffer {
        &mut self.0
    }
}

pub struct ComputeCommandBuffer(pub CommandBuffer);

pub struct TransferCommandBuffer(pub CommandBuffer);

pub struct SubpassCommandBuffer(pub CommandBuffer);

#[derive(Debug, Hash)]
pub struct Heap(pub vk::DeviceMemory);

#[derive(Debug, Hash)]
pub struct Buffer(pub vk::Buffer);

#[derive(Debug, Hash)]
pub struct Image(pub vk::Image);

#[derive(Debug, Hash)]
pub struct RenderTargetView {
    pub image: vk::Image,
    pub view: vk::ImageView,
}

#[derive(Debug, Hash)]
pub struct DepthStencilView {
    pub image: vk::Image,
    pub view: vk::ImageView,
}