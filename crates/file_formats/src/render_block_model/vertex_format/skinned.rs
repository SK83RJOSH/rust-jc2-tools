use binrw::{binrw, BinRead, BinWrite};

use crate::{
    math::{Vec2, Vec3},
    render_block_model::PackedNormalU32,
};

use super::{GenericVertex, Vertex};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertex {
    pub position: Vec3<f32>,
    pub bone_weights: [u8; 8],
    pub bone_indices: [u8; 8],
    pub normal: PackedNormalU32,
    pub tangent: PackedNormalU32,
    pub binormal: PackedNormalU32,
    pub uv0: Vec2<f32>,
}

impl From<GenericVertex> for SkinnedVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        value.into()
    }
}

impl From<SkinnedVertex> for GenericVertex {
    #[inline]
    fn from(value: SkinnedVertex) -> Self {
        Self {
            position: value.position,
            bone_weights: [
                value.bone_weights[0] as f32 / 255.0,
                value.bone_weights[1] as f32 / 255.0,
                value.bone_weights[2] as f32 / 255.0,
                value.bone_weights[3] as f32 / 255.0,
                value.bone_weights[4] as f32 / 255.0,
                value.bone_weights[5] as f32 / 255.0,
                value.bone_weights[6] as f32 / 255.0,
                value.bone_weights[7] as f32 / 255.0,
            ],
            bone_indices: [
                value.bone_indices[0] as u32,
                value.bone_indices[1] as u32,
                value.bone_indices[2] as u32,
                value.bone_indices[3] as u32,
                value.bone_indices[4] as u32,
                value.bone_indices[5] as u32,
                value.bone_indices[6] as u32,
                value.bone_indices[7] as u32,
            ],
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            binormal: value.binormal.into(),
            uv0: value.uv0,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertexPosition {
    pub position: Vec3<f32>,
    pub bone_weights: [u8; 8],
    pub bone_indices: [u8; 8],
}

impl Vertex for SkinnedVertexPosition {
    type VertexArgs = (bool,);
}

impl BinRead for SkinnedVertexPosition {
    type Args<'a> = (bool,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(match args {
            (false,) => SkinnedVertex4Position::read_options(reader, endian, ())?.into(),
            (true,) => SkinnedVertex8Position::read_options(reader, endian, ())?.into(),
        })
    }
}

impl BinWrite for SkinnedVertexPosition {
    type Args<'a> = (bool,);

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let value = self.clone();
        match args {
            (false,) => SkinnedVertex4Position::from(value).write_options(writer, endian, ()),
            (true,) => SkinnedVertex8Position::from(value).write_options(writer, endian, ()),
        }
    }
}

impl From<SkinnedVertex4Position> for SkinnedVertexPosition {
    #[inline]
    fn from(value: SkinnedVertex4Position) -> Self {
        let bone_weights: [u32; 2] = [bytemuck::must_cast(value.bone_weights), 0u32];
        let bone_indices: [u32; 2] = [bytemuck::must_cast(value.bone_indices), 0u32];
        Self {
            position: value.position,
            bone_weights: bytemuck::must_cast(bone_weights),
            bone_indices: bytemuck::must_cast(bone_indices),
        }
    }
}

impl From<SkinnedVertex8Position> for SkinnedVertexPosition {
    #[inline]
    fn from(value: SkinnedVertex8Position) -> Self {
        Self {
            position: value.position,
            bone_weights: bytemuck::must_cast(value.bone_weights),
            bone_indices: bytemuck::must_cast(value.bone_indices),
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertex4Position {
    pub position: Vec3<f32>,
    pub bone_weights: u32,
    pub bone_indices: u32,
}

impl From<SkinnedVertexPosition> for SkinnedVertex4Position {
    #[inline]
    fn from(value: SkinnedVertexPosition) -> Self {
        value.into()
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertex8Position {
    pub position: Vec3<f32>,
    pub bone_weights: [u32; 2],
    pub bone_indices: [u32; 2],
}

impl From<SkinnedVertexPosition> for SkinnedVertex8Position {
    #[inline]
    fn from(value: SkinnedVertexPosition) -> Self {
        value.into()
    }
}

#[binrw]
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertexData {
    pub normal: PackedNormalU32,
    pub tangent: PackedNormalU32,
    pub binormal: PackedNormalU32,
    pub uv0: Vec2<f32>,
}

impl Vertex for SkinnedVertexData {
    type VertexArgs = ();
}
