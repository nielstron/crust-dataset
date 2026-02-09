use libtinyfseq::tinyfseq::{tf_var_header_read, TFVarHeader};

fn var_header_template() -> TFVarHeader {
    TFVarHeader {
        size: 0,
        id: [0, 0],
    }
}

#[test]
fn test_tf_var_header_read_fuzz_like_inputs() {
    let samples: [&[u8]; 6] = [
        &[],
        &[0],
        &[0, 1, 2, 3],
        &[0; 8],
        &[0xFF; 12],
        b"TINYFSEQ",
    ];

    for sample in samples {
        let mut header = var_header_template();
        let mut vd: [u8; 32] = [0; 32];
        let _ = tf_var_header_read(sample, &mut header, &mut vd, None);
    }
}

fn main() {}
