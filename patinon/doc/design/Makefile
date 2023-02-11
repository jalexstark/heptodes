
BASENAMES=Molecule
MDINTERMEDIATES=$(addsuffix -intermediate.md,$(BASENAMES))

ATOMS=atoms/Salient-3854.md

HTMLTARGETS=$(addsuffix .html,$(BASENAMES))
PDFTARGETS=$(addsuffix .pdf,$(BASENAMES))
TARGETS=$(HTMLTARGETS) $(PDFTARGETS)

all:	depdirs $(TARGETS)

include ../../../patinon/common/styling/RequiredStyles.mk

Molecule-intermediate.md:	$(ATOMS)

$(MDINTERMEDIATES): %-intermediate.md:	%-raw.md
	awk '/INCLUDE-FILE:/ { system("sed -e " "sX^#X##X\\;/.*--\\ Standalone\\ only\\ --.*/d " $$2); next } { print }' $< > $@

$(HTMLTARGETS):	%.html: %-intermediate.md $(PANDOC_HTML_DEPS)
	pandoc --toc --standalone --to html5 --self-contained ../styling/header-html.html --metadata-file ../styling/defs-html.yaml -c crested.css \
	--mathjax="${MATHJAX_URL}?config=TeX-AMS_HTML" -f markdown -t html --number-sections $< -o $@

$(PDFTARGETS):	%.pdf:	%-intermediate.md $(PANDOC_PDF_DEPS)
	pandoc --toc -f markdown --pdf-engine=pdflatex -H ../styling/header-pdflatex.tex --metadata-file ../styling/defs-pdflatex.yaml \
	--pdf-engine=xelatex $< -o $@

clean:	tidy
	rm -f $(TARGETS) $(REQ_STYLES)

tidy:
	rm -f *~ *.bak $(MDINTERMEDIATES)
