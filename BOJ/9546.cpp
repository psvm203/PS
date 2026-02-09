#include <bits/stdc++.h>
#define int int64_t
using namespace std;
signed main(){
    cin.tie(0)->sync_with_stdio(0);
    int t;
    cin>>t;
    while(t--){
        int n;
        cin>>n;
        int res=0;
        while(n--)res=res*2+1;
        cout<<res<<'\n';
    }
    return 0;
}