#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int t, n;
    cin >> t >> n;

    auto day_to_num = [&](string &day) {
        if (day == "Mon") {
            return 0;
        }
        if (day == "Tue") {
            return 1;
        }
        if (day == "Wed") {
            return 2;
        }
        if (day == "Thu") {
            return 3;
        }
        return 4;
    };

    for (int i = 0; i < n; i++) {
        string day1, day2;
        int hour1, hour2;
        cin >> day1 >> hour1 >> day2 >> hour2;
        int begin = day_to_num(day1) * 24 + hour1;
        int end = day_to_num(day2) * 24 + hour2;
        int slept = end - begin;
        t -= slept;
    }

    auto answer = [&]() {
        if (t > 48) {
            return -1;
        }
        if (t <= 0) {
            return 0;
        }
        return t;
    };
    cout << answer();

    return 0;
}
