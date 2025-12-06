





/* to do first , i need to 
    [] split between commas, 
    [] split between - 
    [] store first in start
    [] store second in end
    [] make loop
    [] i = start
    [] while i <= end
    [] check if the curr is valid with function isvalid
    [] is valid contain these condition
    [] if number is made only of sequence of digits repeated twice
    [] repeated twice.
    [] so 55 twice
    [] 6464  64 twice, and 123123 123 tiwce
    [] after i sum of all he id 
    [] aparently there is a regec for this pattern
    [] ^(\d+)\1
    explanation of this pattern : debut de la chaine
    (\d+) capture un ou plusieurs chiffres dans le groupe 1
    \1: reference arriere (backreference) qui doit correspondre exactement a ce qui
    a ete capture dans le groupe 1
    $ fin de la chaine
    exemple 
    Texte à analyser : 123123
    Regex : ^(\d+)\1$

    Étape 1 : ^ 
    → On est au début ✓

    Étape 2 : (\d+)
    → Cherche des chiffres, trouve "123"
    → Mémorise dans groupe 1 : "123"

    Étape 3 : \1
     → Doit correspondre au contenu du groupe 1, donc "123"
    → Compare avec les caractères suivants : "123"
    → Ça correspond ! ✓

    Étape 4 : $
    → On est à la fin ✓

    Résultat : MATCH !

    Texte à analyser : 123123
    Regex : ^(\d+)\1$

    Étape 1 : ^ 
    → On est au début ✓

    Étape 2 : (\d+)
    → Cherche des chiffres, trouve "123"
    → Mémorise dans groupe 1 : "123"

    Étape 3 : \1
    → Doit correspondre au contenu du groupe 1, donc "123"
    → Compare avec les caractères suivants : "123"
    → Ça correspond ! ✓

    Étape 4 : $
    → On est à la fin ✓

    Résultat : MATCH !
    /*
