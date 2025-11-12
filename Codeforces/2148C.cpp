#include <iostream>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int n, m;
    cin >> n >> m;

    int side = 0;
    int prev_time = 0;
    int point = 0;
    for (int i = 0; i < n; i++) {
      int time, req_side;
      cin >> time >> req_side;

      side ^= (time - prev_time) % 2;
      point += time - prev_time;
      if (side != req_side) {
        side = req_side;
        point -= 1;
      }

      prev_time = time;
    }

    point += m - prev_time;

    cout << point << '\n';
  }

  return 0;
}
