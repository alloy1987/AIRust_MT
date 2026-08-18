<p align="center">
  <img src="public/app-icon.svg" width="140" alt="Logo AIRust_MT" />
</p>

<h1 align="center">AIRust_MT</h1>

<p align="center">
  Éditeur de bureau Markdown WYSIWYG (tel-écrit-tel-affiché) reconstruit avec <b>Rust + Tauri 2</b><br />
  Retrouve l'expérience d'écriture de MarkText avec une taille réduite et une consommation de ressources moindre
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-v0.1.3-1f6feb" alt="version v0.1.3" />
  <img src="https://img.shields.io/badge/license-MIT-31a354" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-Tauri%202-ff6b6b" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Vue-3%20%2B%20TypeScript-42b883" alt="Vue 3 + TypeScript" />
  <img src="https://img.shields.io/badge/AI-vibe_coding-ff6b6b" alt="vibe-coding" />
  <img src="https://img.shields.io/badge/editor-alloy1987-7048e8" alt="alloy1987" />
</p>

> La grande majorité du code de ce projet a été écrite par l'auteur en **vibe coding** — avec [DeepSeek V4 Flash](https://www.deepseek.com/) et [Qwen 3.8](https://www.qianwenai.com/) comme modèles de codage et [opencode](https://opencode.ai) comme agent de programmation IA, sous conception et revue humaines. Voir [Méthode de développement](#méthode-de-développement) ci-dessous.

---

## 📖 Sommaire

