# Raghul_dilemna

Well hello there good people, my fellow githubbers, githubsies and geekhubbies

First of all, dilemma is spelt wrong - broski its DILEMMA not DILEMNA

A bit of context on this problem - this is an IB (high school) math question, worth 5 (out of 110) marks.
The creator spent 2 hours yesterday mulling it over - and still did not come up with a solution, resorting to having to use rust to code a simulation today. Bear in mind, this is meant to take 5-10 minutes, and the poor soul spent the whole night awake thinking about it.

Now, I am not an avid github user, I don't really know how to do anything, but wanted to post my solution which doesn't require any programming.

1) Out of 30 integers, 10 of them must be of the form 3n (so multiples of 3). 10 integers must be of the form (3n-1) - so the number before the multiple of 3. 10 integers must be of the form (3n-2) - so two numbers before the multiple of 3. The first multiple of 3 is 3.
2) For the sum of three numbers to be a multiple of 3, the three numbers must be of the same form or three of completely different forms:
   - Either 3n + 3n + 3n OR (3n-1) + (3n-1) + (3n-1) OR (3n-2) + (3n-2) + (3n-2)
   - The sum of 3n + (3n-1) + (3n-2)  
3) For each of the first options, we need to choose 3 numbers out of 10 possible integers using a combination. This yields 120 possible ways to select 3 out of 10 numbers of the form (3n), 120 possible ways to select 3 out of 10 numbers of the form (3n-1) and 120 possible ways to select 3 out of 10 numbers of the form (3n-2)
4) For the second option, there are 10 possible options for each of the 3n, (3n-1) and (3n-2) integers, so 10*10*10 = 1000 possible ways to add three numbers of different forms.

5) The total number of ways, and the answer to this problem, is therefore 1360

Badum tsss

