#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n;
    cin >> n;
    string num = to_string(n);

    int pops = 0;
    string buffer;
    for (int i = 1; i <= 100000; i++) {
        buffer += to_string(i);
        if (buffer.size() > 12) {
            int pop = to_string(i - 1).size();
            pops += pop;
            buffer.erase(0, pop);
        }
        int index = buffer.find(num);
        if (index != string::npos) {
            cout << index + pops + 1;
            return 0;
        }
    }

    return 1;
}
