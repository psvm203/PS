#include <string>
#include <vector>
#include <functional>
#include <numeric>
#include <unordered_map>

using namespace std;

int solution(int n, vector<vector<int>> wires) {
    vector<int> parents(n + 1);
    function<int(int)> find = [&](int x) {
        if (x == parents[x]) {
            return x;
        }
        return parents[x] = find(parents[x]);
    };
    auto merge = [&](int a, int b) {
        a = find(a);
        b = find(b);
        if (a < b) {
            swap(a, b);
        }
        parents[a] = b;
    };
    
    int min_diff = 987654321;
    for (int i = 0; i < n - 1; i++) {
        iota(parents.begin(), parents.end(), 0);
        for (int j = 0; j < n - 1; j++) {
            if (i == j) {
                continue;
            }
            merge(wires[j][0], wires[j][1]);
        }
        unordered_map<int, int> map;
        for (int j = 1; j <= n; j++) {
            map[find(j)]++;
        }
        auto it = map.begin();
        int count_a = it->second;
        it++;
        int count_b = it->second;
        int diff = abs(count_a - count_b);
        min_diff = min(min_diff, diff);
    }
    return min_diff;
}