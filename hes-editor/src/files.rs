use anyhow::Result;
use base64::prelude::*;
use brotli::{CompressorWriter, Decompressor};
use leptos::window;
use std::io::{Cursor, Read, Write};

use hes_engine::World;

fn serialize(world: &World) -> Result<String> {
    let bytes = rmp_serde::encode::to_vec_named(&world)?;
    let compressed = compress(&bytes)?;
    Ok(BASE64_STANDARD.encode(compressed))
}

fn compress(input: &[u8]) -> Result<Vec<u8>> {
    let mut compressed = Vec::new();
    {
        // Maximum compression
        let mut compressor = CompressorWriter::new(
            &mut compressed,
            4096,
            11,
            24,
        );
        compressor.write_all(input)?;
    }
    Ok(compressed)
}

fn deserialize(data: String) -> Result<World> {
    let bytes = BASE64_STANDARD.decode(data)?;
    let decompressed = decompress(&bytes)?;
    let world: World =
        rmp_serde::decode::from_slice(&decompressed)?;
    Ok(world)
}

fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    let mut decompressed = Vec::new();
    let mut decompressor =
        Decompressor::new(Cursor::new(input), 4096);
    decompressor.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

pub fn save_session(world: &World) -> Result<()> {
    if let Some(storage) = window().local_storage().unwrap() {
        let data = serialize(world)?;
        storage.set_item("session-world", &data).unwrap();
    }
    Ok(())
}

pub fn load_session() -> Result<Option<World>> {
    if let Some(storage) = window().local_storage().unwrap() {
        if let Some(data) =
            storage.get_item("session-world").unwrap()
        {
            let world = deserialize(data)?;
            return Ok(Some(world));
        }
    }
    Ok(None)
}
