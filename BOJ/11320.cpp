#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(0)->sync_with_stdio(0);
	
	int t;
	cin >> t;
	
	while (t--) {
		int a, b;
		cin >> a >> b;
		int result = a / b;
		result *= result;
		cout << result << '\n';
	}
	
	return 0;
}
