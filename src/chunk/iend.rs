// PNG Pong
//
// Copyright © 2019-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{Chunk, DecoderResult, EncoderError};
use crate::{consts};
use std::io::{Write};

/// Image End Chunk Data (IEND)
#[derive(Copy, Clone, Debug)]
pub struct ImageEnd;

impl ImageEnd {
    pub(crate) fn parse() -> DecoderResult<Chunk> {
        Ok(Chunk::ImageEnd(ImageEnd))
    }

    pub(crate) fn write<W: Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncoderError> {
        super::encode_chunk(writer, consts::IMAGE_END, &[])
    }
}
