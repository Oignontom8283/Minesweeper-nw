
# Optimizations

## Memory

### Bitpacking

J'utilise du bitpacking pour stocker les données des cellules de la grille de jeu.
Au lieu d'utiliser une structure qui prendrait plusieurs octets pour les données d'une seule cellule,
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
| Bit 3    | `0b0000_1000` | *non utilisé* |
| Bits 4–7 | `0b1111_0000` | **Mines adjacentes** - nombre de 0 à 8     |

J'alloue 4 bits pour le nombre de mines adjacentes, car `2^4 = 16`, ce qui est suffisant.
Pourquoi pas 3 bits ? Car `2^3 = 8` donc `8-1=7` ; j'aurais dû faire des opérations de décalage de bit et je n'en avais pas la motivation 😅.

Ce système réduit l'utilisation en mémoire par 4. Une struct standard dans sa meilleure
configuration prendrait un octet par champ, donc 4 octets au lieu d'un. Ainsi, pour une grille de `10*10 = 100` -> `100 * 4 = 400`,
ce qui donne 0,39 Ko, alors qu'avec le bitpacking on utilise seulement `100 * 1 = 100` soit 0,097 Ko.

Vous pouvez trouver le code spécifié dans [`src/grid.rs`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/grid.rs)
au début du fichier.

### Fragmentation de la mémoire

Pour éviter la fragmentation de la mémoire (ce qui peut arriver très vite), au lieu de désallouer le `Vec`
qui stocke les données de la grille, je le réutilise avec `.clear() + .resize()`.
Cela évite d'avoir des trous dans la mémoire et donc d'avoir des espaces libres inutilisables car trop petits.

Emplacement dans le code : [`src/scenes/playing.rs#L14`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L14)

## Algorithms

La fonction `reveal_infect` est utilisée pour révéler les cases vides (0 mines adjacentes) et leurs voisines.
J'ai utilisé une approche itérative (avec ma propre liste des cases à révéler) plutôt que
récursive pour éviter les problèmes de débordement de pile (stack overflow).

Elle ne fait que retourner les cellules à redessiner (redraw) et s'occupe de les définir comme révélées.

Également, je n'aime pas les fonctions récursives, peu importe le langage.
c'est souvent plus gourmand en ressources qu'une approche itérative.

La fonction : [`src/grid.rs#L132`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/grid.rs#L132)

## Rendu

### Dirty render

L'écran de la calculatrice a une forte déchirure d'image (screen tearing), ce qui m'empêche de redessiner l'écran à chaque actualisation.

J'utilise donc un système de Dirty rect, via un pipeline d'instructions de rendu,
je ne réaffiche que les éléments qui ont été modifiés (dirty) et pas tout l'écran.

#### Cursor

Lorsque le curseur bouge, seule la cellule d'où il provient est redessinée.
Pour la cellule où il se déplace, le curseur est dessiné par-dessus l'affichage existant.

[`src/render.rs#L64`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/render.rs#L64)

#### Diffusion des cellules révélées

La fonction `reveal_infect` renvoie la liste des cellules à redessiner,
et c'est la logique du jeu qui s'occupe d'envoyer cette liste de cellules dans le pipeline de rendu.

[`src/scenes/playing.rs#215`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L215)
[`src/grid.rs#L132`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/grid.rs#L132)


#### Timer

L'affichage de l'horloge du jeu est mis à jour une fois par seconde, juste après que le temps s'est incrémenté d'une seconde.
C'est le même problème : je ne peux pas redessiner l'horloge constamment car cela créerait des artefacts visuels à l'écran.

[`src/scenes/playing.rs#234`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L234)

## Sauvegarde

Le système de fichiers de la calculatrice a une taille très limitée (42 Ko) ; on ne peut donc pas enregistrer de gros fichiers sans raison.

C'est pour cela qu'au lieu d'utiliser un format de fichier texte classique (genre JSON), j'enregistre directement des binaires
qui sont des sérialisations de structures (comme `GameSave`), sérialisées/désérialisées avec `serde` et `postcard`.
Ce qui rend les fichiers de sauvegarde très compacts.

[`src/save.rs`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/save.rs)
[`src/common.rs#127`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/common.rs#L127)
[`src/common.rs#142`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/common.rs#L142)
