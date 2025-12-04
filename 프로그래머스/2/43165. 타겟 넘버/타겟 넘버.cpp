#include <string>
#include <vector>
#include <functional>

using namespace std;

int solution(vector<int> numbers, int target) {
    function<int(int, int)> dfs = [&](int index, int cur) {
        if (index >= numbers.size()) {
            return 0 + (cur == target);
        }
        
        int plus = dfs(index + 1, cur + numbers[index]);
        int minus = dfs(index + 1, cur - numbers[index]);
        return plus + minus;
    };
    
    return dfs(0, 0);
}