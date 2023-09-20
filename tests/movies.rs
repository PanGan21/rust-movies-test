use movies::types::Genre;

#[test]
fn random_movie_when_empty_array_provided() {
    let genres = vec![];
    let movies = movies::get_filtered_movies(genres);
    assert_eq!(movies.len(), 1);
}

#[test]
fn top_matched_movies_when_passed_3_genres() {
    let genres: Vec<Genre> = vec![
        "Crime".to_string(),
        "Drama".to_string(),
        "Music".to_string(),
    ];

    let expected_movies_titles = vec![
        "The Cotton Club",
        "The Shawshank Redemption",
        "City of God",
        "Taxi Driver",
        "Scarface",
        "Pulp Fiction",
        "American History X",
        "Once Upon a Time in America",
        "12 Angry Men",
        "Whiplash",
        "To Kill a Mockingbird",
        "The Great Beauty",
        "Gran Torino",
        "One Flew Over the Cuckoo's Nest",
        "Requiem for a Dream",
        "Lost in Translation",
        "Boyhood",
    ];

    let actual_movies_found: Vec<String> = movies::get_filtered_movies(genres)
        .into_iter()
        .map(|m| m.title)
        .collect();

    assert_eq!(actual_movies_found, expected_movies_titles);
}

#[test]
fn top_matched_movies_when_passed_2_genres() {
    let genres: Vec<Genre> = vec!["Crime".to_string(), "Drama".to_string()];

    let expected_movies_titles = vec![
        "The Shawshank Redemption",
        "City of God",
        "Taxi Driver",
        "Scarface",
        "Pulp Fiction",
        "American History X",
        "Once Upon a Time in America",
        "12 Angry Men",
        "To Kill a Mockingbird",
        "The Great Beauty",
        "Gran Torino",
        "One Flew Over the Cuckoo's Nest",
        "Requiem for a Dream",
        "Lost in Translation",
        "Boyhood",
    ];

    let actual_movies_found: Vec<String> = movies::get_filtered_movies(genres)
        .into_iter()
        .map(|m| m.title)
        .collect();

    assert_eq!(actual_movies_found, expected_movies_titles);
}

#[test]
fn top_matched_movies_when_passed_1_genres() {
    let genres: Vec<Genre> = vec!["Drama".to_string()];

    let expected_movies_titles = vec![
        "The Great Beauty",
        "Gran Torino",
        "One Flew Over the Cuckoo's Nest",
        "Requiem for a Dream",
        "Lost in Translation",
        "Boyhood",
    ];

    let actual_movies_found: Vec<String> = movies::get_filtered_movies(genres)
        .into_iter()
        .map(|m| m.title)
        .collect();

    assert_eq!(actual_movies_found, expected_movies_titles);
}
