# Genealogy Keeper

Genealogy Keeper is a hobby side-project for exploring a new take on genealogical data and editor. It will aspirationally support both a desktop and web application view, with all data owned and stored by the user. The app interface should be intuitive and all features able to be explained within the app. It will use a new data format separate from GEDCOM.

https://harrisonmilbradt.com/articles/canvas-panning-and-zooming
https://en.wikipedia.org/wiki/Genealogy_software

Use [juniper](https://docs.rs/juniper/latest/juniper/) to create GraphQL api for frontend to access backend. Backend will scan project directories and load content as necessary, serving access & mutations via GraphQL to frontend.

Tldraw canvas
- disable top-left menu bar
- add file-bar hamburger to left of center-bottom menu bar like legendkeeper. It should contain the top-left bar's "preferences, languages, keyboard shortcuts"
- disable top-right color palette & center-bottom bar's pencil, eraser, arrow, text, sticky image, and shape tools (read-only mode)
