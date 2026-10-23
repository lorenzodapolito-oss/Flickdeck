<h1 align="center">Flickdeck</h1>

<p align="center">
  <b>Programma open source per presentazioni, scritto in Rust, con tema pastello e interfaccia in italiano.</b>
</p>

Flickdeck è una versione italiana di [DeckCraft](https://github.com/storytold/deckcraft), il programma
di presentazioni open source dell'ArtCraft Team scritto in puro Rust. Funziona in modo nativo su
Windows, macOS e Linux. Usa il formato **.pptx** come formato nativo, quindi i file si scambiano
normalmente con PowerPoint.

## Cosa cambia rispetto a DeckCraft

- **Nome e programma**: il file eseguibile si chiama `Flickdeck.exe`.
- **Tema pastello**: avorio, azzurro polvere e verde salvia, testo antracite, nessun bianco abbagliante
  (il bianco puro resta solo nei campi di testo).
- **Modalità portatile**: crea una cartella `flickdeck-data` accanto al programma e impostazioni e log
  restano lì (vedi sotto).
- **Interfaccia in italiano**: in corso. DeckCraft non ha un sistema di traduzione, quindi va costruito:
  finché non è completo, ciò che manca appare in inglese.

Tutto il resto, cioè motore delle diapositive, forme, tabelle, grafici, transizioni, animazioni e
presentazione a schermo intero, è il lavoro di DeckCraft, che è in sviluppo iniziale.

## Come compilare su Windows

Servono [Rust](https://rustup.rs) (toolchain `x86_64-pc-windows-gnu`) e un compilatore C per Windows,
ad esempio [WinLibs](https://winlibs.com) (UCRT, 64 bit). **Il percorso della cartella di WinLibs non
deve contenere spazi.**

```
git clone https://github.com/lorenzodapolito-oss/Flickdeck
cd Flickdeck
packaging\windows\Flickdeck.bat
```

Il programma finito è `dist\Flickdeck\Flickdeck.exe`. La prima compilazione richiede parecchio tempo.
Per provarlo direttamente: `cargo run --release -p flickdeck -- --sample`.

## Modalità portatile

Crea una cartella chiamata `flickdeck-data` nella stessa cartella di `Flickdeck.exe`: impostazioni,
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
