This crate adds some useful macros for the `process_mining` crate. Specifically, it provides macros to easily create simple event logs, for instance, for creating test cases.


# Examples

## Events

- Events are created based on an activity name and, optionally, a timestamp. By default,
  the unix epoch is used.

```rust
use process_mining_macros::event;

event!(a); // Creates an event with activity "a"
event!("more complicated name"); // Creates an event with activity "more complicated name"
event!(a; timestamp=EPOCH); // Create an event with timestamp 0
event!(a; timestamp=NOW); // Create an event with the current time as timestamp

// Use a custom timestamp
use chrono::{DateTime, FixedOffset};
let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
event!(a; timestamp=dt); // Create an event with a custom [chrono::Datetime] as timestamp
```

## Traces

- Traces are a sequence of events, indicated by their activity.
- The first event has timestamp `base_timestamp`, and each following event is 1 hour
  later.
    - `base_timestamp` defaults to unix epoch.

```rust
use process_mining_macros::trace;

trace!(a,b,c,d); // Creates a trace with events with activities "a", "b", "c", and "d"
trace!(a,b,c,d; base_timestamp=NOW); // Use the current timestamp as the base timestamp of the trace
trace!(a,b,c,d; base_timestamp=EPOCH); // Use Epoch 0 as the base timestamp of the trace of the trace

// Use a custom base timestamp
use chrono::{DateTime, FixedOffset};
let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
trace!(a,b,c,d; base_timestamp=dt); // Use a custom [chrono::Datetime] as the base timestamp
```

## Event Logs

- Event logs are built by traces, indicated in brackets `[]`.
- All traces start at `base_timestamp`, which defaults to the unix epoch.


```rust
use process_mining_macros::event_log;

// Create an event log with two traces
event_log!([a,b,c,d], [a,c,b,d]);
// Create an event log where all traces start at the current timestamp
event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=NOW);
// Create an event log where all traces start at timestamp 0
event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=EPOCH);

// Use a custom base timestamp
use chrono::{DateTime, FixedOffset};
let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
// Create an event log where all traces start at a custom timestamp
event_log!([a,b,c,d], [a,c,b,d]; base_timestamp=dt);
```
