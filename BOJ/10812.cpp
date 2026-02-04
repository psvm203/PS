#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n, m;
	cin >> n >> m;

	vector<int> seq(n);
	iota(seq.begin(), seq.end(), 1);

	for (int _ = 0; _ < m; _++) {
		int i, j, k;
		cin >> i >> j >> k;
		auto first = seq.begin();
		auto mid = seq.begin();
		auto end = seq.begin();
		advance(first, i - 1);
		advance(mid, k - 1);
		advance(end, j);
		rotate(first, mid, end);
	}

	for (int num: seq) {
		cout << num << ' ';
	}

	return 0;
}
