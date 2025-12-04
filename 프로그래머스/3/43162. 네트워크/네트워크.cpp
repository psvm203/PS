#include <string>
#include <vector>
#include <numeric>
#include <functional>
#include <unordered_set>

using namespace std;

int solution(int n, vector<vector<int>> computers) {
    vector<int> roots(n);
    iota(roots.begin(), roots.end(), 0);
    
    function<int(int)> find = [&](int x) {
        if (roots[x] == x) {
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
    
    for (int i = 0; i + 1 < n; i++) {
        for (int j = i + 1; j < n; j++) {
            if (computers[i][j]) {
                merge(i, j);
            }
        }
    }
    
    for (int i = 0; i < n; i++) {
        find(i);
    }
    
    unordered_set<int> set(roots.begin(), roots.end());
    int answer = set.size();
    return answer;
}