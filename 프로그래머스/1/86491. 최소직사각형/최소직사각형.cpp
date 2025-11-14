#include <string>
#include <vector>
#include <algorithm>

using namespace std;

int solution(vector<vector<int>> sizes) {
    int short_length = 0;
    int long_length = 0;
    for (vector<int> size: sizes) {
        auto [mn, mx] = minmax(size[0], size[1]);
        short_length = max(short_length, mn);
        long_length = max(long_length, mx);
    }
    
    int answer = short_length * long_length;
    return answer;
}