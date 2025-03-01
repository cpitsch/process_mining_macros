#[macro_export]
/// Create an [process_mining::event_log::Event]
///
/// # Examples
///
/// ```
/// event!(a) // Creates an event with activity "a"
/// event!("more complicated name") // Creates an event with activity "more complicated name"
/// event!(a; base_timestamp=expr) // Create an event with a custom [chrono::Datetime] as timestamp
/// event!(a; base_timestamp=NOW) // Create an event with the current time as timestamp
/// event!(a; base_timestamp=EPOCH) // Create an event with timestamp 0
/// ```
/// `
macro_rules! event {
    // Convert identifiers to strings
    ($name:ident) => {
        $crate::event!(stringify!($name))
    };
    ($name:ident; $($rest:tt)*) => {
        $crate::event!(stringify!($name); $($rest)*)
    };
    // Default to current time
    ($name:expr) => {
        $crate::event!($name; timestamp=NOW)
    };
    // Use BASE_EPOCH to use Epoch 0 as the base timestamp
    ($name:expr; timestamp=EPOCH) => {
        $crate::event!($name; timestamp=chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset())
    };
    // Use BASE_NOW to use the current timestamp as the base timestamp
    ($name:expr; timestamp=NOW) => {
        $crate::event!($name; timestamp=chrono::Utc::now().fixed_offset())
    };
    ($name:expr; timestamp=$timestamp:expr) => {
        process_mining::event_log::Event {
            attributes: vec![
                process_mining::event_log::Attribute::new(
                    "time:timestamp".to_string(),
                    process_mining::event_log::AttributeValue::Date($timestamp),
                ),
                process_mining::event_log::Attribute::new(
                    "concept:name".to_string(),
                    process_mining::event_log::AttributeValue::String($name.to_string()),
                ),
            ],
        }
    };
}

#[macro_export]
/// Create a [process_mining::event_log::Trace]
///
/// # Examples
///
/// ```
/// trace!(a,b,c,d) // Creates a trace with events with activities "a", "b", "c", and "d"
/// trace!(a,b,c,d; base_timestamp=expr) // Use a custom [chrono::Datetime] as the base timestamp
/// trace!(a,b,c,d; base_timestamp=NOW) // Use the current timestamp as the base timestamp of the trace
/// trace!(a,b,c,d; base_timestamp=EPOCH) // Use Epoch 0 as the base timestamp of the trace
/// of the trace
///
/// ````
/// `
macro_rules! trace {
    // Default to the current timestamp as the base timestamp
    ($($name:tt),*) => {
        $crate::trace!($($name),*; base_timestamp=NOW)
    };
    // Use BASE_EPOCH to use Epoch 0 as the base timestamp
    ($($name:tt),*; base_timestamp=EPOCH) => {
        $crate::trace!($($name),*; base_timestamp=chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset())
    };
    // Use BASE_NOW to use the current timestamp as the base timestamp
    ($($name:tt),*; base_timestamp=NOW) => {
        $crate::trace!($($name),*; base_timestamp=chrono::Utc::now().fixed_offset())
    };
    ($($name:tt),*; base_timestamp=$base:expr) => {
        process_mining::event_log::Trace {
            attributes: vec![process_mining::event_log::Attribute::new(
                "concept:name".to_string(),
                process_mining::event_log::AttributeValue::ID(uuid::Uuid::new_v4()),
            )],
            // TODO: Ideally, instead of setting the timestamp retroactively, it would be passed
            // straight into the macro, but we have no way of knowing the total number of generated
            // events
            events: [
                $($crate::event!($name)),*
            ].into_iter().enumerate().map(|(idx, mut evt)| {
                process_mining::event_log::XESEditableAttribute::get_by_key_mut(&mut evt.attributes, "time:timestamp")
                    .unwrap()
                    .value = process_mining::event_log::AttributeValue::Date($base + chrono::TimeDelta::hours(idx as i64));
                evt
            }).collect()
        }
    };
}

#[macro_export]
/// Create an [process_mining::event_log::EventLog].
///
/// # Examples
///
/// ```
/// // Create an event log with two traces
/// event_log!([a,b,c,d], [a,c,b,d])
/// // Create an event log where all traces start at a custom timestamp
/// event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=expr)
/// // Create an event log where all traces start at the current timestamp
/// event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=NOW)
/// // Create an event log where all traces start at timestamp 0
/// event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=EPOCH)
/// ````
macro_rules! event_log {
    // *$(,)? --> Allow trailing comma
    ($([$($items:tt),*]),*; base_timestamp=EPOCH$(,)?) => {
        $crate::event_log!($([$($items),*]),*; base_timestamp=chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset())
    };
    ($([$($items:tt),*]),*; base_timestamp=NOW$(,)?) => {
        $crate::event_log!($([$($items),*]),*; base_timestamp=chrono::Utc::now().fixed_offset())
    };
    ($([$($items:tt),*]),*; base_timestamp=$base:expr) => {
        process_mining::event_log::EventLog {
            attributes: process_mining::event_log::Attributes::new(),
            traces: vec![
                $(
                    $crate::trace!($($items),*; base_timestamp=$base)
                ),*
            ]
            ,
            extensions: None,
            classifiers: None,
            global_trace_attrs: None,
            global_event_attrs: None,
        }
    };
    // If nothing is specified, just use whatever the default is for traces
    ($([$($items:tt),*]),*$(,)?) => {
        process_mining::event_log::EventLog {
            attributes: process_mining::event_log::Attributes::new(),
            traces: vec![
                $(
                    $crate::trace!($($items),*)
                ),*
            ]
            ,
            extensions: None,
            classifiers: None,
            global_trace_attrs: None,
            global_event_attrs: None,
        }
    }
}

#[cfg(test)]
mod tests {}
