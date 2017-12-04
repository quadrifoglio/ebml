//! Error handling functionality.

error_chain! {
    errors {
        UnexpectedEof {
            description("Unexpected end of file")
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
