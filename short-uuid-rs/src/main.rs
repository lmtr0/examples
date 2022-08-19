use sodiumoxide::base64;
use uuid::{Uuid};

fn main() {


    for i in 0..10 {
        let id = Uuid::new_v4();
    
        let simple = id.as_simple().to_string();
        let by = id.to_bytes_le();
    
        let complex = base64::encode(by, base64::Variant::UrlSafeNoPadding);
    
        println!("================================");
        println!("Simple: {simple}");
        println!("Complx: {complex}");
    }

}
