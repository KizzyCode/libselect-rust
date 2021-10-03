use crate::{
    error::Result, io_handle::InOutHandle,
    select_impl::{ self, SelectSet, Events }
};
use std::time::Duration;


/// Performs a `select` call with the given `handles` for the given `events` and returns the handles for which an event has
/// occurred
pub fn select_event<'a, T, TE>(handles: T, events: Events, timeout: Duration) -> Result<Vec<(TE, Events)>>
    where T: IntoIterator<Item = TE>, TE: InOutHandle
{
    // Collect the handles
    let handles: Vec<_> = handles.into_iter().collect();
    
    // Create the select set
    let mut select_set = SelectSet::new();
    for handle in handles.iter() {
        select_set.insert(handle, events)?;
    }
    select_impl::select(&mut select_set, timeout)?;

    // Call select and return the handles where an event occurred
    let mut with_event = Vec::new();
    for handle in handles {
        // Get the events for the handle
        let events = select_set.get(&handle)
            .expect("Failed to get file descriptor from valid handle?!")
            .expect("Missing handle in select set?!");
        if events.has_event() {
            with_event.push((handle, *events))
        }
    }
    Ok(with_event)
}


/// Performs a `select` call with the given `handles` for read- and exception-events and returns the handles for which an
/// event has occurred
pub fn select_read<'a, T, TE>(handles: T, timeout: Duration) -> Result<Vec<(TE, Events)>>
    where T: IntoIterator<Item = TE>, TE: InOutHandle
{
    let events = Events { read: true, write: false, exception: true };
    select_event(handles, events, timeout)
}


/// Performs a `select` call with the given `handles` for write- and exception-events and returns the handles for which an
/// event has occurred
pub fn select_write<'a, T, TE>(handles: T, timeout: Duration) -> Result<Vec<(TE, Events)>>
    where T: IntoIterator<Item = TE>, TE: InOutHandle
{
    let events = Events { read: false, write: true, exception: true };
    select_event(handles, events, timeout)
}


/// Performs a `select` call with the given `handles` for read-, write- and exception-events and returns the handles for
/// which an event has occurred
pub fn select_readwrite<'a, T, TE>(handles: T, timeout: Duration) -> Result<Vec<(TE, Events)>>
    where T: IntoIterator<Item = TE>, TE: InOutHandle
{
    let events = Events { read: true, write: true, exception: true };
    select_event(handles, events, timeout)
}
