#include <string>
#include <vector>
#include <algorithm>

using namespace std;

string solution(vector<int> numbers) {
    vector<string> nums;
    nums.reserve(numbers.size());
    for (int number: numbers) {
        nums.emplace_back(to_string(number));
    }
    sort(nums.begin(), nums.end(), [](string &a, string &b) {
        return a + b > b + a;
    });
    
    string answer;
    for (string &num: nums) {
        answer += num;
    }
    if (answer[0] == '0') {
        return "0";
    }
    return answer;
}