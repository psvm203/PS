#include <bits/stdc++.h>

using namespace std;

int main() {
	cin.tie(nullptr)->sync_with_stdio(false);

	char a, b, c, d;
	cin >> a >> b >> c >> d;

	auto win = [&](char x, char y) {
		return x == 'R' && y == 'S' || x == 'S' && y == 'P' || x == 'P' && y == 'R';
	};

	string result;
	if (a == b && (win(c, a) || win(d, a))) {
		result = "TK";
	} else if (c == d && (win(a, c) || win(b, c))) {
		result = "MS";
	} else {
		result = "?";
	}

	cout << result;

	return 0;
}
