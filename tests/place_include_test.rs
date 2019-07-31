extern crate musicbrainz_rs;
use musicbrainz_rs::model::place;
use musicbrainz_rs::model::place::*;
use musicbrainz_rs::QueryAble;
use std::{thread, time};

#[test]
fn should_get_place_aliases() {
    let blue_note = Place::fetch()
        .id("327c29c6-da63-4dc9-a117-1917ee691ce4")
        .include(place::Include::Aliases)
        .execute()
        .unwrap();

    assert!(blue_note.aliases.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_place_tags() {
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .include(place::Include::Tags)
        .execute()
        .unwrap();

    assert!(olympia.tags.is_some());

    thread::sleep(time::Duration::from_secs(1));
}

#[test]
fn should_get_place_genres() {
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .include(place::Include::Genres)
        .execute()
        .unwrap();

    assert!(olympia.genres.is_some());

    thread::sleep(time::Duration::from_secs(1));
}
