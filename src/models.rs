use crate::schema::links;

#[derive(Insertable)]
#[table_name = "links"]
pub struct MakeLink<'a> {
    pub discord_tag: &'a str,
    pub ign: &'a str
}

#[derive(Queryable, AsChangeset, Debug)]
pub struct Link {
    pub id: i32,
    pub discord_tag: String,
    pub ign: String
}