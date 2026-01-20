#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n, m, a;
	cin >> n >> m >> a;
	int mid = (m + 1) / 2;
	a--;

	int num;
	cin >> num;
	while (num != mid) {
		a += num - mid;
		a += n;
		a %= n;
		cout << a + 1 << '\n';
		cin >> num;
	}
	cout << '0';

	return 0;
}
