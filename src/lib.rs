#[macro_export]
macro_rules! event {
    ($name:ident) => {
        event!(stringify!($name))
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
macro_rules! trace {
    ($($name:tt),*) => {
        process_mining::event_log::Trace {
            attributes: vec![process_mining::event_log::Attribute::new(
                "concept:name".to_string(),
                // TODO: Use AttributeValue::Id?
                // process_mining::event_log::AttributeValue::ID(uuid::Uuid::new_v4()),
                process_mining::event_log::AttributeValue::String(uuid::Uuid::new_v4().into()),
            )],
            events: [
                $($crate::event!($name)),*
            ].into_iter().enumerate().map(|(idx, mut evt)| {
                evt.attributes
                    .get_by_key_mut("time:timestamp")
                    .unwrap()
                    // Use epoch 0 as base timestamp
                    // .value = process_mining::event_log::AttributeValue::Date(chrono::DateTime::from_timestamp_millis(0).unwrap().fixed_offset() + chrono::TimeDelta::hours(idx as i64));
                    // TODO: Could probably just use the existing timestamp and add the timedelta
                    .value = process_mining::event_log::AttributeValue::Date(chrono::Utc::now().fixed_offset() + chrono::TimeDelta::hours(idx as i64));
                evt
            }).collect()
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
