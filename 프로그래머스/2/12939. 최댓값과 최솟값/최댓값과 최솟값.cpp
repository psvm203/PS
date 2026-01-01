#include <bits/stdc++.h>

using namespace std;

string solution(string s) {
    stringstream sin(s);
    int num;
    int mn = 987654321;
    int mx = -mn;
    while (sin >> num) {
        cout << num;
        mn = min(mn, num);
        mx = max(mx, num);
    }
    
    string answer;
    answer += to_string(mn);
    answer += ' ';
    answer += to_string(mx);
    return answer;
}