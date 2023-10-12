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

### TODO
- Stop current session upon server shutdown
- Add status formatting
