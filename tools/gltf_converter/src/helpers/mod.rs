use std::{mem::size_of_val, slice};

use jc2_file_formats::render_block_model::{Material, RenderBlock};

mod deformable;
mod general;
mod simple;
mod skinned;

type AccessorType = gltf_json::accessor::Type;
type AccessorComponentType = gltf_json::accessor::ComponentType;
type MeshSemantic = gltf_json::mesh::Semantic;

pub type GltfMeshAccessor = (AccessorType, AccessorComponentType, MeshSemantic, usize);

pub trait GltfMeshAccessors {
    fn get_mesh_accessors() -> Vec<GltfMeshAccessor>;
}

pub type GltfMeshMode = gltf_json::mesh::Mode;

pub trait GltfHelpers {
    fn vertex_count(&self) -> usize;
    fn index_count(&self) -> usize;

    fn vertex_stride(&self) -> usize;
    fn index_stride(&self) -> usize;

    fn vertices_as_bytes(&self) -> &[u8];
    fn indices_as_bytes(&self) -> &[u8];

    fn textures(&self) -> [&str; 8];
    fn mesh_mode(&self) -> GltfMeshMode;
    fn accessors(&self) -> Vec<GltfMeshAccessor>;
}

fn count<T>(value: &[T]) -> usize {
    value.len()
}

fn stride<T>(_: &[T]) -> usize {
    std::mem::size_of::<T>()
}

#[inline]
fn bytes<T>(value: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(value.as_ptr() as *const u8, size_of_val(value)) }
}

#[inline]
fn accessors<T: GltfMeshAccessors>(_: &[T]) -> Vec<GltfMeshAccessor> {
    T::get_mesh_accessors()
}

#[inline]
fn mesh_mode(material: &Material) -> GltfMeshMode {
    use jc2_file_formats::render_block_model::PrimitiveType::*;
    match material.primitive_type {
        TriangleList | IndexedTriangleList => GltfMeshMode::Triangles,
        TriangleStrip | IndexedTriangleStrip => GltfMeshMode::TriangleStrip,
        TriangleFan | IndexedTriangleFan => GltfMeshMode::TriangleFan,
        LineList => GltfMeshMode::Lines,
        PointSprite | IndexedPointSprite => GltfMeshMode::Points,
    }
}

#[inline]
fn textures(material: &Material) -> [&str; 8] {
    [
        material.textures[0].as_ref(),
        material.textures[1].as_ref(),
        material.textures[2].as_ref(),
        material.textures[3].as_ref(),
        material.textures[4].as_ref(),
        material.textures[5].as_ref(),
        material.textures[6].as_ref(),
        material.textures[7].as_ref(),
    ]
}

impl GltfHelpers for RenderBlock {
    #[inline]
    fn vertex_count(&self) -> usize {
        match self {
            RenderBlock::CarPaint(data) => count(&data.vertices),
            RenderBlock::CarPaintSimple(data) => count(&data.vertices),
            RenderBlock::General(data) => count(&data.vertices),
            RenderBlock::Lambert(data) => count(&data.vertices),
            RenderBlock::SkinnedGeneral(data) => count(&data.vertices),
        }
    }

    #[inline]
    fn index_count(&self) -> usize {
        match self {
            RenderBlock::CarPaint(data) => count(&data.indices),
            RenderBlock::CarPaintSimple(data) => count(&data.indices),
            RenderBlock::General(data) => count(&data.indices),
            RenderBlock::Lambert(data) => count(&data.indices),
            RenderBlock::SkinnedGeneral(data) => count(&data.indices),
        }
    }

    #[inline]
    fn vertex_stride(&self) -> usize {
        match self {
            RenderBlock::CarPaint(data) => stride(&data.vertices),
            RenderBlock::CarPaintSimple(data) => stride(&data.vertices),
            RenderBlock::General(data) => stride(&data.vertices),
            RenderBlock::Lambert(data) => stride(&data.vertices),
            RenderBlock::SkinnedGeneral(data) => stride(&data.vertices),
        }
    }

    #[inline]
    fn index_stride(&self) -> usize {
        match self {
            RenderBlock::CarPaint(data) => stride(&data.indices),
            RenderBlock::CarPaintSimple(data) => stride(&data.indices),
            RenderBlock::General(data) => stride(&data.indices),
            RenderBlock::Lambert(data) => stride(&data.indices),
            RenderBlock::SkinnedGeneral(data) => stride(&data.indices),
        }
    }

    #[inline]
    fn vertices_as_bytes(&self) -> &[u8] {
        match self {
            RenderBlock::CarPaint(data) => bytes(&data.vertices),
            RenderBlock::CarPaintSimple(data) => bytes(&data.vertices),
            RenderBlock::General(data) => bytes(&data.vertices),
            RenderBlock::Lambert(data) => bytes(&data.vertices),
            RenderBlock::SkinnedGeneral(data) => bytes(&data.vertices),
        }
    }

    #[inline]
    fn indices_as_bytes(&self) -> &[u8] {
        match self {
            RenderBlock::CarPaint(data) => bytes(&data.indices),
            RenderBlock::CarPaintSimple(data) => bytes(&data.indices),
            RenderBlock::General(data) => bytes(&data.indices),
            RenderBlock::Lambert(data) => bytes(&data.indices),
            RenderBlock::SkinnedGeneral(data) => bytes(&data.indices),
        }
    }

    #[inline]
    fn accessors(&self) -> Vec<GltfMeshAccessor> {
        match self {
            RenderBlock::CarPaint(data) => accessors(&data.vertices),
            RenderBlock::CarPaintSimple(data) => accessors(&data.vertices),
            RenderBlock::General(data) => accessors(&data.vertices),
            RenderBlock::Lambert(data) => accessors(&data.vertices),
            RenderBlock::SkinnedGeneral(data) => accessors(&data.vertices),
        }
    }

    #[inline]
    fn mesh_mode(&self) -> GltfMeshMode {
        match self {
            RenderBlock::CarPaint(data) => mesh_mode(&data.material),
            RenderBlock::CarPaintSimple(data) => mesh_mode(&data.material),
            RenderBlock::General(data) => mesh_mode(&data.material),
            RenderBlock::Lambert(data) => mesh_mode(&data.material),
            RenderBlock::SkinnedGeneral(data) => mesh_mode(&data.material),
        }
    }

    #[inline]
    fn textures(&self) -> [&str; 8] {
        match self {
            RenderBlock::CarPaint(data) => textures(&data.material),
            RenderBlock::CarPaintSimple(data) => textures(&data.material),
            RenderBlock::General(data) => textures(&data.material),
            RenderBlock::Lambert(data) => textures(&data.material),
            RenderBlock::SkinnedGeneral(data) => textures(&data.material),
        }
    }
}
