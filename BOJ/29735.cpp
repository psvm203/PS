#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int a, b, c, d, n, t;
	char dummy;
	cin >> a >> dummy >> b >> c >> dummy >> d >> n >> t;

	int begin = a * 60 + b;
	int end = c * 60 + d;
	int cur = begin;
	int day = 0;
	for (int i = 0; i < n + 1; i++) {
		if (cur + t < end) {
			cur += t;
		} else {
			cur = begin + t;
			day++;
		}
	}

	int hour = cur / 60;
	int minute = cur % 60;
	printf("%d\n%02d:%02d", day, hour, minute);

	return 0;
}
