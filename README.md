# fishybot
The discord bot uses the Hypixel API to generate an infographic containing a users lobby fishing stats.

This is my first project using rust so be warned the code may not be top tier :)

**Setup**\
Compile with ```cargo build --release```\
Ensure the following enviroment variables are set:
```
DISCORD_TOKEN=...
MOJANG_PROFILE_API_ENDPOINT="https://api.mojang.com/users/profiles/minecraft"
HYPIXEL_PROFILE_API_ENDPOINT="https://api.hypixel.net/player"
HYPIXEL_API_KEY=...
DATABASE_DIR=$PWD/database
TEMPLATE_DIR=$PWD/reasources
SPECIAL="player1 player2 player3"
```
If they are put in a file named ```.env``` in the working directory, then the program will automatically load them.\
To start the discord bot run ```target/release/fishbot```.
