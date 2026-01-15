#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	vector<int> lengths(3);
	for (int i = 0; i < 3; i++) {
		cin >> lengths[i];
	}
	sort(lengths.begin(), lengths.end());

	int result = lengths[0] + lengths[1] + min(lengths[0] + lengths[1] - 1, lengths[2]);
	cout << result;

	return 0;
}
