//! Error handling functionality.

error_chain! {
    errors {
        UnexpectedElementId {
            description("Received an unexpected EBML element ID")
        }
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
