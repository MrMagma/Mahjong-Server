# API

Messages are ASCII text in the form `<message type>: <message content>`

## Send

| Type | Content |
|---|---|
| ID | The unique ID of the game session the client either created or joined. Should be the first (non-error) message sent to the client in any connection |
| Error | A description of the error that ocurred. Sent when the server is unable to add the client to a session or an internal error affecting the client occurs. |

## Receive

Server currently just echoes back any messages it receives.