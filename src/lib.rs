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

/// A utility macro to expand magic identifiers for attribute values.
#[macro_export]
macro_rules! expand_value {
    (NOW) => {
        chrono::Utc::now().fixed_offset()
    };
    (EPOCH) => {
        chrono::DateTime::from_timestamp_millis(0)
            .unwrap()
            .fixed_offset()
    };
    ($val:expr) => {
        $val
    };
}

#[macro_export]
/// Create an [process_mining::event_log::Event] without adding any automatic attributes
///
/// # Examples
///
/// ```rust
/// use process_mining_macros::_event;
///
/// _event!("a"); // Creates an event with activity "more complicated name"
/// // Create an event with the current time as timestamp
/// _event!("a"; {
///     "time:timestamp" => NOW
/// });
/// // Create an event with timestamp 0
/// _event!("a"; {
///     "time:timestamp" => EPOCH
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
    ($name:expr; { $($key:expr => $val:tt),* $(,)? }) => {
        process_mining::event_log::Event {
            attributes: vec![
                process_mining::event_log::Attribute::new(
                    "concept:name".to_string(),
                    process_mining::event_log::AttributeValue::String(
                        $name.into()
                    )
                ),
                $(
                    process_mining::event_log::Attribute::new(
                        $key.into(),
                        process_mining::event_log::AttributeValue::from(
                            $crate::expand_value!($val)
                        )
                    )
                ),*
            ],
        }
    };
}

#[macro_export]
/// Create an [process_mining::event_log::Event]
///
/// # Examples
///
/// ```rust
/// use process_mining_macros::event;
///
/// event!("a"); // Creates an event with activity "more complicated name"
/// // Create an event with the current time as timestamp
/// event!("a"; {
///     "time:timestamp" => NOW
/// });
/// // Create an event with timestamp 0
/// event!("a"; {
///     "time:timestamp" => EPOCH
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
                process_mining::event_log::Attribute::new(
                    "time:timestamp".to_string(),
                    process_mining::event_log::AttributeValue::Date($crate::expand_value!(EPOCH)),
                ),
            )
        }
        evt
    }};
}

#[macro_export]
macro_rules! trace {
    (
        { $($key:expr => $val:expr),* $(,)? };
        $(
            $activity:expr $(; { $($keys:expr => $values:tt),* $(,)?})?
        ),*
    ) => {{
        let mut trace = process_mining::event_log::Trace {
            attributes: vec![
                $(
                    process_mining::event_log::Attribute::new(
                        $key.into(),
                        process_mining::event_log::AttributeValue::from(
                            $crate::expand_value!($val)
                        )
                    )
                ),*
            ],
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
                    process_mining::event_log::Attribute::new(
                        "time:timestamp".to_string(),
                        process_mining::event_log::AttributeValue::from(
                            $crate::expand_value!(EPOCH)
                        )
                    )
                )
            }
        }

        for i in 1..trace.events.len() {

            if process_mining::event_log::XESEditableAttribute::get_by_key(&trace.events[i].attributes, "time:timestamp").is_none() {
                let prev_timestamp = *process_mining::event_log::XESEditableAttribute::get_by_key(&trace.events[i-1].attributes, "time:timestamp").unwrap().value.try_as_date().expect("Timestamp should be a date.");
                process_mining::event_log::XESEditableAttribute::add_attribute(
                    &mut trace.events[i].attributes,
                    process_mining::event_log::Attribute::new(
                        "time:timestamp".to_string(),
                        process_mining::event_log::AttributeValue::from(
                            prev_timestamp + delta,
                        )
                    )
                )
            }
        }


        if process_mining::event_log::XESEditableAttribute::get_by_key(&trace.attributes, "concept:name").is_none() {
            process_mining::event_log::XESEditableAttribute::add_attribute(
                &mut trace.attributes,
                process_mining::event_log::Attribute::new(
                    "concept:name".to_string(),
                    $crate::id_value!()
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
/// //     ["a","b","c","d"] {"org:resource" => "Cameron"},
/// event_log!({};
///    ["a", "b", "c", "d"]
/// );
///
/// ```
macro_rules! event_log {
    (
        $({ $($key:expr => $val:expr),* $(,)? };)?
        $(
            [$($events:tt)*] $({ $($keys:expr => $vals:expr),* $(,)? })?
        ),* $(,)?
     ) => {
         process_mining::event_log::EventLog {
            attributes: vec![
                $(
                    $(
                        process_mining::event_log::Attribute::new(
                            $key.into(),
                            process_mining::event_log::AttributeValue::from(
                                $crate::expand_value!($val)
                            )
                        )
                    ),*
                )?
            ],
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
        let event_1 = event!("a"; {"time:timestamp" => NOW});
        let event_2 = event!("a"; {"time:timestamp"=> EPOCH});
        let timestamp = chrono::Utc::now().fixed_offset();
        let event_3 = event!("a"; {"time:timestamp"=> timestamp});

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
        let trace_1 = trace!("a"; {"time:timestamp" => EPOCH}, "b", "c", "d");
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
        let trace_2 = trace!("a"; {"time:timestamp" => timestamp},"b","c","d");
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
}
