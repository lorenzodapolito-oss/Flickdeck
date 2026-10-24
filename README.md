<h1 align="center">Flickdeck.it</h1>

<p align="center">
  <b>Programma open source per presentazioni, scritto in Rust, con tema pastello e interfaccia in italiano.</b>
</p>

Flickdeck.it è una versione italiana di [DeckCraft](https://github.com/storytold/deckcraft), il programma
di presentazioni open source dell'ArtCraft Team scritto in puro Rust. Funziona in modo nativo su
Windows, macOS e Linux. Usa il formato **.pptx** come formato nativo, quindi i file si scambiano
normalmente con PowerPoint.

## Cosa cambia rispetto a DeckCraft

- **Nome e programma**: il file eseguibile si chiama `Flickdeck.it.exe`.
- **Tema pastello**: avorio, azzurro polvere e verde salvia, testo antracite, nessun bianco abbagliante
  (il bianco puro resta solo nei campi di testo).
- **Modalità portatile**: crea una cartella `flickdeck-data` accanto al programma e impostazioni e log
  restano lì (vedi sotto).
- **Interfaccia in italiano**: nastro, menu, finestre di dialogo e pannelli sono tradotti
  (circa 1.200 testi, in `crates/ui-egui/src/i18n/it.tsv`). Ciò che non è ancora tradotto appare in
  inglese. Per l'interfaccia inglese avvia il programma con la variabile `FLICKDECK_LANG=en`.

Tutto il resto, cioè motore delle diapositive, forme, tabelle, grafici, transizioni, animazioni e
presentazione a schermo intero, è il lavoro di DeckCraft, che è in sviluppo iniziale.

## Dizionari

Controllo ortografico e sinonimi usano i dizionari Hunspell e MyThes (italiano per impostazione
predefinita). Il programma li cerca nella cartella `dictionaries` accanto a `Flickdeck.it.exe`, in
`flickdeck-data\dictionaries` e nella cartella indicata da `DOCPLANE_DICTIONARIES`: è la stessa
cartella che usano WordBook.it e SheetCalc.it nel pacchetto DocPlane.it, che li scarica per te.
I dizionari non sono nel repository: quello italiano è sotto licenza GPL 3.

## Come compilare su Windows

Servono [Rust](https://rustup.rs) (toolchain `x86_64-pc-windows-gnu`) e un compilatore C per Windows,
ad esempio [WinLibs](https://winlibs.com) (UCRT, 64 bit). **Il percorso della cartella di WinLibs non
deve contenere spazi.**

```
git clone https://github.com/lorenzodapolito-oss/Flickdeck
cd Flickdeck
packaging\windows\Flickdeck.bat
```

Il programma finito è `dist\Flickdeck.it\Flickdeck.it.exe`. La prima compilazione richiede parecchio tempo.
Per provarlo direttamente: `cargo run --release -p flickdeck -- --sample`.

## Modalità portatile

Crea una cartella chiamata `flickdeck-data` nella stessa cartella di `Flickdeck.it.exe`: impostazioni,
preferenze e log vengono salvati lì invece che nel profilo di Windows. Cancellala per tornare al
comportamento normale.

## Per chi vuole approfondire

La documentazione tecnica del progetto originale (architettura, canale di controllo per gli agenti,
MCP, versione web) è nella cartella [`docs/`](docs/) e nel
[repository di DeckCraft](https://github.com/storytold/deckcraft).

## Licenza e crediti

`MIT OR Apache-2.0`, come il progetto originale. Copyright dell'ArtCraft Team e dei collaboratori di
DeckCraft; le modifiche di questo fork sono rilasciate con la stessa licenza. Le risorse grafiche e i
loro autori sono elencati in [ATTRIBUTION.md](ATTRIBUTION.md).

## Scarica per Windows

**[Scarica DocPlane.it (zip)](https://github.com/lorenzodapolito-oss/DocPlane/releases/latest/download/DocPlane.it.zip)** · [tutte le versioni](https://github.com/lorenzodapolito-oss/DocPlane/releases)

Flickdeck.it è nel pacchetto **DocPlane.it**, insieme agli altri due programmi (WordBook.it, SheetCalc.it, Flickdeck.it)
e ai dizionari. Decomprimi lo zip in una cartella e apri `DocPlane.it.exe`, oppure direttamente `Flickdeck.it.exe`.
Al primo avvio Windows può mostrare «Windows ha protetto il PC» (i programmi non sono firmati):
scegli *Ulteriori informazioni*, poi *Esegui comunque*.
