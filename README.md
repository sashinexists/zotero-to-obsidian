# Zotero to Obsidian script
This is a script that takes a better bibtex JSON file exported by Zotero and generates an organised collection of reference notes in Obsidian

In you obsidian vault have a hidden folder `./library`

In this `.library` folder export you better bibtex JSON file (select include notes and keep updated), name it "library.json".

Make sure that the built script is in the root directory of the vault (`copy /target/debug/zotero-to-obsidian`).

Make sure that the `Meta` folder is also in the root directory of the vault (it contains all the templates used to generate notes)

Every time you run the script, it will go through the library.json file and create notes according to the templates in `Meta/Templates/Resource`.

Next up... making it run automatically every time the JSON file updates!