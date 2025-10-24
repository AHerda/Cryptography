#import "@preview/lilaq:0.5.0" as lq
#set page(width: 16cm, height: 8cm)
#show: lq.set-diagram(width: 100%, height: 100%)

#let data = csv("benchmark_md5.csv").slice(1)
#let xs = data.map(row => float(row.at(0)))
#let md1 = data.map(row => float(row.at(1)))
#let md2 = data.map(row => float(row.at(2)))

#lq.diagram(
  title: [md5 benchmark],
  xlabel: [String length], 
  ylabel: [time (ns)],

  lq.plot(xs, md1, mark: none, label: [My MD5]),
  lq.plot(xs, md2, mark: none, label: [Cargo library])
)