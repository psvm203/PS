#include <bits/stdc++.h>

using namespace std;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int cute = 0;
    int n;
    cin >> n;
    while (n--) {
        int is_cute;
        cin >> is_cute;
        if (is_cute) {
            cute++;
        } else {
            cute--;
        }
    }

    string result = cute >= 0 ? "Junhee is cute!" : "Junhee is not cute!";
    cout << result;

    return 0;
}
