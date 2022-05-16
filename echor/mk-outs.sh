#!/usr/bin/env fish

set OUTDIR "tests/expected"

# TODO: Does $OUTDIR need to be quoted here as it is suggested in, e.g. bash?
# (fish does not further expand variables-- does this then alleviate the need?
if test ! -d $OUTDIR 
	mkdir -p $OUTDIR
end

echo > $OUTDIR/hello0.txt
echo "Hello" > $OUTDIR/hello1.txt
echo "Hello there" > $OUTDIR/hello2.txt
echo "Hello  there" > $OUTDIR/hello3.txt
echo "Hello" "there" > $OUTDIR/hello4.txt

echo -n> $OUTDIR/hello0.n.txt
echo -n "Hello" > $OUTDIR/hello1.n.txt
echo -n "Hello there" > $OUTDIR/hello2.n.txt
echo -n "Hello  there" > $OUTDIR/hello3.n.txt
echo -n "Hello" "there" > $OUTDIR/hello4.n.txt

echo -s> $OUTDIR/hello0.s.txt
echo -s "Hello" > $OUTDIR/hello1.s.txt
echo -s "Hello there" > $OUTDIR/hello2.s.txt
echo -s "Hello  there" > $OUTDIR/hello3.s.txt
echo -s "Hello" "there" > $OUTDIR/hello4.s.txt

echo -n -s> $OUTDIR/hello0.n.s.txt
echo -n -s "Hello" > $OUTDIR/hello1.n.s.txt
echo -n -s "Hello there" > $OUTDIR/hello2.n.s.txt
echo -n -s "Hello  there" > $OUTDIR/hello3.n.s.txt
echo -n -s "Hello" "there" > $OUTDIR/hello4.n.s.txt

