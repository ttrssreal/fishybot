import sqlite3, requests

# This script is for taking the database from a player -> ign model to a player -> uuid one.
# If it encounters a username that has been changed it will delete it from the database and
# the user will have to update manually with the bot.

uuid_api = "https://api.mojang.com/users/profiles/minecraft/"

conn = sqlite3.connect("../../database/database")

require_update = []

def get_uuid(ign):
    return requests.get(uuid_api + ign).json()["id"]

for i in conn.execute("SELECT * FROM links").fetchall():
    try:
        conn.execute("UPDATE links SET uuid=? WHERE id=?", (get_uuid(i[2]), i[0]))
        print(conn.execute("SELECT * FROM links WHERE id=?", (i[0],)).fetchall())
    except Exception as e:
        print(i, "changed username:", i[2])
        try:
            conn.execute("DELETE FROM links WHERE id=?", (i[0],))
            print("Removed:", i[2])
            require_update.append(i[2])
        except Exception as e:
            print(i, "couldn't rm:", i[2])

conn.commit()
conn.close()

print("Done.")
print("\nThe following users need to update:")
print(require_update)