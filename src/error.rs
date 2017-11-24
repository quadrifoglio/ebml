//! Error handling functionality.

error_chain! {
    errors {
        UnexpectedElementId {
            description("Received an unexpected EBML element ID")
        }

        NoChildren {
            description("The EBML element does not contain any child elements")
        }

        InvalidFloatSize {
            description("Invalid floating point size (was not 32 nor 64 bits)")
        }

        InvalidDataType {
            description("Invalid data type")
        }
    }

    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
    }
}
