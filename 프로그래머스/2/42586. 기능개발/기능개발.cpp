#include <string>
#include <vector>

using namespace std;

vector<int> solution(vector<int> progresses, vector<int> speeds) {
    int n = progresses.size();
    int lo = 0;
    vector<int> answer;
    answer.reserve(n);
    while (lo < n) {
        int day = (100 - progresses[lo] + speeds[lo] - 1) / speeds[lo];
        for (int hi = lo; hi < n; hi++) {
            progresses[hi] += speeds[hi] * day;
        }
        int count = 0;
        for (int hi = lo; hi < n; hi++) {
            if (progresses[hi] < 100) {
                break;
            }
            count++;
            lo = hi + 1;
        }
        answer.emplace_back(count);
    }
    return answer;
}