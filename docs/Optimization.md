
# Optimizations

## Memory

### Bitpacking

J'utilise du Bitpacking pour stocker les données des cellules de la grille de jeu.
À la place d'utiliser une structure qui prendrait plusieurs octets pour les données d'une seule cellule,
j'utilise un seul octet `u8`. Je stocke les données sur l'octet de la manière suivante :

```
Bit :  7  6  5  4  3  2  1  0
       └───┬────┘     │  │  └─ Is mine
           │          │  └──── Is revealed
           │          └─────── Is flagged
           │       
           └────────────────── Mines adjacentes (0-8)
```

Pour les connaisseurs du bas niveau, les masques sont les suivants :
| Bits     | Masque        | Rôle                                       |
|----------|---------------|--------------------------------------------|
| Bit 0    | `0b0000_0001` | **Mine** - 1 = il y a une mine             |
| Bit 1    | `0b0000_0010` | **Révélée** - 1 = la case a été découverte |
| Bit 2    | `0b0000_0100` | **Flag** - 1 = drapeau posé                |
| Bit 3    | `0b0000_1000` | *non utilisé*                              |
| Bits 4–7 | `0b1111_0000` | **Mines adjacentes** - nombre de 0 à 8     |

J'alloue 4 bits pour le nombre de mines adjacentes, car `2^4 = 16` ce qui est suffisant.
Pourquoi pas 3 bits ? Car `2^3 = 8` donc `8-1=7`, j'aurais dû faire des opérations de décalage de bit et je n'en avais pas la motivation 😅.

Ce système réduit l'utilisation en mémoire de 4 fois. Une struct standard dans sa meilleure
configuration prendrait un octet par champ, donc 4 octets au lieu d'un. Ainsi pour une grille de `10*10 = 100` -> `100 * 4 = 400`
ce qui donne 0,33 Ko, alors qu'avec le bitpacking on utilise seulement `100 * 1 = 100` soit 0,097 Ko.

Vous pouvais trouver le code spécifier dans [`src/grid.rs`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/grid.rs)
au début du fichier.

### Fragmentation de la mémoire

Pour éviter la fragmentation de la mémoire (et cela peut arriver très vite), a la place de déssalouer le `Vec`
qui stocke les données de la grille, je le réutilise avec `.clear() + .resize()`.
Cela évite d'avoir des trous dans la mémoire ce qui et donc d'avoirs des esapce libre inutilisable car trop petite.

Emplacement dans le code : [`src/scenes/playing.rs#L14`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L14)

## Algorithms

la fonction [`reveal_infect`]()
est utilisée pour révéler les cases vides (0 mines adjacentes) et leurs voisins.
J'ai utilisé une approche itérativee (avec ma propre list des cases à révéler) plutôt que récursive pour éviter les problèmes de débordement de pile (stack overflow).

Elle ne fait que return les cellules a redraw, et s'occupe de les set comme revlée.

Également, je n'aime pas les fonction récursive peut importent le langage,
c'est souvent plus gourmand en ressources que approche itérative.

La fonction : [`src/grid.rs#L132`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/grid.rs#L132)

## Rendu

### Dirty render

L'écran de la calculatrice a une forte déchirrure d'image (screen tearing) ce qui m'empêche donc de redessiner l'écran a chaque actuallisation.

Donc j'utilise un system de Dirty rect, ou via un pipline d'instruction de rendu,
je ne reaffiche que les éléments qui ont été modifiés (dirty) et pas tout l'écran.

#### Cursor

L'ors que le curseur bouge, seulement la cellul d'ou il provien est redessiner.
Pour la cellule ou il ce déplace, le curseur est déssiner par dessus l'affichage existant.

[`src/render.rs#L64`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/render.rs#L64)

#### Difusion des cellules révélées

La fonction `reveal_infect` renvois la list des cellules a redessiner,
et c'est la logique du jeu qui s'occupe d'envoyer cette list de cellules dans le pipline de rendu.

[`src/scenes/playing.rs#215`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L215)


#### Timer

L'affichage de l'orloge du jeu est mie a jours une fois tout les seconde, juste après le temps ce sois incrémenter d'une seconde.
C'est le même probléme, je ne peut pas redessiner lorgue constament car ça crérais des artefact visuel a l'écran.

[`src/scenes/playing.rs#234`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L234)

## Sauvegarde

Le system de fichier de la calculatrice a une taille très limitée (42Ko) l'on ne peut donc pas enregistrée de gros fichier sens raison.

C'est pour cela qu'aulieu d'utiliser un format de fichier texte classique (genre json) j'enregistre directement des binaire 
de qui son des Serialisation de structure (comme `GameSave`), sérialiser/désérialiser avec `serde` et `postcard`.

[`src/save.rs`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/save.rs)
[`src/common.rs#127`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/common.rs#L127)
[`src/common.rs#142`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/common.rs#L142)
