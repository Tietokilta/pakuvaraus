#import sys: inputs

#set text(
  font: "Libertinus Serif",
  size: 12pt,
)

#let full_name = inputs.at("full_name")
#let address = inputs.at("address")
#let phone_number = inputs.at("phone_number")
#let email = inputs.at("email")
#let hetu = inputs.at("hetu")
#let used_outside_finland = inputs.at("used_outside_finland")
#let start_time = inputs.at("start_time")
#let end_time = inputs.at("end_time")

#align(center)[
  #text(size: 18pt, weight: "bold")[Tietokilta ry:n pakettiauton luovutussopimus]\
  #text(size: 15pt)[Lainattava ajoneuvo: Opel Vivaro, rekisteritunnus FOL-572]
]

Allekirjoittamalla tämän sopimuksen hyväksyn seuraavat ehdot:

#pad(left: 1em)[
+ Lainaaja on Tietokilta ry:n (jäljempänä Kilta) jäsen.
+ Laina-aika alkaa ajoneuvon luovutuksen kuittauksen yhteydessä, ja päättyy, kun Killan edustaja on
  kuitannut ajoneuvon palautetuksi.
+ Lainaaja noudattaa laina-aikana Suomen lakia ja lainaajalla on voimassa oleva B-luokan ajo-oikeus.
+ Lainaaja on vastuussa ajoneuvosta, sen varusteista ja sen käytöstä laina-aikana.
+ Lainaaja ei luovuta ajoneuvoa Killan ulkopuoliselle osapuolelle. Killan jäsenet voivat toimia
  laina-aikana ajoneuvon kuljettajana, mutta vastuu ajoneuvon käytöstä säilyy sopimuksen
  allekirjoittaneella.
+ Lainaaja maksaa ajoneuvon käytöstä voimassaolevan hinnaston mukaisen käyttökorvauksen.
+ Lainaaja tarkistaa ajoneuvon luovutuksen yhteydessä, että kaikki ajoneuvossa luovutushetkellä
  olevat viat sekä varusteet on merkitty listaan sopimuksen kääntöpuolella.
+ Lainaaja on velvollinen korvaamaan autolle ja sen varusteille vuokra-aikana sattuneet vahingot.
  Kilta laskuttaa asiakasta Killan valitseman korjaamon korjauskustannuslaskelman mukaan.
  Korvausvastuun yläraja on 500 euroa.
+ Lainaaja ilmoittaa syntyneestä vahingosta välittömästi Killan edustajalle. Ilmoittamattomista
  vahingoista korvausvastuun yläraja on kaksinkertainen.
+ Mikäli lainaaja aiheuttaa vaurioita toiseen autoon, laskuttaa Kilta lainaajaa toisen auton
  korjauskustannusten mukaan korvausvastuun ylärajaan asti, riippumatta siitä, korvaako Killan
  ajoneuvon liikennevakuutus vaurion.
+ Kilta ei ole velvollinen korjauttamaan ajoneuvolle sattuneita vaurioita laskuttaakseen lainaajaa
  aiheutetuista vahingoista.
+ Lainaaja on velvollinen maksamaan vuokra-aikana auton käyttämisestä aiheutuvat rikesakko-,
  pysäköinti-, ja pysäköintivirhemaksut kaikkine kustannuksineen. Maksamattomista maksuista Kilta
  perii veloitettavan summan lisäksi 50 euron käsittelymaksun.
+ Lainaaja on velvollinen korvaamaan auton epätavanomaisesta likaantumisesta johtuvat siivous- ja
  puhdistuskustannukset.
+ Lainaaja valokuvaa ajoneuvon joka puolelta ennen ajoa ja ajon jälkeen, jotta lainaaja voi
  tarvittaessa todistaa, missä kunnossa ajoneuvo luovutettiin.
+ Ajoneuvon vienti Suomen ulkopuolelle ilman Killan kirjallista lupaa on kielletty.
]
\
\

#let yes-no-box(label, checked: false) = align(center)[
  #label
  #v(-0.5em)
  #box(
    width: 1.2em, height: 1.2em,
    stroke: 0.6pt + black,
  )[
    #align(center + horizon)[
      #if checked [#text(weight: "bold")[#sym.crossmark]]
    ]
  ]
]

#let yes-no-question(value, body) = grid(
  columns: (auto, 1fr),
  column-gutter: 1.5em,
  align: (horizon, horizon),
  grid(
    columns: 2,
    column-gutter: 1.5em,
    yes-no-box("KYLLÄ", checked: value),
    yes-no-box("EI", checked: not value),
  ),
  body,
)

#yes-no-question(used_outside_finland)[
  Ajoneuvoa käytetään laina-aikana Suomen ulkopuolella. Olen vastaanottanut
  ulkomailla tarvittavan maastavientiluvan Killan edustajalta. Palautan
  asiakirjan ajoneuvon palautuksen yhteydessä.
]

#pagebreak()

#let field(label, value) = grid(
  columns: (4cm, 1fr),
  column-gutter: 0.6em,
  [#label], [#value],
)

#let form-fields(..pairs) = stack(
  spacing: 1em,
  ..pairs.pos().map(p => field(p.at(0), p.at(1)))
)

Lainaajan tiedot:

#form-fields(
  ("Nimi:", full_name),
  ("Osoite:", address),
  ("Puhelinnumero:", phone_number),
  ("Sähköposti:", email),
  ("Henkilötunnus:", hetu),
  ("Alkuaika:", start_time),
  ("Loppuaika:", end_time)
)
