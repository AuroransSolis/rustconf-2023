.PHONY: all main.pdf clean cleanall

all: main.pdf

main.pdf: main.tex
	latexmk -pdf -pdflatex="lualatex --shell-escape --synctex=1 --interaction=nonstopmode" -use-make main.tex

clean:
	latexmk -CA
	for file in main.{nav,snm,vrb}; \
	do \
		if [ -f $$file ]; \
		then \
			rm $$file; \
		fi; \
	done

cleanall: clean
	for dir in _minted-main svg-inkscape; \
	do \
		if [ -d $$dir ]; \
		then \
			rm -r $$dir; \
		fi; \
	done
