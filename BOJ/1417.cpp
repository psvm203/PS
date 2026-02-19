#include <bits/stdc++.h>
#include <cmath>

using namespace std;

int main() {
  cin.tie(0)->sync_with_stdio(0);

  int n;
  cin >> n;

  if (n == 1) {
    cout << '0';
    return 0;
  }

  int dasom;
  cin >> dasom;

  multiset<int> votes;
  for (int i = 0; i < n - 1; i++) {
    int vote;
    cin >> vote;
    votes.emplace(vote);
  }

  int result = 0;
  while (true) {
    auto most_vote = votes.end();
    most_vote--;
    if (*most_vote < dasom)
      break;
    votes.emplace(*most_vote - 1);
    votes.erase(most_vote);
    dasom++;
    result++;
  }

  cout << result;

  return 0;
}
