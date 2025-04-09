#[cfg(feature = "uuid")]
#[macro_export]
macro_rules! id_value {
    () => {
        process_mining::event_log::AttributeValue::ID(uuid::Uuid::new_v4())
    };
}

#[cfg(not(feature = "uuid"))]
#[macro_export]
macro_rules! id_value {
    () => {
        process_mining::event_log::AttributeValue::Int(0)
    };
}

#[macro_export]
/// Create an [process_mining::event_log::Event]
///
/// # Examples
///
/// ```rust,ignore
/// use process_mining_macros::event;
/// event!(a); // Creates an event with activity "a"
/// event!("more complicated name"); // Creates an event with activity "more complicated name"
/// event!(a; timestamp=expr); // Create an event with a custom [chrono::Datetime] as timestamp
/// event!(a; timestamp=NOW); // Create an event with the current time as timestamp
/// event!(a; timestamp=EPOCH); // Create an event with timestamp 0
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
    // Default to timestamp 0 for easier comparisons for tests
    ($name:expr) => {
        $crate::event!($name; timestamp=EPOCH)
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
/// ```rust,ignore
/// use process_mining_macros::trace;
/// trace!(a,b,c,d); // Creates a trace with events with activities "a", "b", "c", and "d"
/// trace!(a,b,c,d; base_timestamp=expr); // Use a custom [chrono::Datetime] as the base timestamp
/// trace!(a,b,c,d; base_timestamp=NOW); // Use the current timestamp as the base timestamp of the trace
/// trace!(a,b,c,d; base_timestamp=EPOCH); // Use Epoch 0 as the base timestamp of the trace
/// of the trace
///
/// ````
/// `
macro_rules! trace {
    // Temporary (?) fix to make empty trace `trace!()` possible. Note that this does not
    // allow for constructs such as `trace!(; base_timestamp=NOW)`.
    () => {
        process_mining::event_log::Trace {
            attributes: vec![process_mining::event_log::Attribute::new(
                "concept:name".to_string(),
                $crate::id_value!()
            )],
            events: vec![]
        }

    };
    // Default to EPOCH 0 as the base timestamp for easier comparability in tests
    // (which is the main use-case of these macros)
    ($($name:tt),*) => {
        $crate::trace!($($name),*; base_timestamp=EPOCH)
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
                $crate::id_value!()
            )],
            // TODO: Ideally, instead of setting the timestamp retroactively, it would be passed
            // straight into the macro, but we have no way of knowing the total number of generated
            // events
            // Kind of like: [$($crate::event!($name; timestamp=$base + chrono::TimeDelta::hours($index))),*]
            // For now, initially use EPOCH since it is likely faster than getting the current time
            events: [
                $($crate::event!($name; timestamp=EPOCH)),*
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
/// ```rust,ignore
/// use process_mining_macros::event_log;
/// // Create an event log with two traces
/// event_log!([a,b,c,d], [a,c,b,d]);
/// // Create an event log where all traces start at a custom timestamp
/// event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=expr);
/// // Create an event log where all traces start at the current timestamp
/// event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=NOW);
/// // Create an event log where all traces start at timestamp 0
/// event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=EPOCH);
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
mod tests {
    use chrono::{DateTime, FixedOffset};
    use process_mining::{
        event_log::{Event, Trace, XESEditableAttribute},
        EventLog,
    };

    fn event_to_activity(event: &Event) -> &str {
        event
            .attributes
            .get_by_key("concept:name")
            .unwrap()
            .value
            .try_as_string()
            .unwrap()
            .as_str()
    }

    fn event_to_timestamp(event: &Event) -> DateTime<FixedOffset> {
        *event
            .attributes
            .get_by_key("time:timestamp")
            .unwrap()
            .value
            .try_as_date()
            .unwrap()
    }

    fn trace_to_activities(trace: &Trace) -> Vec<&str> {
        trace.events.iter().map(event_to_activity).collect()
    }

    fn trace_to_timestamps(trace: &Trace) -> Vec<DateTime<FixedOffset>> {
        trace.events.iter().map(event_to_timestamp).collect()
    }

    fn log_to_activities(event_log: &EventLog) -> Vec<Vec<&str>> {
        event_log.traces.iter().map(trace_to_activities).collect()
    }

    #[test]
    fn simple_event() {
        let event_1 = event!(a);
        let event_2 = event!("name with spaces");

        assert_eq!(event_to_activity(&event_1), "a");
        assert_eq!(event_to_activity(&event_2), "name with spaces");
    }

    #[test]
    fn timed_event() {
        let event_1 = event!(a; timestamp=NOW);
        let event_2 = event!(a; timestamp=EPOCH);
        let timestamp = chrono::Utc::now().fixed_offset();
        let event_3 = event!(a; timestamp=timestamp);

        // Some time _will_ have passed since the "now" timestamp was computed, so allow
        // at most 1s difference (very pessimistic)
        assert!(
            chrono::Utc::now().fixed_offset() - event_to_timestamp(&event_1)
                < chrono::TimeDelta::seconds(1)
        );
        assert_eq!(
            event_to_timestamp(&event_2),
            chrono::DateTime::from_timestamp_nanos(0)
        );
        assert_eq!(event_to_timestamp(&event_3), timestamp);

        // Check the same thing with "name with spaces"
        let event_1 = event!("name with spaces"; timestamp=NOW);
        let event_2 = event!("name with spaces"; timestamp=EPOCH);
        let timestamp = chrono::Utc::now().fixed_offset();
        let event_3 = event!("name with spaces"; timestamp=timestamp);

        // Some time _will_ have passed since the "now" timestamp was computed, so allow
        // at most 1s difference (very pessimistic)
        assert!(
            chrono::Utc::now().fixed_offset() - event_to_timestamp(&event_1)
                < chrono::TimeDelta::seconds(1)
        );
        assert_eq!(
            event_to_timestamp(&event_2),
            chrono::DateTime::from_timestamp_nanos(0)
        );
        assert_eq!(event_to_timestamp(&event_3), timestamp);
    }

    #[test]
    fn simple_trace() {
        let trace = trace!(a, b, c, d);
        let expected = vec!["a", "b", "c", "d"];

        assert_eq!(trace_to_activities(&trace), expected);
    }

    #[test]
    fn empty_trace() {
        let empty_trace = trace!();
        assert!(empty_trace.events.is_empty());
        // let empty_trace = trace!(; base_timestamp=chrono::Utc::now().fixed_offset());
    }

    #[test]
    fn timed_trace() {
        let trace_1 = trace!(a,b,c,d; base_timestamp=EPOCH);
        let epoch = chrono::DateTime::from_timestamp_nanos(0);
        assert_eq!(
            trace_to_timestamps(&trace_1),
            vec![
                epoch,
                epoch + chrono::TimeDelta::hours(1),
                epoch + chrono::TimeDelta::hours(2),
                epoch + chrono::TimeDelta::hours(3),
            ]
        );

        let timestamp = chrono::Utc::now().fixed_offset();
        let trace_2 = trace!(a,b,c,d; base_timestamp=timestamp);
        assert_eq!(
            trace_to_timestamps(&trace_2),
            vec![
                timestamp,
                timestamp + chrono::TimeDelta::hours(1),
                timestamp + chrono::TimeDelta::hours(2),
                timestamp + chrono::TimeDelta::hours(3),
            ]
        );
    }

    #[test]
    fn simple_log() {
        let log = event_log!([a, b, c, d], [a, c, b, d], ["names that", "have spaces"]);

        assert_eq!(
            log_to_activities(&log),
            vec![
                vec!["a", "b", "c", "d"],
                vec!["a", "c", "b", "d"],
                vec!["names that", "have spaces"],
            ]
        );
    }

    #[test]
    fn empty_log() {
        assert!(event_log!().traces.is_empty());
        assert!(event_log!(; base_timestamp=NOW).traces.is_empty());
        assert!(event_log!(; base_timestamp=EPOCH).traces.is_empty());
        assert!(
            event_log!(; base_timestamp=chrono::Utc::now().fixed_offset())
                .traces
                .is_empty()
        );
    }

    #[test]
    /// Test that passing timestamps into event log macros works as expected.
    ///
    /// All traces should have this timestamp as their base (starting) timestamp
    fn timed_log() {
        let log_now = event_log!([a, b, c, d], [a, c, b, d], ["names that", "have spaces"]; base_timestamp=NOW);
        let now = chrono::Utc::now().fixed_offset();

        log_now.traces.into_iter().for_each(|trace| {
            assert!(
                now - event_to_timestamp(trace.events.first().unwrap())
                    < chrono::TimeDelta::seconds(1)
            )
        });

        let log_epoch = event_log!([a, b, c, d], [a, c, b, d], ["names that", "have spaces"]; base_timestamp=EPOCH);
        log_epoch.traces.into_iter().all(|trace| {
            event_to_timestamp(trace.events.first().unwrap())
                == chrono::DateTime::from_timestamp_nanos(0).fixed_offset()
        });

        let timestamp = chrono::DateTime::from_timestamp_nanos(1000000000).fixed_offset();
        let log_custom = event_log!([a, b, c, d], [a, c, b, d], ["names that", "have spaces"]; base_timestamp=timestamp);
        log_custom
            .traces
            .into_iter()
            .all(|trace| event_to_timestamp(trace.events.first().unwrap()) == timestamp);
    }
}
