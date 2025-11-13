#include <string>
#include <vector>
#include <algorithm>
#include <queue>

using namespace std;

int solution(vector<vector<int>> jobs) {
    int n = jobs.size();
    vector<vector<int>> new_jobs;
    new_jobs.reserve(n);
    for (int i = 0; i < n; i++) {
        new_jobs.emplace_back(vector{jobs[i][0], jobs[i][1], i}); // 요청 시각, 작업 시간, 번호
    }
    sort(new_jobs.begin(), new_jobs.end());
    
    int pos = 0;
    int time = 0;
    int cost = 0;
    priority_queue<vector<int>, vector<vector<int>>, greater<>> pq;
    while (pos < n) {
        if (pq.empty()) {
            time = max(time, new_jobs[pos][0]);
        }
        for (; pos < n; pos++) {
            vector<int> &job = new_jobs[pos];
            if (job[0] > time) {
                break;
            }
            pq.emplace(vector{job[1], job[0], job[2]}); // 작업 시간, 요청 시각, 번호
        }
        if (pq.size()) {
            vector<int> cur = pq.top();
            pq.pop();
            time += cur[0];
            cost += time - cur[1];
        }
    }
    
    while (pq.size()) {
        vector<int> cur = pq.top();
        pq.pop();
        time += cur[0];
        cost += time - cur[1];
    }
    
    int answer = cost / n;
    return answer;
}