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

To determine the big O notation of the given code, the code should be broken down in its main parts and analyze the time complexity of each part:

1. Reading and parsing JSON data from a file:
   Reading a file and parsing it into a DbData struct takes O(n) time, where n is the size of the JSON data in the file.

2. Filtering movies by matching all provided genres:
   The code iterates through each movie in the db.movies vector and checks if all the genres match the provided genres. This is done using the .iter().all(...) function and may require iterating through all genres of each movie.
   In the worst case, it needs to check each movie against all provided genres, which results in O(m \* k) time, where m is the number of movies and k is the number of provided genres.
3. Removing duplicate movies:
   The code uses a HashSet (selected_movie_ids) to keep track of selected movies to ensure there are no duplicates. Inserting into a HashSet and checking for existence takes O(1) time on average.
   In the worst case, it may insert and check all movies, which results in O(m) time.
4. Filtering movies by matching the provided genres exactly:
   Similar to step 2, the code iterates through each movie and checks if it matches the provided genres exactly. This also takes O(m \* k) time in the worst case.
5. Sorting the matching movies by title:
   Sorting the matching_movies vector by title using sort_by takes O(m \* log(m)) time in the worst case.

6. Choosing a random movie:
   Choosing a random movie from the matching_movies vector is done in constant time, O(1).

The dominant term in terms of time complexity is the sorting step O(m \* log(m)), so the overall time complexity of the get_filtered_movies function is O(m \* log(m)) in the worst case.

## Improvements

In order to have the Big O Notation improved then the sorting algorithm could be improved.
Another way to improve Big O Notation the filtering step could be improved. Now the complexity on the filtering step is O(m \* k). Using a Trie data structure for genre matching can help improve the overall efficiency of filtering movies based on genres. Using a Trie data structure to store movies and filter them the following would change:

- Instead of iterating through all movies and checking if each genre is contained in the provided genres, you can now traverse the Trie.
- Start at the root of the Trie and follow the branches corresponding to the provided genres.
- This process can efficiently identify movies that match the provided genres because Trie traversal has a time complexity of O(k), where k is the number of provided genres.
