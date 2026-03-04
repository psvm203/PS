#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n;
  cin >> n;

  map<string, string> rooms{
      {"Algorithm", "204"},
      {"DataAnalysis", "207"},
      {"ArtificialIntelligence", "302"},
      {"CyberSecurity", "B101"},
      {"Network", "303"},
      {"Startup", "501"},
      {"TestStrategy", "105"},
  };

  while (n--) {
    string seminar;
    cin >> seminar;
    cout << rooms[seminar] << '\n';
  }

  return 0;
}
