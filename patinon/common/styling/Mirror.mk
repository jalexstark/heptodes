STYLE_FILES=crested.css defs-html.yaml defs-pdflatex.yaml header-html.html header-pdflatex.tex\
	definitions.tex

all:	$(STYLE_FILES)

$(STYLE_FILES):	%:	../../../patinon/common/styling/%
	cp $< .
