#include <string>
#include <vector>
#include <functional>

using namespace std;

int solution(int k, vector<vector<int>> dungeons) {
    int n = dungeons.size();
    vector<bool> visited(n);
    function<int(int)> dfs = [&](int count) {
        int max_count = count;
        for (int i = 0; i < n; i++) {
            if (visited[i] || k < dungeons[i][0]) {
                continue;
            }
            visited[i] = true;
            k -= dungeons[i][1];
            max_count = max(max_count, dfs(count + 1));
            visited[i] = false;
            k += dungeons[i][1];
        }
        return max_count;
    };
    return dfs(0);
}