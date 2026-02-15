#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(0)->sync_with_stdio(0);
	
	int n, m;
	cin >> n >> m;
	
	function<void(int, vector<int>)> dfs = [&](int cur, vector<int> nums) {
		if (nums.size() == m) {
			for (int num: nums) {
				cout << num << ' ';
			}
			cout << '\n';
			return;
		}
		
		if (cur == n + 1) {
			return;
		}
		
		while (nums.size() != m) {
			nums.emplace_back(cur);
		}
		
		while (nums.back() == cur) {
			dfs(cur + 1, nums);
			nums.pop_back();
		}
		dfs(cur + 1, nums);
	};
	dfs(1, {});
	
	return 0;
}
