use libtinyfseq::tinyfseq::{tf_compression_block_read, TFCompressionBlock};

fn compression_block_template() -> TFCompressionBlock {
    TFCompressionBlock {
        first_frame_id: 0,
        size: 0,
    }
}

#[test]
fn test_tf_compression_block_read_fuzz_like_inputs() {
    let samples: [&[u8]; 6] = [&[], &[0], &[0, 1, 2, 3], &[0; 8], &[0xFF; 12], b"TINYFSEQ"];

    for sample in samples {
        let mut block = compression_block_template();
        let _ = tf_compression_block_read(sample, &mut block, None);
    }
}

fn main() {}
