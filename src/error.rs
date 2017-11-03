error_chain!{
    errors {
        IntegerTooBig {
            description("The Integer can not be represented in 64 bits or less")
        }
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
