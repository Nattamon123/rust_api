use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::infrastructure::postgres::schema::quests;
#[derive(Debug, Clone,Identifiable,Queryable,Selectable)]
#[diesel(table_name = quests)]
pub struct QuestEntity {
    pub id : i32,
    pub name : String,
    pub description : Option<String>,
    pub status : String,
    pub guild_commander_id : i32,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime
}