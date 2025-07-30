This crate adds some useful macros for the `process_mining` crate. Specifically, it provides macros to easily create simple event logs, for instance, for creating test cases.


# Examples

## Attributes

- Attributes are created as a key-value mapping:

```rust
use process_mining_macros::attribute;

attribute!("count" => 5);
attribute!("concept:name" => "Approve");
attribute!("approved" => true);
```

- A collection of attributes (`process_mining::event_log::Attributes`) is created
  as a comma-separated sequence of key-value mappings:

```rust
use process_mining_macros::attributes;

attributes!(
  "count" => 5,
  "concept:name" => "Approve",
  "approved" => true
);
```

## Events

- Events are created based on an activity name and other optional attributes. 
  - To provide attributes, the activity name is followed by curly braces, separated
    by a semicolon `;`. The braces contain comma-separated `key => value` mappings.
- By default, the unix epoch is used as the timestamp.

```rust
use process_mining_macros::event;
use chrono::{DateTime, FixedOffset, Utc};

event!("a"); // Create an event with activity "a" and default timestamp 0
event!("a"; {"time:timestamp" => DateTime::UNIX_EPOCH}); // Create an event with timestamp 0
event!("a"; {"time:timestamp" => Utc::now()}); // Create an event with the current time as timestamp
event!("a"; {"cost" => 2.75, "org:resource" => "John"}); // Create an event with custom attributes

// Use a custom timestamp
let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
event!("a"; {"time:timestamp" => dt}); // Create an event with a custom [chrono::Datetime] as timestamp
```

## Traces

- Traces are a sequence of events, indicated by their activity.
  - The macro invoction may optionally begin with an attribute definition, indicated by
    curly braces containing comma-separated `key => value` mappings.
  - Events may be given attributes following the syntax of the `event` macro.
- Events with no timestamp set are placed 1 hour after the previous events' `time:timestamp`

```rust
use process_mining_macros::trace;
use chrono::{DateTime, FixedOffset, Utc};

trace!("a","b","c","d"); // Creates a trace with events with activities "a", "b", "c", and "d"
trace!("a"; {"time:timestamp" => Utc::now()},"b","c","d"); // Use the current timestamp as the base timestamp of the trace
trace!("a"; {"time:timestamp" => DateTime::UNIX_EPOCH},"b","c","d"); // Use unix epoch 0 as the base timestamp of the trace of the trace

// Use a custom base timestamp
let dt: DateTime<FixedOffset> = "2025-01-01T00:00:00+02:00".parse().unwrap();
trace!("a"; {"time:timestamp" => dt},"b","c","d"); // Use a custom [chrono::Datetime] as the base timestamp
```

## Event Logs

- Event logs consist of traces, indicated by brackets `[]`.
  - The macro invocation may optionally begin with an attribute definition, indicated by
    curly braces containing comma-separated `key => value` mappings.
  - Traces (`[]`) may also optionally be followed by an attribute definition.
  - Events can also be given attributes, following the same syntax as the `event`
    macro


```rust
use process_mining_macros::event_log;
use chrono::Utc;

// Create an event log with two traces
event_log!(["a","b","c","d"], ["a","c","b","d"]);
// Create an event log where all traces start at the current timestamp
event_log!(
  ["a"; {"time:timestamp" => Utc::now()},"b","c","d"],
  ["a"; {"time:timestamp" => Utc::now()},"c","b","d"],
);

// Create an event log with trace attributes
event_log!(
  ["a","b","c","d"] {"concept:name" => "CustomId001"},
  ["a","c","b","d"] {"concept:name" => "CustomId002", "outcome" => "approved"},
);
```
