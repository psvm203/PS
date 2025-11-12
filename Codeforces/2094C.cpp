#include <iostream>
#include <vector>

using namespace std;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int t;
  cin >> t;

  while (t--) {
    int n;
    cin >> n;

    vector<vector<int>> board(n, vector<int>(n));
    for (auto &row : board) {
      for (int c = 0; c < n; c++) {
        cin >> row[c];
      }
    }

    vector<int> result(1);
    result.reserve(n * 2);
    for (int num : board[0]) {
      result.emplace_back(num);
    }
    for (int c = 1; c < n; c++) {
      result.emplace_back(board[n - 1][c]);
    }

    int sum = 0;
    for (int num : result) {
      sum += num;
    }
    result[0] = n * (n * 2 + 1) - sum;

    for (int num : result) {
      cout << num << ' ';
    }
    cout << '\n';
  }

  return 0;
}
