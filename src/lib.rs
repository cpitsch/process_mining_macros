#![doc = include_str!("../README.md")]

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
macro_rules! attribute {
    ($key:expr => $val:expr) => {
        process_mining::event_log::Attribute::new(
            $key.into(),
            process_mining::event_log::AttributeValue::from($val),
        )
    };
    ($key:expr, $val:expr) => {
        $crate::attribute!($key => $val)
    };
}

#[macro_export]
macro_rules! attributes {
    ($($key:expr => $value:expr),*) => {
        vec![
            $(
                $crate::attribute!($key => $value)
            ),*
        ]
    };
}

#[macro_export]
/// Create an [process_mining::event_log::Event] without adding any automatic attributes.
///
/// # Examples
///
/// ```rust
/// use process_mining_macros::_event;
///
/// _event!("a"); // Creates an event with activity "more complicated name"
/// // Create an event with the current time as timestamp
/// _event!("a"; {
///     "time:timestamp" => chrono::Utc::now()
/// });
/// // Create an event with timestamp 0
/// _event!("a"; {
///     "time:timestamp" => chrono::DateTime::UNIX_EPOCH
/// });
///
/// use chrono::{DateTime, FixedOffset};
/// let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
/// // Create an event with a custom [chrono::Datetime] as timestamp
/// _event!("a"; {
///     "time:timestamp" => dt
/// });
/// ```
macro_rules! _event {
    ($name:expr) => {
        $crate::_event!($name; {})
    };
    ($name:expr; { $($key:expr => $value:expr),* $(,)? }) => {
        process_mining::event_log::Event {
            attributes: vec![
                $crate::attribute!("concept:name" => $name),
                $(
                    $crate::attribute!($key => $value)
                ),*
            ]
        }
    };
}

#[macro_export]
/// Create an [process_mining::event_log::Event].
///
/// # Examples
///
/// ```rust
/// use process_mining_macros::event;
///
/// event!("a"); // Creates an event with activity "more complicated name"
/// // Create an event with the current time as timestamp
/// event!("a"; {
///     "time:timestamp" => chrono::Utc::now()
/// });
/// // Create an event with timestamp 0
/// event!("a"; {
///     "time:timestamp" => chrono::DateTime::UNIX_EPOCH
/// });
///
/// use chrono::{DateTime, FixedOffset};
/// let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
/// // Create an event with a custom [chrono::Datetime] as timestamp
/// event!("a"; {
///     "time:timestamp" => dt
/// });
/// ```
macro_rules! event {
    ($($input:tt)*) => {{
        let mut evt = $crate::_event!($($input)*);

        if process_mining::event_log::XESEditableAttribute::get_by_key(
            &evt.attributes,
            "time:timestamp",
        )
        .is_none()
        {
            process_mining::event_log::XESEditableAttribute::add_attribute(
                &mut evt.attributes,
                $crate::attribute!(
                    "time:timestamp" => chrono::DateTime::UNIX_EPOCH
                )
            )
        }
        evt
    }};
}

#[macro_export]
macro_rules! trace {
    (
        { $($key:expr => $value:expr),* $(,)? };
        $(
            $activity:expr $(; { $($keys:expr => $values:expr),* $(,)?})?
        ),*
    ) => {{
        let mut trace = process_mining::event_log::Trace {
            attributes: $crate::attributes!(
                            $($key => $value),*
                        ),
            events: vec![
                $(
                    $crate::_event!($activity; {
                        $(
                            $(
                                $keys => $values
                            ),*
                        )?
                    })
                ),*
            ]
        };

        let delta = chrono::TimeDelta::hours(1);

        // Make sure the first event has a timestamp, then fill with previous timestamp + 1h
        if let Some(evt) = trace.events.first_mut() {
            if process_mining::event_log::XESEditableAttribute::get_by_key(
                &evt.attributes,
                "time:timestamp"
                ).is_none()
            {
                process_mining::event_log::XESEditableAttribute::add_attribute(&mut evt.attributes,
                    $crate::attribute!(
                        "time:timestamp" => chrono::DateTime::UNIX_EPOCH
                    )
                )
            }
        }

        for i in 1..trace.events.len() {

            if process_mining::event_log::XESEditableAttribute::get_by_key(&trace.events[i].attributes, "time:timestamp").is_none() {
                let prev_timestamp = *process_mining::event_log::XESEditableAttribute::get_by_key(&trace.events[i-1].attributes, "time:timestamp").unwrap().value.try_as_date().expect("Timestamp should be a date.");
                process_mining::event_log::XESEditableAttribute::add_attribute(
                    &mut trace.events[i].attributes,
                    $crate::attribute!(
                        "time:timestamp" => prev_timestamp + delta
                    )
                )
            }
        }


        if process_mining::event_log::XESEditableAttribute::get_by_key(&trace.attributes, "concept:name").is_none() {
            process_mining::event_log::XESEditableAttribute::add_attribute(
                &mut trace.attributes,
                $crate::attribute!(
                    "concept:name" => $crate::id_value!()
                )
            )
        }

        trace
    }};
    ($($content:tt)*) => {
        $crate::trace!({}; $($content)*)
    }
}

