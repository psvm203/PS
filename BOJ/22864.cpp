#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int a, b, c, m;
	cin >> a >> b >> c >> m;

	int fatigue = 0;
	int result = 0;
	for (int i = 0; i < 24; i++) {
		int next_fatigue = fatigue + a;
		if (next_fatigue <= m) {
			fatigue = next_fatigue;
			result += b;
		} else {
			fatigue = max(0, fatigue - c);
		}
	}

	cout << result;

	return 0;
}
