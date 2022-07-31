#!/bin/sed -nf

/^$/ d
: gobble
/^%beginlatex/,/^%endlatex/ d




: blockify
/\n$/ b concat
/\\begin{abstract}/ b quotify
/\\begin{itemize}/ b itemize
/\\begin{enumerate}/ b enumerate
/\\bottomcaption\*{/ b bottomcaption
/^%beginlatex/ b gobble
s/%pandoc \?\([^\n]*\)/\1/
N
b gobble

: concat
s/\n/@@@@@@@/g
# s/@@@@@@@@@@@@@@$/\n\n/g

# Structural
s/%@@@@@@@\s*//g   # Trailing percents gobble whitespace.

# a HelloWorld

s/\.~/\. /g   # A.N.^Other
# s/\\nocite{\*}//g
# s/\\title[[][^]]*]{\([^}]*\)}/\# \1/g
# s/\\author{\([^}]*\)}/\n*   Author: *\1*/g
# s/\\date{\([^}]*\)}/*   Date: *\1*/g
s/``/\"/g
s/''/\"/g
s/`/'/g
# `
s/\\UtopiaOld/`Utopia84`/g
s/\\Utop/`Utopia27`/g
s/\\ldots/.../g
s/\\hyph /-/g
s/\\-//g

s/\\\(ckw\|cop\|csituation\|ccaps\|ccomment\|clab\|ctype\){\([^}]*\)}/\2/g
s/\\\(ckw\|cop\|csituation\|ccaps\|ccomment\|clab\|ctype\)\[\([^]]*\)\]/\2/g
s/\\cellipsis\[\([^]]*\)\]/<\1...>/g
s/\\cbrack\[\([^]]*\)\]/[\1]/g
s/\\cgoto/-> /g
s/\\cend/\//g
s/\\cdedent/|/g
s/\\csitop/|== /g
# s/\\csituation{\([^}]*\)}/\1/g
s/\\ckwbreak/break/g
# s/\\ckwcontinue{\([^}]*\)}/\1/g

s/\\situation{\([^}]*\)}/**\1**/g

s/\({[^{}]*\){\([^}]*\)}/\1\2/g
s/\({[^{}]*\){\([^}]*\)}/\1\2/g
s/\({[^{}]*\){\([^}]*\)}/\1\2/g
s/\({[^{}]*\){\([^}]*\)}/\1\2/g

s/\\section{\([^}]*\)}/\#\# \1/g
s/\\subsection{\([^}]*\)}/\#\#\# \1/g
s/\\subsubsection{\([^}]*\)}/\#\#\#\# \1/g
s/\\emph{\([^}]*\)}/*\1*/g
s/\\textbf{\([^}]*\)}/**\1**/g
s/\\url{\([^}]*\)}/\1/g
# s/\\href{\([^}]*\)}/\1/g

s/\\texttt{\([^}]*\)}/`\1`/g
s/|\([a-zA-Z:/]\+\)|/`\1`/g

s/\\ref{\([^}]*\)}/[ref](#\1)/g
s/\\label{\([^}]*\)}/{#\1}/g

s/\\cite{\([^}]*\)}/[citation](#\1)/g
s/\\bibitem{\([^}]*\)}@@@@@@@\(.*\)@@@@@@@$/1.   {#\1}@@@@@@@\2/g
s/@@@@@@@\s*}/}/g
s/\\providecommand[^@]*@@@@@@@//g

s/\\begin{offsideBlue}@@@@@@@\\begin{PVerbatim}/@@@@@@@```/g #`
s/\\end{PVerbatim}@@@@@@@\\end{offsideBlue}/```@@@@@@@/g #`

s/\\begin{thebibliography}{[0-9]*}/## Bibliography/

: output
s/@@@@@@@/\n/g
s/\n%[^\n]*$//g
s/^%[^\n]*$//g
s/\n%[^\n]*\n/\n/g
s/^%[^\n]*\n/\n/g
p
b end

:quotify
s/\\begin{abstract}//g
N
s/\\end{abstract}//g
T quotify
#
s/\n /\n> /g
b gobble



:itemize
s/\\begin{itemize}//g
N
s/\\end{itemize}//g
T itemize
#
s/\n\\item\[[^]]*\]/\n\\item/g
s/\n\\item/\n*   /g
b gobble



:enumerate
s/\\begin{enumerate}//g
N
s/\\end{enumerate}//
T enumerate
#
s/\n  \\item/\n    *   /g
s/\n\\item/\n*   /g
b gobble

:bottomcaption
s/\\bottomcaption\*{/\n  /g
s/\n \([^\n]*\)$/\n> \1/
N
s/}\n\\end{codex}/\n---/
T bottomcaption
#
b gobble


# defn
# codex
# anchors
# graphviz
# nested lists
# description lists
x

: end
