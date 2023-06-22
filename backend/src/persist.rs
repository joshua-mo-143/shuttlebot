use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;

#[derive(Clone, Deserialize, Serialize)]
struct UserSessions {
    user_sessions: Vec<UserSession>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserSession {
    pub session_id: String,
    pub expires_at: DateTime<Utc>,
}

pub struct Persist;

impl Persist {
    pub fn add_record(persist: PersistInstance, session: UserSession) -> Result<(), anyhow::Error> {
        let mut instance = if let Ok(res) = persist.load::<UserSessions>("usersessions") {
            res
        } else {
            UserSessions {
                user_sessions: Vec::new(),
            }
        };

        instance.user_sessions.push(session);

        persist
            .save::<UserSessions>("usersessions", instance)
            .expect("Failed to save persist instance");
        Ok(())
    }

    pub fn delete_record(
        persist: PersistInstance,
        session_id: String,
    ) -> Result<(), anyhow::Error> {
        let mut instance = if let Ok(res) = persist.load::<UserSessions>("usersessions") {
            res
        } else {
            return Ok(());
        };

        let new_instance = instance
            .user_sessions
            .into_iter()
            .filter(|user| user.session_id == session_id)
            .collect::<Vec<UserSession>>();

        instance.user_sessions = new_instance;

        persist
            .save::<UserSessions>("usersessions", instance)
            .expect("Failed to save persist instance");
        Ok(())
    }

    pub fn filter_records(persist: PersistInstance) -> Result<(), anyhow::Error> {
        let mut instance = if let Ok(res) = persist.load::<UserSessions>("usersessions") {
            res
        } else {
            return Ok(());
        };

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc);

        let new_instance = instance
            .user_sessions
            .into_iter()
            .filter(|user| user.expires_at < dt)
            .collect::<Vec<UserSession>>();

        instance.user_sessions = new_instance;

        persist
            .save::<UserSessions>("usersessions", instance)
            .expect("Failed to save persist instance");

        Ok(())
    }
}
