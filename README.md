This crate adds some useful macros for the `process_mining` crate. Specifically, it provides macros to easily create simple event logs, for instance, for creating test cases.


# Examples

## Events

- Events are created based on an activity name and other optional attributes. 
  By default, the unix epoch is used as the timestamp.

```rust
use process_mining_macros::event;

event!("a"); // Creates an event with activity "a"
event!("a"; {"time:timestamp" => EPOCH}); // Create an event with timestamp 0
event!("a"; {"time:timestamp" => NOW}); // Create an event with the current time as timestamp

// Use a custom timestamp
use chrono::{DateTime, FixedOffset};
let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
event!("a"; {"time:timestamp" => dt}); // Create an event with a custom [chrono::Datetime] as timestamp
```

## Traces

- Traces are a sequence of events, indicated by their activity.
- Events with no timestamp set are placed 1 hour after the previous events' `time:timestamp`

```rust
use process_mining_macros::trace;

trace!("a","b","c","d"); // Creates a trace with events with activities "a", "b", "c", and "d"
trace!("a"; {"time:timestamp" => NOW},"b","c","d"); // Use the current timestamp as the base timestamp of the trace
trace!("a"; {"time:timestamp" => EPOCH},"b","c","d"); // Use Epoch 0 as the base timestamp of the trace of the trace

// Use a custom base timestamp
use chrono::{DateTime, FixedOffset};
let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
trace!("a"; {"time:timestamp" => dt},"b","c","d"); // Use a custom [chrono::Datetime] as the base timestamp
```

## Event Logs

- Event logs are built by traces, indicated in brackets `[]`.


```rust
use process_mining_macros::event_log;

// Create an event log with two traces
event_log!(["a","b","c","d"], ["a","c","b","d"]);
// Create an event log where all traces start at the current timestamp
event_log!(
  ["a"; {"time:timestamp" => NOW},"b","c","d"],
  ["a"; {"time:timestamp" => NOW},"c","b","d"],
);

// Create an event log with trace attributes
event_log!(
  ["a","b","c","d"] {"concept:name" => "CustomId001"},
  ["a","c","b","d"] {"concept:name" => "CustomId002", "outcome" => "approved"},
);
```
