#include <bits/stdc++.h>
#include <float.h>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n, m;
  cin >> n >> m;
  vector<string> board(n);
  for (auto &row : board) {
    cin >> row;
  }

  int dr[]{-1, 0, 1, 0};
  int dc[]{0, -1, 0, 1};
  int result = 0;
  for (int r = 0; r < n; r++) {
    for (int c = 0; c < m; c++) {
      if (board[r][c] != 'X')
        continue;
      for (int dir = 0; dir < 4; dir++) {
        int nr = r + dr[dir];
        int nc = c + dc[dir];
        if (nr == -1 || nr == n || nc == -1 || nc == m)
          continue;
        result += board[nr][nc] == 'Y';
      }
    }
  }
  cout << result;

  return 0;
}
