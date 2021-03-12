#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use hound;
    use id666_rs::ID666;

    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            let mut data = include_bytes!("../test_fixtures/twinbeeeeeeeee.spc").to_vec();
            let spc = spc_new();
            let filter = spc_filter_new();
            let id6 = ID666::from(&mut data).unwrap();

            spc_load_spc(spc, data.as_mut_ptr() as *mut c_void, data.len() as i64);

            spc_clear_echo(spc);
            spc_filter_clear(filter);
            spc_filter_set_gain(filter, 0x180);

            let mut total_frames = id6.total_len.unwrap() / 2;

            let mut buf = [0i16; 2 * 4096];

            let spec = hound::WavSpec {
                channels: 2,
                sample_rate: 32000,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::create("twinbeeeeeeeee.wav", spec).unwrap();

            while total_frames != 0 {
                let fc = if total_frames < 4096 {
                    total_frames
                } else {
                    4096
                };
                spc_play(spc, (fc * 2) as i32, buf.as_mut_ptr());
                spc_filter_run(filter, buf.as_mut_ptr(), (fc * 2) as i32);

                for &b in buf.iter() {
                    writer.write_sample(b).unwrap();
                }

                total_frames -= fc;
            }

            spc_delete(spc);
            spc_filter_delete(filter);

            writer.finalize().unwrap();
        }
    }
}
