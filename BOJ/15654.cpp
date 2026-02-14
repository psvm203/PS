#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(0)->sync_with_stdio(0);
	
	int n, m;
	cin >> n >> m;
	vector<string> nums(n);
	for (auto& num: nums) cin >> num;
	sort(nums.begin(), nums.end(), [](auto& a, auto& b) {
		if (a.size() == b.size()) {
			return a < b;
		}
		return a.size() < b.size();
	});
	
	function<void(int, vector<int>)> dfs = [&](int mask, vector<int> indices) {
		if (indices.size() == m) {
			for (int index: indices) cout << nums[index] << ' ';
			cout << '\n';
			return;
		}
		
		for (int i = 0; i < n; i++) {
			if (mask & (1 << i)) {
				continue;
			}
			indices.emplace_back(i);
			dfs(mask | (1 << i), indices);
			indices.pop_back();
		}
	};
	dfs(0, {});
	
	return 0;
}

