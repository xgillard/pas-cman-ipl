# Pas-Cman

![Screen Shot](ScreenShot.png "ScreenShot")

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

## Compiler le Jeu

Comme le jeu en lui même n'est pas écrit en _C_ mais en _Rust_ (il s'agit toutefois de deux langages très proches), 
vous allez devoir utiliser le compilateur Rust pour transformer le code source en fichier executable. 
Pour installer ce compilateur, il vous suffit de suivre les instrcutions qui vous sont données sur 
(cette page)[https://www.rust-lang.org/fr/learn/get-started]. Sur votre machine linux, il vous suffira 
sans doute d'exécuter la ligne de code suivante: 

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Après ca, vous pourrez taper `cargo install --release` dans votre terminal, ce qui aura pour effet de produire un
fichier binaire dans le répertoie `target/release` du projet. Une fois le jeu compilé, vous pourrez copier
ce fichier binaire ainsi que le dossier `resources` qui est nécessaire pour que le jeu puisse s'exécuter
correctement.


## Credits
This game includes artwork by "sethbyrd.com". For more info about this work or its creator, check: "www.sethbyrd.com", 
https://opengameart.org/content/cute-characters-monsters-and-game-assets 