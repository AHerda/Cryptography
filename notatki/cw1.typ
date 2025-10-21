#import "template.typ": *

#show: project.with(
  title: [Kryptografia\ Ćwiczenia 1\ Wykład przypominający],
  date: datetime.today().display(),
  authors: ((name: "Adrian Herda", affiliation: "Politechnika Wrocławska"),)
)

Niech $a, b in ZZ$ a $m in ZZ^+$

$ a eq.triple b mod m <=> m|(b - a) $
$ exists_(k in ZZ) a = k * m + b $
$ b = (a mod m) "- b to reszta z dzielenia " a/ m $

$ {0, 1, ..., m - 1} = ZZ_M $
- $+$ - jak w #sym.ZZ, ale wynik $mod m$
- $dot$ - jak w #sym.ZZ, ale wynik $mod m$

$ -x = (0 - x) = cases(m - x ",jeśli" x > 0, 0 ", jeśli" x = 0) $
$ gcd(a, m) = 1 <=> exists_(k in ZZ_m) a * k = 1 $

Zbiór liczb calkowitych dodatnich $<m$, które są względnie pierwsze z $m$ oznaczamy $ZZ_m^*$, $*_(mod m)$

== Algorytm Euklidesa

$
  a &= r_0 = q_1 dot r_1 + r_2 \
  b &= r_1 = q_2 dot r_2 + r_3 \
  & dots.v
$dajsda

== Równanie diofantyczne

$ a x + b y = gcd(a, b) $

== Chińskie tweirdzenie o resztach (ang. CRP)

Mamy układ kongruencji

$
  x eq.triple& a_1 mod m_1\
  x eq.triple& a_2 mod m_2\
  &dots.v\
  x eq.triple& a_r mod m_r
$

$ M = product_(i = 1)^r m_i $
$ forall_(i, j, i != j) gcd(m_i, m_j) = 1 $

Weźmy funkcję $X(x) = (x mod m_1, x mod m_2, dots.c, x mod m_r)$\
Szukamy funkcji $X^(-1)$

#theorem[Dla $1<= i <= r$,
  $
    x eq.triple a_r mod m_r\
    M_i = M / m_i\
    gcd(M_i, m_i) = 1\
    y_i = M_i^(-1) mod m_i
  $
  $
    phi (a_1, dots.c, a_r) &= sum_(i=1)^r a_i M_i y_i mod M\
    &= X^(-1)
  $
]

= Grupy, ciała i ideały
