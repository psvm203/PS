#include <string>
#include <vector>
#include <functional>

using namespace std;

int solution(string numbers) {
    int n = numbers.size();
    vector<bool> visited(n);
    
    vector<bool> is_prime(1e7, true);
    is_prime[0] = false;
    is_prime[1] = false;
    for (int i = 2; i * i < 1e7; i++) {
        if (!is_prime[i]) {
            continue;
        }
        for (int j = i * i; j < 1e7; j += i) {
            is_prime[j] = false;
        }
    }
    
    int answer = 0;
    function<void(int)> dfs = [&](int x) {
        if (is_prime[x]) {
            is_prime[x] = false;
            answer++;
        }
        for (int i = 0; i < n; i++) {
            if (visited[i]) {
                continue;
            }
            visited[i] = true;
            dfs(x * 10 + numbers[i] - '0');
            visited[i] = false;
        }
    };
    dfs(0);
    return answer;
}