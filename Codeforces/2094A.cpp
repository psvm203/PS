#include <iostream>
#include <sstream>
#include <string>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  string _;
  getline(cin, _);

  while (t--) {
    string line;
    getline(cin, line);

    stringstream ss(line);
    string token;
    while (getline(ss, token, ' ')) {
      cout << token[0];
    }

    cout << '\n';
  }

  return 0;
}
