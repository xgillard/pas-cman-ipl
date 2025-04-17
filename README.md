# Pas-Cman

Pas-cman c'est pas pacman. 

Ce repository contient le code source de l'interface du jeu multi-joueur que vous allez devoir 
réaliser dans le cadre de votre projet pour le cours **BINV2182-2: Programmation Distribuée**
à l'Institut Paul Lambin (HE Léonard de Vinci).

Le code source du jeu a été écrit par vos profs adorés, et le jeu complet (dans sa version 
mono joueur) est disponible (ici)[https://github.com/xgillard/pas-cman]. Ce jeu est écrit 
en Rust, et en utilisant une architecture ECS. Il s'agit donc d'un langage de programmation
que vous ne connaissez pas encore, et d'une architecture logicielle que vous n'avez encore
jamais rencontré non plus. 

**CEST POURQUOI NOUS VOUS ENCOURAGEONS ACTIVEMENT A ATTENDRE QUE LE PROJET SOIT FINI AVANT D'ALLER EN LIRE LE CODE**


## Prérequis d'installation

Afin d'éviter toute une série de problèmes réels et potentiels, le projet ne sera pas implémenté sur le serveur courslinux mais
plutôt en local. Si vous n'avez pas déjà une machine Linux installée sur votre PC, nous vous proposons d'utiliser WSL (Windows Subsystem for Linux).
Développé par Micro$oft, WSL est une fonctionnalité de Windows qui permet d'exécuter des distributions Linux directement sur un système Windows. Contrairement à une machine virtuelle, WSL utilise une couche de compatibilité pour exécuter les binaires Linux de manière native. 
Il permet aux développeurs d'utiliser des outils Linux directement sur Windows, facilitant ainsi le développement multiplateforme.

Voici deux liens expliquant comment installer WSL 2 sur Windows: 

https://blog.stephane-robert.info/docs/admin-serveurs/linux/wsl2/
https://doc.ubuntu-fr.org/wsl

Choisissez la distro `Ubuntu`. Elle correspond à la dernière version LTS d'Ubuntu (24.04).

Si vous travaillez sur un PC Vinci, adressez-vous aux enseignants qui vous expliqueront comment utiliser WSL-Ubuntu sur celui-ci.


## Etape 1: Compiler le Jeu

Afin de compiler le programme `pas-cman-ipl` sur votre machine linux (ou WSL sur windows), vous aurez sans doute besoin d'installer
les librairies suivantes (si vous utilisez une distribution desktop, il y a de fortes chances que tous ces paquets soient 
déjà préinstallés):

```
sudo apt install cargo cmake pkg-config libfontconfig-dev librust-servo-fontconfig-sys-dev libwayland-bin libxrandr2 libxi6 libx11-xcb1 libgl1 libxcursor1
```

Comme le jeu en lui-même n'est pas écrit en _C_ mais en _Rust_ (il s'agit toutefois de deux langages très proches), 
vous allez devoir utiliser le compilateur Rust pour transformer le code source en fichier executable. 
Si celui-ci n'est pas installé, suivez les instrcutions qui vous sont données sur cette page:
https://www.rust-lang.org/fr/learn/get-started. 

Après ça, allez dans le répertoire `student_kit` puis tapez `cargo build --release` dans votre terminal, 
ce qui aura pour effet de produire le fichier binaire `pas-cman-ipl` dans le répertoie `target/release` du projet. 
Une fois le jeu compilé, vous pourrez copier ce fichier binaire ainsi que le dossier `resources` 
qui est nécessaire pour que le jeu puisse s'exécuter correctement.


## Etape 2: Tester l'installation du jeu

Afin de vous aider à y voir plus clair, nous vous avons aussi fourni le programme `exemple.c` qui lit un fichier 
comprenant une représentation textuelle de la map et écrit tous les messages nécessaires sur sa sortie standard.
Ce petit programme d'exemple continue ensuite en faisant courir en boucle un méchant derriere le heros.

Vous pourrez donc tester que tout fonctionne bien chez vous en lançant les commandes suivantes dans le répertoire
`student_kit`. Le jeu bouclera quelques secondes puis s'arrêtera.

```
# dans le répertoire `student_kit`
make
./exemple | ./target/release/pas-cman-ipl
```

Pour faire tourner le programme d'exemple (ou tester votre application en localhost) sur le serveur courslinux:
- dans WSL: se connecter au serveur courslinux via `ssh -X -p 4980 login@courslinux.vinci.be`
- si vous n'avez pas WSL:
      --> installer un server X sur Windows:  https://sourceforge.net/projects/vcxsrv/
      --> se connecter au serveur courslinux via `ssh -X -p 4980 login@courslinux.vinci.be` dans Powershell

```
# copiez localement le répertoire `student_kit` et déplacez-vous dans ce répertoire
make
./exemple | pas-cman-ipl
```


## Etape 3: Se familiariser avec le protocole du jeu

Dans le fichier `pascman.h`, vos professeurs adorés vous ont fourni la définition des structures qui sont nécessaires
afin de pouvoir piloter l'interface de votre jeu depuis un système extérieur. Ce protocole est somme toutes assez
simple: il consiste à envoyer une série de messages (des records binaires) à l'interface graphique en écrivant 
sur son entrée standard.

Vous devez donc commencer par aller lire le header qui vous est fourni afin de comprendre le fonctionnement global de
ce protocole et vous familiariser avec les messages utilisés.


## Credits
This game includes artwork by "sethbyrd.com". For more info about this work or its creator, check: "www.sethbyrd.com", 
https://opengameart.org/content/cute-characters-monsters-and-game-assets 
