//! Error handling functionality.

error_chain! {
    errors {
        UnexpectedEof {
            description("Unexpected end of file")
        }

        ElementNotFound(el: u64) {
            description("Required element not found"),
            display("Required element '0x{:X}' not found", el),
        }

        InvalidFloatSize {
            description("Invalid float size (expected 32 or 64 bits)")
        }
    }

    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
    }
}
