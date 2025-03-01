#[macro_export]
macro_rules! timestamped_event {
    // Convert identifiers to strings
    // ($name:ident) => {
    ($name:ident$rest:tt) => {
        // $crate::timestamped_event!(stringify!($name))
        $crate::timestamped_event!(stringify!($name)$rest)
    };
    // Default to current time
    ($name:expr) => {
        $crate::timestamped_event!($name; BASE_NOW)
    };
    // Use BASE_EPOCH to use Epoch 0 as the base timestamp
    ($name:expr; BASE_EPOCH) => {
        $crate::timestamped_event!($name; timestamp=chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset())
    };
    // Use BASE_NOW to use the current timestamp as the base timestamp
    ($name:expr; BASE_NOW) => {
        $crate::timestamped_event!($name; timestamp=chrono::Utc::now().fixed_offset())
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
/// Create an [process_mining::event_log::Event]
///
/// # Examples
///
/// ```
/// event!(a) // Creates an event with activity "a"
/// event!("more complicated name") // Creates an event with activity "more complicated name"
/// `
macro_rules! event {
    ($name:ident) => {
        $crate::event!(stringify!($name))
    };
    ($name:expr) => {
        process_mining::event_log::Event {
            attributes: vec![
                process_mining::event_log::Attribute::new(
                    "time:timestamp".to_string(),
                    process_mining::event_log::AttributeValue::Date(
                        chrono::Utc::now().fixed_offset(),
                    ),
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
/// trace!(a,b,c,d; BASE_EPOCH) // Use Epoch 0 as the base timestamp of the trace
/// trace!(a,b,c,d; BASE_NOW) // Use the current timestamp as the base timestamp of the trace
/// trace!(a,b,c,d; base_timestamp=expr) // Use a custom [chrono::Datetime] as the base timestamp
/// of the trace
///
/// ````
/// `
macro_rules! trace {
    // Default to the current timestamp as the base timestamp
    ($($name:tt),*) => {
        $crate::trace!($($name),*; BASE_NOW)
    };
    // Use BASE_EPOCH to use Epoch 0 as the base timestamp
    ($($name:tt),*; BASE_EPOCH) => {
        $crate::trace!($($name),*; base_timestamp=chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset())
    };
    // Use BASE_NOW to use the current timestamp as the base timestamp
    ($($name:tt),*; BASE_NOW) => {
        $crate::trace!($($name),*; base_timestamp=chrono::Utc::now().fixed_offset())
    };
    ($($name:tt),*; base_timestamp=$base:expr) => {
        process_mining::event_log::Trace {
            attributes: vec![process_mining::event_log::Attribute::new(
                "concept:name".to_string(),
                process_mining::event_log::AttributeValue::ID(uuid::Uuid::new_v4()),
            )],
            events: [
                $($crate::event!($name)),*
            ].into_iter().enumerate().map(|(idx, mut evt)| {
                process_mining::event_log::XESEditableAttribute::get_by_key_mut(&mut evt.attributes, "time:timestamp")
                    .unwrap()
                    // Use epoch 0 as base timestamp
                    // .value = process_mining::event_log::AttributeValue::Date(chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset() + chrono::TimeDelta::hours(idx as i64));
                    // TODO: Could probably just use the existing timestamp and add the timedelta
                    .value = process_mining::event_log::AttributeValue::Date($base + chrono::TimeDelta::hours(idx as i64));
                evt
            }).collect()
        }
    };

    // ($($name:tt),*) => {
    //     process_mining::event_log::Trace {
    //         attributes: vec![process_mining::event_log::Attribute::new(
    //             "concept:name".to_string(),
    //             // TODO: Use AttributeValue::Id?
    //             // process_mining::event_log::AttributeValue::ID(uuid::Uuid::new_v4()),
    //             process_mining::event_log::AttributeValue::String(uuid::Uuid::new_v4().into()),
    //         )],
    //         events: [
    //             $($crate::event!($name)),*
    //         ].into_iter().enumerate().map(|(idx, mut evt)| {
    //             evt.attributes
    //                 .get_by_key_mut("time:timestamp")
    //                 .unwrap()
    //                 // Use epoch 0 as base timestamp
    //                 // .value = process_mining::event_log::AttributeValue::Date(chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset() + chrono::TimeDelta::hours(idx as i64));
    //                 // TODO: Could probably just use the existing timestamp and add the timedelta
    //                 .value = process_mining::event_log::AttributeValue::Date(chrono::Utc::now().fixed_offset() + chrono::TimeDelta::hours(idx as i64));
    //             evt
    //         }).collect()
    //     }
    // }
}

#[macro_export]
macro_rules! event_log2 {
    // *$(,)? --> Allow trailing comma
    ($([$($items:tt),*]),*; BASE_EPOCH$(,)?) => {
        $crate::event_log2!($([$($items),*]),*; base_timestamp=chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset())
    };
    ($([$($items:tt),*]),*; BASE_NOW$(,)?) => {
        $crate::event_log2!($([$($items),*]),*; base_timestamp=chrono::Utc::now().fixed_offset())
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

#[macro_export]
macro_rules! event_log {
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
