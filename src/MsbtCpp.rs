extern crate libc;
use libc::{c_char, size_t};
use msyt::converter::MsytFile;
use std::{
    f64::consts::E,
    ffi::{CStr, CString},
    fs, io,
    panic::{self, AssertUnwindSafe},
};

extern "C" {
    fn cxx_string_to_binary(text: *const c_char, length: *mut size_t) -> *mut u8;
    fn cxx_binary_to_string(binary: *const u8, length: usize) -> *const c_char;
    fn free_cxx_string(str: *mut c_char);
    fn free_cxx_binary(binary: *mut u8);
}

pub unsafe fn string_to_binary_rust(text: &str) -> Vec<u8> {
    let c_text = CString::new(text).unwrap();
    let mut length = 0;
    let c_binary = cxx_string_to_binary(c_text.as_ptr(), &mut length);
    let binary = std::slice::from_raw_parts(c_binary, length as usize);
    let binary_vec = binary.to_vec(); // Copy data into a Rust-managed Vec<u8>
    free_cxx_binary(c_binary); // Correctly free the C++ allocated memory
    binary_vec
}

pub unsafe fn binary_to_string_rust(binary: &[u8]) -> String {
    let c_string = cxx_binary_to_string(binary.as_ptr(), binary.len());
    let result = CStr::from_ptr(c_string).to_string_lossy().into_owned();
    free_cxx_string(c_string as *mut c_char);
    result
}

pub struct MsbtCpp {
    pub text: String,
    pub binary: Vec<u8>,
    pub endian: Option<roead::Endian>,
}

impl MsbtCpp {
    pub fn from_text(text: &str, endian: roead::Endian) -> io::Result<Self> {
        let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
            unsafe { string_to_binary_rust(text) }
        }));

        if let Ok(binary) = result {
            let endian = Self::check_endianness(&binary.to_vec());
            return Ok(MsbtCpp {
                text: text.to_string(),
                binary,
                endian,
            });
        }
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            MsytFile::text_to_binary(text, endian, None)
        }));

        if let Ok(msbt) = result {
            if let Ok(binary) = msbt {
                let endian = Self::check_endianness(&binary.to_vec());
                return Ok(MsbtCpp {
                    text: text.to_string(),
                    binary: binary,
                    endian,
                });
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unable to convert text to binary",
        ))
    }
    pub fn from_binary(binary: &[u8]) -> io::Result<Self> {
        if binary.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Msbt Binary data is empty",
            ));
        }
        let endian = Self::check_endianness(&binary.to_vec());

        if let Some(e) = &endian {
            if e == &roead::Endian::Little {
                let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
                    unsafe { binary_to_string_rust(binary) }
                }));
                if let Ok(text) = result {
                    if !text.is_empty() {
                        return Ok(MsbtCpp {
                            text,
                            binary: binary.to_vec(),
                            endian,
                        });
                    }
                }
            }
            if let Ok(text) = MsytFile::binary_to_text_safe(binary.to_vec()) {
                if !text.is_empty() {
                    return Ok(MsbtCpp {
                        text,
                        binary: binary.to_vec(),
                        endian,
                    });
                }
            }

            
        }
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unable to process msbt from binary nor determine endianness",
        ));

        // let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
        //     unsafe { binary_to_string_rust(binary) }
        // }));

        // if let Ok(text) = result {
        //     let binary = binary.to_vec();
        //     return Ok(MsbtCpp {
        //         text,
        //         binary,
        //         endian,
        //     });
        // }

        // match MsytFile::binary_to_text_safe(binary.to_vec()) {
        //     Ok(text) => {
        //         return Ok(MsbtCpp {
        //             text,
        //             binary: binary.to_vec(),
        //             endian,
        //         });
        //     }
        //     Err(e) => {
        //         return Err(e);
        //     }
        // }
    }

    pub fn from_binary_file(file_path: &str) -> Result<Self, std::io::Error> {
        match fs::read(file_path) {
            Ok(bytes) => match MsbtCpp::from_binary(&bytes) {
                Ok(res) => Ok(res),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn from_text_file(file_path: &str) -> Result<Self, std::io::Error> {
        match fs::read(file_path) {
            Ok(bytes) => {
                let text = match String::from_utf8(bytes) {
                    Ok(t) => t,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                };
                Ok(MsbtCpp::from_text(&text, roead::Endian::Little)?)
            }
            Err(e) => Err(e),
        }
    }

    pub fn to_binary_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        fs::write(file_path, &self.binary)
    }

    pub fn to_text_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        fs::write(file_path, &self.text)
    }

    fn check_endianness(bytes: &Vec<u8>) -> Option<roead::Endian> {
        if bytes.len() >= 10 {
            // Ensure there are at least 10 bytes to check
            match bytes[8..10] {
                [0xFE, 0xFF] => Some(roead::Endian::Big),    // Big Endian
                [0xFF, 0xFE] => Some(roead::Endian::Little), // Little Endian
                _ => None,                                   // Not matching either pattern
            }
        } else {
            None // Not enough data to determine endianness
        }
    }
}
