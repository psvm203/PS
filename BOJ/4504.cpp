#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n;
	cin >> n;

	auto is_multiple = [&](int x) {
		return x % n == 0;
	};

	int num;
	cin >> num;
	while (num != 0) {
		string result = to_string(num);
		result += " is ";
		if (!is_multiple(num)) {
			result += "NOT ";
		}
		result += "a multiple of ";
		result += to_string(n);
		result += ".\n";
		cout << result;
		cin >> num;
	}

	return 0;
}
