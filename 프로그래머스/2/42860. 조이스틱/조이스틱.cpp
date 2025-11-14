#include <string>
#include <vector>
#include <functional>

using namespace std;

int solution(string name) {
    int n = name.size();
    int visited = 1;
    int answer = 0;
    for (int i = 0; i < n; i++) {
        char ch = name[i];
        if (ch == 'A') {
            visited |= 1 << i;
            continue;
        }
        int up = ch - 'A';
        int down = 'Z' - ch + 1;
        answer += min(up, down);
    }
    
    vector<vector<int>> min_counts(n, vector(1 << n, 987654321));
    function<int(int, int)> dfs = [&](int pos, int count) {
        if (visited == (1 << n) - 1) {
            return count;
        }
        
        int left = (pos - 1 + n) % n;
        int temp_visited = visited;
        visited |= 1 << left;
        int left_count = 987654321;
        if (count + 1 < min_counts[left][visited]) {
            min_counts[left][visited] = count + 1;
            left_count = dfs(left, count + 1);
        }
        visited = temp_visited;
        
        int right = (pos + 1) % n;
        temp_visited = visited;
        visited |= 1 << right;
        int right_count = 987654321;
        if (count + 1 < min_counts[right][visited]) {
            min_counts[right][visited] = count + 1;
            right_count = dfs(right, count + 1);
        }
        visited = temp_visited;
        
        return min(left_count, right_count);
    };
    
    answer += dfs(0, 0);
    return answer;
}