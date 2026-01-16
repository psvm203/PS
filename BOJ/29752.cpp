#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n;
	cin >> n;

	int streak = 0;
	int max_streak = 0;
	for (int i = 0; i < n; i++) {
		int s;
		cin >> s;
		streak += s > 0;
		streak *= s > 0;
		max_streak = max(max_streak, streak);
	}

	cout << max_streak;

	return 0;
}
