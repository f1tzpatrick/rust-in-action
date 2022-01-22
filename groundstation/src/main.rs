#![allow(unused_variables)]

// Clone and Copy traits can be Derived for simple Structs 
#[derive(Debug, Clone, Copy)]
enum SatelliteStatus {
    Ok
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>
}

impl Mailbox {
    fn post(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let message = self.messages.remove(i);
                return Some(message);
            }
        }
        None
    }
}


// Clone and Copy can also be implemented manually
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl Copy for CubeSat { } // Empty because CubeSat wrapps an i64, which has Copy

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat { id: self.id }
    }
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat { id }
    }

    fn receive(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

struct GroundStation {}

impl GroundStation{
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(
        &self,
        mailbox: &mut Mailbox,
        message: Message,
    ) {
        mailbox.post(message);
    }
}


fn get_satellite_inventory() -> Vec<u64> {
    vec![1,2,3]
}


fn check_status(satellite: CubeSat) -> SatelliteStatus {
    SatelliteStatus::Ok
}

 
fn main () {
    let mut mail = Mailbox { messages: vec![] };
    let groundstation = GroundStation {};

    let sat_ids: Vec<u64> = get_satellite_inventory();

    for id in sat_ids {
        let satellite = groundstation.connect(id);
        groundstation.send(
            &mut mail, 
            Message {
                to: satellite.id,
                content: format!("SAT-{}, how copy?", id),
            }
        )
    }

    let sat_ids: Vec<u64> = get_satellite_inventory();

    for id in &sat_ids {
        let satellite = groundstation.connect(*id);
        let message = satellite.receive(&mut mail);
        println!("{:?}: {:?}", satellite, message);
    }

    let sat_a = CubeSat::new(sat_ids[0]);
    let sat_b = CubeSat::new(sat_ids[1]);
    let sat_c = CubeSat::new(sat_ids[2]);
    let a_status = check_status(sat_a.clone());
    let b_status = check_status(sat_b.clone());
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
    // Since we cloned sats a & b, those variables can still be used.
    // Since we implemented Copy for CubeSat, sat_c was implicitly copied
    // (it would have been moved otherwise, and then be unavailble to use in the next line)
    println!("{}", sat_c.id);
 
}
