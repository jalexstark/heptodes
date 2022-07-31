COREBASENAMES=StickySituations RoughMaterial
COREPDFS=$(addsuffix .pdf,$(COREBASENAMES))

PDFNAMES=$(COREPDFS)

all:	$(PDFNAMES)

%.bbl:	%.tex StickySituations.bib
	pdflatex -draftmode $< &&\
		bibtex $*

$(COREPDFS):	%.pdf:	%.bbl

STICKYDOTBASES=SimpleState ReferenceState ConstState FailBranch WindUp \
	SubWindUp
STICKYDOTDOTS=$(addsuffix .dot,$(STICKYDOTBASES))
STICKYDOTPDFS=$(addsuffix .pdf,$(STICKYDOTBASES))

StickySituations.pdf:	$(STICKYDOTPDFS)

StickySituations.md:	StickySituations.tex StickySituations.bbl
	cat $^ | ./convertPandoc.sh > $@


$(STICKYDOTPDFS):	%.pdf:	%.dot
	dot -Tps2 $*.dot | ps2pdf - $*.pdf

#	dot -Tpdf -o $*.pdf $*.dot


$(STICKYDOTDOTS):	StickySituations.tex RoughMaterial.tex
	pdflatex -draftmode $<

$(PDFNAMES):	%.pdf:	%.tex CommonStyle.sty
	pdflatex $< &&\
	pdflatex $<

# -----------------------------------------------------------------

clean:	tidy
	rm -f *~ ${PDFNAMES} CollectTocs.pdf ${COLLECTPDFS} $(STICKYDOTPDFS)

tidy:
	rm -f *.aux *.log *.dvi *.blg *.toc *.out *.bbl *.loc $(STICKYDOTDOTS)

# -----------------------------------------------------------------
