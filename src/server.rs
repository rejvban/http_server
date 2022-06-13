pub struct Server {
    addr: String,
}

impl Server {
    // Self is an alias to Server
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
    }
}
