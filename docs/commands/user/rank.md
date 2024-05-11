# Ranks, hmmm :3
(man i need my meds🙏)

okay so. 

## level roles
first of all you are probably curious, and think like "hmmMmMM, every other bot has level roles, does this one too?" to answer your question, yes it does have level roles, but since this is a bot without a dashboard (yet), we have here a different way of how we configure them

we have in total 2 commands to configure our level roles with 
- `/rank set_level_roles`
- `/rank upload_level_roles`

lets first cover how we set them with the first command

here is a basic example ( things like [string()] represent the type of the slashcommand argument )

```/rank set_level_roles [string(1=1234578901234,5=1234789234789)]```

- the first number before the equal represents the number at which level the user will receive the role
- the number after the equal represents the role_id
- after every new "," you enter your next role

and in case you make a typo somehow in the role id, dont worry, we got you covered with role validation checks, so you basically cant do anything wrong here.

### NOW LETS COVER THE SECOND COMMAND :333 >_-!

aight, so now we have a second option to upload them, through configuration files, at the moment only json and toml are supported! so dont just try to send in a .txt **it has to be either a .json or .toml file with the correct syntax** 

here is a example of how a configuration file might look like

> JSON
```json
{
    "1": "1234789023457892347",
    "5": "1234578901234123543",
    "10": "1234578901234789744"
}
```

> TOML
```toml
1 = "1234789023457892347"

2 = "1234578901234123543"

3 = "1234578901234789744"
```

and yes all of these ids are invalid :) 

okay so how do i use it now ?!?!?! 

simple.... 

use this command `/rank upload_level_roles [string(drop/select json or toml file here)]`

## Oh, uhm uh, but what is this? 

what is what? 
well this red embed saying warning and has 3 buttons?
oh that.
yea basically you have 3 buttons, means 3 choices, a "yes", "no" and "download backup" button. 

lets go through it 

if you have still a level role config in our database, we want to make sure that it just doesnt get lost easily, maybe you used the command accidentally well. in either case, you have either the option to overwrite your current configuration with your new one, cancel the operation by clicking "no" or downloading a backup by clicking "Download Backup".