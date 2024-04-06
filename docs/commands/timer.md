## Timer Commands
- **Description**: This command handles various subcommands related to timers.
- **Usage**:
  - `timer set`: Sets a new timer.
  - `timer list`: Lists all existing timers.
  - `timer delete`: Deletes a specific timer.
  - `timer help`: Provides help information.

### Set Command
- **Description**: Sets a new timer based on the provided duration and unit.
- **Parameters**:
  - `description`: Describes the purpose of the timer.
  - `unit`: Specifies the unit of time (s for seconds, m for minutes, h for hours).
  - `number`: Indicates the duration of the timer.
- **Usage Example**:

```timer set "Study Session" m 30```

### List Command
- **Description**: Lists all existing timers set by the user.
- **Usage**:

```timer list```


### Delete Command
- **Description**: Deletes a specific timer identified by its ID.
- **Parameters**:
- `data_id`: ID of the timer to delete.
- **Usage Example**:

```timer delete 2```


### Help Command
- **Description**: Provides help information about command usage .
- **Usage**:

```timer help```


## Additional Information
- The commands utilize the Poise framework for Discord bot development in Rust.
- Timers are stored and managed in a database (timer.db).
- Each command handles errors and provides appropriate feedback to the user.
