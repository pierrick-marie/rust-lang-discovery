Rust Programming by Example
========================

Tester et expérimenter les exemples du livre « Rust Programming by Example »

![Rust Programming by Example](https://framagit.org/pierrick/test-rust/-/raw/master/wiki/rust-programming-example.jpg)

# License: BSD 3-Clause 

https://raw.githubusercontent.com/pierrick-marie/rust-lang-discovery/master/LICENSE

# Get source code

```
	git clone https://github.com/pierrick-marie/rust-lang-discovery.git
```

# Exemple 1 : Hello World

https://github.com/pierrick-marie/rust-lang-discovery/tree/master/hello-world

Découvrir les bases du langage

* enum
* tuple
* struct
* trait
* macro
* generic

# Exemple 2 : Tetris

https://github.com/pierrick-marie/rust-lang-discovery/tree/master/tetris

Développer un tetris avec la bibliothèque SDL2

* jouer une partie
* enregistrer les scores dans un fichier texte

![Tetris screenshot](https://github.com/pierrick-marie/rust-lang-discovery/raw/master/wiki/tetris/tetris.jpg)

## 2.1. Prérequis

Installer les paquets suivants (Gnu\Linux) : 

* libsdl2-dev 
* libsdl2-gfx-dev 
* libsdl2-image-dev 
* libsdl2-ttf-dev

## 2.2. Contrôles

BAS : descendre la pièce

HAUT : tourner la pièce d'un quart de tour

GAUCHE : déplacer la pièce d'une case vers la gauche

DROITE : déplacer la pièce d'une case vers la droite

ESPACE : déplacer la pièce en bas de la grille

ECHAP : quitte le jeu

## Comment jouer ?

Une nouvelle partie commence dès le lancement du jeux.

La partie s'arrête quand il n'est plus possible d'ajouter une nouvelle pièce dans la grille. Une partie se termine donc quand le joueur est mis en échec.

Le score du joueur est affiché à droite de la grille.
L'historique des scrores du joueur sont enregistré par ordre décroissant de points dans le fichier `assets/score.txt` au format suivant : `date heure minute : points = XX lines = YY`

# Exemple 3 : Music play

https://github.com/pierrick-marie/rust-lang-discovery/tree/master/music-player

Développer un lecteur de music pour desktop

## 3.1. Prérequis

Installer les paquest suivants (Gnu\Linux) :

* libgtk-3-dev
* libmad0-dev
* libpulse-dev
* libgstreamer1.0-dev
* libgstreamer-plugins-base1.0-dev
* gstreamer1.0-plugins-base
* gstreamer1.0-plugins-good
* gstreamer1.0-plugins-bad
* gstreamer1.0-plugins-ugly
* gstreamer1.0-libav
* libgstrtspserver-1.0-dev
* libges-1.0-dev
* libgstreamer-plugins-bad1.0-dev

## Présentation

Le lecteur utilise dans sa version finale la bibliothèque Gstreamer pour lire les fichiers audio.
Pour la partie graphique, le lecteur utilise la bibliothèque Relm (https://github.com/antoyo/relm) basée sur GTK.

![Lecteur de music en rust](https://github.com/pierrick-marie/rust-lang-discovery/raw/master/wiki/music-player/music-player.jpg)

# Command line Server & client FTP

## Documentation

http://abcdrfc.free.fr/rfc-vf/pdf/rfc959.pdf

## Client

https://github.com/pierrick-marie/rust-lang-discovery/tree/master/ftp/client

## Server

https://github.com/pierrick-marie/rust-lang-discovery/tree/master/ftp/server

# Contributing

Do not hesitate to improve to this program. Feel free to send PR or contact me to send comments. You are welcome to fork this project also ;)

# Badges

[![License](https://img.shields.io/badge/License-BSD%203--Clause-green.svg)](https://opensource.org/licenses/BSD-3-Clause) [![made-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
