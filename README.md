# PingMe

PingMe is a small CLI tool to store messages and send out Desktop notifications.

## Building

To build, run `cargo build` on the root directory.

The project was developed using rustc 1.79.0.

## Installing

To install, run `cargo install --path .`.

## Adding a message

The `pingme add` command adds a message.

The command has 2 parameters:

1. text, positional, required - the text to store
2. notify_after, positional, optional - timedelta in string format

The timedelta is represented in the {N}{T} format, where N is an integer and T is any of the following:

1. s, for seconds
2. m, for minutes
3. h, for hours
4. d, for days

So `1d` translates to 1 day; 10h to 10 hours, and so on.

Example of adding a message without a notification time:
```
pingme add "Hello There!"
```

Example of adding a message for which we want to be notified after 1 day:

```
pingme add "Hello There!" 1d
```

Messages are stored in an SQLite database, which is saved at `$HOME/pingme/db.sqlite`.

## Listing messages

The `pingme list` command list the messages stored in the SQLite database.

The command has 3 optional parameters:

1. last (positional) - timedelta in string format, if given returns any messages added up until `now() - timedelta`
2. -l, --limit - if given, limits the output to N messages, where N is the number provided
3. -o, --offset - if given, offsets the query results by N messages, where N is the number provided

## Deleting messages

The `pingme delete ID` command deletes a message. 

The ID of each message can be found via the `pingme list` command. 

## Daemon

The `pingme daemon` command runs the daemon which continuously performs the following:

1. Query the database for messages whose notification time is in the next N seconds
2. Sends Desktop notifications for these messages
3. Waits for N seconds

