bool:

- Description

- current valeur: (toogle)
- valeur par default (Apply Button)

# Default

un node peut avoir 0..\* default value.

sa selection depends du dernier parent modifier

si c'est lui meme -> on utilise sa valeur
si c'est son parent -> on utilise son default

case1: si on modifie only un bool bien nested

- son default sera

rules:

- on affiche le default que quand il appartient au node.
- utiliser le default devra seulement supprimer cet element.
