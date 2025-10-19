
STYLING_DIR=styling

BASENAMES=background/RatQuad background/JaywalkConcepts background/JaywalkBuilder \
	background/DesignDrawableSvg curves/CurveMath curves/CurveIntro curves/Cookbook

HTML_TARGETS=$(addprefix html/,$(addsuffix .html,$(BASENAMES)))
GFM_TARGETS=$(addprefix gfm/,$(addsuffix .md,$(BASENAMES)))
PDF_TARGETS=$(addprefix pdf/,$(addsuffix .pdf,$(BASENAMES)))
TARGETS=$(HTML_TARGETS) $(GFM_TARGETS) $(PDF_TARGETS)

all:	depdirs $(TARGETS)

# ------------------------------------------------------------
# Copied, with directory changes, from
# ../../patinon/common/styling/RequiredStyles.mk

DEP_DIRS=styling.dir
REQ_STYLES=crested.css

PANDOC_HTML_DEPS=$(STYLING_DIR)/crested.css $(STYLING_DIR)/header-html.html $(STYLING_DIR)/defs-html.yaml
PANDOC_PDF_DEPS=$(STYLING_DIR)/header-pdflatex.tex $(STYLING_DIR)/defs-pdflatex.yaml

# .PHONY:	depdirs	$(DEP_DIRS)

depdirs:	$(DEP_DIRS)

$(DEP_DIRS):
	$(MAKE) -C $(subst .dir,,$@)

$(REQ_STYLES):	%:	../../patinon/common/styling/%
	cp $< $(CURDIR)/

# End of copy.
# ------------------------------------------------------------

# include ../../patinon/common/styling/RequiredStyles.mk



RatQuad-FIGS=$(addprefix background/figs-ratquad/, $(addsuffix .svg, \
	RatQuad-A  RatQuad-B  RatQuad-D  RatQuad-E  RatQuad-F  \
	RatQuad-G  RatQuad-I  RatQuad-J  RatQuad-K))
JaywalkBuilder-FIGS=$(addprefix background/figs-builder/, $(addsuffix .svg, \
	Builder-A Builder-B Builder-C Builder-D Builder-E Builder-F Builder-G \
	Builder-H Builder-I Builder-J Builder-K Builder-L Builder-M Builder-N \
	Builder-O Builder-P Builder-Q Builder-R Builder-S Builder-T-1 Builder-T-2))
JaywalkConcepts-FIGS=$(addprefix background/figs-concepts/, $(addsuffix .svg, \
	Concepts-A Concepts-C Concepts-E Concepts-F Concepts-G Concepts-H Concepts-I \
	Concepts-O-1 Concepts-O-2 Concepts-P Concepts-Q Concepts-R Concepts-S \
	Concepts-T Concepts-U Concepts-V Concepts-W Concepts-X))
CurveIntro-FIGS=$(addprefix curves/figs-intro/, $(addsuffix .svg, \
	intro_cubic_controlled intro_cubic_slider intro_cubic_multi))

AUTO_FIGS=$(CurveIntro-FIGS)
SRC_AUTO_FIGS=$(addprefix src/,$(CurveIntro-FIGS))

HTML_FIGS=$(addprefix html/,$(RatQuad-FIGS) $(JaywalkBuilder-FIGS) \
	$(JaywalkConcepts-FIGS) $(AUTO_FIGS))
GFM_FIGS=$(addprefix gfm/,$(RatQuad-FIGS) $(JaywalkBuilder-FIGS) \
	$(JaywalkConcepts-FIGS) $(AUTO_FIGS))

html/background/RatQuad.html:	$(addprefix html/,$(RatQuad-FIGS))
html/background/JaywalkBuilder.html:	$(addprefix html/,$(JaywalkBuilder-FIGS))
html/background/JaywalkConcepts.html:	$(addprefix html/,$(JaywalkConcepts-FIGS))
html/curves/CurveIntro.html:	$(addprefix html/,$(CurveIntro-FIGS))
gfm/background/RatQuad.md:	$(addprefix gfm/,$(RatQuad-FIGS))
gfm/background/JaywalkBuilder.md:	$(addprefix gfm/,$(JaywalkBuilder-FIGS))
gfm/background/JaywalkConcepts.md:	$(addprefix gfm/,$(JaywalkConcepts-FIGS))
gfm/curves/CurveIntro.md:	$(addprefix gfm/,$(CurveIntro-FIGS))

$(SRC_AUTO_FIGS):	src/%:	../crates/zvx-simples/tests/goldenfiles/%
	mkdir -p $(dir $@) && cp $< $@
$(HTML_FIGS):	html/%:	src/%
	mkdir -p $(dir $@) && cp $< $@
$(GFM_FIGS):	gfm/%:	src/%
	mkdir -p $(dir $@) && cp $< $@

# --self-contained does not work together with --mathjax!

# If you are OK with calling out to the mathjax server, you can use:
#
# export MATHJAX_URL="http://cdn.mathjax.org/mathjax/latest/MathJax.js?config=TeX-AMS-MML_HTMLorMML"
$(HTML_TARGETS):	html/%.html: src/%.md $(PANDOC_HTML_DEPS)
	mkdir -p $(dir $@) && \
	cp $(STYLING_DIR)/crested.css $(dir $@)/crested.css && \
	pandoc --toc --standalone --to html5 \
	      $(STYLING_DIR)/header-html.html --metadata-file $(STYLING_DIR)/defs-html.yaml -c crested.css \
	      --mathjax="${MATHJAX_URL}" \
	      -f markdown -t html --number-sections $< -o $@

$(GFM_TARGETS):	gfm/%.md: src/%.md
	mkdir -p $(dir $@) && \
	pandoc --toc --standalone \
	      -f markdown -t gfm --number-sections $< -o $@

# Stashing some attempts at automatically checking figure directories.
#
# CP := )
# OP := (
#
# USED_FIGS="$(shell cat src/background/RatQuad.md | grep '$(OP)figs/' | sed -e 'sX^.*$(OP)figs/XX;s/\.[a-z]\+$(CP)//')" && \
# 	AVAIL_FIGS="$(shell cd $(dir $<)/figs && find . -type f | sed -e 'sX\./XX;s/\.[a-z]\+$(CP)//')" && \
# 	echo $(USED_FIGS)

#	$(shell diff  <"$(USED_FIGS)"  < "$(AVAIL_FIGS)")

# Note!!!! This rule effectively operates within the subdirectory
# since pandoc likes to work relative to the main input file
# location. The output and metadata are given absolute paths.
#
# Ensure that rsvg-convert is installed, as well as pandoc, and
# texlive-fonts-extra.
$(PDF_TARGETS):	pdf/%.pdf:	src/%.md $(PANDOC_PDF_DEPS)
	mkdir -p $(dir $@) && \
	cd $(dir $<) && \
	pandoc --toc -f markdown --pdf-engine=pdflatex \
	      -H $(CURDIR)/$(STYLING_DIR)/header-pdflatex.tex \
	      --metadata-file $(CURDIR)/$(STYLING_DIR)/defs-pdflatex.yaml \
	      --pdf-engine=xelatex $(notdir $<) -o $(CURDIR)/$@

tidy:
	rm -f *~ *.bak missfont.log

# This is also a kind of dist-clean.
clean:	tidy
	rm -f $(HTML_TARGETS) $(PDF_TARGETS) $(REQ_STYLES) $(HTML_FIGS) \
	      html/background/crested.css $(PANDOC_HTML_DEPS) $(PANDOC_PDF_DEPS)

# This cleans all generated content.
ultra-clean:	clean
	rm -rf $(GFM_TARGETS) $(GFM_FIGS)

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
