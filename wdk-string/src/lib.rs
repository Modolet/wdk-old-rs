#![no_std]

extern crate alloc;

use wdk_sys::{ntddk::RtlInitUnicodeString, PUNICODE_STRING, UNICODE_STRING};

pub trait NtToString {
    fn to_string(&self) -> alloc::string::String;
}

pub trait NtFromString {
    fn from_string(s: &str) -> UNICODE_STRING;
}

impl NtToString for UNICODE_STRING {
    fn to_string(&self) -> alloc::string::String {
        unsafe {
            widestring::U16String::from_ptr(self.Buffer, self.Length as usize / 2)
                .to_string()
                .unwrap()
        }
    }
}

impl NtFromString for UNICODE_STRING {
    fn from_string(s: &str) -> UNICODE_STRING {
        let wide_str = widestring::U16String::from_str(s);
        let mut ustring = UNICODE_STRING::default();
        unsafe { RtlInitUnicodeString(&mut ustring, wide_str.as_ptr()) };
        ustring
    }
}

impl NtToString for PUNICODE_STRING {
    fn to_string(&self) -> alloc::string::String {
        let ustring = unsafe { **self };
        ustring.to_string()
    }
}
