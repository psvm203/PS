#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n, x;
	cin >> n >> x;

	cout.precision(7);
	double result = 10000.0 / (100 - x) - 100.0;
	cout << fixed << result;

	return 0;
}
