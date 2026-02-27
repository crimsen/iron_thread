#!/bin/bash

# 1. Entities generieren (wie gehabt)
sea-orm-cli generate entity \
    --database-url sqlite://./dev.sqlite \
    --output-dir src/entities \
    --with-serde both \
    --expanded-format \
    --model-extra-derives ts_rs::TS \
    --model-extra-attributes 'ts(export), serde(rename_all = "camelCase")'
#, ts(rename_all = "camelCase")
# 2. AUTOMATISIERUNG: Dateinamen zu PascalCase konvertieren und als TS-Rename einfügen
for file in src/entities/*.rs; do
    # Ignoriere mod.rs und prelude.rs
    filename=$(basename "$file" .rs)
    if [ "$filename" != "mod" ] && [ "$filename" != "prelude" ]; then
        
        # Konvertiere snake_case (fabric_x_project) zu PascalCase (FabricXProject)
        # Dieser kleine Perl-Befehl macht den ersten Buchstaben groß und nach jedem Unterstrich ebenfalls
        pascal_name=$(echo "$filename" | perl -pe 's/(^|_)./uc($&)/ge;s/_//g')
        js_file_name=$(echo "$pascal_name" | perl -pe 's/^(.)/lc($1)/e')
        
        echo "Verarbeite $filename:"
        echo "  - TS Name: $pascal_name"
        echo "  - TS File: $js_file_name"

        sed -i '/#\[ts(/d' "$file"

        
        # Füge das #[ts(rename = "PascalName")] Attribut ÜBER dem Struct ein
        # Wir suchen nach der Zeile mit 'pub struct Model' und fügen das Attribut davor ein
        sed -i "/pub struct Model/i #[ts(rename = \"$pascal_name\")]" "$file"
        sed -i "/pub struct Model/i #[ts(export_to=\"../src/types/${js_file_name}.ts\")]" "$file"
    fi
done
