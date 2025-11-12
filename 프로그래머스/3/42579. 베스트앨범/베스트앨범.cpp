#include <string>
#include <vector>
#include <unordered_map>
#include <algorithm>

using namespace std;

vector<int> solution(vector<string> genres, vector<int> plays) {
    int n = genres.size();
    unordered_map<string, int> counts;
    counts.reserve(n);
    for (int i = 0; i < n; i++) {
        counts[genres[i]] += plays[i];
    }
    
    vector<vector<int>> songs(n);
    for (int i = 0; i < n; i++) {
        songs[i] = {-counts[genres[i]], -plays[i], i};
    }
    sort(songs.begin(), songs.end());
    
    unordered_map<string, int> genre_visited;
    vector<int> answer;
    answer.reserve(n);
    for (auto song: songs) {
        if (genre_visited[genres[song[2]]] >= 2) {
            continue;
        }
        genre_visited[genres[song[2]]]++;
        answer.emplace_back(song[2]);
    }
    return answer;
}