
use std::sync::{Mutex, Arc};
use std::io::Cursor;
use prost::Message;
mod snazzy_items;
use snazzy_items::Shirt;
use snazzy_items::shirt::Size;

pub fn create_large_shirt(color: String) -> Shirt {
    let mut shirt = Shirt::default();
    shirt.color = color;
    shirt.set_size(Size::Large);
    shirt
}

pub fn serialize_shirt(shirt: &Shirt) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(shirt.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    shirt.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_shirt(buf: &[u8]) -> Result<Shirt, prost::DecodeError> {
    Shirt::decode(&mut Cursor::new(buf))
}


pub fn main() {

    let c = Arc::new(Mutex::new(0));
    let mut h = vec![];
    for i in 0..10 {
        let c = Arc::clone(&c);
        h.push(std::thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
            println!("num: {} (by thread {})", *num, i);
        }));
    }
    for j in h {
        j.join();
    }

    println!("proto bytes: {:?}", snazzy_items::PROTO);
}
