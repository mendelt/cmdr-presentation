SLIDES := $(patsubst %.md,%.slides.pdf,$(wildcard *.md))

all : $(SLIDES)

%.slides.pdf : %.md
	pandoc $^ -t beamer -o $@

clobber : 
	rm -f $(SLIDES)
