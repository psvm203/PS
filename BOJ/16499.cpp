#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n;
	cin >> n;

	set<string> groups;
	for (int i = 0; i < n; i++) {
		string word;
		cin >> word;
		sort(word.begin(), word.end());
		groups.emplace(word);
	}
	cout << groups.size();

	return 0;
}
