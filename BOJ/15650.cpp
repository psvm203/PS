#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(0)->sync_with_stdio(0);
	
	int n, m;
	cin >> n >> m;
	
	function<void(int, int)> dfs = [&](int cur, int mask) {
		if (cur == n + 1) {
			if (__builtin_popcount(mask) != m) {
				return;
			}
			for (int i = 1; i <= n; i++) {
				if (mask & (1 << i)) {
					cout << i << ' ';
				}
			}
			cout << '\n';
			return;
		}
		
		dfs(cur + 1, mask | (1 << cur));
		dfs(cur + 1, mask);
	};
	dfs(1, 0);
	
	return 0;
}
