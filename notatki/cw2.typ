#import "template.typ": *

#show: project.with(
  title: [Kryptografia\ Ćwiczenia 2],
  date: datetime.today().display(),
  authors: ((name: "Adrian Herda", affiliation: "Politechnika Wrocławska"),)
)

= Zadanie 2

$Pr["Enc"(k, m_0) = c] = Pr["Enc"(K, m_1) = c]$, $cal(k)$ - uniform distribution

$ ( forall_(m_0, m_i in cal(M)) ) ( forall_(c in cal(C)) ) Pr[m = m_0 | "Enc"(k, m) = c] = Pr[m = m_0] $
#proof[$
  Pr[m=m_0] &= 1/( |m| )\
  Pr[m = m_0 | "Enc"(k, m) = c] &= Pr[m = m_0 and "Enc"(k,m) = c] / Pr["Enc"(k,m) = c]\
  &= Pr[m = m_0 and "Enc"(k, m_0) = c] / (sum_(m_i in M) Pr[m = m_i and "Enc"(k, m_1) = c])\
  &= ( Pr[m=m_0] dot Pr["Enc"(k, m_0] = c] ) / (sum_(m_i in M) Pr[m = m_i] Pr["Enc"(k, m_i) = c])\
  &= (Pr[m = m_0] Pr["Enc"(k, m_0) = c]) / (sum_(m_i in M) Pr[m = m_i] Pr["Enc"(k,m_0) = c])\
  &= Pr[m = m_0] / (sum_(m_i in M) Pr[m =m_i]) = p_0
$]
