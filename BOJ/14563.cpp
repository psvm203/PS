#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);
	int t;
	cin >> t;

	while (t--) {
		int n;
		cin >> n;

		int sum = 0;
		for (int i = 1; i < n; i++) {
			if (n % i == 0) {
				sum += i;
			}
		}

		string result = "Perfect";
		if (sum > n) {
			result = "Abundant";
		} else if (sum < n) {
			result = "Deficient";
		}
		cout << result << '\n';
	}

	return 0;
}
