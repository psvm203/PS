#include <string>
#include <vector>
#include <set>
#include <sstream>

using namespace std;

vector<int> solution(vector<string> operations) {
    multiset<int> set;
    for (string &operation: operations) {
        istringstream ss(operation);
        string op;
        int num;
        ss >> op >> num;
        if (op == "I") {
            set.emplace(num);
        } else if (num == 1) {
            if (set.empty()) {
                continue;
            }
            auto it = set.end();
            it--;
            set.erase(it);
        } else {
            if (set.empty()) {
                continue;
            }
            set.erase(set.begin());
        }
    }
    
    vector<int> answer;
    if (set.empty()) {
        answer = {0, 0};
    } else {
        answer = {*set.rbegin(), *set.begin()};
    }
    return answer;
}