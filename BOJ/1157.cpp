#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);
	string s;
	cin >> s;

	map<char, int> counts;
	for (char ch: s) {
		ch = toupper(ch);
		counts[ch]++;
	}

	char result = '?';
	int max_count = 0;
	for (auto [ch, count]: counts) {
		if (count > max_count) {
			max_count = count;
			result = ch;
		} else if (count == max_count) {
			result = '?';
		}
	}
	cout << result;

	return 0;
}
