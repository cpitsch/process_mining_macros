// use chrono::Utc;
// use process_mining::event_log::{Attribute, AttributeValue, Event, Trace, XESEditableAttribute};

// pub fn gen_uuid() -> uuid::Uuid {
//     uuid::Uuid::new_v4()
// }

// pub fn create_event(activity: impl Into<String>, offset_s: i64) -> Event {
//     // let mut evt = Event::new(activity.into());
//
//     process_mining::event_log::Event {
//         attributes: vec![
//             process_mining::event_log::Attribute::new(
//                 "time:timestamp".to_string(),
//                 process_mining::event_log::AttributeValue::Date(
//                     chrono::Utc::now().fixed_offset() + chrono::TimeDelta::seconds(offset_s),
//                 ),
//             ),
//             process_mining::event_log::Attribute::new(
//                 "concept:name".to_string(),
//                 process_mining::event_log::AttributeValue::String(activity.into()),
//             ),
//         ],
//     }
//
//     // let mut timestamp = Utc::now().fixed_offset();
//     // timestamp += chrono::TimeDelta::seconds(offset_s);
//     //
//     // evt.attributes.add_attribute(Attribute::new(
//     //     "time:timestamp".to_string(),
//     //     AttributeValue::Date(timestamp),
//     // ));
//     // evt
// }

// pub fn create_trace(activities: Vec<impl Into<String>>, case_id: impl Into<String>) -> Trace {
//     Trace {
//         attributes: vec![Attribute::new(
//             "concept:name".to_string(),
//             AttributeValue::String(case_id.into()),
//         )],
//         events: activities
//             .into_iter()
//             .enumerate()
//             .map(|(idx, act)| create_event(act, idx as i64))
//             .collect(),
//     }
// }

#[macro_export]
macro_rules! event {
    ($name:ident) => {
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
                    process_mining::event_log::AttributeValue::String(
                        stringify!($name).to_string(),
                    ),
                ),
            ],
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($($name:ident),*) => {
        process_mining::event_log::Trace {
            attributes: vec![process_mining::event_log::Attribute::new(
                "concept:name".to_string(),
                process_mining::event_log::AttributeValue::String(uuid::Uuid::new_v4().into()),
            )],
            events: [
                $($crate::event!($name)),*
            ].into_iter().enumerate().map(|(idx, mut evt)| {
                evt.attributes
                    .get_by_key_mut("time:timestamp")
                    .unwrap()
                    .value = process_mining::event_log::AttributeValue::Date(chrono::Utc::now().fixed_offset() + chrono::TimeDelta::hours(idx as i64));
                evt
            }).collect()
            // events: vec![
            //     // TODO: This does not have an offset...
            //     $($crate::event!($name)),*
            // ]
        }
    }
}

#[macro_export]
macro_rules! event_log {
    ($([$($items:tt),*]),*) => {
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
