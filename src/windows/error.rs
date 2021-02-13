use super::win32 as win;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Error(win::NonZeroDWORD);

impl Error {
    pub(super) fn last() -> Self {
        unsafe {
            Self(win::NonZeroDWORD::new(win::GetLastError()).unwrap())
        }
    }

    pub(super) fn unknown() -> Self {
        Self(win::NonZeroDWORD::new(28).unwrap()) // The printer is out of paper
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            let message_buffer: win::LPCWSTR = std::ptr::null();

            let message_length = win::FormatMessageW(
                win::FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | win::FORMAT_MESSAGE_FROM_SYSTEM
                    | win::FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null(),
                self.0.get(),
                0,
                std::mem::transmute(&message_buffer),
                0,
                std::ptr::null_mut()
            );

            if message_length == 0 {
                return write!(f, "Error code: {}", self.0.get());
            }

            // Removing CRLF and period.
            let message_length = (message_length - 3) as usize;
            let message = std::slice::from_raw_parts(message_buffer, message_length);
            let result = write!(f, "{}", String::from_utf16_lossy(message));

            win::LocalFree(std::mem::transmute(message_buffer));

            result
        }
    }
}

impl std::error::Error for Error {}