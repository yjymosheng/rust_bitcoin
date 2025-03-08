use bincode;

pub fn serialize<T>(value: &T) -> Vec<u8>
where
    T: bincode::Encode + ?Sized,
{
    //区别bincode 慢且空间大
    // serde_json::to_vec(value).expect("Failed to serialize")
    let config = bincode::config::standard();
    bincode::encode_to_vec(value, config).unwrap()
}

#[macro_export]
macro_rules! hash_str {
    ($($arg: expr), * ) => {{
            let mut hasher = sha2::Sha256::new();
            $(Digest::update(&mut hasher,& serialize($arg)); )*
            format!("{:x}", hasher.finalize())
    }};
}
