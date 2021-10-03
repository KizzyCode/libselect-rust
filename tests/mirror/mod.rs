#![allow(unused)]

use std::{
    mem, thread,
    io::{ Read, Write },
    net::{ TcpListener, TcpStream },
    sync::mpsc::{ self, SyncSender, Receiver }
};


/// A message that can be send to the server
pub enum Message {
    /// A data message
    Reflect(Vec<u8>)
}


/// A mirror server
pub struct MirrorServer {
    /// The input channel
    receiver: Receiver<Message>,
    /// The outgoing TCP stream
    stream: TcpStream
}
impl MirrorServer {
    /// Starts a server and returns the input queue and a connected TCP stream
    pub fn new() -> (SyncSender<Message>, TcpStream) {
        // Create the listener
        let mut ports = (10_000..=65_535).into_iter();
        let (listener, port) = 'bind_loop: loop {
            // Iterate over the ports
            let port = ports.next().expect("No free port available to bind to");
            match TcpListener::bind(("127.0.0.1", port)) {
                Ok(listener) => break 'bind_loop (listener, port),
                Err(_) => continue 'bind_loop
            };
        };

        // Create the channels and start the server
        let (sender, receiver) = mpsc::sync_channel(1);
        thread::spawn(move || {
            let stream = listener.accept().expect("Failed to accept connection").0;
            Self { receiver, stream }.runloop()
        });

        // Connect to the socket and return the stream
        let stream = TcpStream::connect(("127.0.0.1", port)).expect("Failed to connect to server");
        (sender, stream)
    }
    /// The server runloop
    fn runloop(mut self) {
        let exit_reason = 'runloop: loop {
            // Get the next message
            let message = match self.receiver.recv() {
                Ok(Message::Reflect(message)) => message,
                Err(_) => break 'runloop "Exited due to orphaned input channel",
            };

            // Process the message
            let length = usize::to_be_bytes(message.len());
            self.stream.write_all(&length).expect("Failed to reflect message length");
            self.stream.write_all(&message).expect("Failed to reflect message");
        };
        println!("{}", exit_reason)
    }

    /// Queue a message for reflection
    pub fn send_message<T>(message: T, sender: &mut SyncSender<Message>) where T: Into<Vec<u8>> {
        sender.send(Message::Reflect(message.into())).expect("Failed to queue message for reflection");
    }
    /// Receive a reflected message from a stream
    pub fn receive_message<T>(mut stream: T) -> Vec<u8> where T: Read {
        // Read the length
        let mut len_buf = [0; mem::size_of::<usize>()];
        stream.read_exact(&mut len_buf).expect("Failed to read the reflected message length");
        let len = usize::from_be_bytes(len_buf);

        // Read the message
        let mut message = vec![0; len];
        stream.read_exact(&mut message).expect("Failed to read the reflected message");
        message
    }
}
