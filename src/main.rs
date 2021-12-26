extern crate local_ip_address;

fn main() {
    println!("Hello, world!");

    println!("My IP is: {}", local_ip_address::local_ip().unwrap());

    println!();

    println!("All IPs associated with this computer:");
    for (name, ip) in local_ip_address::list_afinet_netifas().unwrap() {
        println!("{}:\t{:?}", name, ip);
    }
}
