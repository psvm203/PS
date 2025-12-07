#include <string>
#include <vector>
#include <functional>

using namespace std;

int MOD = 1e9 + 7;

int solution(int m, int n, vector<vector<int>> puddles) {
    vector<vector<int>> memo(n, vector<int>(m, -1));
    memo[0][0] = 1;
    for (auto& puddle: puddles) {
        int y, x;
        y = puddle[1] - 1;
        x = puddle[0] - 1;
        memo[y][x] = 0;
    }
    
    function<int(int, int)> dfs = [&](int y, int x) {
        int& m = memo[y][x];
        if (m != -1) {
            return m;
        }
        m = 0;
        if (y >= 1) {
            m += dfs(y - 1, x);
            m %= MOD;
        }
        if (x >= 1) {
            m += dfs(y, x - 1);
            m %= MOD;
        }
        return m;
    };
    
    int answer = dfs(n - 1, m - 1);
    return answer;
}