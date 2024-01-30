#!/bin/bash
pandoc -i README.md -o ../www/blog/introducing-bizkit.html \
       -s --css=blog.css \
       --metadata title="🍪 Introducing Bizkit" \
       -H header.html
