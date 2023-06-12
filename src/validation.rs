use chrono::Utc;

pub fn has_ballot_started(from: &i64) -> bool {
    let dt: chrono::DateTime<Utc> = Utc::now();
    let now: i64 = dt.timestamp(); 
    let has_ballot_started = from <= &now;

    has_ballot_started
}

pub fn has_ballot_in_progress(from: &i64, to: &i64) -> bool {
    let dt: chrono::DateTime<Utc> = Utc::now();
    let now: i64 = dt.timestamp(); 
    let has_ballot_in_progress = from <= &now && &now <= to;

    has_ballot_in_progress
}