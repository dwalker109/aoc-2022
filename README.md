# Advent of Code 2022!

Rust, again. Maybe I will try to avoid crates.io this time?

## Update, 2023-11-18

Finished, and yes, I didn't use any deps this time. Maths was my enemy this year
but I learned a lot and had fun, just in time for 2023!

I will probably use deps next year though. I proved to myself I could do it without,
but it isn't how I would usually do it so I won't make life hard for myself. Plus
I got lucky this year - no rng or hash generation. 

## Notes

### Day 11

Part 2 required some maths knowledge with I don't posess - greatest common divisor. 
I needed to look at hints to progress.

### Day 16

Quite a challenge. I obtained star 2 trying a super stupid solution I really didn't
think would work, which I read about on the subreddit. It worked, but not
on the test data. 

I went back and rewrote this to use something more correct. It now completes in 
a *reasonable* time (5 secs) and works on test and real input. Happy enough to move on.

### Day 17

Completed part 2 with much frustration. I used lots of subreddit hints for this
day, but finally understand who the cycle detection and skipping works.

### Day 21

My initial solution was incredibly slow, since it involves loads of message passing 
and so on. My part 2 was semi-manually brute forced. Subreddit hints clued me
in on the maths involved (simple algebra, really) and I replaced the message
passing stuff with recursive calls which I'm ultimately pretty pleased with.

...**Time Passes**...

### Day 19

Back to complete AoC 2022 after about 11 months away. Day 19 was a challenge and 
I needed hints from the subreddit to learn the optimisation tricks (factory, not time)
to get a decent time. Lots of weirdness but eventually finished.

### Day 24

Made a right meal of this for some reason. Not especially difficult but it took me ages.

### Day 25

Enjoyed this a lot. I figured out we were converting between biased base10 (0..=9) 
and balanced base5 (-2 ..= 2) eventually, and since the subreddit suggestions didn't
help me massively (I don't like to just copy stuff I don't comprehend) I implemented
by own solution by following https://www.ias.ac.in/article/fulltext/reso/023/12/1395-1410
to learn the bias -> balanced base5 part. Super!
