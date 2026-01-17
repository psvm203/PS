#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n;
	cin >> n;

	vector<int> nums(n);
	for (int &num: nums) {
		cin >> num;
	}

	for (int i = 1;; i++) {
		bool is_common_divisor = true;
		for (int num: nums) {
			if (num < i) {
				return 0;
			}
			if (num % i != 0) {
				is_common_divisor = false;
				break;
			}
		}
		if (is_common_divisor) {
			cout << i << '\n';
		}
	}


	return 0;
}
