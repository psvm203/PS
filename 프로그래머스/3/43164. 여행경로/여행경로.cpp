#include <string>
#include <vector>
#include <unordered_map>
#include <map>
#include <functional>

using namespace std;

vector<string> solution(vector<vector<string>> tickets) {
    int n = tickets.size();
    
    unordered_map<string, map<string, int>> map;
    for (auto &ticket: tickets) {
        string &a = ticket[0];
        string &b = ticket[1];
        map[a][b]++;
    }
    
    vector<string> routes {"ICN"};
    routes.reserve(n + 1);
    function<bool(int, string)> dfs = [&](int index, string cur) {
        if (index == n) {
            return true;
        }
        for (auto [next, count]: map[cur]) {
            if (!count) {
                continue;
            }
            map[cur][next]--;
            routes.emplace_back(next);
            if (dfs(index + 1, next)) {
                return true;
            }
            map[cur][next]++;
            routes.pop_back();
        }
        return false;
    };
    
    dfs(0, "ICN");
    return routes;
}