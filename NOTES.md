# All my notes helping me to construct this projects

# Blocks
(without taking care of consensus and validation methods)  
Our blocks should contain:  
 - datas
 - previous block hash
 - timestamp

# Chain

It was pretty hard to understand and choose which type of structure could fit the best with my need.  
Had to choose between linkedList or hashmap

Using LinkedList makes sense when maintaining the same order of items and quick insertion time (adding and removing items at any position) is an important criterion.

HashMap is very efficient at checking if a key exists or retrieving a value based on a key. Those operations take O(1) on average. Using HashMap makes sense only when unique keys are available for the data we want to store. We should use it when searching for items based on a key and quick access time is an important requirement.

**After longs hours learning and understand theses types**   
I'll care about hashmap (i mean, basic hashmap, not linked)
I understood that it could be a good option to use linkedList if position order is a primary requierement. That's not the case, in fact, we don't care were is the block on the list because we will retrieve each block thanks to the previous block hash stored. The goal is to get all block when you have the last block so that we don't care at all of order. Moreover, hashmap gave us an way efficient search exploitation that is a real need. We don't care about removing items because we can't (immutability) and adding a block faster don't seems to be a priority. IMO

sources used:

- https://stackoverflow.com/questions/2592043/what-is-a-hash-map-in-programming-and-where-can-it-be-used
- https://stackoverflow.com/questions/730620/how-does-a-hash-table-work?rq=1
- https://stackoverflow.com/questions/7975802/when-to-use-hashmap-over-linkedlist-or-arraylist-and-vice-versa
- https://www.baeldung.com/java-arraylist-vs-linkedlist-vs-hashmap
- https://www.java67.com/2012/08/difference-between-hashmap-and-LinkedHashMap-Java.html
- https://www.javatpoint.com/linkedhashmap-vs-hashmap-in-java

# Common

https://cryptoast.fr/bloc-blockchain-crypto-explication/
