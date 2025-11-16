#include <string>
#include <vector>

using namespace std;

string solution(string number, int k) {
    string answer;
    for (char num: number) {
        while (answer.size() && num > answer.back() && k) {
            answer.pop_back();
            k--;
        }
        answer += num;
    }
    while (k) {
        answer.pop_back();
        k--;
    }
    return answer;
}