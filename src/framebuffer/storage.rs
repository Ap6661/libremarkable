use zstd;

use std::sync::Arc;

use image;

#[derive(Clone)]
pub struct CompressedCanvasState {
    data: Arc<[u8]>,
    height: u32,
    width: u32,
}

/// For reference, a rectangle with height=1050 and width=1404
/// will have the following size at rest:
///
/// (notice better compression at first due to relatively low-entropy canvas
///  compression plateuaus around 93% as the entropy peaks)
///
///    raw: 5896.8 kB -- zstd: 7.875 kB  (99.86645% compression)
///    raw: 5896.8 kB -- zstd: 25.405 kB  (99.569176% compression)
///    raw: 5896.8 kB -- zstd: 210.628 kB  (96.4281% compression)
///    raw: 5896.8 kB -- zstd: 367.217 kB  (93.7726% compression)
///    raw: 5896.8 kB -- zstd: 367.217 kB  (93.7726% compression)
///    raw: 5896.8 kB -- zstd: 356.432 kB  (93.9555% compression)
///    raw: 5896.8 kB -- zstd: 361.935 kB  (93.86218% compression)
impl CompressedCanvasState {
    /// Creates a CompressedCanvasState from the output of FramebufferIO::dump_region(..)
    pub fn new(buff: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> CompressedCanvasState {
        CompressedCanvasState {
            data: zstd::encode_all(buff.to_vec().as_slice(), 0)
                .unwrap()
                .into(),
            height: buff.height(),
            width: buff.width(),
        }
    }

    /// Returns an ImageBuffer which can be used to restore the contents of a screen
    /// region using the FramebufferIO::restore_region(..)
    pub fn decompress(&self) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let unencoded = zstd::decode_all(&*self.data).unwrap();
        image::ImageBuffer::from_raw(self.width, self.height, unencoded).unwrap()
    }
}