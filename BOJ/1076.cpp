#include <bits/stdc++.h>
#define int int64_t

using namespace std;

signed main() {
	cin.tie(0)->sync_with_stdio(0);
	
	map<string, int> values {
		{"black", 0},
		{"brown", 1},
		{"red", 2},
		{"orange", 3},
		{"yellow", 4},
		{"green", 5},
		{"blue", 6},
		{"violet", 7},
		{"grey", 8},
		{"white", 9},
	};
	
	int result = 0;
	for (int i = 0; i < 2; i++) {
		string color;
		cin >> color;
		result *= 10;
		result += values[color];
	};
	
	map<string, int> multiple {
		{"black", 1},
		{"brown", 1e1},
		{"red", 1e2},
		{"orange", 1e3},
		{"yellow", 1e4},
		{"green", 1e5},
		{"blue", 1e6},
		{"violet", 1e7},
		{"grey", 1e8},
		{"white", 1e9},
	};
	
	string color;
	cin >> color;
	result *= multiple[color];
	
	cout << result;
	
	return 0;
}
