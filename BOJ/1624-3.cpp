#include <bits/stdc++.h>
#define int int64_t

using namespace std;

/*
 처음은 1이다.
 소수와 관련이 있다.
 합성수가 아깝다.
 20000 < 2^15
2*3*5*7*11*13*17*19 = 9699690
1           1
96996909699690
9699690*29699690*2
38798760
77597520

 1 2 4 8 16 ... 8388608 8388608*9 27 81 243
 1 2 4 8 16 ... 3       9         27 81
 */

signed main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int MAX_PRIME = 1e5;
    vector is_prime(MAX_PRIME, true);
    for (int i = 2; i * i < MAX_PRIME; i++) {
        if (!is_prime[i]) {
            continue;
        }
        for (int j = i * i; j < MAX_PRIME; j += i) {
            is_prime[j] = false;
        }
    }
    vector<int> primes;
    for (int i = 2; i < MAX_PRIME; i++) {
        if (is_prime[i]) {
            primes.emplace_back(i);
        }
    }

    int n;
    cin >> n;

    int cur = 1;
    int index = 0;
    bool changed = false;
    while (n--) {
        cout << cur << ' ';
        if (changed) {
            changed = false;
            cur = primes[index];
        } else if (cur * primes[index] * primes[index + 1] > 1e8) {
            index++;
            cur *= primes[index];
            changed = true;
        } else {
            cur *= primes[index];
        }
    }

    return 0;
}
