use crate::*;

use rusqlite::params;
use rusqlite_migration::{Migrations, M};

pub struct DatabaseConnection {
    connection: rusqlite::Connection,
}

impl DatabaseConnection {
    pub fn new() -> DatabaseConnection {
        let migrations = Migrations::new(vec![
            M::up(
                "CREATE TABLE User(
                    identifier TEXT PRIMARY KEY,
                    name TEXT NOT NULL)",
            ),
            M::up(
                "CREATE TABLE Event(
                    identifier TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    authorIdentifier TEXT NOT NULL,
                    FOREIGN KEY (authorIdentifier) REFERENCES User(identifier));",
            ),
            M::up(
                "CREATE TABLE BetDirection(
                    identifier TEXT PRIMARY KEY,
                    value INTEGER NOT NULL)",
            ),
            M::up("INSERT INTO BetDirection (identifier, value) VALUES ('NO', 0)"),
            M::up("INSERT INTO BetDirection (identifier, value) VALUES ('YES', 1)"),
            M::up(
                "CREATE TABLE BetOffer(
                    identifier TEXT PRIMARY KEY,
                    eventIdentifier TEXT NOT NULL,
                    proposerIdentifier TEXT NOT NULL,
                    direction TEXT NOT NULL,
                    stake INTEGER NOT NULL,
                    payout INTEGER NOT NULL,
                    amount INTEGER NOT NULL,
                    FOREIGN KEY (eventIdentifier) REFERENCES Event(identifier),
                    FOREIGN KEY (proposerIdentifier) REFERENCES User(identifier),
                    FOREIGN KEY (direction) REFERENCES BetDirection (identifier));",
            ),
            M::up(
                "CREATE TABLE BetAcceptance(
                    identifier TEXT PRIMARY KEY,
                    betOfferIdentifier TEXT NOT NULL,
                    acceptorIdentifier TEXT NOT NULL,
                    FOREIGN KEY (betOfferIdentifier) REFERENCES BetOffer(identifier),
                    FOREIGN KEY (acceptorIdentifier) REFERENCES User(identifier));",
            ),
            M::up(
                "CREATE TABLE Community(
                    identifier TEXT PRIMARY KEY,
                    name TEXT NOT NULL);",
            ),
            M::up(
                "CREATE TABLE CommunityMembership(
                    communityIdentifier TEXT,
                    userIdentifier TEXT,
                    FOREIGN KEY (communityIdentifier) REFERENCES Community(identifier),
                    FOREIGN KEY (userIdentifier) REFERENCES User(identifier),
                    PRIMARY KEY (communityIdentifier, userIdentifier))",
            ),
            M::up(
                "CREATE TABLE UserBalance(
                    communityIdentifier TEXT,
                    userIdentifier TEXT,
                    amount INTEGER NOT NULL,
                    FOREIGN KEY (communityIdentifier) REFERENCES Community(identifier),
                    FOREIGN KEY (userIdentifier) REFERENCES User(identifier),
                    PRIMARY KEY (communityIdentifier, userIdentifier));",
            ),
        ]);
        let mut connection = rusqlite::Connection::open("./database.db3").unwrap();
        connection
            .pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
            .unwrap();
        migrations.to_latest(&mut connection).unwrap();

        DatabaseConnection { connection }
    }

    pub fn create_event(&self, title: &String, owner: &String) -> Result<(), rusqlite::Error> {
        let identifier = uuid::Uuid::new_v4().to_string();

        self.connection
            .execute(
                "INSERT INTO Event (identifier, title, authorIdentifier) VALUES (?1, ?2, ?3)",
                params![identifier, title, owner],
            )
            .map(|_| ())
    }

    pub fn create_user(&self, name: &String) -> Result<(), rusqlite::Error> {
        let identifier = uuid::Uuid::new_v4().to_string();

        self.connection
            .execute(
                "INSERT INTO User (identifier, name) VALUES (?1, ?2, ?3)",
                params![identifier, name],
            )
            .map(|_| ())
    }

    pub fn get_events(&self) -> Vec<Event> {
        let mut statement = self
            .connection
            .prepare("SELECT identifier, title, authorIdentifier from Event")
            .unwrap();

        statement
            .query_map([], |row| {
                Ok(Event {
                    identifier: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    author_identifier: row.get(2).unwrap(),
                })
            })
            .unwrap()
            .map(Result::unwrap)
            .collect()
    }
}
