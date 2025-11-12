#include <vector>
#include <iostream>

using namespace std;

vector<int> solution(vector<int> arr) 
{
    int n = arr.size();
    int prev = -1;
    vector<int> answer;
    answer.reserve(n);
    for (int a: arr) {
        if (a == prev) {
            continue;
        }
        answer.emplace_back(a);
        prev = a;
    }
    return answer;
}