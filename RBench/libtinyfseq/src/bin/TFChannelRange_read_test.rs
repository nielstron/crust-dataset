use libtinyfseq::tinyfseq::{tf_channel_range_read, TFChannelRange};

fn channel_range_template() -> TFChannelRange {
    TFChannelRange {
        first_channel_number: 0,
        channel_count: 0,
    }
}

#[test]
fn test_tf_channel_range_read_fuzz_like_inputs() {
    let samples: [&[u8]; 6] = [
        &[],
        &[0],
        &[0, 1, 2, 3],
        &[0; 8],
        &[0xFF; 12],
        b"TINYFSEQ",
    ];

    for sample in samples {
        let mut range = channel_range_template();
        let _ = tf_channel_range_read(sample, &mut range, None);
    }
}

fn main() {}
