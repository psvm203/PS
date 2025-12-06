#include <string>
#include <vector>

using namespace std;

int counts[241];

int solution(vector<int> people, int limit) {
    for (int p: people) {
        counts[p]++;
    }
    
    int answer = 0;
    for (int a = 1; a < 241; a++) {
        while (counts[a]) {
            counts[a]--;
            for (int b = limit - a; b >= 1; b--) {
                if (counts[b]) {
                    counts[b]--;
                    break;
                }
            }
            answer++;
        }
    }
    return answer;
}