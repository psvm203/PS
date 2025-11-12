#include <iostream>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    string p, s;
    cin >> p >> s;

    auto solve = [&]() {
      int pi = 0;
      int si = 0;

      while (pi < p.size() && si < s.size()) {
        char ch = p[pi];
        int p_cnt = 0;
        for (; pi < p.size(); pi++) {
          if (p[pi] != ch) {
            break;
          }
          p_cnt++;
        }

        int s_cnt = 0;
        for (; si < s.size(); si++) {
          if (s[si] != ch) {
            break;
          }
          s_cnt++;
        }

        bool is_allowed = p_cnt <= s_cnt && s_cnt <= p_cnt * 2;
        if (!is_allowed) {
          return false;
        }
      }

      return pi == p.size() && si == s.size();
    };

    string result = solve() ? "YES" : "NO";
    cout << result << '\n';
  }

  return 0;
}
