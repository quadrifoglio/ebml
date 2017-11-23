//! Error handling functionality.

error_chain! {
    errors {
        UnexpectedElementId {
            description("Received an unexpected EBML element ID")
        }

        NoChildren {
            description("The EBML element does not contain any child elements")
        }
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
