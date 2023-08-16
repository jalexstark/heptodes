
PATTERN_TARGETS=plain_random.svg \
	four_by_four_random.svg four_by_four_forward.svg four_by_four_switch.svg \
	four_by_four_diagonal.svg four_by_four_anti.svg four_by_four_presort.svg four_by_four_merge.svg \
	four_by_four_append.svg four_by_four_zigzag.svg \
        \
	four_by_four_random_displace.svg four_by_four_forward_displace.svg four_by_four_switch_displace.svg \
	four_by_four_diagonal_displace.svg four_by_four_anti_displace.svg four_by_four_presort_displace.svg four_by_four_merge_displace.svg \
	four_by_four_append_displace.svg four_by_four_zigzag_displace.svg \
	\
	four_by_four_forward_shuffle.svg four_by_four_switch_shuffle.svg

TABLE_TARGETS=TableRandComp.md TablePresort.md TableMergeSub.md TableAppend.md \
	TableBlockDiag.md TableRandBlockPresort.md TableRandBlockRand.md \
	TableBidirection.md TableSizedPresort.md

# TABLE_COMPENDIUM=

AUXILIARY_LOZENGES=standard_lozenge.svg
AUXILIARY_DFS_TREES=standard_p_dfs.svg

BASENAMES=Concatenation-00
MDINTERMEDIATES=$(addsuffix -intermediate.md,$(BASENAMES))

HTMLTARGETS=$(addsuffix .html,$(BASENAMES))
PDFTARGETS=$(addsuffix .pdf,$(BASENAMES))
TARGETS=$(HTMLTARGETS) $(PDFTARGETS)


all:	$(TARGETS) $(PATTERN_TARGETS) $(AUXILIARY_LOZENGES) $(TABLE_TARGETS)

list_comparison_files: ../golden-outputs/comparison_*.m
	ls -1 $^

$(PATTERN_TARGETS):	%.svg:	../golden-outputs/%.csv
	octave --no-gui --eval "render_pattern('$<', '$@');"

standard_lozenge.svg: ../golden-outputs/standard_lozenge.m
standard_p_dfs.svg: ../golden-outputs/standard_dfs_tree.m

$(AUXILIARY_LOZENGES):	%.svg:
	octave --no-gui --path="$(dir $<)" --eval "auxiliary_links=$(basename $(notdir $<)); render_lozenge(auxiliary_links, '$@');"

$(AUXILIARY_DFS_TREES):	%.svg:
	octave --no-gui --path="$(dir $<)" --eval "auxiliary_links=$(basename $(notdir $<)); render_dfs_tree(auxiliary_links, '$@');"

TableCompendium.md: ../golden-outputs/comparison_*.m table_generate.m
	octave --no-gui --path="$(dir $<)" --eval "table_generate;"

$(TABLE_TARGETS): %.md:	TableCompendium.md
	sed -n 's/^$*: //p;' $< > $@

Concatenation-00-intermediate.md:	$(TABLE_TARGETS)

$(MDINTERMEDIATES): %-intermediate.md:	%-raw.md
	awk '/INCLUDE-FILE:/ { system("cat " $$2); next } { print }' $< > $@

%.html: %-intermediate.md crested.css $(PATTERN_TARGETS) $(AUXILIARY_LOZENGES)
	pandoc --toc --standalone --to html5 -H header-html.html --metadata-file defs-html.yaml -c crested.css \
	--mathjax="${MATHJAX_URL}?config=TeX-AMS_HTML" -f markdown -t html --number-sections $< -o $@

%.pdf:	%-intermediate.md $(PATTERN_TARGETS) $(AUXILIARY_LOZENGES)
	pandoc --toc -f markdown --pdf-engine=pdflatex -H header-pdflatex.tex --metadata-file defs-pdflatex.yaml \
	--pdf-engine=xelatex $< -o $@

clean:	tidy
	rm -f $(TARGETS)

tidy:
	rm -f *~ *.bak $(MDINTERMEDIATES)
