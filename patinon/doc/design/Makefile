
BASENAMES=Molecule
MDINTERMEDIATES=$(addsuffix -intermediate.md,$(BASENAMES))

ATOMS=atoms/Salient-3854.md

HTMLTARGETS=$(addsuffix .html,$(BASENAMES))
PDFTARGETS=$(addsuffix .pdf,$(BASENAMES))
TARGETS=$(HTMLTARGETS) $(PDFTARGETS)

all:	$(TARGETS)



# $(BASETARGETS):	%.html: %.md

Molecule-intermediate.md:	$(ATOMS)

$(MDINTERMEDIATES): %-intermediate.md:	%-raw.md
	awk '/INCLUDE-FILE:/ { system("sed -e " "sX^#X##X\\;/.*--\\ Standalone\\ only\\ --.*/d " $$2); next } { print }' $< > $@

%.html: %-intermediate.md crested.css
	pandoc --toc --standalone --to html5 -H header-html.html --metadata-file defs-html.yaml -c crested.css \
	--mathjax="${MATHJAX_URL}?config=TeX-AMS_HTML" -f markdown -t html --number-sections $< -o $@

%.pdf:	%-intermediate.md
	pandoc --toc -f markdown --pdf-engine=pdflatex -H header-pdflatex.tex --metadata-file defs-pdflatex.yaml \
	--pdf-engine=xelatex $< -o $@

clean:	tidy
	rm -f $(TARGETS)

tidy:
	rm -f *~ *.bak $(MDINTERMEDIATES)

### The remaining content is notes and options used previously.

# --self-contained -s

# In script
# genpdf () {
# echo "Generating $1.md"
# cat $1.md \
#     | sed -e 's/\[TOC\]/\\tableofcontents/;s/^:\(.\+\):\(.\+\):$/:\1|\2:/g;s/^:\(.\+\):$/:\1|/;' \
#     | sed -e 's/\\DeclareMathOperator/\\newcommand/;' \
#     | pandoc -f markdown -t latex --pdf-engine=xelatex \
#         --variable monofont="DejaVu Sans Mono" --variable fontsize=10pt \
#         ... \
#         -o ${PDF_DIR}/$1.pdf
# }

# # $ ls *.md | xargs -l echo "genpdf" | sed 's/\.md$//' | sort
# genpdf FILENAME


# OUTPUT_FILE=CombinedPdfs.pdf

# BLANK_FILE=blank.pdf
# echo "" | ps2pdf -sPAPERSIZE=letter - ${BLANK_FILE}

# ALL_FILES=""
# for PDF_FILE in ${INPUT_FILES}; do
#     if [ "${PDF_FILE}" != "${OUTPUT_FILE}" ]; then
#	  pages=$(pdfinfo "${PDF_FILE}" | grep Pages | sed -e 's/^[^0-9]*//')
#	  ALL_FILES="${ALL_FILES} ${PDF_FILE}"
#	  [ $((pages%2)) -ne 0 ] && ALL_FILES="${ALL_FILES} ${BLANK_FILE}"
#     fi
# done

# echo "Executing:"
# echo "    " pdftk ${ALL_FILES} cat output "${OUTPUT_FILE}"

# pdftk ${ALL_FILES} cat output "${OUTPUT_FILE}"
