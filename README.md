# fishybot
The discord bot uses the Hypixel API to generate an infographic containing a users lobby fishing stats.

**Setup**\
Compile with ```cargo build --release```\
Ensure the following enviroment variables are set:
```
DISCORD_TOKEN=...
MOJANG_PROFILE_API_ENDPOINT_IGN="https://api.mojang.com/user/profile"
MOJANG_PROFILE_API_ENDPOINT_UUID="https://api.mojang.com/users/profiles/minecraft"
HYPIXEL_PROFILE_API_ENDPOINT="https://api.hypixel.net/player"
HYPIXEL_API_KEY=...
DATABASE_DIR=$PWD/database
TEMPLATE_DIR=$PWD/reasources
SPECIAL="uuid1 uuid2 uuid3"
```
If they are put in a file named ```.env``` in the working directory, then the program will automatically load them.\
To start the discord bot run ```target/release/fishbot```.
