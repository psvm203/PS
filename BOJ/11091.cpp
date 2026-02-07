#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  auto check_pangram = [&](string &x) {
    set<char> chars;
    for (char ch : x) {
      chars.emplace(tolower(ch));
    }

    string missings;
    for (char ch = 'a'; ch <= 'z'; ch++) {
      if (chars.find(ch) == chars.end()) {
        missings += ch;
      }
    }

    if (missings.empty()) {
      return string("pangram");
    }

    return "missing " + missings;
  };

  int n;
  cin >> n;
  cin.ignore();
  while (n--) {
    string sentence;
    getline(cin, sentence);
    cout << check_pangram(sentence) << '\n';
  }
}