- [✨ Fonctionnalités](#fonctionnalités)
- [📦 Installation](#installation)
- [🔐 Confidentialité](#confidentialité)
- [⚙️ Stack technique](#stack-technique)
- [⌨️ Raccourcis clavier](#raccourcis-clavier)
- [🧱 Structure du projet](#structure-du-projet)
- [🛠️ Compilation](#compilation)
- [🤝 Projets de référence et remerciements](#projets-de-référence-et-remerciements)
- [🤖 Méthode de développement](#méthode-de-développement)
- [🧑‍💻 Mots de l'auteur](#mots-de-lauteur)
- [📜 Licence](#licence)

## ✨ Fonctionnalités

- **Édition WYSIWYG** : basée sur le noyau d'éditeur de MarkText `@muyajs/core`, le rendu est instantané pendant la frappe, sans prévisualisation en panneaux séparés
- **Éléments de bloc riches** : titres, listes, tableaux, blocs de code (coloration syntaxique), formules mathématiques (KaTeX), diagrammes (Mermaid / flowchart / PlantUML / Vega), blocs HTML, Front Matter, etc.
- **Onglets multiples** : éditez plusieurs documents en même temps, avec rappel des modifications non enregistrées
- **Barre latérale de fichiers** : ouvrez un dossier comme espace de travail, avec arborescence de fichiers pour parcourir, créer, renommer et supprimer
- **Édition en texte brut** : au-delà du Markdown, ouvre et édite les fichiers texte brut courants en tant que texte brut (sans analyse) : données et configuration (.json / .yaml / .yml / .xml / .toml / .ini / .csv / .env), documents et pages web (.txt / .html / .htm / .css / .rtf), et code source (.py / .js / .ts / .java / .c / .cpp / .go / .rs) ; l'extension du fichier en cours est affichée dans la barre d'état
- **Détection intelligente de texte** : la possibilité d'ouvrir un fichier est déterminée par son contenu, et non par son extension — les fichiers texte avec des extensions inconnues s'ouvrent sans problème, tandis que les fichiers binaires sont détectés et rejetés avec un avertissement
- **Panneau de plan** : navigation rapide selon les niveaux de titres
- **Rechercher / remplacer** : prise en charge des expressions régulières, de la sensibilité à la casse et de la correspondance de mots entiers
- **Prise en charge des images** : collez ou glissez des images pour les enregistrer automatiquement dans le répertoire du document et les insérer
- **Surveillance des fichiers** : alerte automatique lorsqu'un document est modifié extérieurement sur le disque
- **Détection d'encodage** : détection et conversion automatiques des encodages de fichiers non UTF-8 grâce à `encoding_rs` + `chardetng`
- **Gestion des gros fichiers** : les très gros fichiers s'ouvrent en mode aperçu en lecture seule pour éviter les blocages
- **12 thèmes d'apparence** : Blanc éclatant, Noir sombre, Indigo, Vert émeraude, Orange coucher de soleil, Bleu abyssal, Rose, Or d'aurore, Menthe, Bleu ciel, Rose pêche, Lavande
- **7 langues d'interface** : 中文, English, 日本語, Русский, 한국어, Español, Français
- **Zoom de l'interface** : Ctrl + molette / menu de zoom, adapté aux écrans haute résolution
- **Instance unique** : lors d'un nouveau lancement, la fenêtre existante est mise au premier plan et le fichier est ouvert
- **Installeur NSIS Windows** : assistant d'installation multilingue, prise en charge de l'ouverture de fichiers par glisser-déposer

## 📦 Installation

### Windows (installeur .exe)

- Avant l'installation, assurez-vous que le **runtime Microsoft Edge WebView2** (web2view, abrégé WebView2) est installé sur le système ;
- Pendant l'installation, l'installeur détecte automatiquement si WebView2 est installé :
  - s'il l'est, l'installation se poursuit directement ;
  - sinon, l'installeur affiche une invite et télécharge et installe automatiquement WebView2 via Internet ;
- Vous pouvez également télécharger et installer WebView2 manuellement à l'avance. Page de téléchargement officielle : <https://developer.microsoft.com/microsoft-edge/webview2/>

### macOS et Linux

> Les installeurs pour macOS et Linux ne sont pas encore publiés, mais vous pouvez compiler l'application à partir des sources. Voir la section [Compilation](#compilation) ci-dessous.

## 🔐 Confidentialité

> L'application fonctionne globalement **en local et hors ligne**, à l'exception des fonctions de diagramme suivantes, qui nécessitent une connexion Internet lors du rendu :
>
> - **Diagrammes PlantUML** : le code source du diagramme est envoyé au serveur public `plantuml.com`, qui renvoie l'image rendue ; le contenu du diagramme quitte votre machine ;
> - **Diagrammes de séquence (sequence)** : lors du rendu, les polices sont chargées depuis Google Fonts via webfontloader.
>
> Toutes les autres fonctions (édition, enregistrement, images, détection d'encodage, etc.) n'effectuent aucune requête réseau. Si le contenu de votre document est confidentiel, évitez d'utiliser les deux types de diagrammes ci-dessus.

## ⚙️ Stack technique

| Couche             | Technologie                                                                                                                                                                                               |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Coque de bureau    | [Tauri 2](https://tauri.app) + Rust (`tauri-plugin-dialog` / `tauri-plugin-opener` / `tauri-plugin-single-instance`, surveillance de fichiers `notify`, détection d'encodage `encoding_rs` + `chardetng`) |
| Frontend           | Vue 3 + TypeScript + Vite + Pinia                                                                                                                                                                         |
| Noyau de l'éditeur | [@muyajs/core](https://github.com/marktext/marktext) (muya issu de MarkText, rendu par DOM virtuel snabbdom)                                                                                              |
| Icônes de fichiers | @marktext/file-icons (issu de MarkText)                                                                                                                                                                   |

## ⌨️ Raccourcis clavier

| Raccourci                       | Fonction                   |
| ------------------------------- | -------------------------- |
| `Ctrl + N`                      | Nouveau document           |
| `Ctrl + O`                      | Ouvrir un fichier          |
| `Ctrl + Shift + O`              | Ouvrir un dossier          |
| `Ctrl + S`                      | Enregistrer                |
| `Ctrl + Shift + S`              | Enregistrer sous           |
| `Ctrl + F`                      | Rechercher / remplacer     |
| `Ctrl + Alt + F`                | Basculer la barre latérale |
| `Ctrl + A`                      | Tout sélectionner          |
| `Ctrl + Z`                      | Annuler                    |
| `Ctrl + Shift + Z` / `Ctrl + Y` | Rétablir                   |
| `Ctrl + 0`                      | Zoom 100 %                 |
| `Ctrl + molette`                | Zoom de l'interface        |

> Sous macOS, utilisez `Cmd` à la place de `Ctrl`.

## 🧱 Structure du projet

```
AIRust_MT/
├── src/                  # Frontend (Vue 3 + TypeScript + Pinia)
│   ├── components/       #    Composants d'interface (barre latérale, barre d'onglets, recherche, dialogues, etc.)
│   ├── stores/           #    Gestion d'état (éditeur, thème, zoom)
│   ├── editor/           #    Couche d'adaptation du noyau muya
│   └── api.ts            #    Wrappers des commandes Tauri
├── src-tauri/            # Coque de bureau (Rust + Tauri 2)
│   ├── src/              #    Commandes, surveillance de fichiers, détection d'encodage, gestion des gros fichiers, menus
│   └── nsis/             #    Scripts de crochet de l'installeur NSIS
├── editor/               # Noyau et paquet d'icônes issus de MarkText
│   ├── muya/             #    Noyau WYSIWYG @muyajs/core
│   └── file-icons/       #    Icônes de fichiers @marktext/file-icons
└── public/               # Ressources statiques
```

## 🛠️ Compilation

Prérequis : [Rust](https://www.rust-lang.org/), [Node.js](https://nodejs.org/) ≥ 20, [pnpm](https://pnpmjs.com/) et les [dépendances système de Tauri 2](https://tauri.app/start/prerequisites/).

Prérequis supplémentaires par plateforme :

- **Windows** : [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) (généralement déjà installé) ;
- **Linux** : installez les paquets système listés dans les [prérequis officiels de Tauri](https://tauri.app/start/prerequisites/), par exemple `webkit2gtk-4.1`, `libappindicator3`, `librsvg2-dev` ;
- **macOS** : [Xcode Command Line Tools](https://developer.apple.com/xcode/) (`xcode-select --install`).

```bash
# Installer les dépendances
pnpm install

# Mode développement (rechargement à chaud)
pnpm tauri dev

# Compiler l'installeur de version
pnpm tauri build
```

Résultats de compilation par plateforme :

- Windows : `src-tauri/target/release/bundle/nsis/*.exe`
- macOS : `src-tauri/target/release/bundle/macos/*.app` et `dmg/*.dmg`
- Linux : `src-tauri/target/release/bundle/deb/*.deb`, `rpm/*.rpm`, `appimage/*.AppImage`

## 🤝 Projets de référence et remerciements

Ce projet s'est appuyé sur les projets open source suivants pendant son développement. Merci à leurs auteurs et contributeurs. Le texte intégral des licences correspondantes figure dans la section « Avis tiers » du fichier [LICENSE](LICENSE).

### [MarkText](https://github.com/marktext/marktext) (licence MIT)

Ce projet est une version réécrite de MarkText ; les parties suivantes proviennent directement de MarkText :

| Partie de référence                             | Emplacement dans ce projet                    | Description                                                                                |
| ----------------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------ |
| Noyau d'éditeur muya                            | `editor/muya/` (`@muyajs/core`)               | Capacités d'édition essentielles : édition WYSIWYG, rendu des blocs, raccourcis, formatage |
| Icônes de fichiers                              | `editor/file-icons/` (`@marktext/file-icons`) | Icônes de l'arborescence de fichiers de la barre latérale                                  |
| Forme du produit et conception des interactions | Global                                        | Philosophie d'écriture WYSIWYG, périmètre fonctionnel et modes d'interaction               |

### [Markpad](https://github.com/sftwrdotdev/Markpad) (licence BSD 3-Clause)

De nombreux détails techniques de la partie bureau (Tauri 2) s'inspirent de Markpad, notamment :

| Partie de référence                       | Description                                                                                                                |
| ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Architecture d'application Tauri 2        | Organisation des commandes côté Rust et communication frontend-backend                                                     |
| Menus natifs et distribution d'événements | Définition des éléments de menu et transfert des événements au frontend (voir `src-tauri/src/menu.rs`)                     |
| Pratiques d'empaquetage et d'installeur   | Configuration de l'installeur NSIS, scripts de crochet d'installation (`src-tauri/nsis/`) et autres pratiques d'ingénierie |

### Remerciements particuliers

Je tiens ici à remercier **tout particulièrement Mozilla et la Rust Foundation** : grâce à leurs efforts, nous disposons d'un excellent langage de programmation, Rust !
Dans la vague de la programmation assistée par IA, Rust est devenu un langage de développement au fort potentiel, principalement grâce à ses quatre avantages clés en tant que langage système de bas niveau :

1. **Efficacité d'exécution extrême et contrôle de bas niveau**
   En tant que langage de programmation système proche du matériel, Rust abandonne le mécanisme traditionnel de ramasse-miettes (GC) et offre des abstractions à coût nul. Il atteint ainsi, tout en préservant l'efficacité du développement, des performances d'exécution et un contrôle de la mémoire comparables à C/C++, répondant parfaitement aux besoins de l'ère de l'IA en matière de calcul haute performance et de traitement hautement concurrent.

2. **Sécurité mémoire rigoureuse et garantie de fiabilité**
   Rust est réputé pour sa syntaxe rigoureuse et ses mécanismes uniques de propriété (Ownership) et de vérification des emprunts (Borrow Checker). Il intercepte avec précision, dès la compilation, les dangers de sécurité mémoire tels que les pointeurs nuls et les courses de données. Cette caractéristique de « sécurité dès la compilation » offre un solide filet de sécurité qualité pour le code généré par l'IA, réduisant fortement les risques de plantage à l'exécution.

3. **Un système de types fort comme « contrainte sémantique » pour l'IA**
   Rust possède un système de types fort, hautement standardisé et rigoureux. Dans le contexte de la programmation avec l'IA, ce système de types n'est pas seulement une spécification du code, mais aussi un « navigateur » pour l'IA. Des définitions de types claires aident l'IA à comprendre plus précisément la logique métier et les flux de données, réduisant efficacement le code invalide produit par les « hallucinations » ou les failles logiques de l'IA, rendant le code généré par l'IA intrinsèquement plus robuste.

4. **Le compilateur officiel comme « inspecteur qualité strict »**
   Rust fournit officiellement une chaîne d'outils de compilation mature et extrêmement stricte. Dans les flux de travail de développement assisté par IA, l'IA est chargée de générer rapidement les brouillons de code, tandis que le compilateur Rust joue le rôle de premier et strict portail de contrôle qualité. Si le code généré par l'IA compile, cela signifie que la plupart des erreurs fatales de sécurité mémoire et de correspondance des types ont été éliminées. Ce modèle complémentaire « l'IA produit, le compilateur inspecte » améliore considérablement la qualité de livraison du code de niveau industriel.
   **En résumé, à l'ère de la programmation assistée par IA, choisir Rust comme langage de développement revêt une valeur stratégique extrêmement élevée. J'espère également voir davantage de personnes compétentes rejoindre les rangs de la programmation en Rust, pour que l'écosystème Rust soit toujours plus riche.**

## 🤖 Méthode de développement

Ce projet est une pratique de **programmation assistée par IA (vibe coding)** :

- **Modèle de codage** : [DeepSeek V4 Flash](https://www.deepseek.com/) / [Qwen 3.8](https://www.qianwenai.com/)
- **Modèle d'images** : [Qwen 3.8](https://www.qianwenai.com/)
- **Agents de programmation** : [opencode](https://opencode.ai) (CLI interactif de programmation IA), [Qwencode](https://www.qianwenai.com)
- **Rôle humain** : définition des besoins, décisions d'architecture, revue de code et tests de recette

Le projet prend MarkText comme modèle, remplace sa coque Electron par Rust + Tauri 2, tandis que le noyau de l'éditeur continue d'utiliser et d'adapter le muya de MarkText.

## 🧑‍💻 Mots de l'auteur

**Ingénieur polyvalent** : je travaille depuis longtemps dans le secteur financier. Je ne suis pas diplômé en informatique, mais j'adore la programmation. J'ai appris Python en autodidacte et développé de petits outils pratiques ; niveau Python 5kyu sur Codewars.

**Praticien du vibe coding** : l'IA a permis aux gens ordinaires de franchir le seuil de la programmation. Je crois qu'à l'ère de l'IA, la limite de l'IA est la limite de votre imagination, et que chacun peut poursuivre ses rêves grâce à l'IA.

**Novice en open source** : c'est mon premier projet GitHub, il comporte donc forcément des maladresses et des lacunes.

**Ouvert aux échanges** : les suggestions d'amélioration dans les Issues ou les PR sont les bienvenues. Aidez-moi à grandir !

**E-mail：**  20360505@qq.com

## 📜 Licence

Ce projet est publié dans son ensemble sous la **[licence MIT](LICENSE)**.

Étant donné que ce projet est dérivé de / s'inspire de MarkText (MIT) et de Markpad (BSD 3-Clause), afin de respecter les exigences de conformité de ces deux licences :

- `editor/muya/` et `editor/file-icons/` conservent la mention de droits d'auteur MIT de MarkText ;
- le fichier [LICENSE](LICENSE) comporte une section « Avis tiers (Third-Party Notices) » reprenant intégralement les mentions de droits d'auteur originales et le texte complet des licences de MarkText et de Markpad ;
- les menus « Aide → À propos » et « Aide → Licence » de l'application affichent également les informations d'attribution ci-dessus.

---

<p align="center"><i>AIRust_MT — réinventer un classique avec l'IA et Rust pour un Markdown léger.</i></p>
