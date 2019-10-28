SLIDES := $(patsubst %.md,%.slides.pdf,$(wildcard *.md))

all : $(SLIDES)

%.slides.pdf : %.md
	pandoc $^ -t beamer -o $@

presentation : presentation.slides.pdf
	xdg-open presentation.slides.pdf

clobber : 
	rm -f $(SLIDES)
