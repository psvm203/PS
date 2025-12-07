#include <string>
#include <vector>
#include <algorithm>

using namespace std;

int solution(vector<vector<int>> routes) {
    sort(routes.begin(), routes.end());
    
    int begin = -30000;
    int end = 30000;
    int answer = 1;
    for (auto& route: routes) {
        if (end < route[0]) {
            answer++;
            begin = route[0];
            end = route[1];
            continue;
        }
        begin = route[0];
        end = min(end, route[1]);
    }
    
    return answer;
}