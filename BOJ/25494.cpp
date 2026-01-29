#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int t;
	cin >> t;

	while (t--) {
		int result = 0;
		int a, b, c;
		cin >> a >> b >> c;
		for (int x = 1; x <= a; x++) {
			for (int y = 1; y <= b; y++) {
				for (int z = 1; z <= c; z++) {
					result += x % y == y % z && y % z == z % x;
				}
			}
		}

		cout << result << '\n';
	}

	return 0;
}
