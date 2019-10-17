# tRustyProbability

**A quick, but for myself surprisingly clean little program to answer a random question I thought up.**

Branch of tRustyProbability that uses rusts own internal random functions to shuffle the deck, allowing us many more iterations since the limiting factor otherwise is the bit quota of random.org (which is reasonable). Also faster this way since we don't need to wait for the network.

The question was:

*"On average how many draws do you need to make from a deck of cards to draw 4 aces, if upon drawing an ace you shuffle it back into the deck, but if you draw a card that isn't an ace you leave it out?"*
