LATEX = lualatex --shell-escape --synctex=1 --interaction=nonstopmode

.PHONY: all

all: main.pdf

main.pdf: main.tex
	$(LATEX) main.tex

clean:
	rm main.aux main.log main.nav main.pdf main.snm main.synctex.gz main.toc main.vrb

cleanall: clean
	rm -r svg-inkscape _minted-main
