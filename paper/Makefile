INPUTNAME=main
PAPERNAME=paper
TEXFILES = $(wildcard *.tex) $(wildcard bibs/*.bib) $(wildcard sections/*.tex)
FIGURES = $(wildcard figures/*.tex) $(wildcard figures/*.tikz)
GRAPHICS = $(wildcard graphics/*.pdf) $(wildcard graphics/*.png)
LISTINGS = $(wildcard listings/*.tex)
BUILD=./build
LATEX_CMD=pdflatex
LATEX_FLAGS=-output-directory=$(BUILD) -aux-directory=$(BUILD) 
DEPENDENCIES = $(TEXFILES) $(FIGURES) $(GRAPHICS) $(LISTINGS)

# Default to compile when no specification is provided:
all: $(BUILD)/$(PAPERNAME).pdf

pdf: $(BUILD)/$(PAPERNAME).pdf

$(BUILD)/$(PAPERNAME).pdf: $(DEPENDENCIES)
	mkdir -p $(BUILD)
	pdflatex  $(LATEX_FLAGS) $(INPUTNAME).tex < /dev/null
	bibtex $(BUILD)/$(INPUTNAME).aux
	pdflatex $(LATEX_FLAGS)  $(INPUTNAME).tex < /dev/null
	pdflatex $(LATEX_FLAGS)  $(INPUTNAME).tex < /dev/null
	mv $(BUILD)/$(INPUTNAME).pdf $(BUILD)/$(PAPERNAME).pdf

tar:
	tar --ignore-failed-read -cvzf $(PAPERNAME).tgz $(DEPENDENCIES) Makefile

clean:
	rm -rf $(BUILD)

