use libtinyfseq::tinyfseq::{tf_header_read, TFCompressionType, TFHeader};

fn header_template() -> TFHeader {
    TFHeader {
        channel_data_offset: 0,
        minor_version: 0,
        major_version: 0,
        variable_data_offset: 0,
        channel_count: 0,
        frame_count: 0,
        frame_step_time_millis: 0,
        compression_type: TFCompressionType::TF_COMPRESSION_NONE,
        compression_block_count: 0,
        channel_range_count: 0,
        sequence_uid: 0,
    }
}

#[test]
fn test_tf_header_read_fuzz_like_inputs() {
    let samples: [&[u8]; 6] = [
        &[],
        &[0],
        &[0, 1, 2, 3],
        &[0; 16],
        &[0xFF; 32],
        b"TINYFSEQ",
    ];

    for sample in samples {
        let mut header = header_template();
        let _ = tf_header_read(sample, &mut header, None);
    }
}

fn main() {}