#[macro_export]
/// # Examples
/// ```rust
/// use process_mining_macros::event_log;
///
/// event_log!({};
///     ["a", "b", "c", "d"],
///     ["a", "c", "b", "d"],
/// );
///
///
/// event_log!(
///     ["a", "b", "c", "d"],
///     ["a", "c", "b", "d"],
/// );
///
/// event_log!(
///     ["a", "b"] { "key1" => "value1", "key2" => "value2" },
///     ["c"] { "key3" => "value3" },
///     ["d"],
/// );
///
/// event_log!({};
///    ["a", "b", "c", "d"]
/// );
///
/// ```
macro_rules! event_log {
    (
        $({ $($key:expr => $value:expr),* $(,)? };)?
        $(
            [$($events:tt)*] $({ $($keys:expr => $vals:expr),* $(,)? })?
        ),* $(,)?
     ) => {
         process_mining::event_log::EventLog {
             attributes: $crate::attributes!(
                             $(
                                 $($key => $value),*
                             )?
                         ),
            traces: vec![
                $(
                    $crate::trace!(
                        $({ $($keys => $vals),*};)?
                        $($events)*
                    )

                ),*
            ],
            extensions: None,
            classifiers: None,
            global_trace_attrs: None,
            global_event_attrs: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
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
        let event_1 = event!("a");
        let event_2 = event!("name with spaces");
        let third_activity = String::from("hello, world!");
        let event_3 = event!(third_activity);

        assert_eq!(event_to_activity(&event_1), "a");
        assert_eq!(event_to_activity(&event_2), "name with spaces");
        assert_eq!(event_to_activity(&event_3), "hello, world!");
    }

    #[test]
    fn timed_event() {
        let event_1 = event!("a"; {"time:timestamp" => Utc::now()});
        let event_2 = event!("a"; {"time:timestamp"=> DateTime::UNIX_EPOCH});
        let timestamp = chrono::Utc::now().fixed_offset();
        let event_3 = event!("a"; {"time:timestamp"=> timestamp});

        // Some time _will_ have passed since the "now" timestamp was computed, so allow
        // at most 1s difference (very pessimistic)
        assert!(
            chrono::Utc::now().fixed_offset() - event_to_timestamp(&event_1)
                < chrono::TimeDelta::seconds(1)
        );
        assert_eq!(event_to_timestamp(&event_2), DateTime::UNIX_EPOCH);
        assert_eq!(event_to_timestamp(&event_3), timestamp);
    }

    #[test]
    fn event_attributes() {
        // Just try to see if it compiles
        let _event = event!("a"; {
            "string_attr" => String::from("Wee"),
            "str_attr" => "asd",
            "date_attr" => chrono::Utc::now(),
            "int_attr" => 5,
            "float_attr" => 3.7,
            "bool_attr" => true,
            "id" => uuid::Uuid::new_v4(),
            "list" => vec![],
        });
    }

    #[test]
    fn simple_trace() {
        let trace = trace!("a", "b", "c", "d");
        let expected = vec!["a", "b", "c", "d"];

        assert_eq!(trace_to_activities(&trace), expected);
    }

    #[test]
    fn empty_trace() {
        let empty_trace = trace!();
        assert!(empty_trace.events.is_empty());
    }

    #[test]
    fn timed_trace() {
        // Pass in expression
        let trace_1 = trace!("a"; {"time:timestamp" => DateTime::UNIX_EPOCH}, "b", "c", "d");
        let epoch = DateTime::from_timestamp_nanos(0);
        assert_eq!(
            trace_to_timestamps(&trace_1),
            vec![
                epoch,
                epoch + TimeDelta::hours(1),
                epoch + TimeDelta::hours(2),
                epoch + TimeDelta::hours(3),
            ]
        );

        // Pass in identifier
        let timestamp = chrono::Utc::now();
        let trace_2 = trace!("a"; {"time:timestamp" => timestamp},"b","c","d");
        assert_eq!(
            trace_to_timestamps(&trace_2),
            vec![
                timestamp,
                timestamp + TimeDelta::hours(1),
                timestamp + TimeDelta::hours(2),
                timestamp + TimeDelta::hours(3),
            ]
        );
    }

    #[test]
    fn trace_attributes() {
        // Just try to see if it compiles - trace attributes and events in trace with attributes
        let _trace = trace!({
            "string_attr" => String::from("Wee"),
            "str_attr" => "asd",
            "date_attr" => chrono::Utc::now(),
            "int_attr" => 5,
            "float_attr" => 3.7,
            "bool_attr" => true,
            "id" => uuid::Uuid::new_v4(),
            "list" => vec![],
        }; "a", "b"; {
            "string_attr" => String::from("Wee"),
            "str_attr" => "asd",
            "date_attr" => chrono::Utc::now(),
            "int_attr" => 5,
            "float_attr" => 3.7,
            "bool_attr" => true,
            "id" => uuid::Uuid::new_v4(),
            "list" => vec![],
        });
    }

    #[test]
    fn simple_log() {
        let log = event_log!(
            ["a", "b", "c", "d"],
            ["a", "c", "b", "d"],
            ["names that", "have spaces"]
        );

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
    }

    #[test]
    fn event_log_attributes() {
        // Event log with attributes, trace with attributes, and event with attributes
        let _log = event_log!(
        {
            "string_attr" => String::from("Wee"),
            "str_attr" => "asd",
            "date_attr" => chrono::Utc::now(),
            "int_attr" => 5,
            "float_attr" => 3.7,
            "bool_attr" => true,
            "id" => uuid::Uuid::new_v4(),
            "list" => vec![],
        };
        ["a", "b"; {
            "string_attr" => String::from("Wee"),
            "str_attr" => "asd",
            "date_attr" => chrono::Utc::now(),
            "int_attr" => 5,
            "float_attr" => 3.7,
            "bool_attr" => true,
            "id" => uuid::Uuid::new_v4(),
            "list" => vec![],
        }, "c", "d"] {
            "string_attr" => String::from("Wee"),
            "str_attr" => "asd",
            "date_attr" => chrono::Utc::now(),
            "int_attr" => 5,
            "float_attr" => 3.7,
            "bool_attr" => true,
            "id" => uuid::Uuid::new_v4(),
            "list" => vec![],
        },
        ["no", "attributes", "in", "this", "trace"],
        );
    }
}
