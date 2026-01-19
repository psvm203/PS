#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n, m;
	cin >> n >> m;

	int result = 0;
	int max_cost = 0;
	for (int i = 0; i < m; i++) {
		int a, b;
		cin >> a >> b;
		int cost = max((a + b) / 2 - a, 0);
		max_cost = max(max_cost, cost);
		result += cost;
	}

	cout << result - max_cost;

	return 0;
}
