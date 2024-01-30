#!/bin/bash
pandoc -i README.md -o index.html -s --css=style.css --toc
