#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	int n;
	cin >> n;

	vector<int> is_placed(n + 1);
	int placed = 0;
	int max_placed = 0;
	for (int i = 0; i < n * 2; i++) {
		int sticker;
		cin >> sticker;
		if (is_placed[sticker]) {
			placed--;
		} else {
			placed++;
		}
		is_placed[sticker] ^= true;
		max_placed = max(max_placed, placed);
	}

	cout << max_placed;

	return 0;
}
