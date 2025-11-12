#include <string>
#include <vector>
#include <algorithm>

using namespace std;

bool solution(vector<string> phone_book) {
    sort(phone_book.begin(), phone_book.end());
    
    auto is_prefix = [&](string &a, string &b) {
        auto it_a = a.begin();
        auto it_b = b.begin();
        
        while (it_a != a.end() && it_b != b.end()) {
            if (*it_a != *it_b) {
                return false;
            }
            it_a++;
            it_b++;
        }
        
        return true;
    };
    
    for (int i = 1; i < phone_book.size(); i++) {
        if (is_prefix(phone_book[i - 1], phone_book[i])) {
            return false;
        }
    }
    
    return true;
}