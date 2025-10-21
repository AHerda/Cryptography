// The project function defines how your document looks.
// It takes your content and some metadata and formats it.
// Go ahead and customize it to your liking!
#let project(
  title: [],
  abstract-pl: none,
  abstract-en: none,
  authors: (),
  date: none,
  content: false,
  body,
) = {
  // Set the document's basic properties.
  set document(author: authors.map(a => a.name), title: title)
  set math.equation(numbering: "(1.1)")
  set page(
    numbering: "1",
    number-align: center,
    header:  context { if (counter(page).get().at(0) != 1)  [
      #set text(9pt)
      #if type(title) == type([]) [
        #title.children.filter(s => s != linebreak() and s != [ ]).join(", ")
      ] else if type(title) == str [
        #title.replace("\n", ", ")
      ]
      #h(1fr)
      #if authors.len() > 3 [
        #authors.map(author => author.name.split().at(0).first() + ". " + author.name.split().at(1)).join(", ")
      ] else [
        #authors.map(author => author.name).join(", ")
      ]
      #line(length: 100%)
    ]},
    header-ascent: 20%
  )
  set text(font: "New Computer Modern", lang: "pl")
  show math.equation: set text(weight: 400)
  set heading(numbering: "1.1. ")
  show heading: set block(below: 1em, above: 2em)

  // Title row.
  align(center)[
    #block(text(weight: 700, 1.75em, title))
    #v(1em, weak: true)
    #date
  ]

  // Author information.
  pad(
    top: 0.5em,
    bottom: 0.5em,
    x: 2em,
    grid(
      columns: (1fr,) * calc.min(3, authors.len()),
      gutter: 1em,
      ..authors.map(author => align(center)[
        *#author.name* \
        #author.affiliation.replace(", ", "\n")
      ]),
    ),
  )

  // Main body.
  set par(justify: true, first-line-indent: 1.5em)

  if abstract-pl != none {
    heading(outlined: false, numbering: none, text(0.85em, [Streszczenie]))
    abstract-pl
  }

  if abstract-en != none {
    set text(lang: "en")
    heading(outlined: false, numbering: none, text(0.85em, [Abstract]))
    abstract-en
  }

  if content [
    #outline()
    #pagebreak()
  ]

  body
}

#let c = counter("theorem")
#show heading.where(level: 1): it => [
  #context c.update(i => 0)
  #it
]
#let theorem(body) = block[
  #v(1em)
  #c.step()
  *Theorem #context counter(heading).display().replace(regex("\..*$"), "").#context{c.display()}.*
  #body
  #v(1em)
]

#let proof(body) = block[
  _Proof._ #body
  #align(right)[#sym.qed]
]
