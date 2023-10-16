# Time Tracker

## Commands
* `server` - start the server
* `start <session-id>` - start a tracking session
* `stop` - stop current session
* `status <session-id>` - info for given session

### Workflow
Start the server
```
$ tt server
```
In another window, start a tracking session:
```
$ tt start gios
```
A session backup will be immediately persisted to a file in this form:
```
gios|2023-10-04T20:49:40.273800872+00:00
```
Status can be checked (output TBD):
```
$ tt status gios
```
After some time has passed, stop the current session:
```
$ tt stop
```
The session will be appended to a log:
```
gios|2023-10-04T20:49:40.273800872+00:00|2023-10-04T21:49:40.273800872+00:00
```
When a session is running, new sessions cannot be started. The current one must be stopped first.

### Data model
Sessions are stored in a CSV with the following fields (timestamps in UTC):
```
session_id | start_timestamp | end_timestamp

Example:
gios,2023-10-04T20:49:40.273800872+00:00,2023-10-04T20:49:40.273800872+00:00
```
This file acts as a log with new sessions being appended. The last record will be the previous session.

### Status
Without an id:
```
$ tt status

Current session: GIOS
Started: 2023-10-04 01:32:41 PM
Duration: 1 hour, 16 minutes
```

With an id:
```
$ tt status aos

AOS stats
    - Number of sessions: 14
    - Weekly average: 15.32 hours
    - Total duration: 62 hours, 12 minutes
```

With --all:
```
$ tt status --all

AOS stats
    - Number of sessions: 14
    - Weekly average: 15.32 hours
    - Total duration: 62 hours, 12 minutes

GIOS stats
    - Number of sessions: 14
    - Weekly average: 15.32 hours
    - Total duration: 62 hours, 12 minutes
```

### TODO
- Add status for specific ID
- Add status for all
- Use Clap for CLI arg parsing

### Deploy
```
$ cargo build -r
$ mv target/release/time_tracker ~/bin
```
