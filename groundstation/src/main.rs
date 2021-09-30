#![allow(unused_variables)]

#[derive(Debug)]
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

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
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


fn check_status(satellite: &CubeSat) -> SatelliteStatus {
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

    for id in sat_ids {
        let satellite = groundstation.connect(id);
        let message = satellite.receive(&mut mail);
        println!("{:?}: {:?}", satellite, message);
    }

    // let mut sat_a = CubeSat {
    //     id: 0,
    //     mailbox: Mailbox {
    //         messages: vec![]
    //     }
    // };



    // println!("t0: {:?}", sat_a);

    // groundstation.send(&mut sat_a, Message::from("SAT-A, how copy?"));

    // println!("t1: {:?}", sat_a);

    // let message = sat_a.receive();

    // println!("t2: {:?}", sat_a);

    // println!("{:?}", message);
//    let a_status = check_status(&sat_a);
//    let b_status = check_status(sat_b);
//    let c_status = check_status(sat_c);
//    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
 
//    // "waiting" ...
//    let a_status = check_status(sat_a);
//    let b_status = check_status(sat_b);
//    let c_status = check_status(sat_c);
//    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
