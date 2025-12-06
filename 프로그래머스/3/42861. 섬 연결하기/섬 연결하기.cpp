#include <string>
#include <vector>
#include <numeric>
#include <functional>
#include <algorithm>

using namespace std;

int solution(int n, vector<vector<int>> costs) {
    vector<int> roots(n);
    iota(roots.begin(), roots.end(), 0);
    
    function<int(int)> find = [&](int x) {
        if (x == roots[x]) {
            return x;
        }
        return roots[x] = find(roots[x]);
    };
    
    auto merge = [&](int a, int b) {
        a = find(a);
        b = find(b);
        
        if (a < b) {
            swap(a, b);
        }
        
        roots[a] = b;
    };
    
    vector<vector<int>> edges;
    edges.reserve(n + 1);
    for (auto& cost: costs) {
        edges.emplace_back(vector {cost[2], cost[0], cost[1]});
    }
    sort(edges.begin(), edges.end());
    
    int answer = 0;
    for (auto& edge: edges) {
        if (find(edge[1]) != find(edge[2])) {
            answer += edge[0];
            merge(edge[1], edge[2]);
        }
    }
    return answer;
}