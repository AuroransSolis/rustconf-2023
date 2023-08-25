REQFILES=neat-part.png profile-pic.png time-1.svg time-2.svg time-3.svg

.PHONY: all conference.pdf standalone.pdf clean-conference clean-standalone cleanall

all: conference.pdf standalone.pdf

conference.pdf: conference.tex $(REQFILES)
	latexmk \
		-pdf \
		-pdflatex="lualatex --shell-escape --synctex=1 --interaction=nonstopmode" \
		-use-make conference.tex

standalone.pdf: standalone.tex $(REQFILES)
	latexmk \
		-pdf \
		-pdflatex="lualatex --shell-escape --synctex=1 --interaction=nonstopmode" \
		-use-make standalone.tex

clean-conference:
	latexmk -CA
	for file in conference.{nav,snm,synctex*,vrb}; \
	do \
		if [ -f $$file ]; \
		then \
			rm $$file; \
		fi; \
	done

clean-standalone:
	latexmk -CA
	for file in standalone.{nav,snm,synctex*,vrb}; \
	do \
		if [ -f $$file ]; \
		then \
			rm $$file; \
		fi; \
	done

cleanall: clean-conference clean-standalone
	for dir in _minted* svg-inkscape; \
	do \
		if [ -d $$dir ]; \
		then \
			rm -r $$dir; \
		fi; \
	done
