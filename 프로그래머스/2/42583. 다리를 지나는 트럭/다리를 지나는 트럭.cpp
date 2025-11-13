#include <string>
#include <vector>
#include <queue>

using namespace std;

int solution(int bridge_length, int weight, vector<int> truck_weights) {
    queue<int> bridge;
    auto pop = [&]() {
        weight += bridge.front();
        bridge.pop();
    };
    auto push = [&](int x) {
        weight -= x;
        bridge.emplace(x);
    };
    
    auto it = truck_weights.begin();
    int time = 1;
    while (it != truck_weights.end()) {
        if (bridge.size() == bridge_length) {
            pop();
        }
        
        if (*it > weight) {
            push(0);
        } else {
            push(*it);
            it++;
        }
        
        time++;
    }
    
    int answer = time + bridge_length - 1;
    return answer;
}