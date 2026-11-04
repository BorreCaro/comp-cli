// Descomentar pragmas solo si se requiere optimizacion SIMD
// #pragma GCC optimize("O3,unroll-loops")
// #pragma GCC target("avx2,bmi,bmi2,lzcnt,popcnt")

#include <bits/stdc++.h>
using namespace std;

using ll = long long;
using ull = unsigned long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using vi = vector<int>;
using vll = vector<ll>;

#define forn(i, n) for (int i = 0; i < (int)n; ++i)
#define for1(i, n) for (int i = 1; i <= (int)n; ++i)
#define fore(i, l, r) for (int i = (int)l; i <= (int)r; ++i)
#define ford(i, n) for (int i = (int)(n) - 1; i >= 0; --i)
#define fored(i, l, r) for (int i = (int)r; i >= (int)l; --i)
#define all(x) (x).begin(), (x).end()
#define rall(x) (x).rbegin(), (x).rend()
#define sz(x) (int)(x).size()
#define d(x) cout<< #x<< " " << x<<el
#define pb push_back
#define fi first
#define se second
#define el '\n'

const int INF_INT = 1e9 + 7;
const ll INF = 1e18;
const ld EPS = 1e-9;
const ld PI = acos(-1.0L);

// Direcciones 2D
int dr[] = {1,-1,0, 0,1,-1,-1, 1};
int dc[] = {0, 0,1,-1,1, 1,-1,-1};

// Comparacion aproximada en punto flotante (NO usar a + EPS
// < b en set/map,
// porque la equivalencia |a - b| <= EPS no es transitiva y
// rompe std::set).
inline int eps_sgn(ld x) { return (x > EPS) - (x < -EPS); }
inline bool eps_eq(ld a, ld b) {
    return eps_sgn(a - b) == 0;
}
inline bool eps_lt(ld a, ld b) {
    return eps_sgn(a - b) < 0;
}
// Para usar doubles como clave en set/map con rejilla fija,
// cuantizar a ll:
inline ll quantize_eps(ld x, ld step = EPS) {
    return llround(x / step);
}

void io_formatting_tricks() {
    // 1. Imprimir punto flotante con exactamente 6
    // decimales:
    cout << fixed << setprecision(6) << 3.14159265 << "\n";
    // 2. Rellenar con ceros a la izquierda (ej. hora
    // 05:09):
    int h = 5, m = 9;
    cout << setfill('0') << setw(2) << h << ":" << setw(2)
         << m << "\n";
    // 3. Imprimir en base hexadecimal, octal y volver a
    // decimal:
    cout << hex << 255 << " " << oct << 255 << dec << "\n";
}

ostream& operator<<(ostream& os, const pii& pa) { // DEBUGGING
  return os << "("<< pa.fi << ", " << pa.se << ")";
}

void solve() {
    // Logica por caso de prueba
}

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    int t = 1;
    // cin >> t;
    while (t--) solve();
    return 0;
}
