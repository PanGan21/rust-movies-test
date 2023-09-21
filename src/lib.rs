mod trie;
pub mod types;

use rand::Rng;
use std::io::Read;
use std::{collections::HashSet, fs::File};
use trie::GenreTrie;

use serde::{Deserialize, Serialize};

// Define a struct to deserialize the JSON data
#[derive(Debug, Deserialize, Serialize)]
struct DbData {
    genres: Vec<types::Genre>,
    movies: Vec<types::Movie>,
}

pub fn get_filtered_movies(genres: Vec<types::Genre>) -> Vec<types::Movie> {
    // Read and parse the JSON data from db.json
    let mut db_data = String::new();
    let current_dir = std::env::current_dir().unwrap();

    let db_file_path = current_dir.join("src/db.json");

    let mut file = File::open(db_file_path).expect("Failed to open db.json");
    file.read_to_string(&mut db_data)
        .expect("Failed to read db.json");
    let db: DbData = serde_json::from_str(&db_data).expect("Failed to parse JSON");

    // If genres is empty, return a single random movie
    if genres.is_empty() {
        let random_movie = choose_random_movie(&db.movies);
        return vec![random_movie.clone()];
    }

    // Build a Genre Trie and insert joined genres
    let mut genre_trie = GenreTrie::new();
    let mut sorted_genres: Vec<String> = genres.iter().map(|genre| genre.to_string()).collect();
    sorted_genres.sort();
    genre_trie.insert(&sorted_genres.join(""));

    let mut selected_movie_ids: HashSet<u32> = HashSet::new();
    let mut matching_movies: Vec<types::Movie> = Vec::new();

    for movie in &db.movies {
        let joined_genres = movie
            .genres
            .iter()
            .map(|genre| genre.to_string())
            .collect::<Vec<_>>()
            .join("");

        let has_extra_genres = movie.genres.iter().any(|g| !genres.contains(g));
        let is_subset = movie.genres.iter().all(|g| genres.contains(g));

        if (genre_trie.contains(&joined_genres) || (is_subset && !has_extra_genres))
            && !selected_movie_ids.contains(&movie.id)
        {
            selected_movie_ids.insert(movie.id);
            matching_movies.push(movie.clone());
        }
    }

    // Sort the matching movies by the number of matching genres in descending order
    matching_movies.sort_by(|a, b| {
        let a_match_count = a.genres.iter().filter(|&g| genres.contains(g)).count();
        let b_match_count = b.genres.iter().filter(|&g| genres.contains(g)).count();
        b_match_count.cmp(&a_match_count)
    });

    matching_movies
}

fn choose_random_movie(movies: &[types::Movie]) -> &types::Movie {
    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..movies.len());
    &movies[random_index]
}
