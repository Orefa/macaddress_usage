use mac_address::{self, get_mac_address};
fn main() {
	match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }
	
	match mac_address::MacAddressIterator::new() {
		Ok(iter) => {
			for ma in iter {
				println!("{}", ma);
			}
			
		}
		Err(e) => println!("{:?}", e),
	}
}
