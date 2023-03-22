//TODO: I think we can use trait to achieve this. (later it cover)
pub fn parse_buffer_to_string(buffer: &[u8]) -> String {
    let mut bytes = Vec::new();

    for i in buffer {
        if *i == 0 {
            break;
        }
        bytes.push(*i);
    }

    String::from_utf8(bytes).unwrap()
}
