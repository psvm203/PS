#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n, m;
	cin >> n >> m;

	int result = 1;
	while (n--) {
		int a;
		cin >> a;
		a = max(a, 1);
		result *= a;
		result %= m;
	}
	result %= m;

	cout << result;

	return 0;
}
