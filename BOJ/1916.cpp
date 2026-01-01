#include <bits/stdc++.h>

using namespace std;

int INF = 987654321;

int main() {
    cin.tie(nullptr)->sync_with_stdio(false);

    int n, m;
    cin >> n >> m;
    vector<vector<tuple<int, int> > > paths(n + 1);

    while (m--) {
        int b, e, cost;
        cin >> b >> e >> cost;
        paths[b].emplace_back(cost, e);
    }

    int start, finish;
    cin >> start >> finish;
    vector dists(n + 1, INF);
    dists[start] = 0;

    priority_queue<tuple<int, int> > pq;
    pq.emplace(0, start);
    while (pq.size()) {
        auto [dist, cur] = pq.top();
        pq.pop();
        dist *= -1;
        if (dist > dists[cur]) {
            continue;
        }
        for (auto [cost, next]: paths[cur]) {
            int n_dist = dist + cost;
            if (n_dist >= dists[next]) {
                continue;
            }
            dists[next] = n_dist;
            pq.emplace(-n_dist, next);
        }
    }

    cout << dists[finish];

    return 0;
}
