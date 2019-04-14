use std::collections::HashMap;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::fmt;
use std::marker::PhantomData;


use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};

#[derive(Deserialize, Debug)]
pub struct PlayerSongCurrentlyPlayed {
    pub index_in_playlist: u32,
    pub seconds_played: f32,
    pub duration: String,
    pub artist: String,
    pub album: String,
    pub title: String,
    pub is_remote: bool,
    pub path: String,
}


#[derive(Deserialize, Debug)]
pub struct PlayerMixer {
    pub volume: String,
    pub bass: String,
    pub treble: String,
    pub power: String,
}


#[derive(Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub uuid: String,
    pub id: String,
    pub ip: String,
    pub model: String,
    pub firmware_version: String,
    pub signal_strength: u8,
    pub play_state: String,
    pub mixer : PlayerMixer,
    pub song_currently_played: PlayerSongCurrentlyPlayed,
}


#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
     #[test]
    fn it_xml() {
    
        let json = r#"[
    {
        "name": "Salle chacha",
        "uuid": "********************************",
        "id": "**:**:**:**:**:**",
        "ip": "192.168.*.*:*****",
        "model": "Squeezebox Touch",
        "firmware_version": "7.8.0-r16754",
        "signal_strength": 88,
        "mixer": {
            "volume": "42",
            "bass": "50",
            "treble": "50",
            "power": "on"
        },
        "play_state": "pause",
        "song_currently_played": {
            "index_in_playlist" : 3,
            "seconds_played": 183.890504037857,
            "duration": "258.466",
            "artist": "The Smashing Pumpkins",
            "album": "Mellon Collie and the Infinite Sadness (2012 - Remaster)",
            "title": "Bullet With Butterfly Wings",
            "is_remote": true,
            "path": "spotify://track:4qMzPtAZe0C9KWpWIzvZAP"
        }
    },
    {
    "name": "Musique salle de bain",
    "uuid": "********************************",
    "id": "**:**:**:**:**:**",
    "ip": "192.168.*.*:*****",
    "model": "Squeezebox Radio",
    "firmware_version": "7.7.3-r16676",
    "signal_strength": 88,
    "mixer": {
        "volume": "42",
        "bass": "50",
        "treble": "50",
        "power": "on"
    },
    "play_state": "pause",
    "song_currently_played": {
        "index_in_playlist" : 3,
        "seconds_played": 183.890504037857,
        "duration": "258.466",
        "artist": "The Smashing Pumpkins",
        "album": "Mellon Collie and the Infinite Sadness (2012 - Remaster)",
        "title": "Bullet With Butterfly Wings",
        "is_remote": true,
        "path": "spotify://track:4qMzPtAZe0C9KWpWIzvZAP"
    }
}
]
    "#;
    let request: Vec<super::Player> = serde_json::from_str(json).unwrap();
    println!("{:?}", request);
    println!("{:?}", request)
    }
}
