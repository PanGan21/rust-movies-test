pub mod types;

use rand::Rng;
use std::io::Read;
use std::{collections::HashSet, fs::File};

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

    let mut selected_movie_ids: HashSet<u32> = HashSet::new();
    let mut matching_movies: Vec<types::Movie> = Vec::new();

    // Filter movies that match all provided genres
    let all_genres_matched: Vec<&str> = genres.iter().map(|genre| genre.as_str()).collect();
    let matching_movies_all_genres: Vec<&types::Movie> = db
        .movies
        .iter()
        .filter(|movie| {
            movie
                .genres
                .iter()
                .all(|genre| all_genres_matched.contains(&genre.as_str()))
        })
        .collect();

    // Add movies matching all genres to the result
    for movie in matching_movies_all_genres {
        if !selected_movie_ids.contains(&movie.id) {
            selected_movie_ids.insert(movie.id);
            matching_movies.push(movie.clone());
        }
    }

    let matching_movies_exact_genres: Vec<&types::Movie> = db
        .movies
        .iter()
        .filter(|movie| {
            movie.genres.len() == genres.len()
                && movie.genres.iter().all(|genre| genres.contains(genre))
        })
        .collect();

    // Add movies matching the provided genres exactly to the result
    for movie in matching_movies_exact_genres {
        if !selected_movie_ids.contains(&movie.id) {
            selected_movie_ids.insert(movie.id);
            matching_movies.push(movie.clone());
        }
    }

    // Sort the matching movies by title
    matching_movies.sort_by(|a, b| {
        let a_match_count = a.genres.iter().filter(|&g| genres.contains(g)).count();
        let b_match_count = b.genres.iter().filter(|&g| genres.contains(g)).count();

        // Sort by the number of matching genres in descending order
        b_match_count.cmp(&a_match_count)
    });

    matching_movies
}

fn choose_random_movie(movies: &[types::Movie]) -> &types::Movie {
    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..movies.len());
    &movies[random_index]
}
