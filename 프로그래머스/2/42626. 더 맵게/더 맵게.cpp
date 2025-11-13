#include <string>
#include <vector>
#include <queue>

using namespace std;

int solution(vector<int> scoville, int K) {
    priority_queue<int, vector<int>, greater<>> pq(scoville.begin(), scoville.end());
    int answer = 0;
    while (pq.top() < K) {
        int lo = pq.top();
        pq.pop();
        
        if (pq.empty()) {
            return -1;
        }
        
        int hi = pq.top();
        pq.pop();
        pq.emplace(lo + hi * 2);
        answer++;
    }
    return answer;
}