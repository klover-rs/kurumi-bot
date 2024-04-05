# Configure
- **Description**: This command handles various subcommands related to timers
- **Usage**:
  - `configure set`: set the configurations with parameters
  - `configure get`: get your current configuration
  - `configure upload`: upload a configuration file
  - `configure clear`: delete all of your configurations

### Set Command 
- **Description**: Sets a new configuration with the provided parameters (will ask you to replace the old one if exists)
- **Parameters**:
  - `log_channel`: the channel where your logs of deleted messages will appear
- **Usage Examples**

```configure set <#1202694320986259457>```

### Get Command
- **Description**: List your server configurations
- **Usage**:

```configure get```

### Clear Command
- **Description**: Clear all of your channel configurations
- **Usage**: 

```configure clear```

### Upload Command
- **Description**: Upload a JSON or TOML file with channel configurations
- **parameters** [attachment]
- **Usage**:
```configure upload [upload your file]```
- **expected format**:
.json
```json
{
  "log_channel_id": "0123456789012345"
}
```
or .toml

```toml
log_channel_id = "0123456789012345"
```

