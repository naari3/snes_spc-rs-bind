use std::{
    error::Error,
    ffi::{c_void, CStr},
    fmt::Display,
};

use snes_spc_sys::*;

#[derive(Debug)]
pub struct SNESSpc {
    inner_spc: *mut snes_spc_t,
}

#[derive(Debug)]
pub enum SNESSpcError {
    NotSpcFile,
    CorruptSpcFile,
    SPCEmulationError,
}

impl Error for SNESSpcError {
    fn description<'a>(&'a self) -> &'a str {
        match *self {
            SNESSpcError::NotSpcFile => "Not spc file",
            SNESSpcError::CorruptSpcFile => "Corrupt spc file",
            SNESSpcError::SPCEmulationError => "SPC Emulation Error",
        }
    }
}

impl Display for SNESSpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SNESSpc {
    pub fn new() -> Self {
        unsafe {
            Self {
                inner_spc: spc_new(),
            }
        }
    }

    pub fn from(data: &mut [u8]) -> Result<Self, SNESSpcError> {
        let spc = Self::new();
        spc.load_spc(data)?;
        Ok(spc)
    }

    pub fn init_rom(&self, data: &mut [u8]) {
        unsafe { spc_init_rom(self.inner_spc, data.as_mut_ptr()) }
    }

    pub fn sample_count(&self) -> i32 {
        unsafe { spc_sample_count(self.inner_spc) }
    }

    pub fn reset(&self) {
        unsafe { spc_reset(self.inner_spc) }
    }

    pub fn soft_reset(&self) {
        unsafe { spc_soft_reset(self.inner_spc) }
    }

    pub fn read_port(&self, t: i32, port: i32) -> i32 {
        unsafe { spc_read_port(self.inner_spc, t, port) }
    }

    pub fn write_port(&self, t: i32, port: i32, data: i32) {
        unsafe { spc_write_port(self.inner_spc, t, port, data) }
    }

    pub fn end_frame(&self, t: i32) {
        unsafe { spc_end_frame(self.inner_spc, t) }
    }

    pub fn mute_voices(&self, mask: i32) {
        unsafe { spc_mute_voices(self.inner_spc, mask) }
    }

    pub fn disable_surround(&self, disable: bool) {
        unsafe { spc_disable_surround(self.inner_spc, disable as i32) }
    }

    pub fn set_tempo(&self, tempo: i32) {
        unsafe { spc_set_tempo(self.inner_spc, tempo) }
    }

    pub fn load_spc(&self, data: &mut [u8]) -> Result<(), SNESSpcError> {
        // let data_len =
        let err = unsafe {
            spc_load_spc(
                self.inner_spc,
                data.as_mut_ptr() as *mut c_void,
                data.len() as ::std::os::raw::c_long,
            )
        };
        if err as i8 == 0 {
            return Ok(());
        }
        let err_str = unsafe { CStr::from_ptr(err) };
        match err_str.to_str().unwrap() {
            "Not an SPC file" => Err(SNESSpcError::NotSpcFile),
            "Corrupt SPC file" => Err(SNESSpcError::CorruptSpcFile),
            _ => Ok(()),
        }
    }

    pub fn clear_echo(&self) {
        unsafe { spc_clear_echo(self.inner_spc) }
    }

    pub fn play(&self, count: i32, out: &mut [i16]) -> Result<(), SNESSpcError> {
        let err = unsafe { spc_play(self.inner_spc, count, out.as_mut_ptr()) };
        if err as i8 == 0 {
            return Ok(());
        }
        let err_str = unsafe { CStr::from_ptr(err) };
        match err_str.to_str().unwrap() {
            "SPC emulation error" => Err(SNESSpcError::SPCEmulationError),
            _ => Ok(()),
        }
    }

    pub fn skip(&self, count: i32) -> Result<(), SNESSpcError> {
        let err = unsafe { spc_skip(self.inner_spc, count) };
        if err as i8 == 0 {
            return Ok(());
        }
        let err_str = unsafe { CStr::from_ptr(err) };
        match err_str.to_str().unwrap() {
            "SPC emulation error" => Err(SNESSpcError::SPCEmulationError),
            _ => Ok(()),
        }
    }

    // TODO: there is no functions belows, shoud we implement them?
    //     /**** State save/load (only available with accurate DSP) ****/
    // /* Saves/loads exact emulator state */
    // enum { spc_state_size = 67 * 1024L }; /* maximum space needed when saving */
    // typedef void (*spc_copy_func_t)( unsigned char** io, void* state, size_t );
    // void spc_copy_state( snes_spc_t*, unsigned char** io, spc_copy_func_t );

    // /* Writes minimal SPC file header to spc_out */
    // void spc_init_header( void* spc_out );

    // /* Saves emulator state as SPC file data. Writes spc_file_size bytes to spc_out.
    // Does not set up SPC header; use spc_init_header() for that. */
    // enum { spc_file_size = 0x10200 }; /* spc_out must have this many bytes allocated */
    // void spc_save_spc( snes_spc_t*, void* spc_out );

    // /* Returns non-zero if new key-on events occurred since last check. Useful for
    // trimming silence while saving an SPC. */
    // int spc_check_kon( snes_spc_t* );
}

impl Drop for SNESSpc {
    fn drop(&mut self) {
        unsafe {
            spc_delete(self.inner_spc);
        }
    }
}

#[derive(Debug)]
pub struct SpcFilter {
    inner_filter: *mut spc_filter_t,
}

impl SpcFilter {
    pub fn new() -> Self {
        unsafe {
            Self {
                inner_filter: spc_filter_new(),
            }
        }
    }

    pub fn run(&self, io: &mut [i16], count: i32) {
        unsafe { spc_filter_run(self.inner_filter, io.as_mut_ptr(), count) }
    }

    pub fn clear(&self) {
        unsafe { spc_filter_clear(self.inner_filter) }
    }

    pub fn set_gain(&self, gain: i32) {
        unsafe { spc_filter_set_gain(self.inner_filter, gain) }
    }

    pub fn set_bass(&self, bass: i32) {
        unsafe { spc_filter_set_bass(self.inner_filter, bass) }
    }
}

impl Drop for SpcFilter {
    fn drop(&mut self) {
        unsafe {
            spc_filter_delete(self.inner_filter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hound;
    use id666_rs::ID666;

    #[test]
    fn it_works() {
        let mut data = include_bytes!("../test_fixtures/twinbeeeeeeeee.spc").to_vec();
        let spc = SNESSpc::from(&mut data).unwrap();
        let filter = SpcFilter::new();
        let id6 = ID666::from(&mut data).unwrap();

        spc.clear_echo();
        filter.clear();
        filter.set_gain(0x180);

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
            let stereo_count = (fc * 2) as i32;
            spc.play(stereo_count, &mut buf).unwrap();
            filter.run(&mut buf, stereo_count);

            for &b in buf.iter() {
                writer.write_sample(b).unwrap();
            }

            total_frames -= fc;
        }

        writer.finalize().unwrap();
    }
}
