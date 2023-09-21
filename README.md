## Movies challenge

### Run tests

`cargo test`

### Build

`cargo build`

### Objective

Write an algorithm that would help find the right movie:

- If we provide genres [Comedy, Fantasy, Crime] then the top hits should be movies that have all three of them, then there should be movies that have one of [Comedy, Fantasy], [Comedy, Crime], [Fantasy, Crime] and then those with Comedy only, Fantasy only and Crime only. Similarly applies when the requested array of genres is shorter.

- Of course we don't want to have duplicates.

- If we provide an empty list, then we should get a single random movie. (return type should be a array with single movie)

### Big O Notation

This branch uses a Trie to optimise the Filtering step. For each movie in db.movies, it iterates over its genres, and for each genre, it checks if it's contained in the provided genres. This takes O(M) time for each movie.
It also checks if the movie is a subset and if it has extra genres. Both of these checks take O(M) time.

Overall, the most significant factor in the time complexity is the sorting of genres and sorting the matching movies. While the Trie helps with efficient genre matching, it doesn't significantly affect the overall time complexity.
