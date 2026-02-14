#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(0)->sync_with_stdio(0);
	
	int a, b;
	cin >> a >> b;
	
	int result = 1;
	while (a != b) {
		if (b != 0 && b % 2 == 0) {
			b /= 2;
		} else if (b % 10 == 1) {
			b /= 10;
		} else {
			result = -1;
			break;
		}
		result++;
	}
	cout << result;
	
	return 0;
}
