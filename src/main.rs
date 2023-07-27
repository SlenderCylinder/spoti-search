extern crate reqwest;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct SpotifyResponse {
    tracks: TrackList,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrackList {
    items: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    id: String,  
    name: String,
    artists: Vec<Artist>,
    album: Album,
}

#[derive(Debug, Serialize, Deserialize)]
struct Artist {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Album {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the Spotify API token from the environment variable
    let token = match env::var("SPOTIFY_API_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("Please set the SPOTIFY_API_TOKEN environment variable.");
            return Ok(());
        }
    };

    // Prompt the user to enter a search query
    println!("Enter a song name to search on Spotify:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let query = input.trim();

    // Build the request URL
    let url = format!(
        "https://api.spotify.com/v1/search?q={}&type=track&limit=5",
        query
    );

    // Send the request to Spotify API
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;


    // Parse the response JSON
    let response_json: SpotifyResponse = response.json().await?;

    // Print the search results
    println!("Search Results:");
    for track in response_json.tracks.items {
        let artists = track
            .artists
            .iter()
            .map(|artist| artist.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        println!("Track: {}", track.name);
        println!("Artists: {}", artists);
        println!("Album: {}", track.album.name);
        println!("Link: https://open.spotify.com/track/{}", track.id);
        println!();
    }

    Ok(())
}