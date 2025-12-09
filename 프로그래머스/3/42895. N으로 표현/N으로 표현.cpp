#include <string>
#include <vector>
#include <functional>

using namespace std;

int INF = 987654321;

int solution(int N, int number) {
    function<int(int, int)> dfs = [&](int x, int count) {
        if (x < 0) {
            return INF;
        }
        if (count > 8) {
            return INF;
        }
        if (x == number) {
            return count;
        }
        int cur_n = 0;
        int min_count = INF;
        for (int i = 1; i + count <= 8; i++) {
            cur_n = cur_n * 10 + N;
            min_count = min(min_count, dfs(x + cur_n, count + i));
            min_count = min(min_count, dfs(x - cur_n, count + i));
            min_count = min(min_count, dfs(cur_n - x, count + i));
            min_count = min(min_count, dfs(x * cur_n, count + i));
            min_count = min(min_count, dfs(x / cur_n, count + i));
            if (x != 0) {
                min_count = min(min_count, dfs(cur_n / x, count + i));
            }
        }
        return min_count;
    };
    
    int answer = dfs(0, 0);
    answer = answer == INF ? -1 : answer;
    
    
    return answer;
}