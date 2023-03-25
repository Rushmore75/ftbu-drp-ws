use std::{env, fmt::Display, io::ErrorKind};

use diesel::{prelude::*, associations::HasTable, result::Error};
use dotenvy::dotenv;

use crate::schema::{teams::dsl::*, users::dsl::*};
#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    username: String,
    team: i32
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::teams)]
pub struct Team {
    id: i32,
    name: String 
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: \"{}\", Team Id: {}", self.username.trim(), self.team)
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Team: \"{}\" with id: {}", self.name, self.id)
    }
}

fn connect_to_db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error Connecting to {}", database_url))
}

fn create_user(conn: &mut PgConnection, user_name: &str, team_id: i32) -> Result<User, Error> {
    
    let new_user= User{username: user_name.to_owned(), team: team_id};

    let result = diesel::insert_into(users::table())
        .values(&new_user)
        .get_result::<(String, Option<i32>)>(conn);
    
    match result {
        Ok(o) => {
            if let Some(i) = o.1 {
                Ok(User {
                    username: o.0,
                    team: i,
                })               
            } else {
                Err(Error::NotFound)
            }
        },
        Err(e) => {
            Err(e)
        }
    } 


}

fn create_team(conn: &mut PgConnection, team_name: &str, team_id: i32) -> Result<Team, Error> {
    let new_team = Team{id: team_id, name: team_name.to_owned()};

    let result = diesel::insert_into(teams::table())
        .values(&new_team)
        .get_result::<(i32, String)>(conn);

    match result {
        Ok(o) => {
            Ok(Team {
                id: o.0,
                name: o.1,
            })
        },
        Err(e) => Err(e),
    }

}

#[test]
pub fn get_teams() {
    let connection = &mut connect_to_db();
    
    let result = teams
        .load::<Team>(connection);
    
    match result {
        Ok(v) => {
            println!("{} records found.", v.len());
        },
        Err(e) => panic!("{}", e), 
    }
}

#[test]
fn insert_team() {
    let connection = &mut connect_to_db();

    match create_team(connection, "team_name", 12) {
        Ok(o) => println!("{}", o),
        Err(e) => panic!("{}", e),
    }
}


#[test]
fn insert_user() {
    let connection = &mut connect_to_db();

    create_team(connection, "team_name", 18).unwrap();

    match create_user(connection, "name", 18) {
        Ok(o) => println!("{}", o),
        Err(e) => panic!("{}", e),
    }

}