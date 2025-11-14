#include <string>
#include <vector>

using namespace std;

int solution(int n, vector<int> lost, vector<int> reserve) {
    vector<int> counts(n + 1, 1);
    for (int l: lost) {
        counts[l]--;
    }
    for (int r: reserve) {
        counts[r]++;
    }
    
    int answer = 0;
    for (int i = 1; i <= n; i++) {
        if (counts[i] == 0) {
            if (i - 1 >= 1 && counts[i - 1] == 2) {
                answer++;
            } else if (i + 1 <= n && counts[i + 1] == 2) {
                counts[i + 1]--;
                answer++;
            }
        } else {
            answer++;
        }
    }
    return answer;
}