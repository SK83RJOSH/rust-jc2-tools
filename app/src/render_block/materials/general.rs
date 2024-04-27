use bevy::{
    prelude::*,
    render::{mesh::MeshVertexAttribute, render_asset::*, render_resource::*},
};
use jc2_file_formats::render_block_model::{GeneralAttributes, GeneralFlags};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RenderBlockGeneralMaterialKey {
    cull: bool,
    depth_bias: Option<i32>,
    use_channel_textures: bool,
    use_snow: bool,
}

impl From<&RenderBlockGeneralMaterial> for RenderBlockGeneralMaterialKey {
    #[inline]
    fn from(material: &RenderBlockGeneralMaterial) -> Self {
        RenderBlockGeneralMaterialKey {
            cull: material.cull,
            depth_bias: material.depth_test.then_some(material.depth_bias as i32),
            use_channel_textures: material.use_channel_textures,
            use_snow: material.use_snow,
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct RenderBlockGeneralMaterialUniform {
    pub scale: f32,
    pub specular_power: f32,
    pub uv0_scale: Vec2,
    pub uv1_scale: Vec2,
    pub channel_mask: Vec4,
    pub channel_ambient_occlusion_mask: Vec4,
}

impl AsBindGroupShaderType<RenderBlockGeneralMaterialUniform> for RenderBlockGeneralMaterial {
    #[inline]
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<Image>,
    ) -> RenderBlockGeneralMaterialUniform {
        RenderBlockGeneralMaterialUniform {
            scale: self.scale,
            specular_power: self.specular_power,
            uv0_scale: self.uv0_scale,
            uv1_scale: self.uv1_scale,
            channel_mask: self.channel_mask,
            channel_ambient_occlusion_mask: self.channel_ambient_occlusion_mask,
        }
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Default, Clone)]
#[bind_group_data(RenderBlockGeneralMaterialKey)]
#[uniform(0, RenderBlockGeneralMaterialUniform)]
#[reflect(Default, Debug)]
pub struct RenderBlockGeneralMaterial {
    #[texture(1)]
    #[sampler(2)]
    #[dependency]
    pub diffuse_texture: Option<Handle<Image>>,

    #[texture(3)]
    #[sampler(4)]
    #[dependency]
    pub normal_texture: Option<Handle<Image>>,

    #[texture(5)]
    #[sampler(6)]
    #[dependency]
    pub properties_texture: Option<Handle<Image>>,

    #[texture(7)]
    #[sampler(8)]
    #[dependency]
    pub dirt_color_texture: Option<Handle<Image>>,

    pub scale: f32,
    pub specular_power: f32,
    pub uv0_scale: Vec2,
    pub uv1_scale: Vec2,

    pub cull: bool,
    pub depth_test: bool,
    pub depth_bias: f32,

    pub alpha_test: bool,
    pub alpha_blend: bool,
    pub alpha_additive: bool,

    pub use_channel_textures: bool,
    pub channel_mask: Vec4,
    pub channel_ambient_occlusion_mask: Vec4,

    pub use_palette: bool,
    pub use_sub_surface_scattering: bool,
    pub use_snow: bool,
    pub use_ambient_occlusion: bool,
    pub use_animate: bool,
}

impl From<&GeneralAttributes> for RenderBlockGeneralMaterial {
    #[inline]
    fn from(value: &GeneralAttributes) -> Self {
        Self {
            diffuse_texture: None,
            normal_texture: None,
            properties_texture: None,
            dirt_color_texture: None,
            scale: value.vertex_info.scale,
            specular_power: value.specular_power,
            uv0_scale: Vec2::from_array(value.vertex_info.uv0_extent.into()),
            uv1_scale: Vec2::from_array(value.vertex_info.uv1_extent.into()),
            cull: !value.flags.contains(GeneralFlags::NO_CULLING),
            depth_test: !value.flags.contains(GeneralFlags::NO_DEPTH_TEST),
            depth_bias: value.depth_bias,
            alpha_test: value.flags.contains(GeneralFlags::ALPHA_TEST),
            alpha_blend: value.flags.contains(GeneralFlags::ALPHA_BLENDING),
            alpha_additive: value.flags.contains(GeneralFlags::ADDITIVE_ALPHA),
            use_channel_textures: value.flags.contains(GeneralFlags::USE_CHANNEL_TEXTURES),
            channel_mask: Vec4::from_array(value.channel_mask.into()),
            channel_ambient_occlusion_mask: Vec4::from_array(
                value.channel_ambient_occlusion_mask.into(),
            ),
            use_palette: value.flags.contains(GeneralFlags::USE_PALETTE),
            use_sub_surface_scattering: value
                .flags
                .contains(GeneralFlags::USE_SUB_SURFACE_SCATTERING),
            use_snow: value.flags.contains(GeneralFlags::USE_SNOW_FLAG),
            use_ambient_occlusion: value.flags.contains(GeneralFlags::USE_AMBIENT_OCCLUSION),
            use_animate: value.flags.contains(GeneralFlags::ANIMATE_TEXTURE),
        }
    }
}

pub const ATTRIBUTE_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("Position", 4250932154, VertexFormat::Float32x3);

pub const ATTRIBUTE_UV0: MeshVertexAttribute =
    MeshVertexAttribute::new("UV0", 3070075345, VertexFormat::Float32x2);

pub const ATTRIBUTE_UV1: MeshVertexAttribute =
    MeshVertexAttribute::new("UV1", 615186204, VertexFormat::Float32x2);

pub const ATTRIBUTE_NORMAL: MeshVertexAttribute =
    MeshVertexAttribute::new("Normal", 3848509694, VertexFormat::Float32x3);

pub const ATTRIBUTE_TANGENT: MeshVertexAttribute =
    MeshVertexAttribute::new("Tangent", 2959512384, VertexFormat::Float32x3);

pub const ATTRIBUTE_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("Color", 1706632676, VertexFormat::Float32x4);

impl Material for RenderBlockGeneralMaterial {
    #[inline]
    fn vertex_shader() -> ShaderRef {
        "shaders/general_material.wgsl".into()
    }

    #[inline]
    fn fragment_shader() -> ShaderRef {
        "shaders/general_material.wgsl".into()
    }

    #[inline]
    fn alpha_mode(&self) -> AlphaMode {
        if self.alpha_test {
            if self.alpha_additive {
                AlphaMode::Add
            } else {
                AlphaMode::Blend
            }
        } else {
            AlphaMode::Opaque
        }
    }

    #[inline]
    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }

    #[inline]
    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayout,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_UV0.at_shader_location(1),
            ATTRIBUTE_UV1.at_shader_location(2),
            ATTRIBUTE_NORMAL.at_shader_location(3),
            ATTRIBUTE_TANGENT.at_shader_location(4),
            ATTRIBUTE_COLOR.at_shader_location(5),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];

        if let Some(fragment) = descriptor.fragment.as_mut() {
            let shader_defs = &mut fragment.shader_defs;
            if key.bind_group_data.use_snow {
                shader_defs.push("USE_CHANNEL_TEXTURES".into());
            }
        }

        if let Some(fragment) = descriptor.fragment.as_mut() {
            let shader_defs = &mut fragment.shader_defs;
            if key.bind_group_data.use_snow {
                shader_defs.push("USE_SNOW".into());
            }
        }

        if key.bind_group_data.cull {
            descriptor.primitive.cull_mode = Some(Face::Back);
        }

        if let Some(label) = &mut descriptor.label {
            *label = format!("general_{}", *label).into();
        }

        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            if let Some(depth_bias) = key.bind_group_data.depth_bias {
                depth_stencil.bias.constant = depth_bias;
            } else {
                depth_stencil.depth_compare = CompareFunction::Always;
            }
        }

        Ok(())
    }
}
