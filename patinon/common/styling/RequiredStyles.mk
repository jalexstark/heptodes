
DEP_DIRS=../styling.dir
REQ_STYLES=crested.css

PANDOC_HTML_DEPS=crested.css ../styling/header-html.html ../styling/defs-html.yaml
PANDOC_PDF_DEPS=../styling/header-pdflatex.tex ../styling/defs-pdflatex.yaml

# .PHONY:	depdirs	$(DEP_DIRS)

depdirs:	$(DEP_DIRS)

$(DEP_DIRS):
	$(MAKE) -C $(subst .dir,,$@)

$(REQ_STYLES):	%:	../../../patinon/common/styling/%
	cp $< $(CURDIR)/
