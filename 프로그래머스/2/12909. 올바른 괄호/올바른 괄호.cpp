#include <string>
#include <iostream>
#include <vector>

using namespace std;

bool solution(string s)
{
    vector<bool> stack;
    stack.reserve(s.size());
    for (char ch: s) {
        if (ch == '(') {
            stack.emplace_back(true);
        } else if (stack.empty()) {
            return false;
        } else {
            stack.pop_back();
        }
    }

    return stack.empty();
}