mod mirror;

use mirror::MirrorServer;
use std::time::Duration;


/// A duration of two seconds
const DURATION_2S: Duration = Duration::from_secs(2);


#[test]
fn test_read() {
    // Start the mirror server
    let (mut queue, mut stream) = MirrorServer::spawn();

    // Call select
    let events = libselect::select_read([&stream], DURATION_2S).expect("Failed to call select");
    assert!(events.is_empty(), "Unexpected event on socket: {:#?}", events);
    
    // Reflect a message
    MirrorServer::send_message("Testolope", &mut queue);
    let events = libselect::select_read([&stream], DURATION_2S).expect("Failed to call select");
    
    // Call select and wait for read event
    let event = events.first().expect("Missing expected event on socket");
    assert!(event.has_read());

    // Read the message
    let message = MirrorServer::receive_message(&mut stream);
    assert_eq!(message.as_slice(), b"Testolope");
}
