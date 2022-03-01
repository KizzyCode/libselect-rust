//! Implements a few high-level wrappers around `select`

use crate::{
    error::Result, io::AsInOutHandle,
    select::{ self, Event }
};
use std::time::Duration;


/// Performs a `select` call with the given `handles` for the given `events` and returns the handles for which an event has
/// occurred
fn select_event<'a, I, IT>(handles: I, read: bool, write: bool, exception: bool, timeout: Duration)
    -> Result<Vec<Event<'a>>> where I: IntoIterator<Item = &'a IT>, IT: AsInOutHandle + 'a
{
    // Collect the handles
    let mut event_handles = Vec::new();
    for handle in handles {
        let event_handle = Event::new(handle, read, write, exception)?;
        event_handles.push(event_handle);
    }
    
    // Call select and return the handles where an event occurred
    let event_handles = select::select(event_handles, timeout)?;
    let events = event_handles.into_iter()
        .filter(|e| e.has_event())
        .collect();
    Ok(events)
}


/// Performs a `select` call with the given `handles` for read- and exception-events and returns the handles for which an
/// event has occurred
pub fn select_read<'a, I, IT>(handles: I, timeout: Duration) -> Result<Vec<Event<'a>>>
    where I: IntoIterator<Item = &'a IT>, IT: AsInOutHandle + 'a
{
    select_event(handles, true, false, true, timeout)
}


/// Performs a `select` call with the given `handles` for write- and exception-events and returns the handles for which an
/// event has occurred
pub fn select_write<'a, I, IT>(handles: I, timeout: Duration) -> Result<Vec<Event<'a>>>
    where I: IntoIterator<Item = &'a IT>, IT: AsInOutHandle + 'a
{
    select_event(handles, false, true, true, timeout)
}


/// Performs a `select` call with the given `handles` for read-, write- and exception-events and returns the handles for
/// which an event has occurred
pub fn select_readwrite<'a, I, IT>(handles: I, timeout: Duration) -> Result<Vec<Event<'a>>>
    where I: IntoIterator<Item = &'a IT>, IT: AsInOutHandle + 'a
{
    select_event(handles, true, true, true, timeout)
}
