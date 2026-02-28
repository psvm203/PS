#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  map<char, char> mirror{{'i', 'i'}, {'o', 'o'}, {'v', 'v'},
                         {'w', 'w'}, {'x', 'x'}, {'b', 'd'},
                         {'d', 'b'}, {'p', 'q'}, {'q', 'p'}};

  string word;
  cin >> word;

  auto solve = [&]() {
    string result;
    for (char ch : word) {
      if (mirror.find(ch) == mirror.end()) {
        return string("INVALID");
      }

      result = result + mirror[ch];
    }

    reverse(result.begin(), result.end());
    return result;
  };

  while (word[0] != '#') {
    cout << solve() << '\n';
    cin >> word;
  }

  return 0;
}
