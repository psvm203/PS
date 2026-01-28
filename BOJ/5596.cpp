#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int a, b, c, d, e, f, g, h;
	cin >> a >> b >> c >> d >> e >> f >> g >> h;

	int sum1 = a + b + c + d;
	int sum2 = e + f + g + h;
	int result = max(sum1, sum2);
	cout << result;

	return 0;
}
