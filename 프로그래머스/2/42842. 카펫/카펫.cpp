#include <string>
#include <vector>

using namespace std;

/*
    brown: x * 2 + (y - 2) * 2
    yellow: (x - 2) * (y - 2)
    brown + yellow: xy
*/

vector<int> solution(int brown, int yellow) {
    int xy = brown + yellow;
    for (int x = 3; x < xy; x++) {
        if (xy % x != 0) {
            continue;
        }
        int y = xy / x;
        if ((x - 2) * (y - 2) == yellow) {
            return {max(x, y), min(x, y)};
        }
    }
    
    return {};
}