//! Error handling functionality.

error_chain! {
    errors {
        UnexpectedEof {
            description("Unexpected end of file")
        }
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
