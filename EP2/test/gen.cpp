#include <bits/stdc++.h>

int main() {
	std::mt19937 rng(std::chrono::steady_clock::now().time_since_epoch().count());
	const int N = 999;
	std::vector<std::string> v(N);
	for(int i=0;i<N;i++) {
		v[i] = std::to_string(std::uniform_int_distribution<int>(1,N)(rng));
	}
	std::cout << N << '\n';
	for(auto s: v) std::cout << s << ' ';
	std::cout << '\n';
	std::cout << 3*N+4 << '\n';
	std::cout << 1 << ' ' << N << '\n';
	for(int i=0;i<=N;i++) std::cout << 2 << ' ' << i << '\n';
	for(int i=0;i<=N;i++) std::cout << 3 << ' ' << i << '\n';
	for(int i=0;i<=N;i++) std::cout << 4 << ' ' << i << '\n';
}
