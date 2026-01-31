#[allow(unused)]
struct Media {
    source: Box<Vec<u8>>,
    media_type: MediaType,
}

#[allow(unused)]
enum MediaType {
    Image,
    Video,
}
