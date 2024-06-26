use std::ops::{Deref, DerefMut};

use binrw::{BinRead, BinWrite};
use num_traits::{AsPrimitive, Unsigned};

use crate::render_block_model::RenderBlockError;

pub trait Vertex: Clone
where
    Self::VertexArgs: Clone,
{
    type VertexArgs;
}

#[derive(Clone, Debug, Default)]
pub struct VertexBuffer<T: Vertex>(pub(crate) Vec<T>);

impl<T: Vertex> Deref for VertexBuffer<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Vertex> DerefMut for VertexBuffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Vertex> BinRead for VertexBuffer<T>
where
    T: for<'a> BinRead<Args<'a> = T::VertexArgs> + for<'b> BinWrite<Args<'b> = T::VertexArgs>,
{
    type Args<'a> = T::VertexArgs;

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let length = u32::read_options(reader, endian, ())?;
        let mut vertices = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vertices.push(T::read_options(reader, endian, args.clone())?);
        }
        Ok(Self(vertices))
    }
}

impl<T: Vertex> BinWrite for VertexBuffer<T>
where
    T: for<'a> BinRead<Args<'a> = T::VertexArgs> + for<'b> BinWrite<Args<'b> = T::VertexArgs>,
{
    type Args<'a> = T::VertexArgs;

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        type BinError = binrw::Error;

        if let Ok(length) = u32::try_from(self.len()) {
            length.write_options(writer, endian, ())?;
            for vertex in self.iter() {
                vertex.write_options(writer, endian, args.clone())?;
            }
            Ok(())
        } else {
            Err(BinError::Custom {
                pos: writer.stream_position()?,
                err: Box::new(RenderBlockError::InvalidArrayLength),
            })
        }
    }
}

pub trait Index:
    Clone + for<'a> BinRead<Args<'a> = ()> + for<'b> BinWrite<Args<'b> = ()> + Unsigned
{
}

impl<T> Index for T where
    T: Clone + for<'a> BinRead<Args<'a> = ()> + for<'b> BinWrite<Args<'b> = ()> + Unsigned
{
}

#[derive(Clone, Debug, Default)]
pub struct IndexBuffer<T: Index>(Vec<T>);

impl<T: Index> Deref for IndexBuffer<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Index> DerefMut for IndexBuffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type BinError = binrw::Error;

impl<T: Index + AsPrimitive<usize>> BinRead for IndexBuffer<T> {
    type Args<'a> = (usize,);

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let length = u32::read_options(reader, endian, ())?;
        let mut indices = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let index = T::read_options(reader, endian, ())?;
            if index.as_() > args.0 {
                return Err(BinError::Custom {
                    pos: reader.stream_position()?,
                    err: Box::new(RenderBlockError::InvalidArrayLength),
                });
            }
            indices.push(index);
        }
        Ok(Self(indices))
    }
}

impl<T: Index + AsPrimitive<usize>> BinWrite for IndexBuffer<T> {
    type Args<'a> = (usize,);

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        if let Ok(length) = u32::try_from(self.len()) {
            length.write_options(writer, endian, ())?;
            for index in self.iter() {
                if index.as_() > args.0 {
                    return Err(BinError::Custom {
                        pos: writer.stream_position()?,
                        err: Box::new(RenderBlockError::InvalidArrayLength),
                    });
                }
                index.write_options(writer, endian, ())?;
            }
            Ok(())
        } else {
            Err(BinError::Custom {
                pos: writer.stream_position()?,
                err: Box::new(RenderBlockError::InvalidArrayLength),
            })
        }
    }
}
