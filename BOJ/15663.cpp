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
	
	vector<string> results;
	unordered_set<string> visited;
	function<void(int, vector<int>)> dfs = [&](int mask, vector<int> indices) {
		if (indices.size() == m) {
			string result;
			for (int index: indices) result += nums[index] + ' ';
			if (visited.find(result) == visited.end()) {
				visited.emplace(result);
				results.emplace_back(result);
			}
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
	
	for (auto& result: results) cout << result << '\n';
	
	return 0;
}

